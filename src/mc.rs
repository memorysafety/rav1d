use std::iter;

use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::BitDepth8;
use crate::src::levels::Filter2d8Tap;
use crate::src::levels::Filter2dRust;
use crate::src::levels::Filter8Tap;
use crate::src::tables::dav1d_mc_subpel_filters;

#[cfg(feature = "asm")]
use {libc::ptrdiff_t, paste::paste, std::ffi::c_int};

#[cfg(feature = "asm")]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FnAsmVersion {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    SSE2,
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    SSSE3,
    #[cfg(target_arch = "x86_64")]
    AVX2,
    #[cfg(target_arch = "x86_64")]
    AVX512ICL,
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    Neon,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum FnVersion {
    #[default]
    Rust,
    #[cfg(feature = "asm")]
    Asm(FnAsmVersion),
}

/// [`BitDepthFnAsmMc`] has to be `pub`, but we don't want anyone else calling its methods,
/// so require this public [`Token`] with a private constructor.
pub struct Token(());

pub trait BitDepthFnAsmMc: BitDepth {
    #[cfg(feature = "asm")]
    unsafe fn mc(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        dst: *mut Self::Pixel,
        dst_stride: usize,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
    );

    #[cfg(feature = "asm")]
    unsafe fn mct(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        tmp: *mut i16,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
    );

    #[cfg(feature = "asm")]
    unsafe fn mc_scaled(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        dst: *mut Self::Pixel,
        dst_stride: usize,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
        dx: usize,
        dy: usize,
    );

    #[cfg(feature = "asm")]
    unsafe fn mct_scaled(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        tmp: *mut i16,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
        dx: usize,
        dy: usize,
    );
}

