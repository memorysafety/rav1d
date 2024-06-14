use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::bitdepth::BPC;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dPixelLayoutSubSampled;
use crate::include::dav1d::picture::Rav1dPictureDataComponent;
use crate::src::cpu::CpuFlags;
use crate::src::enum_map::enum_map;
use crate::src::enum_map::enum_map_ty;
use crate::src::enum_map::DefaultValue;
use crate::src::ffi_safe::FFISafe;
use crate::src::internal::SCRATCH_AC_TXTP_LEN;
use crate::src::internal::SCRATCH_EDGE_LEN;
use crate::src::levels::DC_128_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::HOR_PRED;
use crate::src::levels::LEFT_DC_PRED;
use crate::src::levels::N_IMPL_INTRA_PRED_MODES;
use crate::src::levels::PAETH_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::TOP_DC_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::levels::Z1_PRED;
use crate::src::levels::Z2_PRED;
use crate::src::levels::Z3_PRED;
use crate::src::tables::dav1d_dr_intra_derivative;
use crate::src::tables::dav1d_filter_intra_taps;
use crate::src::tables::dav1d_sm_weights;
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use std::cmp;
use std::mem;
use std::slice;
use strum::FromRepr;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
use crate::include::common::bitdepth::bpc_fn;

wrap_fn_ptr!(pub unsafe extern "C" fn angular_ipred(
    dst: *mut DynPixel,
    stride: isize,
    topleft: *const DynPixel,
    width: i32,
    height: i32,
    angle: i32,
    max_width: i32,
    max_height: i32,
    bitdepth_max: i32,
    topleft_off: usize,
) -> ());

impl angular_ipred::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *mut BD::Pixel,
        stride: isize,
        topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
        topleft_off: usize,
        width: i32,
        height: i32,
        angle: i32,
        max_width: i32,
        max_height: i32,
        bd: BD,
    ) {
        let dst = dst.cast();
        let topleft = topleft.as_ptr().add(topleft_off).cast();
        let bd = bd.into_c();
        self.get()(
            dst,
            stride,
            topleft,
            width,
            height,
            angle,
            max_width,
            max_height,
            bd,
            topleft_off,
        )
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn cfl_ac(
    ac: &mut [i16; SCRATCH_AC_TXTP_LEN],
    y_ptr: *const DynPixel,
    stride: isize,
    w_pad: i32,
    h_pad: i32,
    cw: i32,
    ch: i32,
    _y: *const FFISafe<Rav1dPictureDataComponent>,
) -> ());

impl cfl_ac::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        ac: &mut [i16; SCRATCH_AC_TXTP_LEN],
        y: &Rav1dPictureDataComponent,
        y_offset: usize,
        w_pad: i32,
        h_pad: i32,
        cw: i32,
        ch: i32,
    ) {
        let y_ptr = y.as_ptr_at::<BD>(y_offset).cast();
        let stride = y.stride();
        let y = FFISafe::new(y);
        // SAFETY: Fallback `fn cfl_ac_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(ac, y_ptr, stride, w_pad, h_pad, cw, ch, y) }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn cfl_pred(
    dst_ptr: *mut DynPixel,
    stride: isize,
    topleft: *const DynPixel,
    width: i32,
    height: i32,
    ac: &[i16; SCRATCH_AC_TXTP_LEN],
    alpha: i32,
    bitdepth_max: i32,
    _topleft_off: usize,
    _dst: *const FFISafe<Rav1dPictureDataComponent>,
) -> ());

impl cfl_pred::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: &Rav1dPictureDataComponent,
        dst_offset: usize,
        topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
        topleft_off: usize,
        width: i32,
        height: i32,
        ac: &[i16; SCRATCH_AC_TXTP_LEN],
        alpha: i32,
        bd: BD,
    ) {
        let dst_ptr = dst.as_mut_ptr_at::<BD>(dst_offset).cast();
        let stride = dst.stride();
        let topleft = topleft[topleft_off..].as_ptr().cast();
        let bd = bd.into_c();
        let dst = FFISafe::new(dst);
        // SAFETY: Fallback `fn cfl_pred` is safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                dst_ptr,
                stride,
                topleft,
                width,
                height,
                ac,
                alpha,
                bd,
                topleft_off,
                dst,
            )
        }
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn pal_pred(
    dst_ptr: *mut DynPixel,
    stride: isize,
    pal: *const [DynPixel; 8],
    idx: *const u8,
    w: i32,
    h: i32,
    _dst: *const FFISafe<Rav1dPictureDataComponent>,
) -> ());

impl pal_pred::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: &Rav1dPictureDataComponent,
        dst_offset: usize,
        pal: &[BD::Pixel; 8],
        idx: &[u8],
        w: i32,
        h: i32,
    ) {
        // SAFETY: `DisjointMut` is unchecked for asm `fn`s,
        // but passed through as an extra arg for the fallback `fn`.
        let dst_ptr = dst.as_mut_ptr_at::<BD>(dst_offset).cast();
        let stride = dst.stride();
        let pal = pal.as_ptr().cast();
        let idx = idx[..(w * h) as usize / 2].as_ptr();
        let dst = FFISafe::new(dst);
        // SAFETY: Fallback `fn pal_pred_rust` is safe; asm is supposed to do the same.
        unsafe { self.get()(dst_ptr, stride, pal, idx, w, h, dst) }
    }
}

pub struct Rav1dIntraPredDSPContext {
    pub intra_pred: [angular_ipred::Fn; 14],
    pub cfl_ac: enum_map_ty!(Rav1dPixelLayoutSubSampled, cfl_ac::Fn),
    pub cfl_pred: [cfl_pred::Fn; 6],
    pub pal_pred: pal_pred::Fn,
}

#[inline(never)]
unsafe fn splat_dc<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: isize,
    width: i32,
    height: i32,
    dc: i32,
    bd: BD,
) {
    let stride = BD::pxstride(stride);
    let width = width as usize;
    match BD::BPC {
        BPC::BPC8 => {
            assert!(dc <= 0xff);
            if width > 4 {
                let dcN = dc as u64 * 0x101010101010101;
                for _ in 0..height {
                    let slice =
                        slice::from_raw_parts_mut(dst.cast::<u64>(), width / mem::size_of::<u64>());
                    slice.fill(dcN);
                    dst = dst.offset(stride);
                }
            } else {
                let dcN = dc as u32 * 0x1010101;
                for _ in 0..height {
                    let slice =
                        slice::from_raw_parts_mut(dst.cast::<u32>(), width / mem::size_of::<u32>());
                    slice.fill(dcN);
                    dst = dst.offset(stride);
                }
            };
        }
        BPC::BPC16 => {
            assert!(dc <= bd.bitdepth_max().as_::<i32>());
            let dcN = dc as u64 * 0x1000100010001;
            for _ in 0..height {
                let slice = slice::from_raw_parts_mut(
                    dst.cast::<u64>(),
                    width / (mem::size_of::<u64>() >> 1),
                );
                slice.fill(dcN);
                dst = dst.offset(stride);
            }
        }
    }
}

#[inline(never)]
fn cfl_pred<BD: BitDepth>(
    dst: &Rav1dPictureDataComponent,
    mut dst_offset: usize,
    width: i32,
    height: i32,
    dc: i32,
    ac: &[i16; SCRATCH_AC_TXTP_LEN],
    alpha: i32,
    bd: BD,
) {
    let width = width as usize;
    let mut ac = &ac[..width * height as usize];
    for _ in 0..height {
        let slice = &mut *dst.slice_mut::<BD, _>((dst_offset.., ..width));
        for (x, dst) in slice.iter_mut().enumerate() {
            let diff = alpha * ac[x] as i32;
            *dst = bd.iclip_pixel(dc + apply_sign(diff.abs() + 32 >> 6, diff));
        }
        ac = &ac[width..];
        dst_offset = dst_offset.wrapping_add_signed(dst.pixel_stride::<BD>());
    }
}

fn dc_gen_top<BD: BitDepth>(
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    offset: usize,
    width: i32,
) -> u32 {
    let mut dc = width as u32 >> 1;
    for i in 0..width as usize {
        dc += topleft[offset + 1 + i].as_::<u32>();
    }
    return dc >> width.trailing_zeros();
}

fn dc_gen_left<BD: BitDepth>(
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    offset: usize,
    height: i32,
) -> u32 {
    let mut dc = height as u32 >> 1;
    for i in 0..height as usize {
        dc += topleft[offset - (1 + i)].as_::<u32>();
    }
    return dc >> height.trailing_zeros();
}

