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
use cfg_if::cfg_if;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
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
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
    topleft_off: usize,
) -> ());

impl angular_ipred::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        dst: *mut BD::Pixel,
        stride: ptrdiff_t,
        topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
        topleft_off: usize,
        width: c_int,
        height: c_int,
        angle: c_int,
        max_width: c_int,
        max_height: c_int,
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
    stride: ptrdiff_t,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
    _y: *const FFISafe<Rav1dPictureDataComponent>,
) -> ());

impl cfl_ac::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        ac: &mut [i16; SCRATCH_AC_TXTP_LEN],
        y: &Rav1dPictureDataComponent,
        y_offset: usize,
        w_pad: c_int,
        h_pad: c_int,
        cw: c_int,
        ch: c_int,
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
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: &[i16; SCRATCH_AC_TXTP_LEN],
    alpha: c_int,
    bitdepth_max: c_int,
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
        width: c_int,
        height: c_int,
        ac: &[i16; SCRATCH_AC_TXTP_LEN],
        alpha: c_int,
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
    stride: ptrdiff_t,
    pal: *const [DynPixel; 8],
    idx: *const u8,
    w: c_int,
    h: c_int,
    _dst: *const FFISafe<Rav1dPictureDataComponent>,
) -> ());

impl pal_pred::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        dst: &Rav1dPictureDataComponent,
        dst_offset: usize,
        pal: &[BD::Pixel; 8],
        idx: &[u8],
        w: c_int,
        h: c_int,
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
    stride: ptrdiff_t,
    width: c_int,
    height: c_int,
    dc: c_int,
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
            assert!(dc <= bd.bitdepth_max().as_::<c_int>());
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
    width: c_int,
    height: c_int,
    dc: c_int,
    ac: &[i16; SCRATCH_AC_TXTP_LEN],
    alpha: c_int,
    bd: BD,
) {
    let width = width as usize;
    let mut ac = &ac[..width * height as usize];
    for _ in 0..height {
        let slice = &mut *dst.slice_mut::<BD, _>((dst_offset.., ..width));
        for (x, dst) in slice.iter_mut().enumerate() {
            let diff = alpha * ac[x] as c_int;
            *dst = bd.iclip_pixel(dc + apply_sign(diff.abs() + 32 >> 6, diff));
        }
        ac = &ac[width..];
        dst_offset = dst_offset.wrapping_add_signed(dst.pixel_stride::<BD>());
    }
}

fn dc_gen_top<BD: BitDepth>(
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    offset: usize,
    width: c_int,
) -> c_uint {
    let mut dc = width as u32 >> 1;
    for i in 0..width as usize {
        dc += topleft[offset + 1 + i].as_::<c_uint>();
    }
    return dc >> width.trailing_zeros();
}

fn dc_gen_left<BD: BitDepth>(
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    offset: usize,
    height: c_int,
) -> c_uint {
    let mut dc = height as u32 >> 1;
    for i in 0..height as usize {
        dc += topleft[offset - (1 + i)].as_::<c_uint>();
    }
    return dc >> height.trailing_zeros();
}

fn dc_gen<BD: BitDepth>(
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    offset: usize,
    width: c_int,
    height: c_int,
) -> c_uint {
    let (multiplier_1x2, multiplier_1x4, base_shift) = match BD::BPC {
        BPC::BPC8 => (0x5556, 0x3334, 16),
        BPC::BPC16 => (0xAAAB, 0x6667, 17),
    };

    let mut dc = (width + height >> 1) as u32;
    for i in 0..width as usize {
        dc += topleft[offset + i + 1].as_::<c_uint>();
    }
    for i in 0..height as usize {
        dc += topleft[offset - (i + 1)].as_::<c_uint>();
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
        width: c_int,
        height: c_int,
    ) -> c_uint {
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
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
    topleft_off: usize,
) {
    let dc_gen = DcGen::from_repr(DC_GEN).unwrap();
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    splat_dc(
        dst.cast(),
        stride,
        width,
        height,
        dc_gen.call::<BD>(topleft, topleft_off, width, height) as c_int,
        BD::from_c(bitdepth_max),
    );
}

/// # Safety
///
/// Must be called by [`cfl_pred::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn ipred_cfl_c_erased<BD: BitDepth, const DC_GEN: u8>(
    dst_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: &[i16; SCRATCH_AC_TXTP_LEN],
    alpha: c_int,
    bitdepth_max: c_int,
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
    let dc: c_uint = dc_gen.call::<BD>(topleft, topleft_off, width, height);
    cfl_pred(
        dst,
        dst_offset,
        width,
        height,
        dc as c_int,
        ac,
        alpha,
        BD::from_c(bitdepth_max),
    );
}

unsafe extern "C" fn ipred_dc_128_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    _topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
    _topleft_off: usize,
) {
    let bd = BD::from_c(bitdepth_max);
    let dc = bd.bitdepth_max().as_::<c_int>() + 1 >> 1;
    splat_dc(dst.cast(), stride, width, height, dc, bd);
}

