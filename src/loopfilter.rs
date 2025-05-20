#![deny(unsafe_op_in_unsafe_fn)]

use crate::align::Align16;
use crate::cpu::CpuFlags;
use crate::disjoint_mut::DisjointMut;
use crate::ffi_safe::FFISafe;
use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::DynPixel;
use crate::include::common::intops::iclip;
use crate::include::dav1d::picture::Rav1dPictureDataComponentOffset;
use crate::internal::Rav1dFrameData;
use crate::lf_mask::Av1FilterLUT;
use crate::strided::Strided as _;
use crate::with_offset::WithOffset;
use crate::wrap_fn_ptr::wrap_fn_ptr;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use strum::FromRepr;

#[cfg(all(
    feature = "asm",
    not(any(target_arch = "riscv64", target_arch = "riscv32"))
))]
use crate::include::common::bitdepth::bd_fn;

wrap_fn_ptr!(pub unsafe extern "C" fn loopfilter_sb(
    dst_ptr: *mut DynPixel,
    stride: ptrdiff_t,
    mask: &[u32; 3],
    lvl_ptr: *const [u8; 4],
    b4_stride: ptrdiff_t,
    lut: &Align16<Av1FilterLUT>,
    w: c_int,
    bitdepth_max: c_int,
    _dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    _lvl: *const FFISafe<WithOffset<&DisjointMut<Vec<u8>>>>,
) -> ());

impl loopfilter_sb::Fn {
    pub fn call<BD: BitDepth>(
        &self,
        f: &Rav1dFrameData,
        dst: Rav1dPictureDataComponentOffset,
        mask: &[u32; 3],
        lvl: WithOffset<&DisjointMut<Vec<u8>>>,
        w: usize,
    ) {
        let dst_ptr = dst.as_mut_ptr::<BD>().cast();
        let stride = dst.stride();
        assert!(lvl.offset <= lvl.data.len());
        // SAFETY: `lvl.offset` is in bounds, just checked above.
        let lvl_ptr = unsafe { lvl.data.as_mut_ptr().add(lvl.offset) };
        let lvl_ptr = lvl_ptr.cast::<[u8; 4]>();
        let b4_stride = f.b4_stride;
        let lut = &f.lf.lim_lut;
        let w = w as c_int;
        let bd = f.bitdepth_max;
        let dst = FFISafe::new(&dst);
        let lvl = FFISafe::new(&lvl);
        // SAFETY: Fallback `fn loop_filter_sb128_rust` is safe; asm is supposed to do the same.
        unsafe {
            self.get()(
                dst_ptr, stride, mask, lvl_ptr, b4_stride, lut, w, bd, dst, lvl,
            )
        }
    }

    const fn default<BD: BitDepth, const HV: usize, const YUV: usize>() -> Self {
        Self::new(loop_filter_sb128_c_erased::<BD, { HV }, { YUV }>)
    }
}

pub struct LoopFilterHVDSPContext {
    pub h: loopfilter_sb::Fn,
    pub v: loopfilter_sb::Fn,
}

pub struct LoopFilterYUVDSPContext {
    pub y: LoopFilterHVDSPContext,
    pub uv: LoopFilterHVDSPContext,
}

pub struct Rav1dLoopFilterDSPContext {
    pub loop_filter_sb: LoopFilterYUVDSPContext,
}