fn dc_gen<BD: BitDepth>(
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    offset: usize,
    width: i32,
    height: i32,
) -> u32 {
    let (multiplier_1x2, multiplier_1x4, base_shift) = match BD::BPC {
        BPC::BPC8 => (0x5556, 0x3334, 16),
        BPC::BPC16 => (0xAAAB, 0x6667, 17),
    };

    let mut dc = (width + height >> 1) as u32;
    for i in 0..width as usize {
        dc += topleft[offset + i + 1].as_::<u32>();
    }
    for i in 0..height as usize {
        dc += topleft[offset - (i + 1)].as_::<u32>();
    }
    dc >>= (width + height).trailing_zeros();

    if width != height {
        dc *= if width > height * 2 || height > width * 2 {
            multiplier_1x4
        } else {
            multiplier_1x2
        };
        dc >>= base_shift;
    }
    return dc;
}

#[derive(FromRepr)]
#[repr(u8)]
enum DcGen {
    Top,
    Left,
    TopLeft,
}

impl DcGen {
    fn call<BD: BitDepth>(
        &self,
        topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
        offset: usize,
        width: i32,
        height: i32,
    ) -> u32 {
        match self {
            Self::Top => dc_gen_top::<BD>(topleft, offset, width),
            Self::Left => dc_gen_left::<BD>(topleft, offset, height),
            Self::TopLeft => dc_gen::<BD>(topleft, offset, width, height),
        }
    }
}

/// Reconstructs the reference to the topleft edge array from a pointer into the
/// array and an offset from the start of the array.
///
/// The topleft pointer passed to asm is always a pointer into a buffer of
/// length [`SCRATCH_EDGE_LEN`]. For the Rust fallbacks we also pass in the
/// offset from the front of the buffer so that we can reconstruct the original
/// array reference in order to use safe array operations within the fallbacks.
///
/// # Safety
///
/// `topleft_ptr` must be a pointer into an array of length [`SCRATCH_EDGE_LEN`]
/// and is `topleft_off` elements from the beginning of the array. This should
/// be guaranteed by the logic in `angular_ipred::call`.
unsafe fn reconstruct_topleft<'a, BD: BitDepth>(
    topleft_ptr: *const DynPixel,
    topleft_off: usize,
) -> &'a [BD::Pixel; SCRATCH_EDGE_LEN] {
    &*topleft_ptr
        .cast::<BD::Pixel>()
        .sub(topleft_off)
        .cast::<[BD::Pixel; SCRATCH_EDGE_LEN]>()
}

unsafe extern "C" fn ipred_dc_c_erased<BD: BitDepth, const DC_GEN: u8>(
    dst: *mut DynPixel,
    stride: isize,
    topleft: *const DynPixel,
    width: i32,
    height: i32,
    _a: i32,
    _max_width: i32,
    _max_height: i32,
    bitdepth_max: i32,
    topleft_off: usize,
) {
    let dc_gen = DcGen::from_repr(DC_GEN).unwrap();
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    splat_dc(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen.call::<BD>(topleft, topleft_off, width, height) as i32,
        BD::from_c(bitdepth_max),
    );
}

/// # Safety
///
/// Must be called by [`cfl_pred::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn ipred_cfl_c_erased<BD: BitDepth, const DC_GEN: u8>(
    dst_ptr: *mut DynPixel,
    _stride: isize,
    topleft: *const DynPixel,
    width: i32,
    height: i32,
    ac: &[i16; SCRATCH_AC_TXTP_LEN],
    alpha: i32,
    bitdepth_max: i32,
    topleft_off: usize,
    dst: *const FFISafe<Rav1dPictureDataComponent>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `cfl_pred::Fn::call`.
    let dst = unsafe { FFISafe::get(dst) };
    // SAFETY: Reverse of what was done in `fn cfl_pred::Fn::call`.
    let dst_offset =
        unsafe { dst_ptr.cast::<BD::Pixel>().offset_from(dst.as_ptr::<BD>()) } as usize;
    let dc_gen = DcGen::from_repr(DC_GEN).unwrap();
    // SAFETY: `fn cfl_pred::Fn::call` makes `topleft` `topleft_off` from the beginning of the array.
    let topleft = unsafe { reconstruct_topleft::<BD>(topleft, topleft_off) };
    let dc: u32 = dc_gen.call::<BD>(topleft, topleft_off, width, height);
    cfl_pred(
        dst,
        dst_offset,
        width,
        height,
        dc as i32,
        ac,
        alpha,
        BD::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn ipred_dc_128_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: isize,
    _topleft: *const DynPixel,
    width: i32,
    height: i32,
    _a: i32,
    _max_width: i32,
    _max_height: i32,
    bitdepth_max: i32,
    _topleft_off: usize,
) {
    let bd = BD::from_c(bitdepth_max);
    let dc = bd.bitdepth_max().as_::<i32>() + 1 >> 1;
    splat_dc(dst.cast(), stride, width, height, dc, bd);
}

/// # Safety
///
/// Must be called by [`cfl_pred::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn ipred_cfl_128_c_erased<BD: BitDepth>(
    dst_ptr: *mut DynPixel,
    _stride: isize,
    _topleft: *const DynPixel,
    width: i32,
    height: i32,
    ac: &[i16; SCRATCH_AC_TXTP_LEN],
    alpha: i32,
    bitdepth_max: i32,
    _topleft_off: usize,
    dst: *const FFISafe<Rav1dPictureDataComponent>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `cfl_pred::Fn::call`.
    let dst = unsafe { FFISafe::get(dst) };
    // SAFETY: Reverse of what was done in `fn cfl_pred::Fn::call`.
    let dst_offset =
        unsafe { dst_ptr.cast::<BD::Pixel>().offset_from(dst.as_ptr::<BD>()) } as usize;
    let bd = BD::from_c(bitdepth_max);
    let dc = bd.bitdepth_max().as_::<i32>() + 1 >> 1;
    cfl_pred(dst, dst_offset, width, height, dc, ac, alpha, bd);
}

