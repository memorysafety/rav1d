use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::src::align::Align16;
use crate::src::cpu::CpuFlags;
use crate::src::internal::Rav1dFrameData;
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::wrap_fn_ptr::wrap_fn_ptr;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

wrap_fn_ptr!(pub unsafe extern "C" fn loopfilter_sb(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    mask: &[u32; 3],
    lvl: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: &Align16<Av1FilterLUT>,
    w: c_int,
    bitdepth_max: c_int,
) -> ());

impl loopfilter_sb::Fn {
    pub unsafe fn call<BD: BitDepth>(
        &self,
        f: &Rav1dFrameData,
        dst: *mut BD::Pixel,
        stride: ptrdiff_t,
        mask: &[u32; 3],
        lvl: &[[u8; 4]],
        w: c_int,
    ) {
        let dst = dst.cast();
        let lvl = lvl.as_ptr();
        let b4_stride = f.b4_stride;
        let lut = &f.lf.lim_lut;
        let bd = f.bitdepth_max;
        self.get()(dst, stride, mask, lvl, b4_stride, lut, w, bd)
    }
}

pub struct Rav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb::Fn; 2]; 2],
}

#[inline(never)]
unsafe fn loop_filter<BD: BitDepth>(
    dst: *mut BD::Pixel,
    mut E: c_int,
    mut I: c_int,
    mut H: c_int,
    stridea: ptrdiff_t,
    strideb: ptrdiff_t,
    wd: c_int,
    bd: BD,
) {
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let F = 1 << bitdepth_min_8;
    E <<= bitdepth_min_8;
    I <<= bitdepth_min_8;
    H <<= bitdepth_min_8;
    for i in 0..4 {
        let dst = dst.offset(i * stridea);
        let dst = |stride_index| &mut *dst.offset(strideb * stride_index);
        let get_dst = |stride_index| (*dst(stride_index)).as_::<i32>();
        let set_dst = |stride_index, value: i32| *dst(stride_index) = value.as_::<BD::Pixel>();

        let mut p6 = 0;
        let mut p5 = 0;
        let mut p4 = 0;
        let mut p3 = 0;
        let mut p2 = 0;
        let p1 = get_dst(-2);
        let p0 = get_dst(-1);
        let q0 = get_dst(0);
        let q1 = get_dst(1);
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        let mut q5 = 0;
        let mut q6 = 0;
        let mut fm;
        let mut flat8out = 0;
        let mut flat8in = 0;
        fm = ((p1 - p0).abs() <= I
            && (q1 - q0).abs() <= I
            && (p0 - q0).abs() * 2 + ((p1 - q1).abs() >> 1) <= E) as c_int;
        if wd > 4 {
            p2 = get_dst(-3);
            q2 = get_dst(2);
            fm &= ((p2 - p1).abs() <= I && (q2 - q1).abs() <= I) as c_int;
            if wd > 6 {
                p3 = get_dst(-4);
                q3 = get_dst(3);
                fm &= ((p3 - p2).abs() <= I && (q3 - q2).abs() <= I) as c_int;
            }
        }
        if fm == 0 {
            continue;
        }
        if wd >= 16 {
            p6 = get_dst(-7);
            p5 = get_dst(-6);
            p4 = get_dst(-5);
            q4 = get_dst(4);
            q5 = get_dst(5);
            q6 = get_dst(6);
            flat8out = ((p6 - p0).abs() <= F
                && (p5 - p0).abs() <= F
                && (p4 - p0).abs() <= F
                && (q4 - q0).abs() <= F
                && (q5 - q0).abs() <= F
                && (q6 - q0).abs() <= F) as c_int;
        }
        if wd >= 6 {
            flat8in = ((p2 - p0).abs() <= F
                && (p1 - p0).abs() <= F
                && (q1 - q0).abs() <= F
                && (q2 - q0).abs() <= F) as c_int;
        }
        if wd >= 8 {
            flat8in &= ((p3 - p0).abs() <= F && (q3 - q0).abs() <= F) as c_int;
        }
        if wd >= 16 && flat8out & flat8in != 0 {
            set_dst(
                -6,
                p6 + p6 + p6 + p6 + p6 + p6 * 2 + p5 * 2 + p4 * 2 + p3 + p2 + p1 + p0 + q0 + 8 >> 4,
            );
            set_dst(
                -5,
                p6 + p6 + p6 + p6 + p6 + p5 * 2 + p4 * 2 + p3 * 2 + p2 + p1 + p0 + q0 + q1 + 8 >> 4,
            );
            set_dst(
                -4,
                p6 + p6 + p6 + p6 + p5 + p4 * 2 + p3 * 2 + p2 * 2 + p1 + p0 + q0 + q1 + q2 + 8 >> 4,
            );
            set_dst(
                -3,
                p6 + p6 + p6 + p5 + p4 + p3 * 2 + p2 * 2 + p1 * 2 + p0 + q0 + q1 + q2 + q3 + 8 >> 4,
            );
            set_dst(
                -2,
                p6 + p6 + p5 + p4 + p3 + p2 * 2 + p1 * 2 + p0 * 2 + q0 + q1 + q2 + q3 + q4 + 8 >> 4,
            );
            set_dst(
                -1,
                p6 + p5 + p4 + p3 + p2 + p1 * 2 + p0 * 2 + q0 * 2 + q1 + q2 + q3 + q4 + q5 + 8 >> 4,
            );
            set_dst(
                0,
                p5 + p4 + p3 + p2 + p1 + p0 * 2 + q0 * 2 + q1 * 2 + q2 + q3 + q4 + q5 + q6 + 8 >> 4,
            );
            set_dst(
                1,
                p4 + p3 + p2 + p1 + p0 + q0 * 2 + q1 * 2 + q2 * 2 + q3 + q4 + q5 + q6 + q6 + 8 >> 4,
            );
            set_dst(
                2,
                p3 + p2 + p1 + p0 + q0 + q1 * 2 + q2 * 2 + q3 * 2 + q4 + q5 + q6 + q6 + q6 + 8 >> 4,
            );
            set_dst(
                3,
                p2 + p1 + p0 + q0 + q1 + q2 * 2 + q3 * 2 + q4 * 2 + q5 + q6 + q6 + q6 + q6 + 8 >> 4,
            );
            set_dst(
                4,
                p1 + p0 + q0 + q1 + q2 + q3 * 2 + q4 * 2 + q5 * 2 + q6 + q6 + q6 + q6 + q6 + 8 >> 4,
            );
            set_dst(
                5,
                p0 + q0 + q1 + q2 + q3 + q4 * 2 + q5 * 2 + q6 * 2 + q6 + q6 + q6 + q6 + q6 + 8 >> 4,
            );
        } else if wd >= 8 && flat8in != 0 {
            set_dst(-3, p3 + p3 + p3 + 2 * p2 + p1 + p0 + q0 + 4 >> 3);
            set_dst(-2, p3 + p3 + p2 + 2 * p1 + p0 + q0 + q1 + 4 >> 3);
            set_dst(-1, p3 + p2 + p1 + 2 * p0 + q0 + q1 + q2 + 4 >> 3);
            set_dst(0, p2 + p1 + p0 + 2 * q0 + q1 + q2 + q3 + 4 >> 3);
            set_dst(1, p1 + p0 + q0 + 2 * q1 + q2 + q3 + q3 + 4 >> 3);
            set_dst(2, p0 + q0 + q1 + 2 * q2 + q3 + q3 + q3 + 4 >> 3);
        } else if wd == 6 && flat8in != 0 {
            set_dst(-2, p2 + 2 * p2 + 2 * p1 + 2 * p0 + q0 + 4 >> 3);
            set_dst(-1, p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4 >> 3);
            set_dst(0, p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4 >> 3);
            set_dst(1, p0 + 2 * q0 + 2 * q1 + 2 * q2 + q2 + 4 >> 3);
        } else {
            let hev = ((p1 - p0).abs() > H || (q1 - q0).abs() > H) as c_int;

            fn iclip_diff(v: c_int, bitdepth_min_8: u8) -> i32 {
                iclip(
                    v,
                    -128 * (1 << bitdepth_min_8),
                    128 * (1 << bitdepth_min_8) - 1,
                )
            }

            if hev != 0 {
                let mut f = iclip_diff(p1 - q1, bitdepth_min_8);
                f = iclip_diff(3 * (q0 - p0) + f, bitdepth_min_8);

                let f1 = cmp::min(f + 4, (128 << bitdepth_min_8) - 1) >> 3;
                let f2 = cmp::min(f + 3, (128 << bitdepth_min_8) - 1) >> 3;

                *dst(-1) = bd.iclip_pixel(p0 + f2);
                *dst(0) = bd.iclip_pixel(q0 - f1);
            } else {
                let mut f = iclip_diff(3 * (q0 - p0), bitdepth_min_8);

                let f1 = cmp::min(f + 4, (128 << bitdepth_min_8) - 1) >> 3;
                let f2 = cmp::min(f + 3, (128 << bitdepth_min_8) - 1) >> 3;

                *dst(-1) = bd.iclip_pixel(p0 + f2);
                *dst(0) = bd.iclip_pixel(q0 - f1);

                f = (f1 + 1) >> 1;
                *dst(-2) = bd.iclip_pixel(p1 + f);
                *dst(1) = bd.iclip_pixel(q1 - f);
            }
        }
    }
}