#[inline(never)]
fn loop_filter<BD: BitDepth>(
    dst: Rav1dPictureDataComponentOffset,
    e: u8,
    i: u8,
    h: u8,
    stridea: ptrdiff_t,
    strideb: ptrdiff_t,
    wd: c_int,
    bd: BD,
) {
    let bitdepth_min_8 = bd.bitdepth() - 8;
    let [f, e, i, h] = [1, e, i, h].map(|n| (n as i32) << bitdepth_min_8);

    for idx in 0..4 {
        let dst = dst + (idx * stridea);
        let dst = |stride_index: isize| (dst + (strideb * stride_index)).index_mut::<BD>();

        let get_dst = |stride_index| (*dst(stride_index)).as_::<i32>();
        let set_dst = |stride_index, pixel: i32| {
            *dst(stride_index) = pixel.as_::<BD::Pixel>();
        };
        let set_dst_clipped = |stride_index, pixel: i32| {
            *dst(stride_index) = bd.iclip_pixel(pixel);
        };

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
        let mut flat8out = false;
        let mut flat8in = false;

        let mut fm = (p1 - p0).abs() <= i
            && (q1 - q0).abs() <= i
            && (p0 - q0).abs() * 2 + ((p1 - q1).abs() >> 1) <= e;

        if wd > 4 {
            p2 = get_dst(-3);
            q2 = get_dst(2);

            fm &= (p2 - p1).abs() <= i && (q2 - q1).abs() <= i;

            if wd > 6 {
                p3 = get_dst(-4);
                q3 = get_dst(3);

                fm &= (p3 - p2).abs() <= i && (q3 - q2).abs() <= i;
            }
        }
        if !fm {
            continue;
        }

        if wd >= 16 {
            p6 = get_dst(-7);
            p5 = get_dst(-6);
            p4 = get_dst(-5);
            q4 = get_dst(4);
            q5 = get_dst(5);
            q6 = get_dst(6);

            flat8out = (p6 - p0).abs() <= f
                && (p5 - p0).abs() <= f
                && (p4 - p0).abs() <= f
                && (q4 - q0).abs() <= f
                && (q5 - q0).abs() <= f
                && (q6 - q0).abs() <= f;
        }

        if wd >= 6 {
            flat8in = (p2 - p0).abs() <= f
                && (p1 - p0).abs() <= f
                && (q1 - q0).abs() <= f
                && (q2 - q0).abs() <= f;
        }

        if wd >= 8 {
            flat8in &= (p3 - p0).abs() <= f && (q3 - q0).abs() <= f;
        }

        if wd >= 16 && flat8out && flat8in {
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
        } else if wd >= 8 && flat8in {
            set_dst(-3, p3 + p3 + p3 + 2 * p2 + p1 + p0 + q0 + 4 >> 3);
            set_dst(-2, p3 + p3 + p2 + 2 * p1 + p0 + q0 + q1 + 4 >> 3);
            set_dst(-1, p3 + p2 + p1 + 2 * p0 + q0 + q1 + q2 + 4 >> 3);
            set_dst(0, p2 + p1 + p0 + 2 * q0 + q1 + q2 + q3 + 4 >> 3);
            set_dst(1, p1 + p0 + q0 + 2 * q1 + q2 + q3 + q3 + 4 >> 3);
            set_dst(2, p0 + q0 + q1 + 2 * q2 + q3 + q3 + q3 + 4 >> 3);
        } else if wd == 6 && flat8in {
            set_dst(-2, p2 + 2 * p2 + 2 * p1 + 2 * p0 + q0 + 4 >> 3);
            set_dst(-1, p2 + 2 * p1 + 2 * p0 + 2 * q0 + q1 + 4 >> 3);
            set_dst(0, p1 + 2 * p0 + 2 * q0 + 2 * q1 + q2 + 4 >> 3);
            set_dst(1, p0 + 2 * q0 + 2 * q1 + 2 * q2 + q2 + 4 >> 3);
        } else {
            let hev = (p1 - p0).abs() > h || (q1 - q0).abs() > h;

            fn iclip_diff(v: c_int, bitdepth_min_8: u8) -> i32 {
                iclip(
                    v,
                    -128 * (1 << bitdepth_min_8),
                    128 * (1 << bitdepth_min_8) - 1,
                )
            }

            if hev {
                let f = iclip_diff(p1 - q1, bitdepth_min_8);
                let f = iclip_diff(3 * (q0 - p0) + f, bitdepth_min_8);

                let f1 = cmp::min(f + 4, (128 << bitdepth_min_8) - 1) >> 3;
                let f2 = cmp::min(f + 3, (128 << bitdepth_min_8) - 1) >> 3;

                set_dst_clipped(-1, p0 + f2);
                set_dst_clipped(0, q0 - f1);
            } else {
                let f = iclip_diff(3 * (q0 - p0), bitdepth_min_8);

                let f1 = cmp::min(f + 4, (128 << bitdepth_min_8) - 1) >> 3;
                let f2 = cmp::min(f + 3, (128 << bitdepth_min_8) - 1) >> 3;

                set_dst_clipped(-1, p0 + f2);
                set_dst_clipped(0, q0 - f1);

                let f = (f1 + 1) >> 1;
                set_dst_clipped(-2, p1 + f);
                set_dst_clipped(1, q1 - f);
            }
        }
    }
}

#[derive(FromRepr)]
enum HV {
    H,
    V,
}

#[derive(FromRepr)]
enum YUV {
    Y,
    UV,
}