unsafe fn ipred_v_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: isize,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: i32,
    height: i32,
) {
    let width = width as usize;

    for _ in 0..height {
        BD::pixel_copy(
            slice::from_raw_parts_mut(dst, width),
            &topleft[topleft_off + 1..][..width],
            width,
        );
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe extern "C" fn ipred_v_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: isize,
    topleft: *const DynPixel,
    width: i32,
    height: i32,
    _a: i32,
    _max_width: i32,
    _max_height: i32,
    _bitdepth_max: i32,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_v_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_h_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: isize,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: i32,
    height: i32,
) {
    let width = width as usize;

    for y in 0..height as usize {
        BD::pixel_set(
            slice::from_raw_parts_mut(dst, width),
            topleft[topleft_off - (1 + y)],
            width,
        );
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe extern "C" fn ipred_h_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: isize,
    topleft: *const DynPixel,
    width: i32,
    height: i32,
    _a: i32,
    _max_width: i32,
    _max_height: i32,
    _bitdepth_max: i32,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_h_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_paeth_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: isize,
    tl: &[BD::Pixel; SCRATCH_EDGE_LEN],
    tl_off: usize,
    width: i32,
    height: i32,
) {
    let topleft = tl[tl_off].as_::<i32>();
    for y in 0..height as usize {
        let left = tl[tl_off - (y + 1)].as_::<i32>();
        let dst_slice = slice::from_raw_parts_mut(dst, width as usize);
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let top = tl[tl_off + 1 + x].as_::<i32>();
            let base = left + top - topleft;
            let ldiff = (left - base).abs();
            let tdiff = (top - base).abs();
            let tldiff = (topleft - base).abs();

            *dst = (if ldiff <= tdiff && ldiff <= tldiff {
                left
            } else if tdiff <= tldiff {
                top
            } else {
                topleft
            })
            .as_::<BD::Pixel>();
        }
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe extern "C" fn ipred_paeth_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: isize,
    tl_ptr: *const DynPixel,
    width: i32,
    height: i32,
    _a: i32,
    _max_width: i32,
    _max_height: i32,
    _bitdepth_max: i32,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(tl_ptr, topleft_off);
    ipred_paeth_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_smooth_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: isize,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: i32,
    height: i32,
) {
    let [width, height] = [width, height].map(|it| it as usize);

    let weights_hor = &dav1d_sm_weights.0[width..][..width];
    let weights_ver = &dav1d_sm_weights.0[height..][..height];
    let right = topleft[topleft_off + width].as_::<i32>();
    let bottom = topleft[topleft_off - height].as_::<i32>();

    for y in 0..height {
        let dst_slice = slice::from_raw_parts_mut(dst, width);
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let pred = weights_ver[y] as i32 * topleft[topleft_off + 1 + x].as_::<i32>()
                + (256 - weights_ver[y] as i32) * bottom
                + weights_hor[x] as i32 * topleft[topleft_off - (1 + y)].as_::<i32>()
                + (256 - weights_hor[x] as i32) * right;
            *dst = (pred + 256 >> 9).as_::<BD::Pixel>();
        }
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe extern "C" fn ipred_smooth_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: isize,
    topleft: *const DynPixel,
    width: i32,
    height: i32,
    _a: i32,
    _max_width: i32,
    _max_height: i32,
    _bitdepth_max: i32,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_smooth_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_smooth_v_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: isize,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: i32,
    height: i32,
) {
    let [width, height] = [width, height].map(|it| it as usize);

    let weights_ver = &dav1d_sm_weights.0[height..][..height];
    let bottom = topleft[topleft_off - height].as_::<i32>();

    for y in 0..height {
        let dst_slice = slice::from_raw_parts_mut(dst, width);
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let pred = weights_ver[y] as i32 * topleft[topleft_off + 1 + x].as_::<i32>()
                + (256 - weights_ver[y] as i32) * bottom;
            *dst = (pred + 128 >> 8).as_::<BD::Pixel>();
        }
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe extern "C" fn ipred_smooth_v_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: isize,
    topleft: *const DynPixel,
    width: i32,
    height: i32,
    _a: i32,
    _max_width: i32,
    _max_height: i32,
    _bitdepth_max: i32,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_smooth_v_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_smooth_h_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: isize,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: i32,
    height: i32,
) {
    let [width, height] = [width, height].map(|it| it as usize);

    let weights_hor = &dav1d_sm_weights.0[width..][..width];
    let right = topleft[topleft_off + width].as_::<i32>();

    for y in 0..height {
        let dst_slice = slice::from_raw_parts_mut(dst, width);
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let pred = weights_hor[x] as i32 * topleft[topleft_off - (y + 1)].as_::<i32>()
                + (256 - weights_hor[x] as i32) * right;
            *dst = (pred + 128 >> 8).as_::<BD::Pixel>();
        }
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe extern "C" fn ipred_smooth_h_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: isize,
    topleft: *const DynPixel,
    width: i32,
    height: i32,
    _a: i32,
    _max_width: i32,
    _max_height: i32,
    _bitdepth_max: i32,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_smooth_h_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

#[inline(never)]
fn get_filter_strength(wh: i32, angle: i32, is_sm: bool) -> i32 {
    if is_sm {
        match (wh, angle) {
            (..=8, 64..) => 2,
            (..=8, 40..) => 1,
            (..=8, ..) => 0,
            (..=16, 48..) => 2,
            (..=16, 20..) => 1,
            (..=16, ..) => 0,
            (..=24, 4..) => 3,
            (..=24, ..) => 0,
            (.., _) => 3,
        }
    } else {
        match (wh, angle) {
            (..=8, 56..) => 1,
            (..=8, ..) => 0,
            (..=16, 40..) => 1,
            (..=16, ..) => 0,
            (..=24, 32..) => 3,
            (..=24, 16..) => 2,
            (..=24, 8..) => 1,
            (..=24, ..) => 0,
            (..=32, 32..) => 3,
            (..=32, 4..) => 2,
            (..=32, ..) => 1,
            (.., _) => 3,
        }
    }
}

#[inline(never)]
fn filter_edge<BD: BitDepth>(
    out: &mut [BD::Pixel],
    sz: i32,
    lim_from: i32,
    lim_to: i32,
    r#in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    in_off: usize,
    from: i32,
    to: i32,
    strength: i32,
) {
    static kernel: [[u8; 5]; 3] = [[0, 4, 8, 4, 0], [0, 5, 6, 5, 0], [2, 4, 4, 4, 2]];

    assert!(strength > 0);
    let mut i = 0;
    while i < cmp::min(sz, lim_from) {
        out[i as usize] = r#in[in_off + iclip(i, from, to - 1) as usize];
        i += 1;
    }
    while i < cmp::min(lim_to, sz) {
        let mut s = 0;
        for j in 0..5 {
            s += r#in[in_off.wrapping_add_signed(iclip(i - 2 + j, from, to - 1) as isize)]
                .as_::<i32>()
                * kernel[(strength - 1) as usize][j as usize] as i32;
        }
        out[i as usize] = (s + 8 >> 4).as_::<BD::Pixel>();
        i += 1;
    }
    while i < sz {
        out[i as usize] = r#in[in_off + iclip(i, from, to - 1) as usize];
        i += 1;
    }
}

#[inline]
fn get_upsample(wh: i32, angle: i32, is_sm: bool) -> bool {
    angle < 40 && wh <= (16 >> is_sm as u8)
}

#[inline(never)]
fn upsample_edge<BD: BitDepth>(
    out: &mut [BD::Pixel],
    hsz: i32,
    r#in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    in_off: usize,
    from: i32,
    to: i32,
    bd: BD,
) {
    static kernel: [i8; 4] = [-1, 9, 9, -1];
    for i in 0..hsz - 1 {
        out[(i * 2) as usize] = r#in[in_off + iclip(i, from, to - 1) as usize];
        let mut s = 0;
        for j in 0..4 {
            s += r#in[in_off.wrapping_add_signed(iclip(i + j - 1, from, to - 1) as isize)]
                .as_::<i32>()
                * kernel[j as usize] as i32;
        }
        out[(i * 2 + 1) as usize] =
            iclip(s + 8 >> 4, 0, bd.bitdepth_max().as_::<i32>()).as_::<BD::Pixel>();
    }
    let i = hsz - 1;
    out[(i * 2) as usize] = r#in[in_off + iclip(i, from, to - 1) as usize];
}

unsafe fn ipred_z1_rust<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: isize,
    topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_in_off: usize,
    width: i32,
    height: i32,
    mut angle: i32,
    _max_width: i32,
    _max_height: i32,
    bd: BD,
) {
    let is_sm = (angle >> 9) & 1 != 0;
    let enable_intra_edge_filter = (angle >> 10) != 0;
    angle &= 511;
    assert!(angle < 90);
    let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as i32;
    let mut top_out = [0.into(); 64 + 64];
    let upsample_above = if enable_intra_edge_filter {
        get_upsample(width + height, 90 - angle, is_sm)
    } else {
        false
    };
    let (top, max_base_x) = if upsample_above {
        upsample_edge::<BD>(
            &mut top_out,
            width + height,
            topleft_in,
            topleft_in_off + 1,
            -1,
            width + cmp::min(width, height),
            bd,
        );
        dx <<= 1;

        (top_out.as_slice(), 2 * (width + height) - 2)
    } else {
        let filter_strength = if enable_intra_edge_filter {
            get_filter_strength(width + height, 90 - angle, is_sm)
        } else {
            0
        };
        if filter_strength != 0 {
            filter_edge::<BD>(
                &mut top_out,
                width + height,
                0,
                width + height,
                topleft_in,
                topleft_in_off + 1,
                -1,
                width + cmp::min(width, height),
                filter_strength,
            );
            (top_out.as_slice(), width + height - 1)
        } else {
            (
                &topleft_in[topleft_in_off + 1..],
                width + cmp::min(width, height) - 1,
            )
        }
    };
    let width = width as usize;
    let max_base_x = max_base_x as usize;
    let base_inc = 1 + upsample_above as usize;
    for y in 0..height {
        let xpos = (y + 1) * dx;
        let frac = xpos & 0x3e;

        let dst_slice =
            slice::from_raw_parts_mut(dst.offset(BD::pxstride(stride) * y as isize), width);
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let base = (xpos >> 6) as usize + base_inc * x;
            if base < max_base_x {
                let v = top[base].as_::<i32>() * (64 - frac) + top[base + 1].as_::<i32>() * frac;
                *dst = (v + 32 >> 6).as_::<BD::Pixel>();
            } else {
                BD::pixel_set(&mut dst_slice[x..], top[max_base_x], width - x);
                break;
            }
        }
    }
}

unsafe fn ipred_z2_rust<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: isize,
    topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_in_off: usize,
    width: i32,
    height: i32,
    mut angle: i32,
    max_width: i32,
    max_height: i32,
    bd: BD,
) {
    let is_sm = (angle >> 9) & 1 != 0;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511;
    assert!(angle > 90 && angle < 180);
    let mut dy = dav1d_dr_intra_derivative[(angle - 90 >> 1) as usize] as i32;
    let mut dx = dav1d_dr_intra_derivative[(180 - angle >> 1) as usize] as i32;
    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 180 - angle, is_sm)
    } else {
        false
    };
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 90, is_sm)
    } else {
        false
    };
    let mut edge = [0.into(); 64 + 64 + 1];
    let topleft = 64;

    if upsample_above {
        upsample_edge::<BD>(
            &mut edge[topleft..],
            width + 1,
            topleft_in,
            topleft_in_off,
            0,
            width + 1,
            bd,
        );
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90, is_sm)
        } else {
            0
        };
        if filter_strength != 0 {
            filter_edge::<BD>(
                &mut edge[topleft + 1..],
                width,
                0,
                max_width,
                topleft_in,
                topleft_in_off + 1,
                -1,
                width,
                filter_strength,
            );
        } else {
            let width = width as usize;
            BD::pixel_copy(
                &mut edge[topleft + 1..][..width],
                &topleft_in[topleft_in_off + 1..][..width],
                width,
            );
        }
    }
    if upsample_left {
        upsample_edge::<BD>(
            &mut edge[topleft - height as usize * 2..],
            height + 1,
            topleft_in,
            topleft_in_off - height as usize,
            0,
            height + 1,
            bd,
        );
        dy <<= 1;
    } else {
        let filter_strength_0 = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180 - angle, is_sm)
        } else {
            0
        };
        if filter_strength_0 != 0 {
            filter_edge::<BD>(
                &mut edge[topleft - height as usize..],
                height,
                height - max_height,
                height,
                topleft_in,
                topleft_in_off - height as usize,
                0,
                height + 1,
                filter_strength_0,
            );
        } else {
            let height = height as usize;
            BD::pixel_copy(
                &mut edge[topleft - height..][..height],
                &topleft_in[topleft_in_off - height..][..height],
                height,
            );
        }
    }
    edge[topleft] = topleft_in[topleft_in_off];

    let base_inc_x = 1 + upsample_above as usize;
    let left = topleft - (1 + upsample_left as usize);
    for y in 0..height {
        let xpos = (1 + (upsample_above as i32) << 6) - (dx * (y + 1));
        let base_x = xpos >> 6;
        let frac_x = xpos & 0x3e;

        let dst_slice = slice::from_raw_parts_mut(
            dst.offset(BD::pxstride(stride) * y as isize),
            width as usize,
        );
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let ypos = (y << 6 + upsample_left as i32) - (dy * (x + 1) as i32);
            let base_x = base_x + (base_inc_x * x) as i32;
            let v = if base_x >= 0 {
                edge[topleft + base_x as usize].as_::<i32>() * (64 - frac_x)
                    + edge[topleft + base_x as usize + 1].as_::<i32>() * frac_x
            } else {
                let base_y = ypos >> 6;
                assert!(base_y >= -(1 + upsample_left as i32));
                let frac_y = ypos & 0x3e;
                edge[left.wrapping_add_signed(-base_y as isize)].as_::<i32>() * (64 - frac_y)
                    + edge[left.wrapping_add_signed(-(base_y + 1) as isize)].as_::<i32>() * frac_y
            };
            *dst = (v + 32 >> 6).as_::<BD::Pixel>();
        }
    }
}