unsafe extern "C" fn loop_filter_h_sb128y_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: &[u32; 3],
    l: *const [u8; 4],
    b4_stride: isize,
    lut: &Align16<Av1FilterLUT>,
    h: c_int,
    bitdepth_max: c_int,
) {
    let dst = dst.cast();
    let b4_stride = b4_stride as usize;
    let bd = BD::from_c(bitdepth_max);
    loop_filter_h_sb128y_rust(dst, stride, vmask, l, b4_stride, lut, h, bd)
}

unsafe fn loop_filter_h_sb128y_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    vmask: &[u32; 3],
    mut l: *const [u8; 4],
    b4_stride: usize,
    lut: &Align16<Av1FilterLUT>,
    _h: c_int,
    bd: BD,
) {
    let vm = vmask[0] | vmask[1] | vmask[2];
    let mut y = 1u32;
    while vm & !y.wrapping_sub(1) != 0 {
        if vm & y != 0 {
            let L = if (*l.offset(0))[0] != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.offset(-1))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = lut.0.e[L as usize] as c_int;
                let I = lut.0.i[L as usize] as c_int;
                let idx = if vmask[2] & y != 0 {
                    2
                } else {
                    (vmask[1] & y != 0) as c_int
                };
                loop_filter(dst, E, I, H, BD::pxstride(stride), 1, 4 << idx, bd);
            }
        }
        y <<= 1;
        dst = dst.offset(4 * BD::pxstride(stride));
        l = l.add(b4_stride);
    }
}