fn loop_filter_sb128_rust<BD: BitDepth, const HV: usize, const YUV: usize>(
    mut dst: Rav1dPictureDataComponentOffset,
    vmask: &[u32; 3],
    mut lvl: WithOffset<&DisjointMut<Vec<u8>>>,
    b4_stride: usize,
    lut: &Align16<Av1FilterLUT>,
    _wh: c_int,
    bd: BD,
) {
    let hv = HV::from_repr(HV).unwrap();
    let yuv = YUV::from_repr(YUV).unwrap();

    let stride = dst.pixel_stride::<BD>();
    let (stridea, strideb) = match hv {
        HV::H => (stride, 1),
        HV::V => (1, stride),
    };
    let (b4_stridea, b4_strideb) = match hv {
        HV::H => (b4_stride, 1),
        HV::V => (1, b4_stride),
    };

    let vm = match yuv {
        YUV::Y => vmask[0] | vmask[1] | vmask[2],
        YUV::UV => vmask[0] | vmask[1],
    };
    let mut xy = 1u32;
    while vm & !xy.wrapping_sub(1) != 0 {
        'block: {
            if vm & xy == 0 {
                break 'block;
            }
            let l = *lvl.data.index(lvl.offset);
            let l = if l != 0 {
                l
            } else {
                let lvl = lvl - 4 * b4_strideb;
                *lvl.data.index(lvl.offset)
            };
            if l == 0 {
                break 'block;
            }
            let h = l >> 4;
            let e = lut.0.e[l as usize];
            let i = lut.0.i[l as usize];
            let idx = match yuv {
                YUV::Y => {
                    let idx = if vmask[2] & xy != 0 {
                        2
                    } else {
                        (vmask[1] & xy != 0) as c_int
                    };
                    4 << idx
                }
                YUV::UV => {
                    let idx = (vmask[1] & xy != 0) as c_int;
                    4 + 2 * idx
                }
            };
            loop_filter(dst, e, i, h, stridea, strideb, idx, bd);
        }
        xy <<= 1;
        dst += 4 * stridea;
        lvl += 4 * b4_stridea;
    }
}

/// # Safety
///
/// Must be called by [`loopfilter_sb::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn loop_filter_sb128_c_erased<BD: BitDepth, const HV: usize, const YUV: usize>(
    _dst_ptr: *mut DynPixel,
    _stride: ptrdiff_t,
    vmask: &[u32; 3],
    _lvl_ptr: *const [u8; 4],
    b4_stride: isize,
    lut: &Align16<Av1FilterLUT>,
    wh: c_int,
    bitdepth_max: c_int,
    dst: *const FFISafe<Rav1dPictureDataComponentOffset>,
    lvl: *const FFISafe<WithOffset<&DisjointMut<Vec<u8>>>>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `loopfilter_sb::Fn::call`.
    let dst = *unsafe { FFISafe::get(dst) };
    // SAFETY: Was passed as `FFISafe::new(_)` in `loopfilter_sb::Fn::call`.
    let lvl = *unsafe { FFISafe::get(lvl) };
    let b4_stride = b4_stride as usize;
    let bd = BD::from_c(bitdepth_max);
    loop_filter_sb128_rust::<BD, { HV }, { YUV }>(dst, vmask, lvl, b4_stride, lut, wh, bd)
}

impl Rav1dLoopFilterDSPContext {
    pub const fn default<BD: BitDepth>() -> Self {
        use HV::*;
        use YUV::*;
        Self {
            loop_filter_sb: LoopFilterYUVDSPContext {
                y: LoopFilterHVDSPContext {
                    h: loopfilter_sb::Fn::default::<BD, { H as _ }, { Y as _ }>(),
                    v: loopfilter_sb::Fn::default::<BD, { V as _ }, { Y as _ }>(),
                },
                uv: LoopFilterHVDSPContext {
                    h: loopfilter_sb::Fn::default::<BD, { H as _ }, { UV as _ }>(),
                    v: loopfilter_sb::Fn::default::<BD, { V as _ }, { UV as _ }>(),
                },
            },
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86<BD: BitDepth>(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.loop_filter_sb.y.h = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_y, ssse3);
        self.loop_filter_sb.y.v = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_y, ssse3);
        self.loop_filter_sb.uv.h = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_uv, ssse3);
        self.loop_filter_sb.uv.v = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_uv, ssse3);

        #[cfg(target_arch = "x86_64")]
        {
            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.loop_filter_sb.y.h = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_y, avx2);
            self.loop_filter_sb.y.v = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_y, avx2);
            self.loop_filter_sb.uv.h = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_uv, avx2);
            self.loop_filter_sb.uv.v = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_uv, avx2);

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            self.loop_filter_sb.y.v = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_y, avx512icl);
            self.loop_filter_sb.uv.v = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_uv, avx512icl);

            if !flags.contains(CpuFlags::SLOW_GATHER) {
                self.loop_filter_sb.y.h = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_y, avx512icl);
                self.loop_filter_sb.uv.h =
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

        self.loop_filter_sb.y.h = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_y, neon);
        self.loop_filter_sb.y.v = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_y, neon);
        self.loop_filter_sb.uv.h = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_h_sb_uv, neon);
        self.loop_filter_sb.uv.v = bd_fn!(loopfilter_sb::decl_fn, BD, lpf_v_sb_uv, neon);

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