unsafe fn ipred_z3_rust<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: isize,
    topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_in_off: usize,
    width: i32,
    height: i32,
    mut angle: i32,
    _max_width: i32,
    _max_height: i32,
    bd: BD,
) {
    let stride = BD::pxstride(stride);
    let is_sm = (angle >> 9) & 1 != 0;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511;
    assert!(angle > 180);
    let mut dy = dav1d_dr_intra_derivative[(270 - angle >> 1) as usize] as i32;
    let mut left_out = [0.into(); 64 + 64];
    let left;
    let left_off;
    let max_base_y;
    let upsample_left = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, angle - 180, is_sm)
    } else {
        false
    };
    if upsample_left {
        upsample_edge::<BD>(
            &mut left_out,
            width + height,
            topleft_in,
            topleft_in_off - (width + height) as usize,
            cmp::max(width - height, 0),
            width + height + 1,
            bd,
        );
        left = left_out.as_slice();
        left_off = 2 * (width + height) as usize - 2;
        max_base_y = 2 * (width + height) - 2;
        dy <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180, is_sm)
        } else {
            0
        };

        if filter_strength != 0 {
            filter_edge::<BD>(
                &mut left_out,
                width + height,
                0,
                width + height,
                topleft_in,
                topleft_in_off - (width + height) as usize,
                cmp::max(width - height, 0),
                width + height + 1,
                filter_strength,
            );
            left = left_out.as_slice();
            left_off = (width + height - 1) as usize;
            max_base_y = width + height - 1;
        } else {
            left = topleft_in.as_slice();
            left_off = topleft_in_off - 1;
            max_base_y = height + cmp::min(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_left as i32;
    for x in 0..width {
        let ypos = dy * (x + 1);
        let frac = ypos & 0x3e;

        for y in 0..height {
            let base = (ypos >> 6) + base_inc * y;
            if base < max_base_y {
                let v = left[left_off.wrapping_add_signed(-base as isize)].as_::<i32>()
                    * (64 - frac)
                    + left[left_off.wrapping_add_signed(-(base + 1) as isize)].as_::<i32>() * frac;
                *dst.offset(y as isize * stride + x as isize) = (v + 32 >> 6).as_::<BD::Pixel>();
            } else {
                for y in y..height {
                    *dst.offset(y as isize * stride + x as isize) =
                        left[left_off.wrapping_add_signed(-max_base_y as isize)];
                }
                break;
            }
        }
    }
}

unsafe extern "C" fn ipred_z_c_erased<BD: BitDepth, const Z: usize>(
    dst: *mut DynPixel,
    stride: isize,
    topleft_in: *const DynPixel,
    width: i32,
    height: i32,
    angle: i32,
    max_width: i32,
    max_height: i32,
    bitdepth_max: i32,
    topleft_off: usize,
) {
    let topleft_in = reconstruct_topleft::<BD>(topleft_in, topleft_off);
    [ipred_z1_rust, ipred_z2_rust, ipred_z3_rust][Z - 1](
        dst.cast(),
        stride,
        topleft_in,
        topleft_off,
        width,
        height,
        angle,
        max_width,
        max_height,
        BD::from_c(bitdepth_max),
    )
}

fn filter_fn(flt_ptr: &[i8], p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i32 {
    let flt_ptr = &flt_ptr[..48 + 1];
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        flt_ptr[0] as i32 * p0
            + flt_ptr[1] as i32 * p1
            + flt_ptr[16] as i32 * p2
            + flt_ptr[17] as i32 * p3
            + flt_ptr[32] as i32 * p4
            + flt_ptr[33] as i32 * p5
            + flt_ptr[48] as i32 * p6
    } else {
        flt_ptr[0] as i32 * p0
            + flt_ptr[8] as i32 * p1
            + flt_ptr[16] as i32 * p2
            + flt_ptr[24] as i32 * p3
            + flt_ptr[32] as i32 * p4
            + flt_ptr[40] as i32 * p5
            + flt_ptr[48] as i32 * p6
    }
}

const FLT_INCR: usize = if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
    2
} else {
    1
};

unsafe fn ipred_filter_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: isize,
    topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: i32,
    height: i32,
    filt_idx: i32,
    _max_width: i32,
    _max_height: i32,
    bd: BD,
) {
    let width = width as usize;
    let height = height as usize;
    let stride = BD::pxstride(stride);
    let filt_idx = filt_idx & 511;
    assert!(filt_idx < 5);

    let filter = &dav1d_filter_intra_taps[filt_idx as usize];
    let mut top = topleft_in[topleft_off + 1..].as_ptr();
    for y in (0..height).step_by(2) {
        let mut topleft = topleft_in.as_ptr().add(topleft_off - y);
        let mut left = topleft.sub(1);
        let mut left_stride = -1;
        for x in (0..width).step_by(4) {
            let top_slice = slice::from_raw_parts(top, 4);
            let p0 = (*topleft).as_::<i32>();
            let p1 = top_slice[0].as_::<i32>();
            let p2 = top_slice[1].as_::<i32>();
            let p3 = top_slice[2].as_::<i32>();
            let p4 = top_slice[3].as_::<i32>();
            let p5 = (*left.offset(0 * left_stride)).as_::<i32>();
            let p6 = (*left.offset(1 * left_stride)).as_::<i32>();
            let mut ptr = dst.add(x);
            let mut flt_ptr = filter.as_slice();

            for _yy in 0..2 {
                let ptr_slice = slice::from_raw_parts_mut(ptr, 4);
                for xx in ptr_slice {
                    let acc = filter_fn(flt_ptr, p0, p1, p2, p3, p4, p5, p6);
                    *xx = bd.iclip_pixel(acc + 8 >> 4);
                    flt_ptr = &flt_ptr[FLT_INCR..];
                }
                ptr = ptr.offset(stride);
            }
            left = dst.add(x + 4 - 1);
            left_stride = stride;
            top = top.offset(4);
            topleft = top.sub(1);
        }
        top = dst.offset(stride);
        dst = dst.offset(stride * 2);
    }
}