unsafe extern "C" fn loop_filter_v_sb128y_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: &[u32; 3],
    l: *const [u8; 4],
    b4_stride: isize,
    lut: &Align16<Av1FilterLUT>,
    w: c_int,
    bitdepth_max: c_int,
) {
    let dst = dst.cast();
    let b4_stride = b4_stride as usize;
    let bd = BD::from_c(bitdepth_max);
    loop_filter_v_sb128y_rust(dst, stride, vmask, l, b4_stride, lut, w, bd);
}

unsafe fn loop_filter_v_sb128y_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    vmask: &[u32; 3],
    mut l: *const [u8; 4],
    b4_stride: usize,
    lut: &Align16<Av1FilterLUT>,
    _w: c_int,
    bd: BD,
) {
    let vm = vmask[0] | vmask[1] | vmask[2];
    let mut x = 1u32;
    while vm & !x.wrapping_sub(1) != 0 {
        if vm & x != 0 {
            let L = if (*l.offset(0))[0] != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.sub(b4_stride))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = lut.0.e[L as usize] as c_int;
                let I = lut.0.i[L as usize] as c_int;
                let idx = if vmask[2] & x != 0 {
                    2
                } else {
                    (vmask[1] & x != 0) as c_int
                };
                loop_filter(dst, E, I, H, 1, BD::pxstride(stride), 4 << idx, bd);
            }
        }
        x <<= 1;
        dst = dst.offset(4);
        l = l.offset(1);
    }
}

unsafe extern "C" fn loop_filter_h_sb128uv_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: &[u32; 3],
    l: *const [u8; 4],
    b4_stride: isize,
    lut: &Align16<Av1FilterLUT>,
    h: c_int,
    bitdepth_max: c_int,
) {
    let dst = dst.cast();
    let b4_stride = b4_stride as usize;
    let bd = BD::from_c(bitdepth_max);
    loop_filter_h_sb128uv_rust(dst, stride, vmask, l, b4_stride, lut, h, bd)
}