/// # Safety
///
/// Must be called by [`cfl_pred::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn ipred_cfl_128_c_erased<BD: BitDepth>(
    dst_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    _topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    ac: &[i16; SCRATCH_AC_TXTP_LEN],
    alpha: c_int,
    bitdepth_max: c_int,
    _topleft_off: usize,
    dst: *const FFISafe<Rav1dPictureDataComponent>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `cfl_pred::Fn::call`.
    let dst = unsafe { FFISafe::get(dst) };
    // SAFETY: Reverse of what was done in `fn cfl_pred::Fn::call`.
    let dst_offset =
        unsafe { dst_ptr.cast::<BD::Pixel>().offset_from(dst.as_ptr::<BD>()) } as usize;
    let bd = BD::from_c(bitdepth_max);
    let dc = bd.bitdepth_max().as_::<c_int>() + 1 >> 1;
    cfl_pred(dst, dst_offset, width, height, dc, ac, alpha, bd);
}

unsafe fn ipred_v_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: c_int,
    height: c_int,
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
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_v_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_h_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: c_int,
    height: c_int,
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
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_h_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_paeth_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    tl: &[BD::Pixel; SCRATCH_EDGE_LEN],
    tl_off: usize,
    width: c_int,
    height: c_int,
) {
    let topleft = tl[tl_off].as_::<c_int>();
    for y in 0..height as usize {
        let left = tl[tl_off - (y + 1)].as_::<c_int>();
        let dst_slice = slice::from_raw_parts_mut(dst, width as usize);
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let top = tl[tl_off + 1 + x].as_::<c_int>();
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
    stride: ptrdiff_t,
    tl_ptr: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(tl_ptr, topleft_off);
    ipred_paeth_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_smooth_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: c_int,
    height: c_int,
) {
    let [width, height] = [width, height].map(|it| it as usize);

    let weights_hor = &dav1d_sm_weights.0[width..][..width];
    let weights_ver = &dav1d_sm_weights.0[height..][..height];
    let right = topleft[topleft_off + width].as_::<c_int>();
    let bottom = topleft[topleft_off - height].as_::<c_int>();

    for y in 0..height {
        let dst_slice = slice::from_raw_parts_mut(dst, width);
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let pred = weights_ver[y] as c_int * topleft[topleft_off + 1 + x].as_::<c_int>()
                + (256 - weights_ver[y] as c_int) * bottom
                + weights_hor[x] as c_int * topleft[topleft_off - (1 + y)].as_::<c_int>()
                + (256 - weights_hor[x] as c_int) * right;
            *dst = (pred + 256 >> 9).as_::<BD::Pixel>();
        }
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe extern "C" fn ipred_smooth_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_smooth_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_smooth_v_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: c_int,
    height: c_int,
) {
    let [width, height] = [width, height].map(|it| it as usize);

    let weights_ver = &dav1d_sm_weights.0[height..][..height];
    let bottom = topleft[topleft_off - height].as_::<c_int>();

    for y in 0..height {
        let dst_slice = slice::from_raw_parts_mut(dst, width);
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let pred = weights_ver[y] as c_int * topleft[topleft_off + 1 + x].as_::<c_int>()
                + (256 - weights_ver[y] as c_int) * bottom;
            *dst = (pred + 128 >> 8).as_::<BD::Pixel>();
        }
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe extern "C" fn ipred_smooth_v_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_smooth_v_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

unsafe fn ipred_smooth_h_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_off: usize,
    width: c_int,
    height: c_int,
) {
    let [width, height] = [width, height].map(|it| it as usize);

    let weights_hor = &dav1d_sm_weights.0[width..][..width];
    let right = topleft[topleft_off + width].as_::<c_int>();

    for y in 0..height {
        let dst_slice = slice::from_raw_parts_mut(dst, width);
        for (x, dst) in dst_slice.iter_mut().enumerate() {
            let pred = weights_hor[x] as c_int * topleft[topleft_off - (y + 1)].as_::<c_int>()
                + (256 - weights_hor[x] as c_int) * right;
            *dst = (pred + 128 >> 8).as_::<BD::Pixel>();
        }
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe extern "C" fn ipred_smooth_h_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft: *const DynPixel,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
    topleft_off: usize,
) {
    let topleft = reconstruct_topleft::<BD>(topleft, topleft_off);
    ipred_smooth_h_rust::<BD>(dst.cast(), stride, topleft, topleft_off, width, height);
}

#[inline(never)]
fn get_filter_strength(wh: c_int, angle: c_int, is_sm: bool) -> c_int {
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
    sz: c_int,
    lim_from: c_int,
    lim_to: c_int,
    r#in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    in_off: usize,
    from: c_int,
    to: c_int,
    strength: c_int,
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
                .as_::<c_int>()
                * kernel[(strength - 1) as usize][j as usize] as c_int;
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
fn get_upsample(wh: c_int, angle: c_int, is_sm: bool) -> bool {
    angle < 40 && wh <= (16 >> is_sm as u8)
}

#[inline(never)]
fn upsample_edge<BD: BitDepth>(
    out: &mut [BD::Pixel],
    hsz: c_int,
    r#in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    in_off: usize,
    from: c_int,
    to: c_int,
    bd: BD,
) {
    static kernel: [i8; 4] = [-1, 9, 9, -1];
    for i in 0..hsz - 1 {
        out[(i * 2) as usize] = r#in[in_off + iclip(i, from, to - 1) as usize];
        let mut s = 0;
        for j in 0..4 {
            s += r#in[in_off.wrapping_add_signed(iclip(i + j - 1, from, to - 1) as isize)]
                .as_::<c_int>()
                * kernel[j as usize] as c_int;
        }
        out[(i * 2 + 1) as usize] =
            iclip(s + 8 >> 4, 0, bd.bitdepth_max().as_::<c_int>()).as_::<BD::Pixel>();
    }
    let i = hsz - 1;
    out[(i * 2) as usize] = r#in[in_off + iclip(i, from, to - 1) as usize];
}

unsafe fn ipred_z1_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_in_off: usize,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bd: BD,
) {
    let is_sm = (angle >> 9) & 1 != 0;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle < 90) {
        unreachable!();
    }
    let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as c_int;
    let mut top_out = [0.into(); 64 + 64];
    let top: *const BD::Pixel;
    let max_base_x;
    let upsample_above = if enable_intra_edge_filter != 0 {
        get_upsample(width + height, 90 - angle, is_sm)
    } else {
        false
    };
    if upsample_above {
        upsample_edge::<BD>(
            &mut top_out,
            width + height,
            topleft_in,
            topleft_in_off + 1,
            -(1 as c_int),
            width + cmp::min(width, height),
            bd,
        );
        top = top_out.as_mut_ptr();
        max_base_x = 2 * (width + height) - 2;
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 90 - angle, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            filter_edge::<BD>(
                &mut top_out,
                width + height,
                0 as c_int,
                width + height,
                topleft_in,
                topleft_in_off + 1,
                -(1 as c_int),
                width + cmp::min(width, height),
                filter_strength,
            );
            top = top_out.as_mut_ptr();
            max_base_x = width + height - 1;
        } else {
            top = topleft_in[topleft_in_off + 1..].as_ptr();
            max_base_x = width + cmp::min(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_above as c_int;
    let mut y = 0;
    let mut xpos = dx;
    while y < height {
        let frac = xpos & 0x3e as c_int;
        let mut x = 0;
        let mut base = xpos >> 6;
        while x < width {
            if base < max_base_x {
                let v = (*top.offset(base as isize)).as_::<c_int>() * (64 - frac)
                    + (*top.offset((base + 1) as isize)).as_::<c_int>() * frac;
                *dst.offset(x as isize) = (v + 32 >> 6).as_::<BD::Pixel>();
                x += 1;
                base += base_inc;
            } else {
                let width = width.try_into().unwrap();
                let x = x as usize;
                BD::pixel_set(
                    &mut slice::from_raw_parts_mut(dst, width)[x..],
                    *top.offset(max_base_x as isize),
                    width - x,
                );
                break;
            }
        }
        y += 1;
        dst = dst.offset(BD::pxstride(stride));
        xpos += dx;
    }
}

unsafe fn ipred_z2_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_in_off: usize,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bd: BD,
) {
    let is_sm = (angle >> 9) & 1 != 0;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle > 90 && angle < 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[(angle - 90 >> 1) as usize] as c_int;
    let mut dx = dav1d_dr_intra_derivative[(180 - angle >> 1) as usize] as c_int;
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
    let mut edge = [0.into(); 129];
    let topleft = 64;
    if upsample_above {
        upsample_edge::<BD>(
            &mut edge[topleft..],
            width + 1,
            topleft_in,
            topleft_in_off,
            0 as c_int,
            width + 1,
            bd,
        );
        dx <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 90, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            filter_edge::<BD>(
                &mut edge[topleft + 1..],
                width,
                0 as c_int,
                max_width,
                topleft_in,
                topleft_in_off + 1,
                -(1 as c_int),
                width,
                filter_strength,
            );
        } else {
            let width = width.try_into().unwrap();
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
            0 as c_int,
            height + 1,
            bd,
        );
        dy <<= 1;
    } else {
        let filter_strength_0 = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, 180 - angle, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength_0 != 0 {
            filter_edge::<BD>(
                &mut edge[topleft - height as usize..],
                height,
                height - max_height,
                height,
                topleft_in,
                topleft_in_off - height as usize,
                0 as c_int,
                height + 1,
                filter_strength_0,
            );
        } else {
            BD::pixel_copy(
                &mut edge[topleft - height as usize..][..height as usize],
                &topleft_in[topleft_in_off - height as usize..][..height as usize],
                height.try_into().unwrap(),
            );
        }
    }
    edge[topleft] = topleft_in[topleft_in_off];
    let base_inc_x = 1 + upsample_above as c_int;
    let left = edge[topleft - (1 + upsample_left as usize)..].as_mut_ptr();
    let mut y = 0;
    let mut xpos = (1 + (upsample_above as c_int) << 6) - dx;
    while y < height {
        let mut base_x = xpos >> 6;
        let frac_x = xpos & 0x3e as c_int;
        let mut x = 0;
        let mut ypos = (y << 6 + upsample_left as c_int) - dy;
        while x < width {
            let v;
            if base_x >= 0 {
                v = edge[topleft + base_x as usize].as_::<c_int>() * (64 - frac_x)
                    + edge[topleft + base_x as usize + 1].as_::<c_int>() * frac_x;
            } else {
                let base_y = ypos >> 6;
                if !(base_y >= -(1 + upsample_left as c_int)) {
                    unreachable!();
                }
                let frac_y = ypos & 0x3e as c_int;
                v = (*left.offset(-base_y as isize)).as_::<c_int>() * (64 - frac_y)
                    + (*left.offset(-(base_y + 1) as isize)).as_::<c_int>() * frac_y;
            }
            *dst.offset(x as isize) = (v + 32 >> 6).as_::<BD::Pixel>();
            x += 1;
            base_x += base_inc_x;
            ypos -= dy;
        }
        y += 1;
        xpos -= dx;
        dst = dst.offset(BD::pxstride(stride));
    }
}

unsafe fn ipred_z3_rust<BD: BitDepth>(
    dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft_in: &[BD::Pixel; SCRATCH_EDGE_LEN],
    topleft_in_off: usize,
    width: c_int,
    height: c_int,
    mut angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bd: BD,
) {
    let is_sm = (angle >> 9) & 1 != 0;
    let enable_intra_edge_filter = angle >> 10;
    angle &= 511 as c_int;
    if !(angle > 180) {
        unreachable!();
    }
    let mut dy = dav1d_dr_intra_derivative[(270 - angle >> 1) as usize] as c_int;
    let mut left_out: [BD::Pixel; 128] = [0.into(); 128];
    let left: *const BD::Pixel;
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
            cmp::max(width - height, 0 as c_int),
            width + height + 1,
            bd,
        );
        left = &mut *left_out
            .as_mut_ptr()
            .offset((2 * (width + height) - 2) as isize) as *mut BD::Pixel;
        max_base_y = 2 * (width + height) - 2;
        dy <<= 1;
    } else {
        let filter_strength = if enable_intra_edge_filter != 0 {
            get_filter_strength(width + height, angle - 180, is_sm)
        } else {
            0 as c_int
        };
        if filter_strength != 0 {
            filter_edge::<BD>(
                &mut left_out,
                width + height,
                0 as c_int,
                width + height,
                topleft_in,
                topleft_in_off - (width + height) as usize,
                cmp::max(width - height, 0 as c_int),
                width + height + 1,
                filter_strength,
            );
            left =
                &mut *left_out.as_mut_ptr().offset((width + height - 1) as isize) as *mut BD::Pixel;
            max_base_y = width + height - 1;
        } else {
            left = topleft_in[topleft_in_off - 1..].as_ptr();
            max_base_y = height + cmp::min(width, height) - 1;
        }
    }
    let base_inc = 1 + upsample_left as c_int;
    let mut x = 0;
    let mut ypos = dy;
    while x < width {
        let frac = ypos & 0x3e as c_int;
        let mut y = 0;
        let mut base = ypos >> 6;
        while y < height {
            if base < max_base_y {
                let v = (*left.offset(-base as isize)).as_::<c_int>() * (64 - frac)
                    + (*left.offset(-(base + 1) as isize)).as_::<c_int>() * frac;
                *dst.offset((y as isize * BD::pxstride(stride) + x as isize) as isize) =
                    (v + 32 >> 6).as_::<BD::Pixel>();
                y += 1;
                base += base_inc;
            } else {
                loop {
                    *dst.offset((y as isize * BD::pxstride(stride) + x as isize) as isize) =
                        *left.offset(-max_base_y as isize);
                    y += 1;
                    if !(y < height) {
                        break;
                    }
                }
                break;
            }
        }
        x += 1;
        ypos += dy;
    }
}