unsafe extern "C" fn ipred_filter_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: isize,
    topleft_in: *const DynPixel,
    width: i32,
    height: i32,
    filt_idx: i32,
    max_width: i32,
    max_height: i32,
    bitdepth_max: i32,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft_in, topleft_off);
    ipred_filter_rust(
        dst.cast(),
        stride,
        topleft,
        topleft_off,
        width,
        height,
        filt_idx,
        max_width,
        max_height,
        BD::from_c(bitdepth_max),
    );
}

#[inline(never)]
fn cfl_ac_rust<BD: BitDepth>(
    ac: &mut [i16; SCRATCH_AC_TXTP_LEN],
    y_src: &Rav1dPictureDataComponent,
    mut y_src_offset: usize,
    w_pad: i32,
    h_pad: i32,
    width: usize,
    height: usize,
    is_ss_hor: bool,
    is_ss_ver: bool,
) {
    let ac = &mut ac[..width * height];
    let [w_pad, h_pad] = [w_pad, h_pad].map(|pad| usize::try_from(pad).unwrap() * 4);
    assert!(w_pad < width);
    assert!(h_pad < height);
    let [ss_hor, ss_ver] = [is_ss_hor, is_ss_ver].map(|is_ss| is_ss as u8);
    let y_pxstride = y_src.pixel_stride::<BD>();

    for y in 0..height - h_pad {
        let aci = y * width;
        let y_src = |i| (*y_src.index::<BD>(y_src_offset.wrapping_add_signed(i))).as_::<i32>();
        for x in 0..width - w_pad {
            let sx = (x << ss_hor) as isize;
            let mut ac_sum = y_src(sx);
            if is_ss_hor {
                ac_sum += y_src(sx + 1);
            }
            if is_ss_ver {
                ac_sum += y_src(sx + y_pxstride);
                if is_ss_hor {
                    ac_sum += y_src(sx + y_pxstride + 1);
                }
            }
            ac[aci + x] = (ac_sum << 1 + !is_ss_ver as u8 + !is_ss_hor as u8) as i16;
        }
        for x in width - w_pad..width {
            ac[aci + x] = ac[aci + x - 1];
        }
        y_src_offset = y_src_offset.wrapping_add_signed(y_pxstride << ss_ver);
    }
    for y in height - h_pad..height {
        let aci = y * width;
        let (src, dst) = ac.split_at_mut(aci);
        dst[..width].copy_from_slice(&src[src.len() - width..]);
    }

    let log2sz = width.trailing_zeros() + height.trailing_zeros();
    let mut sum = 1 << log2sz >> 1;
    for y in 0..height {
        let aci = y * width;
        for x in 0..width {
            sum += ac[aci + x] as i32;
        }
    }
    let sum = (sum >> log2sz) as i16;

    // subtract DC
    for y in 0..height {
        let aci = y * width;
        for x in 0..width {
            ac[aci + x] -= sum;
        }
    }
}

/// # Safety
///
/// Must be called by [`cfl_ac::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn cfl_ac_c_erased<BD: BitDepth, const IS_SS_HOR: bool, const IS_SS_VER: bool>(
    ac: &mut [i16; SCRATCH_AC_TXTP_LEN],
    y_ptr: *const DynPixel,
    _stride: isize,
    w_pad: i32,
    h_pad: i32,
    cw: i32,
    ch: i32,
    y: *const FFISafe<Rav1dPictureDataComponent>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `cfl_ac::Fn::call`.
    let y = unsafe { FFISafe::get(y) };
    // SAFETY: Reverse of what was done in `cfl_ac::Fn::call`.
    let y_offset = unsafe { y_ptr.cast::<BD::Pixel>().offset_from(y.as_ptr::<BD>()) } as usize;
    let cw = cw as usize;
    let ch = ch as usize;
    cfl_ac_rust::<BD>(ac, y, y_offset, w_pad, h_pad, cw, ch, IS_SS_HOR, IS_SS_VER);
}

fn pal_pred_rust<BD: BitDepth>(
    dst: &Rav1dPictureDataComponent,
    mut dst_offset: usize,
    pal: &[BD::Pixel; 8],
    idx: &[u8],
    w: i32,
    h: i32,
) {
    let w = w as usize;
    let h = h as usize;
    let idx = &idx[..w * h / 2];

    let mut j = 0;
    for _ in 0..h {
        for x in (0..w).step_by(2) {
            let i = idx[j];
            j += 1;
            assert!((i & 0x88) == 0);
            let dst = &mut *dst.slice_mut::<BD, _>((dst_offset + x.., ..2));
            dst[0] = pal[(i & 7) as usize];
            dst[1] = pal[(i >> 4) as usize];
        }
        dst_offset = dst_offset.wrapping_add_signed(dst.pixel_stride::<BD>());
    }
}

/// # Safety
///
/// Must be called by [`pal_pred::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn pal_pred_c_erased<BD: BitDepth>(
    dst_ptr: *mut DynPixel,
    _stride: isize,
    pal: *const [DynPixel; 8],
    idx: *const u8,
    w: i32,
    h: i32,
    dst: *const FFISafe<Rav1dPictureDataComponent>,
) {
    let dst_ptr = dst_ptr.cast::<BD::Pixel>();
    // SAFETY: Was passed as `FFISafe::new(dst)` in `pal_pred::Fn::call`.
    let dst = unsafe { FFISafe::get(dst) };
    // SAFETY: Reverse of what was done in `pal_pred::Fn::call`.
    let dst_offset = unsafe { dst_ptr.offset_from(dst.as_ptr::<BD>()) } as usize;
    // SAFETY: Undoing dyn cast in `pal_pred::Fn::call`.
    let pal = unsafe { &*pal.cast() };
    // SAFETY: Length sliced in `pal_pred::Fn::call`.
    let idx = unsafe { slice::from_raw_parts(idx, (w * h) as usize / 2) };
    pal_pred_rust::<BD>(dst, dst_offset, pal, idx, w, h)
}

#[cfg(all(feature = "asm", target_arch = "aarch64"))]
mod neon {
    use super::*;

    use to_method::To;

    #[cfg(feature = "bitdepth_8")]
    use crate::include::common::bitdepth::BitDepth8;

    #[cfg(feature = "bitdepth_16")]
    use crate::include::common::bitdepth::BitDepth16;

    wrap_fn_ptr!(unsafe extern "C" fn z13_fill(
        dst: *mut DynPixel,
        stride: isize,
        topleft: *const DynPixel,
        width: i32,
        height: i32,
        dxy: i32,
        max_base_xy: i32,
    ) -> ());

