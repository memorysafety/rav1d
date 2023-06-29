use crate::include::common::bitdepth::BitDepth;
#[cfg(feature = "bitdepth_16")]
use crate::include::common::bitdepth::BitDepth16;
#[cfg(feature = "bitdepth_8")]
use crate::include::common::bitdepth::BitDepth8;
use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::int16_t;
use crate::include::stdint::uint32_t;
use crate::src::align::Align16;
use crate::src::cpu::FnVersion;

pub type LrEdgeFlags = libc::c_uint;
pub const LR_HAVE_BOTTOM: LrEdgeFlags = 8;
pub const LR_HAVE_TOP: LrEdgeFlags = 4;
pub const LR_HAVE_RIGHT: LrEdgeFlags = 2;
pub const LR_HAVE_LEFT: LrEdgeFlags = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LooprestorationParams_sgr {
    pub s0: uint32_t,
    pub s1: uint32_t,
    pub w0: int16_t,
    pub w1: int16_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union LooprestorationParams {
    pub filter: Align16<[[int16_t; 8]; 2]>,
    pub sgr: LooprestorationParams_sgr,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: FnVersion,
    pub sgr: FnVersion,
}

impl Dav1dLoopRestorationDSPContext {
    pub unsafe fn call<BD: BitDepthFnLoopRestoration>(
        &self,
        filter: FilterFn,
        mut p: *mut BD::Pixel,
        stride: ptrdiff_t,
        left: *const [BD::Pixel; 4],
        mut lpf: *const BD::Pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
        bd: BD,
    ) {
        use FilterFn::*;

        let fn_version = match filter {
            Wiener5 | Wiener7 => self.wiener,
            Sgr5x5 | Sgr3x3 | SgrMix => self.sgr,
        };

        bd.call(
            fn_version, filter, p, stride, left, lpf, w, h, params, edges,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FilterFn {
    Wiener7,
    Wiener5,
    Sgr5x5,
    Sgr3x3,
    SgrMix,
}

impl FilterFn {
    /// Converts a raw Wiener filter index into the corresponding filter type.
    pub fn wiener(raw: usize) -> Self {
        match raw {
            0 => Self::Wiener7,
            1 => Self::Wiener5,
            _ => unreachable!(),
        }
    }

    /// Converts a raw SGR filter index into the corresponding filter type.
    pub fn sgr(raw: usize) -> Self {
        match raw {
            0 => Self::Sgr5x5,
            1 => Self::Sgr3x3,
            2 => Self::SgrMix,
            _ => unreachable!(),
        }
    }
}

pub trait BitDepthFnLoopRestoration: BitDepth {
    unsafe fn call(
        &self,
        version: FnVersion,
        filter: FilterFn,
        p: *mut Self::Pixel,
        stride: ptrdiff_t,
        left: *const [Self::Pixel; 4],
        lpf: *const Self::Pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
    );
}

#[cfg(feature = "bitdepth_8")]
impl BitDepthFnLoopRestoration for BitDepth8 {
    unsafe fn call(
        &self,
        version: FnVersion,
        filter: FilterFn,
        p: *mut Self::Pixel,
        stride: ptrdiff_t,
        left: *const [Self::Pixel; 4],
        lpf: *const Self::Pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
    ) {
        #[cfg(feature = "asm")]
        use crate::src::cpu::FnAsmVersion::*;
        use crate::src::looprestoration_tmpl_8::{sgr_3x3_c, sgr_5x5_c, sgr_mix_c, wiener_c};
        #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
        use crate::src::looprestoration_tmpl_8::{
            sgr_filter_3x3_neon, sgr_filter_5x5_neon, sgr_filter_mix_neon,
        };
        use FilterFn::*;

        macro_rules! call_fn {
            ($name:ident) => {
                $name(p, stride, left, lpf, w, h, params, edges)
            };
        }

        #[cfg(all(feature = "asm", not(target_arch = "arm")))]
        macro_rules! extern_fn {
            ($name:ident) => {{
                extern "C" {
                    fn $name(
                        p: *mut <BitDepth8 as BitDepth>::Pixel,
                        stride: ptrdiff_t,
                        left: *const [<BitDepth8 as BitDepth>::Pixel; 4],
                        lpf: *const <BitDepth8 as BitDepth>::Pixel,
                        w: libc::c_int,
                        h: libc::c_int,
                        params: *const LooprestorationParams,
                        edges: LrEdgeFlags,
                    );
                }

                call_fn!($name)
            }};
        }

        macro_rules! wiener_fn {
            ($version:expr, $name:ident) => {{
                paste::paste! {
                    match $version {
                        FnVersion::Rust => call_fn!(wiener_c),

                        #[cfg(feature = "asm")]
                        FnVersion::Asm(asm) => match asm {
                            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                            SSE2 => extern_fn!([<dav1d_wiener $name _8bpc_sse2>]),
                            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                            SSSE3 => extern_fn!([<dav1d_wiener $name _8bpc_ssse3>]),
                            #[cfg(target_arch = "x86_64")]
                            AVX2 => extern_fn!([<dav1d_wiener $name _8bpc_avx2>]),
                            #[cfg(target_arch = "x86_64")]
                            AVX512ICL => extern_fn!(
                                // With VNNI we don't need a 5-tap version.
                                dav1d_wiener_filter7_8bpc_avx512icl
                            ),

                            #[cfg(any(/* target_arch = "arm", */ target_arch = "aarch64"))]
                            Neon => extern_fn!([<dav1d_wiener $name _8bpc_neon>]),

                            // TODO(perl): enable 32 bit arm assembly routines here
                            #[cfg(target_arch = "arm")]
                            Neon => unreachable!(),
                        }
                    }
                }
            }};
        }

        macro_rules! sgr_fn {
            ($version:expr, $name:ident) => {{
                paste::paste! {
                    match $version {
                        FnVersion::Rust => call_fn!([<sgr $name _c>]),

                        #[cfg(feature = "asm")]
                        FnVersion::Asm(asm) => match asm {
                            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                            SSSE3 => extern_fn!([<dav1d_sgr_filter $name _8bpc_ssse3>]),
                            #[cfg(target_arch = "x86_64")]
                            AVX2 => extern_fn!([<dav1d_sgr_filter $name _8bpc_avx2>]),
                            #[cfg(target_arch = "x86_64")]
                            AVX512ICL => extern_fn!([<dav1d_sgr_filter $name _8bpc_avx512icl>]),
                            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                            Neon => call_fn!([<sgr_filter $name _neon>]),

                            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                            SSE2 => unreachable!(),
                        }
                    }
                }
            }};
        }

        match filter {
            Wiener7 => wiener_fn!(version, _filter7),
            Wiener5 => wiener_fn!(version, _filter5),
            Sgr5x5 => sgr_fn!(version, _5x5),
            Sgr3x3 => sgr_fn!(version, _3x3),
            SgrMix => sgr_fn!(version, _mix),
        }
    }
}

#[cfg(feature = "bitdepth_16")]
impl BitDepthFnLoopRestoration for BitDepth16 {
    unsafe fn call(
        &self,
        version: FnVersion,
        filter: FilterFn,
        p: *mut Self::Pixel,
        stride: ptrdiff_t,
        left: *const [Self::Pixel; 4],
        lpf: *const Self::Pixel,
        w: libc::c_int,
        h: libc::c_int,
        params: *const LooprestorationParams,
        edges: LrEdgeFlags,
    ) {
        #[cfg(feature = "asm")]
        use crate::src::cpu::FnAsmVersion::*;
        use crate::src::looprestoration_tmpl_16::{sgr_3x3_c, sgr_5x5_c, sgr_mix_c, wiener_c};
        #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
        use crate::src::looprestoration_tmpl_16::{
            sgr_filter_3x3_neon, sgr_filter_5x5_neon, sgr_filter_mix_neon,
        };
        use FilterFn::*;

        macro_rules! call_fn {
            ($name:ident) => {
                $name(
                    p,
                    stride,
                    left,
                    lpf,
                    w,
                    h,
                    params,
                    edges,
                    self.bitdepth_max().into(),
                )
            };
        }

        #[cfg(all(feature = "asm", not(target_arch = "arm")))]
        macro_rules! extern_fn {
            ($name:ident) => {{
                extern "C" {
                    fn $name(
                        p: *mut <BitDepth16 as BitDepth>::Pixel,
                        stride: ptrdiff_t,
                        left: *const [<BitDepth16 as BitDepth>::Pixel; 4],
                        lpf: *const <BitDepth16 as BitDepth>::Pixel,
                        w: libc::c_int,
                        h: libc::c_int,
                        params: *const LooprestorationParams,
                        edges: LrEdgeFlags,
                        bitdepth_max: libc::c_int,
                    );
                }

                call_fn!($name)
            }};
        }

        macro_rules! wiener_fn {
            ($version:expr, $name:ident) => {{
                paste::paste! {
                    match $version {
                        FnVersion::Rust => call_fn!(wiener_c),

                        #[cfg(feature = "asm")]
                        FnVersion::Asm(asm) => match asm {
                            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                            SSSE3 => extern_fn!([<dav1d_wiener $name _16bpc_ssse3>]),
                            #[cfg(target_arch = "x86_64")]
                            AVX2 => extern_fn!([<dav1d_wiener $name _16bpc_avx2>]),
                            #[cfg(target_arch = "x86_64")]
                            AVX512ICL => extern_fn!([<dav1d_wiener $name _16bpc_avx512icl>]),

                            #[cfg(any(/* target_arch = "arm", */ target_arch = "aarch64"))]
                            Neon => extern_fn!([<dav1d_wiener $name _16bpc_neon>]),

                            // TODO(perl): enable 32 bit arm assembly routines here
                            #[cfg(target_arch = "arm")]
                            Neon => unreachable!(),

                            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                            SSE2 => unreachable!(),
                        }
                    }
                }
            }};
        }

        macro_rules! sgr_fn {
            ($version:expr, $name:ident) => {{
                paste::paste! {
                    match $version {
                        FnVersion::Rust => call_fn!([<sgr $name _c>]),

                        #[cfg(feature = "asm")]
                        FnVersion::Asm(asm) => match asm {
                            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                            SSSE3 => extern_fn!([<dav1d_sgr_filter $name _16bpc_ssse3>]),
                            #[cfg(target_arch = "x86_64")]
                            AVX2 => extern_fn!([<dav1d_sgr_filter $name _16bpc_avx2>]),
                            #[cfg(target_arch = "x86_64")]
                            AVX512ICL => extern_fn!([<dav1d_sgr_filter $name _16bpc_avx512icl>]),
                            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                            Neon => call_fn!([<sgr_filter $name _neon>]),

                            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                            SSE2 => unreachable!(),
                        }

                    }
                }
            }};
        }

        match filter {
            Wiener7 => wiener_fn!(version, _filter7),
            Wiener5 => wiener_fn!(version, _filter5),
            Sgr5x5 => sgr_fn!(version, _5x5),
            Sgr3x3 => sgr_fn!(version, _3x3),
            SgrMix => sgr_fn!(version, _mix),
        }
    }
}