unsafe extern "C" fn ipred_z_c_erased<BD: BitDepth, const Z: usize>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
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

unsafe fn filter_fn(
    flt_ptr: *const i8,
    p0: c_int,
    p1: c_int,
    p2: c_int,
    p3: c_int,
    p4: c_int,
    p5: c_int,
    p6: c_int,
) -> c_int {
    if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        *flt_ptr.offset(0) as c_int * p0
            + *flt_ptr.offset(1) as c_int * p1
            + *flt_ptr.offset(16) as c_int * p2
            + *flt_ptr.offset(17) as c_int * p3
            + *flt_ptr.offset(32) as c_int * p4
            + *flt_ptr.offset(33) as c_int * p5
            + *flt_ptr.offset(48) as c_int * p6
    } else {
        *flt_ptr.offset(0) as c_int * p0
            + *flt_ptr.offset(8) as c_int * p1
            + *flt_ptr.offset(16) as c_int * p2
            + *flt_ptr.offset(24) as c_int * p3
            + *flt_ptr.offset(32) as c_int * p4
            + *flt_ptr.offset(40) as c_int * p5
            + *flt_ptr.offset(48) as c_int * p6
    }
}

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        const FLT_INCR: isize = 2;
    } else {
        const FLT_INCR: isize = 1;
    }
}