    impl z13_fill::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            dst: *mut BD::Pixel,
            stride: isize,
            topleft: &[BD::Pixel],
            width: i32,
            height: i32,
            dxy: i32,
            max_base_xy: i32,
        ) {
            let dst = dst.cast();
            let topleft = topleft.as_ptr().cast();
            self.get()(dst, stride, topleft, width, height, dxy, max_base_xy)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn z2_fill(
        dst: *mut DynPixel,
        stride: isize,
        top: *const DynPixel,
        left: *const DynPixel,
        width: i32,
        height: i32,
        dx: i32,
        dy: i32,
    ) -> ());

    impl z2_fill::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            dst: *mut BD::Pixel,
            stride: isize,
            top: &[BD::Pixel],
            left: &[BD::Pixel],
            width: i32,
            height: i32,
            dx: i32,
            dy: i32,
        ) {
            let dst = dst.cast();
            let top = top.as_ptr().cast();
            let left = left.as_ptr().cast();
            self.get()(dst, stride, top, left, width, height, dx, dy)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn z1_upsample_edge(
        out: *mut DynPixel,
        hsz: i32,
        in_0: *const DynPixel,
        end: i32,
        _bitdepth_max: i32,
    ) -> ());

    impl z1_upsample_edge::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            out: &mut [BD::Pixel],
            hsz: i32,
            in_0: &[BD::Pixel],
            end: i32,
            bd: BD,
        ) {
            let out = out.as_mut_ptr().cast();
            let in_0 = in_0.as_ptr().cast();
            let bd = bd.into_c();
            self.get()(out, hsz, in_0, end, bd)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn z1_filter_edge(
        out: *mut DynPixel,
        sz: i32,
        in_0: *const DynPixel,
        end: i32,
        strength: i32,
    ) -> ());

    impl z1_filter_edge::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            out: &mut [BD::Pixel],
            sz: i32,
            in_0: &[BD::Pixel],
            end: i32,
            strength: i32,
        ) {
            let out = out.as_mut_ptr().cast();
            let in_0 = in_0.as_ptr().cast();
            self.get()(out, sz, in_0, end, strength)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn z2_upsample_edge(
        out: *mut DynPixel,
        hsz: i32,
        in_0: *const DynPixel,
        _bitdepth_max: i32,
    ) -> ());

    impl z2_upsample_edge::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            out: &mut [BD::Pixel],
            hsz: i32,
            in_0: &[BD::Pixel],
            bd: BD,
        ) {
            let out = out.as_mut_ptr().cast();
            let in_0 = in_0.as_ptr().cast();
            let bd = bd.into_c();
            self.get()(out, hsz, in_0, bd)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn reverse(
        dst: *mut DynPixel,
        src: *const DynPixel,
        n: i32,
    ) -> ());

    impl reverse::Fn {
        pub unsafe fn call<BD: BitDepth>(&self, dst: &mut [BD::Pixel], src: &[BD::Pixel], n: i32) {
            let dst = dst.as_mut_ptr().cast();
            let src = src.as_ptr().cast();
            self.get()(dst, src, n)
        }
    }

    unsafe fn rav1d_ipred_pixel_set_neon<BD: BitDepth>(
        out: &mut [BD::Pixel],
        px: BD::Pixel,
        n: i32,
    ) {
        // `pixel_set` takes a `px: BD::Pixel`.
        // Since it's not behind a ptr, we can't make it a `DynPixel`
        // and call it uniformly with `bd_fn!`.

        extern "C" {
            #[cfg(feature = "bitdepth_8")]
            fn dav1d_ipred_pixel_set_8bpc_neon(
                out: *mut DynPixel,
                px: <BitDepth8 as BitDepth>::Pixel,
                n: i32,
            );

            #[cfg(feature = "bitdepth_16")]
            fn dav1d_ipred_pixel_set_16bpc_neon(
                out: *mut DynPixel,
                px: <BitDepth16 as BitDepth>::Pixel,
                n: i32,
            );
        }

        let out = out.as_mut_ptr().cast();
        match BD::BPC {
            BPC::BPC8 => dav1d_ipred_pixel_set_8bpc_neon(
                out,
                // Really a no-op cast, but it's difficult to do it properly with generics.
                px.to::<u16>() as <BitDepth8 as BitDepth>::Pixel,
                n,
            ),
            BPC::BPC16 => dav1d_ipred_pixel_set_16bpc_neon(out, px.into(), n),
        }
    }

    unsafe fn ipred_z1_neon<BD: BitDepth>(
        dst: *mut BD::Pixel,
        stride: isize,
        topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
        topleft_off: usize,
        width: i32,
        height: i32,
        mut angle: i32,
        _max_width: i32,
        _max_height: i32,
        bd: BD,
    ) {
        let topleft_in = &topleft_in[topleft_off..];
        let is_sm = (angle >> 9) & 1 != 0;
        let enable_intra_edge_filter = angle >> 10;
        angle &= 511;
        let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as i32;
        const TOP_OUT_SIZE: usize = 64 + 64 * (64 + 15) * 2 + 16;
        let mut top_out = [0.into(); TOP_OUT_SIZE];
        let max_base_x;
        let upsample_above = if enable_intra_edge_filter != 0 {
            get_upsample(width + height, 90 - angle, is_sm)
        } else {
            false
        };
        if upsample_above {
            bd_fn!(z1_upsample_edge::decl_fn, BD, ipred_z1_upsample_edge, neon).call(
                &mut top_out,
                width + height,
                topleft_in,
                width + cmp::min(width, height),
                bd,
            );
            max_base_x = 2 * (width + height) - 2;
            dx <<= 1;
        } else {
            let filter_strength = if enable_intra_edge_filter != 0 {
                get_filter_strength(width + height, 90 - angle, is_sm)
            } else {
                0
            };
            if filter_strength != 0 {
                bd_fn!(z1_filter_edge::decl_fn, BD, ipred_z1_filter_edge, neon).call::<BD>(
                    &mut top_out,
                    width + height,
                    topleft_in,
                    width + cmp::min(width, height),
                    filter_strength,
                );
                max_base_x = width + height - 1;
            } else {
                max_base_x = width + cmp::min(width, height) - 1;
                let len = max_base_x as usize + 1;
                top_out[..len].copy_from_slice(&topleft_in[1..][..len]);
            }
        }
        let base_inc = 1 + upsample_above as i32;
        let pad_pixels = width + 15;
        let px = top_out[max_base_x as usize];
        rav1d_ipred_pixel_set_neon::<BD>(
            &mut top_out[max_base_x as usize + 1..],
            px,
            pad_pixels * base_inc,
        );
        if upsample_above {
            bd_fn!(z13_fill::decl_fn, BD, ipred_z1_fill2, neon)
                .call::<BD>(dst, stride, &top_out, width, height, dx, max_base_x);
        } else {
            bd_fn!(z13_fill::decl_fn, BD, ipred_z1_fill1, neon)
                .call::<BD>(dst, stride, &top_out, width, height, dx, max_base_x);
        };
    }

    unsafe fn ipred_z2_neon<BD: BitDepth>(
        dst: *mut BD::Pixel,
        stride: isize,
        topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
        topleft_off: usize,
        width: i32,
        height: i32,
        mut angle: i32,
        max_width: i32,
        max_height: i32,
        bd: BD,
    ) {
        let topleft_in = &topleft_in[topleft_off..];
        let is_sm = (angle >> 9) & 1 != 0;
        let enable_intra_edge_filter = angle >> 10;
        angle &= 511;
        assert!(angle > 90 && angle < 180);
        let mut dy = dav1d_dr_intra_derivative[((angle - 90) >> 1) as usize] as i32;
        let mut dx = dav1d_dr_intra_derivative[((180 - angle) >> 1) as usize] as i32;
        let mut buf = [0.to::<BD::Pixel>(); 3 * (64 + 1)]; // NOTE: C code doesn't initialize

        // The asm can underread below the start of top[] and left[]; to avoid
        // surprising behaviour, make sure this is within the allocated stack space.
        let left_offset = 2 * (64 + 1);
        let top_offset = 1 * (64 + 1);
        let flipped_offset = 0 * (64 + 1);

        let upsample_left = if enable_intra_edge_filter != 0 {
            get_upsample(width + height, 180 - angle, is_sm)
        } else {
            false
        };
        let upsample_above = if enable_intra_edge_filter != 0 {
            get_upsample(width + height, angle - 90, is_sm)
        } else {
            false
        };

        if upsample_above {
            bd_fn!(z2_upsample_edge::decl_fn, BD, ipred_z2_upsample_edge, neon).call(
                &mut buf[top_offset..],
                width,
                topleft_in,
                bd,
            );
            dx <<= 1;
        } else {
            let filter_strength = if enable_intra_edge_filter != 0 {
                get_filter_strength(width + height, angle - 90, is_sm)
            } else {
                0
            };

            if filter_strength != 0 {
                bd_fn!(z1_filter_edge::decl_fn, BD, ipred_z1_filter_edge, neon).call::<BD>(
                    &mut buf[1 + top_offset..],
                    cmp::min(max_width, width),
                    topleft_in,
                    width,
                    filter_strength,
                );

                if max_width < width {
                    let len = (width - max_width) as usize;
                    buf[top_offset + 1 + max_width as usize..][..len]
                        .copy_from_slice(&topleft_in[1 + max_width as usize..][..len]);
                }
            } else {
                BD::pixel_copy(
                    &mut buf[1 + top_offset..],
                    &topleft_in[1..][..width as usize],
                    width as usize,
                );
            }
        }

        if upsample_left {
            buf[flipped_offset] = topleft_in[0];
            bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                &mut buf[1 + flipped_offset..],
                topleft_in,
                height,
            );
            let (src, dst) = buf.split_at_mut(left_offset);
            bd_fn!(z2_upsample_edge::decl_fn, BD, ipred_z2_upsample_edge, neon).call(
                dst,
                height,
                &src[flipped_offset..],
                bd,
            );
            dy <<= 1;
        } else {
            let filter_strength = if enable_intra_edge_filter != 0 {
                get_filter_strength(width + height, 180 - angle, is_sm)
            } else {
                0
            };
            if filter_strength != 0 {
                buf[flipped_offset] = topleft_in[0];
                bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                    &mut buf[1 + flipped_offset..],
                    topleft_in,
                    height,
                );
                let (src, dst) = buf.split_at_mut(1 + left_offset);
                bd_fn!(z1_filter_edge::decl_fn, BD, ipred_z1_filter_edge, neon).call::<BD>(
                    dst,
                    cmp::min(max_height, height),
                    &src[flipped_offset..],
                    height,
                    filter_strength,
                );
                if max_height < height {
                    let len = (height - max_height) as usize;
                    let (src, dst) = buf[1 + max_height as usize..].split_at_mut(left_offset);
                    dst[..len].copy_from_slice(&src[flipped_offset..][..len]);
                }
            } else {
                bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                    &mut buf[left_offset + 1..],
                    topleft_in,
                    height,
                );
            }
        }
        buf[top_offset] = topleft_in[0];
        buf[left_offset] = topleft_in[0];

        assert!(!(upsample_above && upsample_left));

        if !upsample_above && !upsample_left {
            bd_fn!(z2_fill::decl_fn, BD, ipred_z2_fill1, neon).call::<BD>(
                dst,
                stride,
                &buf[top_offset..],
                &buf[left_offset..],
                width,
                height,
                dx,
                dy,
            );
        } else if upsample_above {
            bd_fn!(z2_fill::decl_fn, BD, ipred_z2_fill2, neon).call::<BD>(
                dst,
                stride,
                &buf[top_offset..],
                &buf[left_offset..],
                width,
                height,
                dx,
                dy,
            );
        } else {
            bd_fn!(z2_fill::decl_fn, BD, ipred_z2_fill3, neon).call::<BD>(
                dst,
                stride,
                &buf[top_offset..],
                &buf[left_offset..],
                width,
                height,
                dx,
                dy,
            );
        };
    }

    unsafe fn ipred_z3_neon<BD: BitDepth>(
        dst: *mut BD::Pixel,
        stride: isize,
        topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
        topleft_off: usize,
        width: i32,
        height: i32,
        mut angle: i32,
        _max_width: i32,
        _max_height: i32,
        bd: BD,
    ) {
        let topleft_in = &topleft_in[topleft_off..];
        let is_sm = (angle >> 9) & 1 != 0;
        let enable_intra_edge_filter = angle >> 10;
        angle &= 511;
        assert!(angle > 180);
        let mut dy = dav1d_dr_intra_derivative[(270 - angle >> 1) as usize] as i32;
        let mut flipped = [0.into(); 64 + 64 + 16];
        let mut left_out = [0.into(); 64 + 64 + (64 + 15) * 2];
        let max_base_y;
        let upsample_left = if enable_intra_edge_filter != 0 {
            get_upsample(width + height, angle - 180, is_sm)
        } else {
            false
        };
        if upsample_left {
            flipped[0] = topleft_in[0];
            bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                &mut flipped[1..],
                topleft_in,
                height + cmp::max(width, height),
            );
            bd_fn!(z1_upsample_edge::decl_fn, BD, ipred_z1_upsample_edge, neon).call(
                &mut left_out,
                width + height,
                &flipped,
                height + cmp::min(width, height),
                bd,
            );
            max_base_y = 2 * (width + height) - 2;
            dy <<= 1;
        } else {
            let filter_strength = if enable_intra_edge_filter != 0 {
                get_filter_strength(width + height, angle - 180, is_sm)
            } else {
                0
            };
            if filter_strength != 0 {
                flipped[0] = topleft_in[0];
                bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                    &mut flipped[1..],
                    topleft_in,
                    height + cmp::max(width, height),
                );
                bd_fn!(z1_filter_edge::decl_fn, BD, ipred_z1_filter_edge, neon).call::<BD>(
                    &mut left_out,
                    width + height,
                    &flipped,
                    height + cmp::min(width, height),
                    filter_strength,
                );
                max_base_y = width + height - 1;
            } else {
                bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                    &mut left_out,
                    topleft_in,
                    height + cmp::min(width, height),
                );
                max_base_y = height + cmp::min(width, height) - 1;
            }
        }
        let base_inc = 1 + upsample_left as i32;
        let pad_pixels = cmp::max(64 - max_base_y - 1, height + 15);
        let px = left_out[max_base_y as usize];
        rav1d_ipred_pixel_set_neon::<BD>(
            &mut left_out[max_base_y as usize + 1..],
            px,
            pad_pixels * base_inc,
        );
        if upsample_left {
            bd_fn!(z13_fill::decl_fn, BD, ipred_z3_fill2, neon)
                .call::<BD>(dst, stride, &left_out, width, height, dy, max_base_y);
        } else {
            bd_fn!(z13_fill::decl_fn, BD, ipred_z3_fill1, neon)
                .call::<BD>(dst, stride, &left_out, width, height, dy, max_base_y);
        };
    }

    /// # Safety
    ///
    /// Must be called from [`angular_ipred::Fn::call`].
    pub unsafe extern "C" fn ipred_z_neon_erased<BD: BitDepth, const Z: usize>(
        dst: *mut DynPixel,
        stride: isize,
        topleft_in: *const DynPixel,
        width: i32,
        height: i32,
        angle: i32,
        max_width: i32,
        max_height: i32,
        bitdepth_max: i32,
        topleft_off: usize,
    ) {
        // SAFETY: Reconstructed from args passed by `angular_ipred::Fn::call`.
        let topleft_in = unsafe { reconstruct_topleft::<BD>(topleft_in, topleft_off) };
        [ipred_z1_neon, ipred_z2_neon, ipred_z3_neon][Z - 1](
            dst.cast(),
            stride,
            topleft_in,
            topleft_off,
            width,
            height,
            angle,
            max_width,
            max_height,
            BD::from_c(bitdepth_max),
        )
    }
}