unsafe fn loop_filter_h_sb128uv_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    vmask: &[u32; 3],
    mut l: *const [u8; 4],
    b4_stride: usize,
    lut: &Align16<Av1FilterLUT>,
    _h: c_int,
    bd: BD,
) {
    let vm = vmask[0] | vmask[1];
    let mut y = 1u32;
    while vm & !y.wrapping_sub(1) != 0 {
        if vm & y != 0 {
            let L = if (*l.offset(0))[0] != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.offset(-1))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = lut.0.e[L as usize] as c_int;
                let I = lut.0.i[L as usize] as c_int;
                let idx = (vmask[1] & y != 0) as c_int;
                loop_filter(dst, E, I, H, BD::pxstride(stride), 1, 4 + 2 * idx, bd);
            }
        }
        y <<= 1;
        dst = dst.offset(4 * BD::pxstride(stride));
        l = l.add(b4_stride);
    }
}

unsafe extern "C" fn loop_filter_v_sb128uv_c_erased<BD: BitDepth>(
    dst: *mut DynPixel,
    stride: ptrdiff_t,
    vmask: &[u32; 3],
    l: *const [u8; 4],
    b4_stride: isize,
    lut: &Align16<Av1FilterLUT>,
    w: c_int,
    bitdepth_max: c_int,
) {
    let dst = dst.cast();
    let b4_stride = b4_stride as usize;
    let bd = BD::from_c(bitdepth_max);
    loop_filter_v_sb128uv_rust(dst, stride, vmask, l, b4_stride, lut, w, bd)
}

unsafe fn loop_filter_v_sb128uv_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    vmask: &[u32; 3],
    mut l: *const [u8; 4],
    b4_stride: usize,
    lut: &Align16<Av1FilterLUT>,
    _w: c_int,
    bd: BD,
) {
    let vm = vmask[0] | vmask[1];
    let mut x = 1u32;
    while vm & !x.wrapping_sub(1) != 0 {
        if vm & x != 0 {
            let L = if (*l.offset(0))[0] != 0 {
                (*l.offset(0))[0] as c_int
            } else {
                (*l.sub(b4_stride))[0] as c_int
            };
            if !(L == 0) {
                let H = L >> 4;
                let E = lut.0.e[L as usize] as c_int;
                let I = lut.0.i[L as usize] as c_int;
                let idx = (vmask[1] & x != 0) as c_int;
                loop_filter(dst, E, I, H, 1, BD::pxstride(stride), 4 + 2 * idx, bd);
            }
        }
        x <<= 1;
        dst = dst.offset(4);
        l = l.offset(1);
    }
}

impl Rav1dLoopFilterDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        Self {
            loop_filter_sb: [
                [
                    loopfilter_sb::Fn::new(loop_filter_h_sb128y_c_erased::<BD>),
                    loopfilter_sb::Fn::new(loop_filter_v_sb128y_c_erased::<BD>),
                ],
                [
                    loopfilter_sb::Fn::new(loop_filter_h_sb128uv_c_erased::<BD>),
                    loopfilter_sb::Fn::new(loop_filter_v_sb128uv_c_erased::<BD>),
                ],
            ],
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.loop_filter_sb[0][0] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_y, ssse3);
        self.loop_filter_sb[0][1] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_y, ssse3);
        self.loop_filter_sb[1][0] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_uv, ssse3);
        self.loop_filter_sb[1][1] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_uv, ssse3);

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.loop_filter_sb[0][0] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_y, avx2);
            self.loop_filter_sb[0][1] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_y, avx2);
            self.loop_filter_sb[1][0] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_uv, avx2);
            self.loop_filter_sb[1][1] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_uv, avx2);

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            self.loop_filter_sb[0][1] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_y, avx512icl);
            self.loop_filter_sb[1][1] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_uv, avx512icl);

            if !flags.contains(CpuFlags::SLOW_GATHER) {
                self.loop_filter_sb[0][0] =
                    bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_y, avx512icl);
                self.loop_filter_sb[1][0] =
                    bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_uv, avx512icl);
            }
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        self.loop_filter_sb[0][0] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_y, neon);
        self.loop_filter_sb[0][1] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_y, neon);
        self.loop_filter_sb[1][0] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_uv, neon);
        self.loop_filter_sb[1][1] = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_uv, neon);

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