impl BitDepthFnAsmMc for BitDepth8 {
    #[cfg(feature = "asm")]
    unsafe fn mc(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        dst: *mut Self::Pixel,
        dst_stride: usize,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
    ) {
        let [dst_stride, src_stride] = [dst_stride, src_stride].map(|it: usize| it as ptrdiff_t);
        let [w, h, mx, my] = [w, h, mx, my].map(|it| it as c_int);
        let args = (dst, dst_stride, src, src_stride, w, h, mx, my);

        macro_rules! extern_fn {
            ($args:expr, $name:ident) => {{
                extern "C" {
                    fn $name(
                        dst: *mut <BitDepth8 as BitDepth>::Pixel,
                        dst_stride: ptrdiff_t,
                        src: *const <BitDepth8 as BitDepth>::Pixel,
                        src_stride: ptrdiff_t,
                        w: c_int,
                        h: c_int,
                        mx: c_int,
                        my: c_int,
                    );
                }

                let (dst, dst_stride, src, src_stride, w, h, mx, my) = args;
                $name(dst, dst_stride, src, src_stride, w, h, mx, my)
            }};
            ($args:expr, $asm:expr, unreachable) => {{
                let _ = args;
                unreachable!("{asm:?}");
            }};
        }

        macro_rules! asm_fn {
            ($asm:expr, $args:expr, $name:ident) => {{
                use FnAsmVersion::*;
                paste! {
                    match asm {
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSE2 => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSSE3 => extern_fn!($args, [<dav1d_put $name _8bpc_ssse3>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX2 => extern_fn!($args, [<dav1d_put $name _8bpc_avx2>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX512ICL => extern_fn!($args, [<dav1d_put $name _8bpc_avx512icl>]),
                        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                        Neon => extern_fn!($args, [<dav1d_put $name _8bpc_neon>]),
                    }
                }
            }};
        }

        use Filter8Tap::*;
        match filter_2d {
            Filter2dRust::Tap8(Filter2d8Tap { h, v }) => match (h, v) {
                (Regular, Regular) => asm_fn!(asm, args, _8tap_regular),
                (Regular, Smooth) => asm_fn!(asm, args, _8tap_regular_smooth),
                (Regular, Sharp) => asm_fn!(asm, args, _8tap_regular_sharp),
                (Smooth, Regular) => asm_fn!(asm, args, _8tap_smooth_regular),
                (Smooth, Smooth) => asm_fn!(asm, args, _8tap_smooth),
                (Smooth, Sharp) => asm_fn!(asm, args, _8tap_smooth_sharp),
                (Sharp, Regular) => asm_fn!(asm, args, _8tap_sharp_regular),
                (Sharp, Smooth) => asm_fn!(asm, args, _8tap_sharp_smooth),
                (Sharp, Sharp) => asm_fn!(asm, args, _8tap_sharp),
            },
            Filter2dRust::BiLinear => asm_fn!(asm, args, _bilin),
        }
    }

    #[cfg(feature = "asm")]
    unsafe fn mct(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        tmp: *mut i16,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
    ) {
        let src_stride = src_stride as ptrdiff_t;
        let [w, h, mx, my] = [w, h, mx, my].map(|it| it as c_int);
        let args = (tmp, src, src_stride, w, h, mx, my);

        macro_rules! extern_fn {
            ($args:expr, $name:ident) => {{
                extern "C" {
                    fn $name(
                        tmp: *mut i16,
                        src: *const <BitDepth8 as BitDepth>::Pixel,
                        src_stride: ptrdiff_t,
                        w: c_int,
                        h: c_int,
                        mx: c_int,
                        my: c_int,
                    );
                }

                let (tmp, src, src_stride, w, h, mx, my) = args;
                $name(tmp, src, src_stride, w, h, mx, my)
            }};
            ($args:expr, $asm:expr, unreachable) => {{
                let _ = args;
                unreachable!("{asm:?}");
            }};
        }

        macro_rules! asm_fn {
            ($asm:expr, $args:expr, $name:ident) => {{
                use FnAsmVersion::*;
                paste! {
                    match asm {
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSE2 => extern_fn!($args, [<dav1d_prep $name _8bpc_sse2>]),
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSSE3 => extern_fn!($args, [<dav1d_prep $name _8bpc_ssse3>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX2 => extern_fn!($args, [<dav1d_prep $name _8bpc_avx2>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX512ICL => extern_fn!($args, [<dav1d_prep $name _8bpc_avx512icl>]),
                        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                        Neon => extern_fn!($args, [<dav1d_prep $name _8bpc_neon>]),
                    }
                }
            }};
        }

        use Filter8Tap::*;
        match filter_2d {
            Filter2dRust::Tap8(Filter2d8Tap { h, v }) => match (h, v) {
                (Regular, Regular) => asm_fn!(asm, args, _8tap_regular),
                (Regular, Smooth) => asm_fn!(asm, args, _8tap_regular_smooth),
                (Regular, Sharp) => asm_fn!(asm, args, _8tap_regular_sharp),
                (Smooth, Regular) => asm_fn!(asm, args, _8tap_smooth_regular),
                (Smooth, Smooth) => asm_fn!(asm, args, _8tap_smooth),
                (Smooth, Sharp) => asm_fn!(asm, args, _8tap_smooth_sharp),
                (Sharp, Regular) => asm_fn!(asm, args, _8tap_sharp_regular),
                (Sharp, Smooth) => asm_fn!(asm, args, _8tap_sharp_smooth),
                (Sharp, Sharp) => asm_fn!(asm, args, _8tap_sharp),
            },
            Filter2dRust::BiLinear => asm_fn!(asm, args, _bilin),
        }
    }

    #[cfg(feature = "asm")]
    unsafe fn mc_scaled(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        dst: *mut Self::Pixel,
        dst_stride: usize,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
        dx: usize,
        dy: usize,
    ) {
        let [dst_stride, src_stride] = [dst_stride, src_stride].map(|it: usize| it as ptrdiff_t);
        let [w, h, mx, my, dx, dy] = [w, h, mx, my, dx, dy].map(|it| it as c_int);
        let args = (dst, dst_stride, src, src_stride, w, h, mx, my, dx, dy);

        macro_rules! extern_fn {
            ($args:expr, $name:ident) => {{
                extern "C" {
                    fn $name(
                        dst: *mut <BitDepth8 as BitDepth>::Pixel,
                        dst_stride: ptrdiff_t,
                        src: *const <BitDepth8 as BitDepth>::Pixel,
                        src_stride: ptrdiff_t,
                        w: c_int,
                        h: c_int,
                        mx: c_int,
                        my: c_int,
                        dx: c_int,
                        dy: c_int,
                    );
                }

                let (dst, dst_stride, src, src_stride, w, h, mx, my, dx, dy) = args;
                $name(dst, dst_stride, src, src_stride, w, h, mx, my, dx, dy)
            }};
            ($args:expr, $asm:expr, unreachable) => {{
                let _ = args;
                unreachable!("{asm:?}");
            }};
        }

        macro_rules! asm_fn {
            ($asm:expr, $args:expr, $name:ident) => {{
                use FnAsmVersion::*;
                paste! {
                    match asm {
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSE2 => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSSE3 => extern_fn!($args, [<dav1d_put $name _8bpc_ssse3>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX2 => extern_fn!($args, [<dav1d_put $name _8bpc_avx2>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX512ICL => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                        Neon => extern_fn!($args, asm, unreachable),
                    }
                }
            }};
        }

        use Filter8Tap::*;
        match filter_2d {
            Filter2dRust::Tap8(Filter2d8Tap { h, v }) => match (h, v) {
                (Regular, Regular) => asm_fn!(asm, args, _8tap_scaled_regular),
                (Regular, Smooth) => asm_fn!(asm, args, _8tap_scaled_regular_smooth),
                (Regular, Sharp) => asm_fn!(asm, args, _8tap_scaled_regular_sharp),
                (Smooth, Regular) => asm_fn!(asm, args, _8tap_scaled_smooth_regular),
                (Smooth, Smooth) => asm_fn!(asm, args, _8tap_scaled_smooth),
                (Smooth, Sharp) => asm_fn!(asm, args, _8tap_scaled_smooth_sharp),
                (Sharp, Regular) => asm_fn!(asm, args, _8tap_scaled_sharp_regular),
                (Sharp, Smooth) => asm_fn!(asm, args, _8tap_scaled_sharp_smooth),
                (Sharp, Sharp) => asm_fn!(asm, args, _8tap_scaled_sharp),
            },
            Filter2dRust::BiLinear => asm_fn!(asm, args, _bilin_scaled),
        }
    }

    #[cfg(feature = "asm")]
    unsafe fn mct_scaled(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        tmp: *mut i16,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
        dx: usize,
        dy: usize,
    ) {
        let src_stride = src_stride as ptrdiff_t;
        let [w, h, mx, my, dx, dy] = [w, h, mx, my, dx, dy].map(|it| it as c_int);
        let args = (tmp, src, src_stride, w, h, mx, my, dx, dy);

        macro_rules! extern_fn {
            ($args:expr, $name:ident) => {{
                extern "C" {
                    fn $name(
                        tmp: *mut i16,
                        src: *const <BitDepth8 as BitDepth>::Pixel,
                        src_stride: ptrdiff_t,
                        w: c_int,
                        h: c_int,
                        mx: c_int,
                        my: c_int,
                        dx: c_int,
                        dy: c_int,
                    );
                }

                let (tmp, src, src_stride, w, h, mx, my, dx, dy) = args;
                $name(tmp, src, src_stride, w, h, mx, my, dx, dy)
            }};
            ($args:expr, $asm:expr, unreachable) => {{
                let _ = args;
                unreachable!("{asm:?}");
            }};
        }

        macro_rules! asm_fn {
            ($asm:expr, $args:expr, $name:ident) => {{
                use FnAsmVersion::*;
                paste! {
                    match asm {
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSE2 => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSSE3 => extern_fn!($args, [<dav1d_prep $name _8bpc_ssse3>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX2 => extern_fn!($args, [<dav1d_prep $name _8bpc_avx2>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX512ICL => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                        Neon => extern_fn!($args, asm, unreachable),
                    }
                }
            }};
        }

        use Filter8Tap::*;
        match filter_2d {
            Filter2dRust::Tap8(Filter2d8Tap { h, v }) => match (h, v) {
                (Regular, Regular) => asm_fn!(asm, args, _8tap_scaled_regular),
                (Regular, Smooth) => asm_fn!(asm, args, _8tap_scaled_regular_smooth),
                (Regular, Sharp) => asm_fn!(asm, args, _8tap_scaled_regular_sharp),
                (Smooth, Regular) => asm_fn!(asm, args, _8tap_scaled_smooth_regular),
                (Smooth, Smooth) => asm_fn!(asm, args, _8tap_scaled_smooth),
                (Smooth, Sharp) => asm_fn!(asm, args, _8tap_scaled_smooth_sharp),
                (Sharp, Regular) => asm_fn!(asm, args, _8tap_scaled_sharp_regular),
                (Sharp, Smooth) => asm_fn!(asm, args, _8tap_scaled_sharp_smooth),
                (Sharp, Sharp) => asm_fn!(asm, args, _8tap_scaled_sharp),
            },
            Filter2dRust::BiLinear => asm_fn!(asm, args, _bilin_scaled),
        }
    }
}

impl BitDepthFnAsmMc for BitDepth16 {
    #[cfg(feature = "asm")]
    unsafe fn mc(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        dst: *mut Self::Pixel,
        dst_stride: usize,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
    ) {
        let [dst_stride, src_stride] = [dst_stride, src_stride].map(|it: usize| it as ptrdiff_t);
        let [w, h, mx, my] = [w, h, mx, my].map(|it| it as c_int);
        let args = (
            dst,
            dst_stride,
            src,
            src_stride,
            w,
            h,
            mx,
            my,
            self.bitdepth_max().as_::<c_int>(),
        );

        macro_rules! extern_fn {
            ($args:expr, $name:ident) => {{
                extern "C" {
                    fn $name(
                        dst: *mut <BitDepth16 as BitDepth>::Pixel,
                        dst_stride: ptrdiff_t,
                        src: *const <BitDepth16 as BitDepth>::Pixel,
                        src_stride: ptrdiff_t,
                        w: c_int,
                        h: c_int,
                        mx: c_int,
                        my: c_int,
                        bitdepth_max: c_int,
                    );
                }

                let (dst, dst_stride, src, src_stride, w, h, mx, my, bitdepth_max) = args;
                $name(dst, dst_stride, src, src_stride, w, h, mx, my, bitdepth_max)
            }};
            ($args:expr, $asm:expr, unreachable) => {{
                let _ = args;
                unreachable!("{asm:?}");
            }};
        }

        macro_rules! asm_fn {
            ($asm:expr, $args:expr, $name:ident) => {{
                use FnAsmVersion::*;
                paste! {
                    match asm {
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSE2 => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSSE3 => extern_fn!($args, [<dav1d_put $name _16bpc_ssse3>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX2 => extern_fn!($args, [<dav1d_put $name _16bpc_avx2>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX512ICL => extern_fn!($args, [<dav1d_put $name _16bpc_avx512icl>]),
                        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                        Neon => extern_fn!($args, [<dav1d_put $name _16bpc_neon>]),
                    }
                }
            }};
        }

        use Filter8Tap::*;
        match filter_2d {
            Filter2dRust::Tap8(Filter2d8Tap { h, v }) => match (h, v) {
                (Regular, Regular) => asm_fn!(asm, args, _8tap_regular),
                (Regular, Smooth) => asm_fn!(asm, args, _8tap_regular_smooth),
                (Regular, Sharp) => asm_fn!(asm, args, _8tap_regular_sharp),
                (Smooth, Regular) => asm_fn!(asm, args, _8tap_smooth_regular),
                (Smooth, Smooth) => asm_fn!(asm, args, _8tap_smooth),
                (Smooth, Sharp) => asm_fn!(asm, args, _8tap_smooth_sharp),
                (Sharp, Regular) => asm_fn!(asm, args, _8tap_sharp_regular),
                (Sharp, Smooth) => asm_fn!(asm, args, _8tap_sharp_smooth),
                (Sharp, Sharp) => asm_fn!(asm, args, _8tap_sharp),
            },
            Filter2dRust::BiLinear => asm_fn!(asm, args, _bilin),
        }
    }

    #[cfg(feature = "asm")]
    unsafe fn mct(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        tmp: *mut i16,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
    ) {
        let src_stride = src_stride as ptrdiff_t;
        let [w, h, mx, my] = [w, h, mx, my].map(|it| it as c_int);
        let args = (
            tmp,
            src,
            src_stride,
            w,
            h,
            mx,
            my,
            self.bitdepth_max().as_::<c_int>(),
        );

        macro_rules! extern_fn {
            ($args:expr, $name:ident) => {{
                extern "C" {
                    fn $name(
                        tmp: *mut i16,
                        src: *const <BitDepth16 as BitDepth>::Pixel,
                        src_stride: ptrdiff_t,
                        w: c_int,
                        h: c_int,
                        mx: c_int,
                        my: c_int,
                        bitdepth_max: c_int,
                    );
                }

                let (tmp, src, src_stride, w, h, mx, my, bitdepth_max) = args;
                $name(tmp, src, src_stride, w, h, mx, my, bitdepth_max)
            }};
            ($args:expr, $asm:expr, unreachable) => {{
                let _ = args;
                unreachable!("{asm:?}");
            }};
        }

        macro_rules! asm_fn {
            ($args:expr, $asm:expr, $name:ident) => {{
                use FnAsmVersion::*;
                paste! {
                    match asm {
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSE2 => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSSE3 => extern_fn!($args, [<dav1d_prep $name _16bpc_ssse3>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX2 => extern_fn!($args, [<dav1d_prep $name _16bpc_avx2>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX512ICL => extern_fn!($args, [<dav1d_prep $name _16bpc_avx512icl>]),
                        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                        Neon => extern_fn!($args, [<dav1d_prep $name _16bpc_neon>]),
                    }
                }
            }};
        }

        use Filter8Tap::*;
        match filter_2d {
            Filter2dRust::Tap8(Filter2d8Tap { h, v }) => match (h, v) {
                (Regular, Regular) => asm_fn!(asm, args, _8tap_regular),
                (Regular, Smooth) => asm_fn!(asm, args, _8tap_regular_smooth),
                (Regular, Sharp) => asm_fn!(asm, args, _8tap_regular_sharp),
                (Smooth, Regular) => asm_fn!(asm, args, _8tap_smooth_regular),
                (Smooth, Smooth) => asm_fn!(asm, args, _8tap_smooth),
                (Smooth, Sharp) => asm_fn!(asm, args, _8tap_smooth_sharp),
                (Sharp, Regular) => asm_fn!(asm, args, _8tap_sharp_regular),
                (Sharp, Smooth) => asm_fn!(asm, args, _8tap_sharp_smooth),
                (Sharp, Sharp) => asm_fn!(asm, args, _8tap_sharp),
            },
            Filter2dRust::BiLinear => asm_fn!(asm, args, _bilin),
        }
    }

    #[cfg(feature = "asm")]
    unsafe fn mc_scaled(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        dst: *mut Self::Pixel,
        dst_stride: usize,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
        dx: usize,
        dy: usize,
    ) {
        let [dst_stride, src_stride] = [dst_stride, src_stride].map(|it: usize| it as ptrdiff_t);
        let [w, h, mx, my, dx, dy] = [w, h, mx, my, dx, dy].map(|it| it as c_int);
        let args = (
            dst,
            dst_stride,
            src,
            src_stride,
            w,
            h,
            mx,
            my,
            dx,
            dy,
            self.bitdepth_max().as_::<c_int>(),
        );

        macro_rules! extern_fn {
            ($args:expr, $name:ident) => {{
                extern "C" {
                    fn $name(
                        dst: *mut <BitDepth16 as BitDepth>::Pixel,
                        dst_stride: ptrdiff_t,
                        src: *const <BitDepth16 as BitDepth>::Pixel,
                        src_stride: ptrdiff_t,
                        w: c_int,
                        h: c_int,
                        mx: c_int,
                        my: c_int,
                        dx: c_int,
                        dy: c_int,
                        bitdepth_max: c_int,
                    );
                }

                let (dst, dst_stride, src, src_stride, w, h, mx, my, dx, dy, bitdepth_max) = args;
                $name(
                    dst,
                    dst_stride,
                    src,
                    src_stride,
                    w,
                    h,
                    mx,
                    my,
                    dx,
                    dy,
                    bitdepth_max,
                )
            }};
            ($args:expr, $asm:expr, unreachable) => {{
                let _ = args;
                unreachable!("{asm:?}");
            }};
        }

        macro_rules! asm_fn {
            ($asm:expr, $args:expr, $name:ident) => {{
                use FnAsmVersion::*;
                paste! {
                    match asm {
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSE2 => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSSE3 => extern_fn!($args, [<dav1d_put $name _16bpc_ssse3>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX2 => extern_fn!($args, [<dav1d_put $name _16bpc_avx2>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX512ICL => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                        Neon => extern_fn!($args, asm, unreachable),
                    }
                }
            }};
        }

        use Filter8Tap::*;
        match filter_2d {
            Filter2dRust::Tap8(Filter2d8Tap { h, v }) => match (h, v) {
                (Regular, Regular) => asm_fn!(asm, args, _8tap_scaled_regular),
                (Regular, Smooth) => asm_fn!(asm, args, _8tap_scaled_regular_smooth),
                (Regular, Sharp) => asm_fn!(asm, args, _8tap_scaled_regular_sharp),
                (Smooth, Regular) => asm_fn!(asm, args, _8tap_scaled_smooth_regular),
                (Smooth, Smooth) => asm_fn!(asm, args, _8tap_scaled_smooth),
                (Smooth, Sharp) => asm_fn!(asm, args, _8tap_scaled_smooth_sharp),
                (Sharp, Regular) => asm_fn!(asm, args, _8tap_scaled_sharp_regular),
                (Sharp, Smooth) => asm_fn!(asm, args, _8tap_scaled_sharp_smooth),
                (Sharp, Sharp) => asm_fn!(asm, args, _8tap_scaled_sharp),
            },
            Filter2dRust::BiLinear => asm_fn!(asm, args, _bilin_scaled),
        }
    }

    #[cfg(feature = "asm")]
    unsafe fn mct_scaled(
        &self,
        _: Token,
        asm: FnAsmVersion,
        filter_2d: Filter2dRust,
        tmp: *mut i16,
        src: *const Self::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
        dx: usize,
        dy: usize,
    ) {
        let src_stride = src_stride as ptrdiff_t;
        let [w, h, mx, my, dx, dy] = [w, h, mx, my, dx, dy].map(|it| it as c_int);
        let args = (
            tmp,
            src,
            src_stride,
            w,
            h,
            mx,
            my,
            dx,
            dy,
            self.bitdepth_max().as_::<c_int>(),
        );

        macro_rules! extern_fn {
            ($args:expr, $name:ident) => {{
                extern "C" {
                    fn $name(
                        tmp: *mut i16,
                        src: *const <BitDepth16 as BitDepth>::Pixel,
                        src_stride: ptrdiff_t,
                        w: c_int,
                        h: c_int,
                        mx: c_int,
                        my: c_int,
                        dx: c_int,
                        dy: c_int,
                        bitdepth_max: c_int,
                    );
                }

                let (tmp, src, src_stride, w, h, mx, my, dx, dy, bitdepth_max) = args;
                $name(tmp, src, src_stride, w, h, mx, my, dx, dy, bitdepth_max)
            }};
            ($args:expr, $asm:expr, unreachable) => {{
                let _ = args;
                unreachable!("{asm:?}");
            }};
        }

        macro_rules! asm_fn {
            ($asm:expr, $args:expr, $name:ident) => {{
                use FnAsmVersion::*;
                paste! {
                    match asm {
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSE2 => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                        SSSE3 => extern_fn!($args, [<dav1d_prep $name _16bpc_ssse3>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX2 => extern_fn!($args, [<dav1d_prep $name _16bpc_avx2>]),
                        #[cfg(target_arch = "x86_64")]
                        AVX512ICL => extern_fn!($args, asm, unreachable),
                        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                        Neon => extern_fn!($args, asm, unreachable),
                    }
                }
            }};
        }

        use Filter8Tap::*;
        match filter_2d {
            Filter2dRust::Tap8(Filter2d8Tap { h, v }) => match (h, v) {
                (Regular, Regular) => asm_fn!(asm, args, _8tap_scaled_regular),
                (Regular, Smooth) => asm_fn!(asm, args, _8tap_scaled_regular_smooth),
                (Regular, Sharp) => asm_fn!(asm, args, _8tap_scaled_regular_sharp),
                (Smooth, Regular) => asm_fn!(asm, args, _8tap_scaled_smooth_regular),
                (Smooth, Smooth) => asm_fn!(asm, args, _8tap_scaled_smooth),
                (Smooth, Sharp) => asm_fn!(asm, args, _8tap_scaled_smooth_sharp),
                (Sharp, Regular) => asm_fn!(asm, args, _8tap_scaled_sharp_regular),
                (Sharp, Smooth) => asm_fn!(asm, args, _8tap_scaled_sharp_smooth),
                (Sharp, Sharp) => asm_fn!(asm, args, _8tap_scaled_sharp),
            },
            Filter2dRust::BiLinear => asm_fn!(asm, args, _bilin_scaled),
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Dav1dMCDSPContextRust {
    pub mc: FnVersion,
    pub mct: FnVersion,
    pub mc_scaled: FnVersion,
    pub mct_scaled: FnVersion,
}

impl Dav1dMCDSPContextRust {
    pub unsafe fn mc<BD: BitDepthFnAsmMc>(
        &self,
        bd: BD,
        filter_2d: Filter2dRust,
        dst: *mut BD::Pixel,
        dst_stride: usize,
        src: *const BD::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
    ) {
        match self.mc {
            FnVersion::Rust => match filter_2d {
                Filter2dRust::Tap8(filter_type) => put_8tap_rust(
                    bd,
                    dst,
                    dst_stride,
                    src,
                    src_stride,
                    w,
                    h,
                    mx,
                    my,
                    filter_type,
                ),
                Filter2dRust::BiLinear => {
                    put_bilin_rust(bd, dst, dst_stride, src, src_stride, w, h, mx, my)
                }
            },
            #[cfg(feature = "asm")]
            FnVersion::Asm(asm) => bd.mc(
                Token(()),
                asm,
                filter_2d,
                dst,
                dst_stride,
                src,
                src_stride,
                w,
                h,
                mx,
                my,
            ),
        }
    }

    pub unsafe fn mct<BD: BitDepthFnAsmMc>(
        &self,
        bd: BD,
        filter_2d: Filter2dRust,
        tmp: *mut i16,
        src: *const BD::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
    ) {
        match self.mct {
            FnVersion::Rust => match filter_2d {
                Filter2dRust::Tap8(filter_type) => {
                    prep_8tap_rust(bd, tmp, src, src_stride, w, h, mx, my, filter_type)
                }
                Filter2dRust::BiLinear => prep_bilin_rust(bd, tmp, src, src_stride, w, h, mx, my),
            },
            #[cfg(feature = "asm")]
            FnVersion::Asm(asm) => bd.mct(
                Token(()),
                asm,
                filter_2d,
                tmp,
                src,
                src_stride,
                w,
                h,
                mx,
                my,
            ),
        }
    }

    pub unsafe fn mc_scaled<BD: BitDepthFnAsmMc>(
        &self,
        bd: BD,
        filter_2d: Filter2dRust,
        dst: *mut BD::Pixel,
        dst_stride: usize,
        src: *const BD::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
        dx: usize,
        dy: usize,
    ) {
        match self.mc_scaled {
            FnVersion::Rust => match filter_2d {
                Filter2dRust::Tap8(filter_type) => put_8tap_scaled_rust(
                    bd,
                    dst,
                    dst_stride,
                    src,
                    src_stride,
                    w,
                    h,
                    mx,
                    my,
                    dx,
                    dy,
                    filter_type,
                ),
                Filter2dRust::BiLinear => put_bilin_scaled_rust(
                    bd, dst, dst_stride, src, src_stride, w, h, mx, my, dx, dy,
                ),
            },
            #[cfg(feature = "asm")]
            FnVersion::Asm(asm) => bd.mc_scaled(
                Token(()),
                asm,
                filter_2d,
                dst,
                dst_stride,
                src,
                src_stride,
                w,
                h,
                mx,
                my,
                dx,
                dy,
            ),
        }
    }

    pub unsafe fn mct_scaled<BD: BitDepthFnAsmMc>(
        &self,
        bd: BD,
        filter_2d: Filter2dRust,
        tmp: *mut i16,
        src: *const BD::Pixel,
        src_stride: usize,
        w: usize,
        h: usize,
        mx: usize,
        my: usize,
        dx: usize,
        dy: usize,
    ) {
        match self.mct_scaled {
            FnVersion::Rust => match filter_2d {
                Filter2dRust::Tap8(filter_type) => prep_8tap_scaled_rust(
                    bd,
                    tmp,
                    src,
                    src_stride,
                    w,
                    h,
                    mx,
                    my,
                    dx,
                    dy,
                    filter_type,
                ),
                Filter2dRust::BiLinear => {
                    prep_bilin_scaled_rust(bd, tmp, src, src_stride, w, h, mx, my, dx, dy)
                }
            },
            #[cfg(feature = "asm")]
            FnVersion::Asm(asm) => bd.mct_scaled(
                Token(()),
                asm,
                filter_2d,
                tmp,
                src,
                src_stride,
                w,
                h,
                mx,
                my,
                dx,
                dy,
            ),
        }
    }
}

#[inline(never)]
unsafe fn put_rust<BD: BitDepth>(
    dst: *mut BD::Pixel,
    dst_stride: usize,
    src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
) {
    let [dst_len, src_len] =
        [dst_stride, src_stride].map(|stride| if h == 0 { 0 } else { stride * (h - 1) + w });
    let mut dst = std::slice::from_raw_parts_mut(dst, dst_len);
    let mut src = std::slice::from_raw_parts(src, src_len);
    for (dst, src) in iter::zip(dst.chunks_mut(dst_stride), src.chunks(src_stride)) {
        BD::pixel_copy(dst, src, w);
    }
}

#[inline(never)]
unsafe fn prep_rust<BD: BitDepth>(
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    bd: BD,
) {
    let mut tmp = std::slice::from_raw_parts_mut(tmp, w * h);
    let mut src =
        std::slice::from_raw_parts(src, if h == 0 { 0 } else { src_stride * (h - 1) + w });
    let intermediate_bits = bd.get_intermediate_bits();
    for (tmp, src) in iter::zip(tmp.chunks_exact_mut(w), src.chunks(src_stride)).take(h) {
        for (tmp, src) in iter::zip(tmp, &src[..w]) {
            *tmp = (((*src).as_::<i32>() << intermediate_bits) - (BD::PREP_BIAS as i32)) as i16;
        }
    }
}

unsafe fn filter_8tap<T: Into<i32>>(src: *const T, x: usize, f: &[i8; 8], stride: usize) -> i32 {
    f.into_iter()
        .enumerate()
        .map(|(i, &f)| {
            let [i, x, stride] = [i, x, stride].map(|it| it as isize);
            let j = x + (i - 3) * stride;
            i32::from(f) * src.offset(j).read().into()
        })
        .sum()
}

unsafe fn dav1d_filter_8tap_rnd<T: Into<i32>>(
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: usize,
    sh: u8,
) -> i32 {
    (filter_8tap(src, x, f, stride) + ((1 << sh) >> 1)) >> sh
}

unsafe fn dav1d_filter_8tap_rnd2<T: Into<i32>>(
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: usize,
    rnd: u8,
    sh: u8,
) -> i32 {
    (filter_8tap(src, x, f, stride) + (rnd as i32)) >> sh
}

unsafe fn dav1d_filter_8tap_clip<BD: BitDepth, T: Into<i32>>(
    bd: BD,
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: usize,
    sh: u8,
) -> BD::Pixel {
    bd.iclip_pixel(dav1d_filter_8tap_rnd(src, x, f, stride, sh))
}

unsafe fn dav1d_filter_8tap_clip2<BD: BitDepth, T: Into<i32>>(
    bd: BD,
    src: *const T,
    x: usize,
    f: &[i8; 8],
    stride: usize,
    rnd: u8,
    sh: u8,
) -> BD::Pixel {
    bd.iclip_pixel(dav1d_filter_8tap_rnd2(src, x, f, stride, rnd, sh))
}

fn get_filter(mxy: usize, wh: usize, filter_type: Filter8Tap) -> Option<&'static [i8; 8]> {
    let mxy = mxy.checked_sub(1)?;
    use Filter8Tap::*;
    let filter_type = match filter_type {
        Regular => 0,
        Smooth => 1,
        Sharp => 2,
    };
    let i = if wh > 4 {
        filter_type
    } else {
        3 + (filter_type & 1)
    };
    Some(&dav1d_mc_subpel_filters[i][mxy])
}

#[inline(never)]
unsafe fn put_8tap_rust<BD: BitDepth>(
    bd: BD,
    dst: *mut BD::Pixel,
    dst_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    filter_type: Filter2d8Tap,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let intermediate_rnd = 32 + (1 << 6 - intermediate_bits >> 1);

    let fh = get_filter(mx, w, filter_type.h);
    let fv = get_filter(my, h, filter_type.v);
    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);

    let mut dst = std::slice::from_raw_parts_mut(dst, dst_stride * h);

    if let Some(fh) = fh {
        if let Some(fv) = fv {
            let tmp_h = h + 7;
            let mut mid = [0i16; 128 * 135]; // Default::default()
            let mut mid_ptr = &mut mid[..];

            src = src.offset(-((src_stride * 3) as isize));
            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = dav1d_filter_8tap_rnd(src, x, fh, 1, 6 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride as isize);
            }

            mid_ptr = &mut mid[128 * 3..];
            for _ in 0..h {
                for x in 0..w {
                    dst[x] = dav1d_filter_8tap_clip(
                        bd,
                        mid_ptr.as_ptr(),
                        x,
                        fv,
                        128,
                        6 + intermediate_bits,
                    );
                }

                mid_ptr = &mut mid_ptr[128..];
                dst = &mut dst[dst_stride..];
            }
        } else {
            let mut src = std::slice::from_raw_parts(src, src_stride * h);
            for _ in 0..h {
                for x in 0..w {
                    dst[x] =
                        dav1d_filter_8tap_clip2(bd, src.as_ptr(), x, fh, 1, intermediate_rnd, 6);
                }

                dst = &mut dst[dst_stride..];
                src = &src[src_stride..];
            }
        }
    } else if let Some(fv) = fv {
        let mut src = std::slice::from_raw_parts(src, src_stride * h);
        for _ in 0..h {
            for x in 0..w {
                dst[x] = dav1d_filter_8tap_clip(bd, src.as_ptr(), x, fv, src_stride, 6);
            }

            dst = &mut dst[dst_stride..];
            src = &src[src_stride..];
        }
    } else {
        put_rust::<BD>(dst.as_mut_ptr(), dst_stride, src, src_stride, w, h);
    }
}

#[inline(never)]
unsafe fn put_8tap_scaled_rust<BD: BitDepth>(
    bd: BD,
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    mut h: usize,
    mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    filter_type: Filter2d8Tap,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let intermediate_rnd = (1 << intermediate_bits) >> 1;
    let tmp_h = ((h - 1) * dy + my >> 10) + 8;
    let mut mid = [0i16; 128 * (256 + 7)]; // Default::default()
    let mut mid_ptr = &mut mid[..];
    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);

    let mut dst = std::slice::from_raw_parts_mut(dst, dst_stride * h);

    src = src.offset(-((src_stride * 3) as isize));
    for _ in 0..tmp_h {
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            let fh = get_filter(imx >> 6, w, filter_type.h);
            mid_ptr[x] = match fh {
                Some(fh) => dav1d_filter_8tap_rnd(src, ioff, fh, 1, 6 - intermediate_bits) as i16,
                None => ((*src.offset(ioff as isize)).as_::<i32>() as i16) << intermediate_bits,
            };
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }

        mid_ptr = &mut mid_ptr[128..];
        src = src.offset(src_stride as isize);
    }
    mid_ptr = &mut mid[128 * 3..];
    for _ in 0..h {
        let fv = get_filter(my >> 6, h, filter_type.v);

        for x in 0..w {
            dst[x] = match fv {
                Some(fv) => {
                    dav1d_filter_8tap_clip(bd, mid_ptr.as_ptr(), x, fv, 128, 6 + intermediate_bits)
                }
                None => {
                    bd.iclip_pixel((i32::from(mid_ptr[x]) + intermediate_rnd) >> intermediate_bits)
                }
            };
        }

        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        dst = &mut dst[dst_stride..];
    }
}

#[inline(never)]
unsafe fn prep_8tap_rust<BD: BitDepth>(
    bd: BD,
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
    filter_type: Filter2d8Tap,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let fh = get_filter(mx, w, filter_type.h);
    let fv = get_filter(my, h, filter_type.v);
    let src_stride = BD::pxstride(src_stride);

    if let Some(fh) = fh {
        if let Some(fv) = fv {
            let tmp_h = h + 7;
            let mut mid = [0i16; 128 * 135]; // Default::default()
            let mut mid_ptr = &mut mid[..];

            src = src.offset(-((src_stride * 3) as isize));
            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = dav1d_filter_8tap_rnd(src, x, fh, 1, 6 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride as isize);
            }

            mid_ptr = &mut mid[128 * 3..];
            for _ in 0..h {
                for x in 0..w {
                    *tmp.offset(x as isize) =
                        (dav1d_filter_8tap_rnd(mid_ptr.as_ptr(), x, fv, 128, 6)
                            - i32::from(BD::PREP_BIAS))
                        .try_into()
                        .unwrap();
                }

                mid_ptr = &mut mid_ptr[128..];
                tmp = tmp.offset(w as isize);
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    *tmp.offset(x as isize) =
                        (dav1d_filter_8tap_rnd(src, x, fh, 1, 6 - intermediate_bits)
                            - i32::from(BD::PREP_BIAS)) as i16;
                }

                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
            }
        }
    } else if let Some(fv) = fv {
        for _ in 0..h {
            for x in 0..w {
                *tmp.offset(x as isize) =
                    (dav1d_filter_8tap_rnd(src, x, fv, src_stride, 6 - intermediate_bits)
                        - i32::from(BD::PREP_BIAS)) as i16;
            }

            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
        }
    } else {
        prep_rust(tmp, src, src_stride, w, h, bd);
    };
}

#[inline(never)]
unsafe fn prep_8tap_scaled_rust<BD: BitDepth>(
    bd: BD,
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mut mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
    filter_type: Filter2d8Tap,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let tmp_h = ((h - 1) * dy + my >> 10) + 8;
    let mut mid = [0i16; 128 * (256 + 7)]; // Default::default()
    let mut mid_ptr = &mut mid[..];
    let src_stride = BD::pxstride(src_stride);

    src = src.offset(-((src_stride * 3) as isize));
    for _ in 0..tmp_h {
        let mut imx = mx;
        let mut ioff = 0;
        for x in 0..w {
            let fh = get_filter(imx >> 6, w, filter_type.h);
            mid_ptr[x] = match fh {
                Some(fh) => dav1d_filter_8tap_rnd(src, ioff, fh, 1, 6 - intermediate_bits) as i16,
                None => ((*src.offset(ioff as isize)).as_::<i32>() as i16) << intermediate_bits,
            };
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }

        mid_ptr = &mut mid_ptr[128..];
        src = src.offset(src_stride as isize);
    }

    mid_ptr = &mut mid[128 * 3..];
    for _ in 0..h {
        let fv = get_filter(my >> 6, h, filter_type.v);
        for x in 0..w {
            *tmp.offset(x as isize) = ((match fv {
                Some(fv) => dav1d_filter_8tap_rnd(mid_ptr.as_ptr(), x, fv, 128, 6),
                None => i32::from(mid_ptr[x]),
            }) - i32::from(BD::PREP_BIAS)) as i16;
        }
        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        tmp = tmp.offset(w as isize);
    }
}

unsafe fn filter_bilin<T: Into<i32>>(src: *const T, x: usize, mxy: usize, stride: usize) -> i32 {
    let src = |i: usize| -> i32 { src.offset(i as isize).read().into() };
    16 * src(x) + ((mxy as i32) * (src(x + stride) - src(x)))
}

unsafe fn filter_bilin_rnd<T: Into<i32>>(
    src: *const T,
    x: usize,
    mxy: usize,
    stride: usize,
    sh: u8,
) -> i32 {
    (filter_bilin(src, x, mxy, stride) + ((1 << sh) >> 1)) >> sh
}

unsafe fn filter_bilin_clip<BD: BitDepth, T: Into<i32>>(
    bd: BD,
    src: *const T,
    x: usize,
    mxy: usize,
    stride: usize,
    sh: u8,
) -> BD::Pixel {
    bd.iclip_pixel(filter_bilin_rnd(src, x, mxy, stride, sh))
}

unsafe fn put_bilin_rust<BD: BitDepth>(
    bd: BD,
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let intermediate_rnd = (1 << intermediate_bits) >> 1;
    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);

    if mx != 0 {
        if my != 0 {
            let mut mid = [0i16; 128 * 129]; // Default::default()
            let mut mid_ptr = &mut mid[..];
            let tmp_h = h + 1;

            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride as isize);
            }
            mid_ptr = &mut mid[..];
            for _ in 0..h {
                for x in 0..w {
                    *dst.offset(x as isize) =
                        filter_bilin_clip(bd, mid_ptr.as_ptr(), x, my, 128, 4 + intermediate_bits);
                }

                mid_ptr = &mut mid_ptr[128..];
                dst = dst.offset(dst_stride as isize);
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    let px = filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits);
                    *dst.offset(x as isize) =
                        bd.iclip_pixel((px + intermediate_rnd) >> intermediate_bits);
                }

                dst = dst.offset(dst_stride as isize);
                src = src.offset(src_stride as isize);
            }
        }
    } else if my != 0 {
        for _ in 0..h {
            for x in 0..w {
                *dst.offset(x as isize) = filter_bilin_clip(bd, src, x, my, src_stride, 4);
            }

            dst = dst.offset(dst_stride as isize);
            src = src.offset(src_stride as isize);
        }
    } else {
        put_rust::<BD>(dst, dst_stride, src, src_stride, w, h);
    };
}

unsafe fn put_bilin_scaled_rust<BD: BitDepth>(
    bd: BD,
    mut dst: *mut BD::Pixel,
    mut dst_stride: usize,
    mut src: *const BD::Pixel,
    mut src_stride: usize,
    w: usize,
    h: usize,
    mut mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let [dst_stride, src_stride] = [dst_stride, src_stride].map(BD::pxstride);
    let tmp_h = ((h - 1) * dy + my >> 10) + 2;
    let mut mid = [0i16; 128 * (256 + 1)];
    let mut mid_ptr = &mut mid[..];

    for _ in 0..tmp_h {
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            mid_ptr[x] = filter_bilin_rnd(src, ioff, imx >> 6, 1, 4 - intermediate_bits) as i16;
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }

        mid_ptr = &mut mid_ptr[128..];
        src = src.offset(src_stride as isize);
    }
    mid_ptr = &mut mid[..];
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) =
                filter_bilin_clip(bd, mid_ptr.as_ptr(), x, my >> 6, 128, 4 + intermediate_bits);
        }

        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        dst = dst.offset(dst_stride as isize);
    }
}

unsafe fn prep_bilin_rust<BD: BitDepth>(
    bd: BD,
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mx: usize,
    my: usize,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let src_stride = BD::pxstride(src_stride);
    if mx != 0 {
        if my != 0 {
            let mut mid = [0i16; 128 * 129];
            let mut mid_ptr = &mut mid[..];
            let tmp_h = h + 1;

            for _ in 0..tmp_h {
                for x in 0..w {
                    mid_ptr[x] = filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits) as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                src = src.offset(src_stride as isize);
            }
            mid_ptr = &mut mid[..];
            for _ in 0..h {
                for x in 0..w {
                    *tmp.offset(x as isize) = (filter_bilin_rnd(mid_ptr.as_ptr(), x, my, 128, 4)
                        - i32::from(BD::PREP_BIAS))
                        as i16;
                }

                mid_ptr = &mut mid_ptr[128..];
                tmp = tmp.offset(w as isize);
            }
        } else {
            for _ in 0..h {
                for x in 0..w {
                    *tmp.offset(x as isize) =
                        (filter_bilin_rnd(src, x, mx, 1, 4 - intermediate_bits)
                            - i32::from(BD::PREP_BIAS)) as i16;
                }

                tmp = tmp.offset(w as isize);
                src = src.offset(src_stride as isize);
            }
        }
    } else if my != 0 {
        for _ in 0..h {
            for x in 0..w {
                *tmp.offset(x as isize) =
                    (filter_bilin_rnd(src, x, my, src_stride, 4 - intermediate_bits)
                        - i32::from(BD::PREP_BIAS)) as i16;
            }

            tmp = tmp.offset(w as isize);
            src = src.offset(src_stride as isize);
        }
    } else {
        prep_rust(tmp, src, src_stride, w, h, bd);
    };
}

unsafe fn prep_bilin_scaled_rust<BD: BitDepth>(
    bd: BD,
    mut tmp: *mut i16,
    mut src: *const BD::Pixel,
    src_stride: usize,
    w: usize,
    h: usize,
    mut mx: usize,
    mut my: usize,
    dx: usize,
    dy: usize,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let src_stride = BD::pxstride(src_stride);
    let mut tmp_h = ((h - 1) * dy + my >> 10) + 2;
    let mut mid = [0i16; 128 * (256 + 1)];
    let mut mid_ptr = &mut mid[..];

    for _ in 0..tmp_h {
        let mut imx = mx;
        let mut ioff = 0;

        for x in 0..w {
            mid_ptr[x] = filter_bilin_rnd(src, ioff, imx >> 6, 1, 4 - intermediate_bits) as i16;
            imx += dx;
            ioff += imx >> 10;
            imx &= 0x3ff;
        }

        mid_ptr = &mut mid_ptr[128..];
        src = src.offset(src_stride as isize);
    }
    mid_ptr = &mut mid[..];
    for _ in 0..h {
        for x in 0..w {
            *tmp.offset(x as isize) = (filter_bilin_rnd(mid_ptr.as_ptr(), x, my >> 6, 128, 4)
                - i32::from(BD::PREP_BIAS)) as i16;
        }

        my += dy;
        mid_ptr = &mut mid_ptr[(my >> 10) * 128..];
        my &= 0x3ff;
        tmp = tmp.offset(w as isize);
    }
}

unsafe fn avg_rust<BD: BitDepth>(
    bd: BD,
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut tmp1: *const i16,
    mut tmp2: *const i16,
    w: usize,
    h: usize,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 1;
    let rnd = (1 << intermediate_bits) + BD::PREP_BIAS * 2;
    let dst_stride = BD::pxstride(dst_stride);
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) = bd.iclip_pixel(
                ((*tmp1.offset(x as isize) + *tmp2.offset(x as isize) + rnd) >> sh).into(),
            );
        }

        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
    }
}

unsafe fn w_avg_rust<BD: BitDepth>(
    bd: BD,
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut tmp1: *const i16,
    mut tmp2: *const i16,
    w: usize,
    h: usize,
    weight: i32,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 4;
    let rnd = (8 << intermediate_bits) + BD::PREP_BIAS * 16;
    let dst_stride = BD::pxstride(dst_stride);
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) = bd.iclip_pixel(
                (*tmp1.offset(x as isize) as i32 * weight
                    + *tmp2.offset(x as isize) as i32 * (16 - weight)
                    + rnd as i32)
                    >> sh,
            );
        }

        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
    }
}

unsafe fn mask_rust<BD: BitDepth>(
    bd: BD,
    mut dst: *mut BD::Pixel,
    dst_stride: usize,
    mut tmp1: *const i16,
    mut tmp2: *const i16,
    w: usize,
    h: usize,
    mut mask: *const u8,
) {
    let intermediate_bits = bd.get_intermediate_bits();
    let sh = intermediate_bits + 6;
    let rnd = (32 << intermediate_bits) + BD::PREP_BIAS * 64;
    let dst_stride = BD::pxstride(dst_stride);
    for _ in 0..h {
        for x in 0..w {
            *dst.offset(x as isize) = bd.iclip_pixel(
                (*tmp1.offset(x as isize) as i32 * *mask.offset(x as isize) as i32
                    + *tmp2.offset(x as isize) as i32 * (64 - *mask.offset(x as isize) as i32)
                    + rnd as i32)
                    >> sh,
            );
        }

        tmp1 = tmp1.offset(w as isize);
        tmp2 = tmp2.offset(w as isize);
        mask = mask.offset(w as isize);
        dst = dst.offset(dst_stride as isize);
    }
}