impl Rav1dIntraPredDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        Self {
            intra_pred: {
                let mut a = [DefaultValue::DEFAULT; N_IMPL_INTRA_PRED_MODES];
                a[DC_PRED as usize] =
                    angular_ipred::Fn::new(ipred_dc_c_erased::<BD, { DcGen::TopLeft as u8 }>);
                a[DC_128_PRED as usize] = angular_ipred::Fn::new(ipred_dc_128_c_erased::<BD>);
                a[TOP_DC_PRED as usize] =
                    angular_ipred::Fn::new(ipred_dc_c_erased::<BD, { DcGen::Top as u8 }>);
                a[LEFT_DC_PRED as usize] =
                    angular_ipred::Fn::new(ipred_dc_c_erased::<BD, { DcGen::Left as u8 }>);
                a[HOR_PRED as usize] = angular_ipred::Fn::new(ipred_h_c_erased::<BD>);
                a[VERT_PRED as usize] = angular_ipred::Fn::new(ipred_v_c_erased::<BD>);
                a[PAETH_PRED as usize] = angular_ipred::Fn::new(ipred_paeth_c_erased::<BD>);
                a[SMOOTH_PRED as usize] = angular_ipred::Fn::new(ipred_smooth_c_erased::<BD>);
                a[SMOOTH_V_PRED as usize] = angular_ipred::Fn::new(ipred_smooth_v_c_erased::<BD>);
                a[SMOOTH_H_PRED as usize] = angular_ipred::Fn::new(ipred_smooth_h_c_erased::<BD>);
                a[Z1_PRED as usize] = angular_ipred::Fn::new(ipred_z_c_erased::<BD, 1>);
                a[Z2_PRED as usize] = angular_ipred::Fn::new(ipred_z_c_erased::<BD, 2>);
                a[Z3_PRED as usize] = angular_ipred::Fn::new(ipred_z_c_erased::<BD, 3>);
                a[FILTER_PRED as usize] = angular_ipred::Fn::new(ipred_filter_c_erased::<BD>);
                a
            },
            cfl_ac: enum_map!(Rav1dPixelLayoutSubSampled => cfl_ac::Fn; match key {
                I420 => cfl_ac::Fn::new(cfl_ac_c_erased::<BD, true, true>),
                I422 => cfl_ac::Fn::new(cfl_ac_c_erased::<BD, true, false>),
                I444 => cfl_ac::Fn::new(cfl_ac_c_erased::<BD, false, false>),
            }),
            cfl_pred: {
                // Not all elements are initialized with fns,
                // so we default initialize first so that there is no uninitialized memory.
                // The defaults just call `unimplemented!()`,
                // which shouldn't slow down the other code paths at all.
                let mut a = [DefaultValue::DEFAULT; 6];
                a[DC_PRED as usize] =
                    cfl_pred::Fn::new(ipred_cfl_c_erased::<BD, { DcGen::TopLeft as u8 }>);
                a[DC_128_PRED as usize] = cfl_pred::Fn::new(ipred_cfl_128_c_erased::<BD>);
                a[TOP_DC_PRED as usize] =
                    cfl_pred::Fn::new(ipred_cfl_c_erased::<BD, { DcGen::Top as u8 }>);
                a[LEFT_DC_PRED as usize] =
                    cfl_pred::Fn::new(ipred_cfl_c_erased::<BD, { DcGen::Left as u8 }>);
                a
            },
            pal_pred: pal_pred::Fn::new(pal_pred_c_erased::<BD>),
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.intra_pred[DC_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_dc, ssse3);
        self.intra_pred[DC_128_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_dc_128, ssse3);
        self.intra_pred[TOP_DC_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_dc_top, ssse3);
        self.intra_pred[LEFT_DC_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_dc_left, ssse3);
        self.intra_pred[HOR_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_h, ssse3);
        self.intra_pred[VERT_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_v, ssse3);
        self.intra_pred[PAETH_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_paeth, ssse3);
        self.intra_pred[SMOOTH_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth, ssse3);
        self.intra_pred[SMOOTH_H_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth_h, ssse3);
        self.intra_pred[SMOOTH_V_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth_v, ssse3);
        self.intra_pred[Z1_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_z1, ssse3);
        self.intra_pred[Z2_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_z2, ssse3);
        self.intra_pred[Z3_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_z3, ssse3);
        self.intra_pred[FILTER_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_filter, ssse3);