unsafe fn ipred_filter_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    topleft_in: *const BD::Pixel,
    width: c_int,
    height: c_int,
    mut filt_idx: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bd: BD,
) {
    filt_idx &= 511 as c_int;
    if !(filt_idx < 5) {
        unreachable!();
    }
    let filter: *const i8 = (dav1d_filter_intra_taps[filt_idx as usize]).as_ptr();
    let mut top: *const BD::Pixel = &*topleft_in.offset(1) as *const BD::Pixel;
    let mut y = 0;
    while y < height {
        let mut topleft: *const BD::Pixel = &*topleft_in.offset(-y as isize) as *const BD::Pixel;
        let mut left: *const BD::Pixel =
            &*topleft.offset(-(1 as c_int) as isize) as *const BD::Pixel;
        let mut left_stride: ptrdiff_t = -(1 as c_int) as ptrdiff_t;
        let mut x = 0;
        while x < width {
            let p0 = (*topleft).as_::<c_int>();
            let p1 = (*top.offset(0)).as_::<c_int>();
            let p2 = (*top.offset(1)).as_::<c_int>();
            let p3 = (*top.offset(2)).as_::<c_int>();
            let p4 = (*top.offset(3)).as_::<c_int>();
            let p5 = (*left.offset((0 * left_stride) as isize)).as_::<c_int>();
            let p6 = (*left.offset((1 * left_stride) as isize)).as_::<c_int>();
            let mut ptr: *mut BD::Pixel = &mut *dst.offset(x as isize) as *mut BD::Pixel;
            let mut flt_ptr: *const i8 = filter;
            let mut yy = 0;
            while yy < 2 {
                let mut xx = 0;
                while xx < 4 {
                    let acc = filter_fn(flt_ptr, p0, p1, p2, p3, p4, p5, p6);
                    *ptr.offset(xx as isize) = bd.iclip_pixel(acc + 8 >> 4);
                    xx += 1;
                    flt_ptr = flt_ptr.offset(FLT_INCR);
                }
                ptr = ptr.offset(BD::pxstride(stride));
                yy += 1;
            }
            left = &mut *dst.offset((x + 4 - 1) as isize) as *mut BD::Pixel;
            left_stride = BD::pxstride(stride);
            top = top.offset(4);
            topleft = &*top.offset(-(1 as c_int) as isize) as *const BD::Pixel;
            x += 4 as c_int;
        }
        top = &mut *dst.offset(BD::pxstride(stride)) as *mut BD::Pixel;
        dst = &mut *dst.offset(BD::pxstride(stride) * 2) as *mut BD::Pixel;
        y += 2 as c_int;
    }
}

unsafe extern "C" fn ipred_filter_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    topleft_in: *const DynPixel,
    width: c_int,
    height: c_int,
    filt_idx: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
    _topleft_off: usize,
) {
    ipred_filter_rust(
        dst.cast(),
        stride,
        topleft_in.cast(),
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
    w_pad: c_int,
    h_pad: c_int,
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
    _stride: ptrdiff_t,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
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
    w: c_int,
    h: c_int,
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
    _stride: ptrdiff_t,
    pal: *const [DynPixel; 8],
    idx: *const u8,
    w: c_int,
    h: c_int,
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

    use libc::memcpy;
    use std::ffi::c_void;
    use to_method::To;

    #[cfg(feature = "bitdepth_8")]
    use crate::include::common::bitdepth::BitDepth8;

    #[cfg(feature = "bitdepth_16")]
    use crate::include::common::bitdepth::BitDepth16;

    wrap_fn_ptr!(unsafe extern "C" fn z13_fill(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        topleft: *const DynPixel,
        width: c_int,
        height: c_int,
        dxy: c_int,
        max_base_xy: c_int,
    ) -> ());

    impl z13_fill::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            dst: *mut BD::Pixel,
            stride: ptrdiff_t,
            topleft: *const BD::Pixel,
            width: c_int,
            height: c_int,
            dxy: c_int,
            max_base_xy: c_int,
        ) {
            let dst = dst.cast();
            let topleft = topleft.cast();
            self.get()(dst, stride, topleft, width, height, dxy, max_base_xy)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn z2_fill(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        top: *const DynPixel,
        left: *const DynPixel,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    ) -> ());

    impl z2_fill::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            dst: *mut BD::Pixel,
            stride: ptrdiff_t,
            top: *const BD::Pixel,
            left: *const BD::Pixel,
            width: c_int,
            height: c_int,
            dx: c_int,
            dy: c_int,
        ) {
            let dst = dst.cast();
            let top = top.cast();
            let left = left.cast();
            self.get()(dst, stride, top, left, width, height, dx, dy)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn z1_upsample_edge(
        out: *mut DynPixel,
        hsz: c_int,
        in_0: *const DynPixel,
        end: c_int,
        _bitdepth_max: c_int,
    ) -> ());

    impl z1_upsample_edge::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            out: *mut BD::Pixel,
            hsz: c_int,
            in_0: *const BD::Pixel,
            end: c_int,
            bd: BD,
        ) {
            let out = out.cast();
            let in_0 = in_0.cast();
            let bd = bd.into_c();
            self.get()(out, hsz, in_0, end, bd)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn z1_filter_edge(
        out: *mut DynPixel,
        sz: c_int,
        in_0: *const DynPixel,
        end: c_int,
        strength: c_int,
    ) -> ());

    impl z1_filter_edge::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            out: *mut BD::Pixel,
            sz: c_int,
            in_0: *const BD::Pixel,
            end: c_int,
            strength: c_int,
        ) {
            let out = out.cast();
            let in_0 = in_0.cast();
            self.get()(out, sz, in_0, end, strength)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn z2_upsample_edge(
        out: *mut DynPixel,
        hsz: c_int,
        in_0: *const DynPixel,
        _bitdepth_max: c_int,
    ) -> ());

    impl z2_upsample_edge::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            out: *mut BD::Pixel,
            hsz: c_int,
            in_0: *const BD::Pixel,
            bd: BD,
        ) {
            let out = out.cast();
            let in_0 = in_0.cast();
            let bd = bd.into_c();
            self.get()(out, hsz, in_0, bd)
        }
    }

    wrap_fn_ptr!(unsafe extern "C" fn reverse(
        dst: *mut DynPixel,
        src: *const DynPixel,
        n: c_int,
    ) -> ());

    impl reverse::Fn {
        pub unsafe fn call<BD: BitDepth>(
            &self,
            dst: *mut BD::Pixel,
            src: *const BD::Pixel,
            n: c_int,
        ) {
            let dst = dst.cast();
            let src = src.cast();
            self.get()(dst, src, n)
        }
    }

    unsafe fn rav1d_ipred_pixel_set_neon<BD: BitDepth>(
        out: *mut BD::Pixel,
        px: BD::Pixel,
        n: c_int,
    ) {
        // `pixel_set` takes a `px: BD::Pixel`.
        // Since it's not behind a ptr, we can't make it a `DynPixel`
        // and call it uniformly with `bd_fn!`.

        extern "C" {
            #[cfg(feature = "bitdepth_8")]
            fn dav1d_ipred_pixel_set_8bpc_neon(
                out: *mut DynPixel,
                px: <BitDepth8 as BitDepth>::Pixel,
                n: c_int,
            );

            #[cfg(feature = "bitdepth_16")]
            fn dav1d_ipred_pixel_set_16bpc_neon(
                out: *mut DynPixel,
                px: <BitDepth16 as BitDepth>::Pixel,
                n: c_int,
            );
        }

        let out = out.cast();
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
        stride: ptrdiff_t,
        topleft_in: *const BD::Pixel,
        width: c_int,
        height: c_int,
        mut angle: c_int,
        _max_width: c_int,
        _max_height: c_int,
        bd: BD,
    ) {
        let is_sm = (angle >> 9) & 1 != 0;
        let enable_intra_edge_filter = angle >> 10;
        angle &= 511 as c_int;
        let mut dx = dav1d_dr_intra_derivative[(angle >> 1) as usize] as c_int;
        const top_out_size: usize = 64 + 64 * (64 + 15) * 2 + 16;
        let mut top_out: [BD::Pixel; top_out_size] = [0.into(); top_out_size];
        let max_base_x;
        let upsample_above = if enable_intra_edge_filter != 0 {
            get_upsample(width + height, 90 - angle, is_sm)
        } else {
            false
        };
        if upsample_above {
            bd_fn!(z1_upsample_edge::decl_fn, BD, ipred_z1_upsample_edge, neon).call(
                top_out.as_mut_ptr(),
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
                0 as c_int
            };
            if filter_strength != 0 {
                bd_fn!(z1_filter_edge::decl_fn, BD, ipred_z1_filter_edge, neon).call::<BD>(
                    top_out.as_mut_ptr(),
                    width + height,
                    topleft_in,
                    width + cmp::min(width, height),
                    filter_strength,
                );
                max_base_x = width + height - 1;
            } else {
                max_base_x = width + cmp::min(width, height) - 1;
                memcpy(
                    top_out.as_mut_ptr() as *mut c_void,
                    &*topleft_in.offset(1) as *const BD::Pixel as *const c_void,
                    ((max_base_x + 1) as usize).wrapping_mul(::core::mem::size_of::<BD::Pixel>()),
                );
            }
        }
        let base_inc = 1 + upsample_above as c_int;
        let pad_pixels = width + 15;
        rav1d_ipred_pixel_set_neon::<BD>(
            top_out.as_mut_ptr().offset((max_base_x + 1) as isize),
            top_out[max_base_x as usize],
            (pad_pixels * base_inc) as c_int,
        );
        if upsample_above {
            bd_fn!(z13_fill::decl_fn, BD, ipred_z1_fill2, neon).call::<BD>(
                dst,
                stride,
                top_out.as_mut_ptr(),
                width,
                height,
                dx,
                max_base_x,
            );
        } else {
            bd_fn!(z13_fill::decl_fn, BD, ipred_z1_fill1, neon).call::<BD>(
                dst,
                stride,
                top_out.as_mut_ptr(),
                width,
                height,
                dx,
                max_base_x,
            );
        };
    }

    unsafe fn ipred_z2_neon<BD: BitDepth>(
        dst: *mut BD::Pixel,
        stride: ptrdiff_t,
        topleft_in: *const BD::Pixel,
        width: c_int,
        height: c_int,
        mut angle: c_int,
        max_width: c_int,
        max_height: c_int,
        bd: BD,
    ) {
        let is_sm = (angle >> 9) & 1 != 0;
        let enable_intra_edge_filter = angle >> 10;
        angle &= 511 as c_int;
        if !(angle > 90 && angle < 180) {
            unreachable!();
        }
        let mut dy = dav1d_dr_intra_derivative[((angle - 90) >> 1) as usize] as c_int;
        let mut dx = dav1d_dr_intra_derivative[((180 - angle) >> 1) as usize] as c_int;
        let mut buf: [BD::Pixel; 3 * (64 + 1)] = [0.into(); 3 * (64 + 1)]; // NOTE: C code doesn't initialize

        // The asm can underread below the start of top[] and left[]; to avoid
        // surprising behaviour, make sure this is within the allocated stack space.
        let left_offset: isize = 2 * (64 + 1);
        let top_offset: isize = 1 * (64 + 1);
        let flipped_offset: isize = 0 * (64 + 1);

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
                buf.as_mut_ptr().offset(top_offset),
                width,
                topleft_in,
                bd,
            );
            dx <<= 1;
        } else {
            let filter_strength = if enable_intra_edge_filter != 0 {
                get_filter_strength(width + height, angle - 90, is_sm)
            } else {
                0 as c_int
            };

            if filter_strength != 0 {
                bd_fn!(z1_filter_edge::decl_fn, BD, ipred_z1_filter_edge, neon).call::<BD>(
                    buf.as_mut_ptr().offset(1 + top_offset),
                    cmp::min(max_width, width),
                    topleft_in,
                    width,
                    filter_strength,
                );

                if max_width < width {
                    memcpy(
                        buf.as_mut_ptr().offset(top_offset + 1 + max_width as isize) as *mut c_void,
                        topleft_in.offset(1 + max_width as isize) as *const c_void,
                        ((width - max_width) as usize)
                            .wrapping_mul(::core::mem::size_of::<BD::Pixel>()),
                    );
                }
            } else {
                BD::pixel_copy(
                    &mut buf[1 + top_offset as usize..],
                    core::slice::from_raw_parts(topleft_in.offset(1), width as usize),
                    width as usize,
                );
            }
        }

        if upsample_left {
            buf[flipped_offset as usize] = *topleft_in;
            bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                buf.as_mut_ptr().offset(1 + flipped_offset),
                topleft_in,
                height,
            );
            bd_fn!(z2_upsample_edge::decl_fn, BD, ipred_z2_upsample_edge, neon).call(
                buf.as_mut_ptr().offset(left_offset),
                height,
                buf.as_ptr().offset(flipped_offset),
                bd,
            );
            dy <<= 1;
        } else {
            let filter_strength = if enable_intra_edge_filter != 0 {
                get_filter_strength(width + height, 180 - angle, is_sm)
            } else {
                0 as c_int
            };
            if filter_strength != 0 {
                buf[flipped_offset as usize] = *topleft_in;
                bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                    buf.as_mut_ptr().offset(1 + flipped_offset),
                    topleft_in,
                    height,
                );
                bd_fn!(z1_filter_edge::decl_fn, BD, ipred_z1_filter_edge, neon).call::<BD>(
                    buf.as_mut_ptr().offset(1 + left_offset),
                    cmp::min(max_height, height),
                    buf.as_ptr().offset(flipped_offset),
                    height,
                    filter_strength,
                );
                if max_height < height {
                    memcpy(
                        buf.as_mut_ptr()
                            .offset(left_offset + 1 + max_height as isize)
                            as *mut c_void,
                        buf.as_mut_ptr()
                            .offset(flipped_offset + 1 + max_height as isize)
                            as *const c_void,
                        ((height - max_height) as usize)
                            .wrapping_mul(::core::mem::size_of::<BD::Pixel>()),
                    );
                }
            } else {
                bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                    buf.as_mut_ptr().offset(left_offset + 1),
                    topleft_in,
                    height,
                );
            }
        }
        buf[top_offset as usize] = *topleft_in;
        buf[left_offset as usize] = *topleft_in;

        if upsample_above && upsample_left {
            unreachable!();
        }

        if !upsample_above && !upsample_left {
            bd_fn!(z2_fill::decl_fn, BD, ipred_z2_fill1, neon).call::<BD>(
                dst,
                stride,
                buf.as_ptr().offset(top_offset),
                buf.as_ptr().offset(left_offset),
                width,
                height,
                dx,
                dy,
            );
        } else if upsample_above {
            bd_fn!(z2_fill::decl_fn, BD, ipred_z2_fill2, neon).call::<BD>(
                dst,
                stride,
                buf.as_ptr().offset(top_offset),
                buf.as_ptr().offset(left_offset),
                width,
                height,
                dx,
                dy,
            );
        } else {
            bd_fn!(z2_fill::decl_fn, BD, ipred_z2_fill3, neon).call::<BD>(
                dst,
                stride,
                buf.as_ptr().offset(top_offset),
                buf.as_ptr().offset(left_offset),
                width,
                height,
                dx,
                dy,
            );
        };
    }

    unsafe fn ipred_z3_neon<BD: BitDepth>(
        dst: *mut BD::Pixel,
        stride: ptrdiff_t,
        topleft_in: *const BD::Pixel,
        width: c_int,
        height: c_int,
        mut angle: c_int,
        _max_width: c_int,
        _max_height: c_int,
        bd: BD,
    ) {
        let is_sm = (angle >> 9) & 1 != 0;
        let enable_intra_edge_filter = angle >> 10;
        angle &= 511 as c_int;
        if !(angle > 180) {
            unreachable!();
        }
        let mut dy = dav1d_dr_intra_derivative[(270 - angle >> 1) as usize] as c_int;
        let mut flipped: [BD::Pixel; 144] = [0.into(); 144];
        let mut left_out: [BD::Pixel; 286] = [0.into(); 286];
        let max_base_y;
        let upsample_left = if enable_intra_edge_filter != 0 {
            get_upsample(width + height, angle - 180, is_sm)
        } else {
            false
        };
        if upsample_left {
            flipped[0] = *topleft_in.offset(0);
            bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                flipped.as_mut_ptr().offset(1),
                topleft_in.offset(0),
                height + cmp::max(width, height),
            );
            bd_fn!(z1_upsample_edge::decl_fn, BD, ipred_z1_upsample_edge, neon).call(
                left_out.as_mut_ptr(),
                width + height,
                flipped.as_mut_ptr(),
                height + cmp::min(width, height),
                bd,
            );
            max_base_y = 2 * (width + height) - 2;
            dy <<= 1;
        } else {
            let filter_strength = if enable_intra_edge_filter != 0 {
                get_filter_strength(width + height, angle - 180, is_sm)
            } else {
                0 as c_int
            };
            if filter_strength != 0 {
                flipped[0] = *topleft_in.offset(0);
                bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                    flipped.as_mut_ptr().offset(1),
                    topleft_in.offset(0),
                    height + cmp::max(width, height),
                );
                bd_fn!(z1_filter_edge::decl_fn, BD, ipred_z1_filter_edge, neon).call::<BD>(
                    left_out.as_mut_ptr(),
                    width + height,
                    flipped.as_mut_ptr(),
                    height + cmp::min(width, height),
                    filter_strength,
                );
                max_base_y = width + height - 1;
            } else {
                bd_fn!(reverse::decl_fn, BD, ipred_reverse, neon).call::<BD>(
                    left_out.as_mut_ptr(),
                    topleft_in.offset(0),
                    height + cmp::min(width, height),
                );
                max_base_y = height + cmp::min(width, height) - 1;
            }
        }
        let base_inc = 1 + upsample_left as c_int;
        let pad_pixels = cmp::max(64 - max_base_y - 1, height + 15);
        rav1d_ipred_pixel_set_neon::<BD>(
            left_out.as_mut_ptr().offset((max_base_y + 1) as isize),
            left_out[max_base_y as usize],
            (pad_pixels * base_inc) as c_int,
        );
        if upsample_left {
            bd_fn!(z13_fill::decl_fn, BD, ipred_z3_fill2, neon).call::<BD>(
                dst,
                stride,
                left_out.as_mut_ptr(),
                width,
                height,
                dy,
                max_base_y,
            );
        } else {
            bd_fn!(z13_fill::decl_fn, BD, ipred_z3_fill1, neon).call::<BD>(
                dst,
                stride,
                left_out.as_mut_ptr(),
                width,
                height,
                dy,
                max_base_y,
            );
        };
    }

    pub unsafe extern "C" fn ipred_z_neon_erased<BD: BitDepth, const Z: usize>(
        dst: *mut DynPixel,
        stride: ptrdiff_t,
        topleft_in: *const DynPixel,
        width: c_int,
        height: c_int,
        angle: c_int,
        max_width: c_int,
        max_height: c_int,
        bitdepth_max: c_int,
        _topleft_off: usize,
    ) {
        [ipred_z1_neon, ipred_z2_neon, ipred_z3_neon][Z - 1](
            dst.cast(),
            stride,
            topleft_in.cast(),
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