        self.cfl_pred[DC_PRED as usize] = bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl, ssse3);
        self.cfl_pred[DC_128_PRED as usize] = bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl_128, ssse3);
        self.cfl_pred[TOP_DC_PRED as usize] = bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl_top, ssse3);
        self.cfl_pred[LEFT_DC_PRED as usize] = bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl_left, ssse3);

        self.cfl_ac = enum_map!(Rav1dPixelLayoutSubSampled => cfl_ac::Fn; match key {
            I420 => bd_fn!(cfl_ac::decl_fn, BD, ipred_cfl_ac_420, ssse3),
            I422 => bd_fn!(cfl_ac::decl_fn, BD, ipred_cfl_ac_422, ssse3),
            I444 => bd_fn!(cfl_ac::decl_fn, BD, ipred_cfl_ac_444, ssse3),
        });

        self.pal_pred = bd_fn!(pal_pred::decl_fn, BD, pal_pred, ssse3);

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.intra_pred[DC_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_dc, avx2);
            self.intra_pred[DC_128_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_dc_128, avx2);
            self.intra_pred[TOP_DC_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_dc_top, avx2);
            self.intra_pred[LEFT_DC_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_dc_left, avx2);
            self.intra_pred[HOR_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_h, avx2);
            self.intra_pred[VERT_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_v, avx2);
            self.intra_pred[PAETH_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_paeth, avx2);
            self.intra_pred[SMOOTH_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth, avx2);
            self.intra_pred[SMOOTH_H_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth_h, avx2);
            self.intra_pred[SMOOTH_V_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth_v, avx2);
            self.intra_pred[Z1_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_z1, avx2);
            self.intra_pred[Z2_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_z2, avx2);
            self.intra_pred[Z3_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_z3, avx2);
            self.intra_pred[FILTER_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_filter, avx2);

            self.cfl_pred[DC_PRED as usize] = bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl, avx2);
            self.cfl_pred[DC_128_PRED as usize] =
                bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl_128, avx2);
            self.cfl_pred[TOP_DC_PRED as usize] =
                bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl_top, avx2);
            self.cfl_pred[LEFT_DC_PRED as usize] =
                bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl_left, avx2);

            self.cfl_ac = enum_map!(Rav1dPixelLayoutSubSampled => cfl_ac::Fn; match key {
                I420 => bd_fn!(cfl_ac::decl_fn, BD, ipred_cfl_ac_420, avx2),
                I422 => bd_fn!(cfl_ac::decl_fn, BD, ipred_cfl_ac_422, avx2),
                I444 => bd_fn!(cfl_ac::decl_fn, BD, ipred_cfl_ac_444, avx2),
            });

            self.pal_pred = bd_fn!(pal_pred::decl_fn, BD, pal_pred, avx2);

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            if let BPC::BPC8 = BD::BPC {
                self.intra_pred[DC_PRED as usize] =
                    bpc_fn!(angular_ipred::decl_fn, 8 bpc, ipred_dc, avx512icl);
                self.intra_pred[DC_128_PRED as usize] =
                    bpc_fn!(angular_ipred::decl_fn, 8 bpc, ipred_dc_128, avx512icl);
                self.intra_pred[TOP_DC_PRED as usize] =
                    bpc_fn!(angular_ipred::decl_fn, 8 bpc, ipred_dc_top, avx512icl);
                self.intra_pred[LEFT_DC_PRED as usize] =
                    bpc_fn!(angular_ipred::decl_fn, 8 bpc, ipred_dc_left, avx512icl);
                self.intra_pred[HOR_PRED as usize] =
                    bpc_fn!(angular_ipred::decl_fn, 8 bpc, ipred_h, avx512icl);
                self.intra_pred[VERT_PRED as usize] =
                    bpc_fn!(angular_ipred::decl_fn, 8 bpc, ipred_v, avx512icl);
                self.intra_pred[Z2_PRED as usize] =
                    bpc_fn!(angular_ipred::decl_fn, 8 bpc, ipred_z2, avx512icl);
            }

            self.intra_pred[PAETH_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_paeth, avx512icl);
            self.intra_pred[SMOOTH_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth, avx512icl);
            self.intra_pred[SMOOTH_H_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth_h, avx512icl);
            self.intra_pred[SMOOTH_V_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth_v, avx512icl);
            self.intra_pred[Z1_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_z1, avx512icl);
            self.intra_pred[Z2_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_z2, avx512icl);
            self.intra_pred[Z3_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_z3, avx512icl);
            self.intra_pred[FILTER_PRED as usize] =
                bd_fn!(angular_ipred::decl_fn, BD, ipred_filter, avx512icl);

            self.pal_pred = bd_fn!(pal_pred::decl_fn, BD, pal_pred, avx512icl);
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        self.intra_pred[DC_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_dc, neon);
        self.intra_pred[DC_128_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_dc_128, neon);
        self.intra_pred[TOP_DC_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_dc_top, neon);
        self.intra_pred[LEFT_DC_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_dc_left, neon);
        self.intra_pred[HOR_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_h, neon);
        self.intra_pred[VERT_PRED as usize] = bd_fn!(angular_ipred::decl_fn, BD, ipred_v, neon);
        self.intra_pred[PAETH_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_paeth, neon);
        self.intra_pred[SMOOTH_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth, neon);
        self.intra_pred[SMOOTH_V_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth_v, neon);
        self.intra_pred[SMOOTH_H_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_smooth_h, neon);
        #[cfg(target_arch = "aarch64")]
        {
            use self::neon::ipred_z_neon_erased;

            self.intra_pred[Z1_PRED as usize] =
                angular_ipred::Fn::new(ipred_z_neon_erased::<BD, 1>);
            self.intra_pred[Z2_PRED as usize] =
                angular_ipred::Fn::new(ipred_z_neon_erased::<BD, 2>);
            self.intra_pred[Z3_PRED as usize] =
                angular_ipred::Fn::new(ipred_z_neon_erased::<BD, 3>);
        }
        self.intra_pred[FILTER_PRED as usize] =
            bd_fn!(angular_ipred::decl_fn, BD, ipred_filter, neon);

        self.cfl_pred[DC_PRED as usize] = bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl, neon);
        self.cfl_pred[DC_128_PRED as usize] = bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl_128, neon);
        self.cfl_pred[TOP_DC_PRED as usize] = bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl_top, neon);
        self.cfl_pred[LEFT_DC_PRED as usize] = bd_fn!(cfl_pred::decl_fn, BD, ipred_cfl_left, neon);

        self.cfl_ac = enum_map!(Rav1dPixelLayoutSubSampled => cfl_ac::Fn; match key {
            I420 => bd_fn!(cfl_ac::decl_fn, BD, ipred_cfl_ac_420, neon),
            I422 => bd_fn!(cfl_ac::decl_fn, BD, ipred_cfl_ac_422, neon),
            I444 => bd_fn!(cfl_ac::decl_fn, BD, ipred_cfl_ac_444, neon),
        });

        self.pal_pred = bd_fn!(pal_pred::decl_fn, BD, pal_pred, neon);

        self
    }

    #[inline(always)]
    const fn init<BD: BitDepth>(self, flags: CpuFlags) -> Self {
        #[cfg(feature = "asm")]
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86::<BD>(flags);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm::<BD>(flags);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        {
            let _ = flags;
            self
        }
    }

    pub const fn new<BD: BitDepth>(flags: CpuFlags) -> Self {
        Self::default::<BD>().init::<BD>(flags)
    }
}
