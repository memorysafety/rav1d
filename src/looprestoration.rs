use crate::include::stddef::ptrdiff_t;
use crate::include::stdint::int16_t;
use crate::include::stdint::uint32_t;
use crate::src::align::Align16;
use crate::src::internal::{
    const_left_pixel_row_16bpc, const_left_pixel_row_8bpc, pixel_16bpc, pixel_8bpc,
};
use crate::src::looprestoration_tmpl_16::{
    dav1d_sgr_filter_3x3_16bpc_avx2, dav1d_sgr_filter_3x3_16bpc_avx512icl,
    dav1d_sgr_filter_3x3_16bpc_ssse3, dav1d_sgr_filter_5x5_16bpc_avx2,
    dav1d_sgr_filter_5x5_16bpc_avx512icl, dav1d_sgr_filter_5x5_16bpc_ssse3,
    dav1d_sgr_filter_mix_16bpc_avx2, dav1d_sgr_filter_mix_16bpc_avx512icl,
    dav1d_sgr_filter_mix_16bpc_ssse3, dav1d_wiener_filter5_16bpc_avx2,
    dav1d_wiener_filter5_16bpc_avx512icl, dav1d_wiener_filter5_16bpc_ssse3,
    dav1d_wiener_filter7_16bpc_avx2, dav1d_wiener_filter7_16bpc_avx512icl,
    dav1d_wiener_filter7_16bpc_ssse3, sgr_3x3_c as sgr_3x3_c_16bpc, sgr_5x5_c as sgr_5x5_c_16bpc,
    sgr_mix_c as sgr_mix_c_16bpc, wiener_c as wiener_c_16bpc,
};
use crate::src::looprestoration_tmpl_8::{
    dav1d_sgr_filter_3x3_8bpc_avx2, dav1d_sgr_filter_3x3_8bpc_avx512icl,
    dav1d_sgr_filter_3x3_8bpc_ssse3, dav1d_sgr_filter_5x5_8bpc_avx2,
    dav1d_sgr_filter_5x5_8bpc_avx512icl, dav1d_sgr_filter_5x5_8bpc_ssse3,
    dav1d_sgr_filter_mix_8bpc_avx2, dav1d_sgr_filter_mix_8bpc_avx512icl,
    dav1d_sgr_filter_mix_8bpc_ssse3, dav1d_wiener_filter5_8bpc_avx2,
    dav1d_wiener_filter5_8bpc_ssse3, dav1d_wiener_filter7_8bpc_avx2,
    dav1d_wiener_filter7_8bpc_avx512icl, dav1d_wiener_filter7_8bpc_ssse3,
    sgr_3x3_c as sgr_3x3_c_8bpc, sgr_5x5_c as sgr_5x5_c_8bpc, sgr_mix_c as sgr_mix_c_8bpc,
    wiener_c as wiener_c_8bpc,
dav1d_wiener_filter7_8bpc_sse2,
dav1d_wiener_filter5_8bpc_sse2,
};
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

macro_rules! looprestoration_filter_fn_enum {
    (
        pub enum $enum_name:ident;

        {
            $(
                $rust_var:ident => { $rust_fn_16bpc:ident, $rust_fn_8bpc:ident },
            )*
        }

        $(
            #[$arch:meta]
            {
                $(
                    $var_name:ident => { $fn_16bpc:ident, $fn_8bpc:ident },
                )*
            }
        )*
    ) => {
        #[derive(Debug, Clone, Copy)]
        #[repr(u8)]
        pub enum $enum_name {
            $(
                $rust_var,
            )*

            $($(
                #[cfg(all(feature = "asm", $arch))]
                $var_name,
            )*)*
        }

        impl $enum_name {
            #[inline]
            pub fn call_16bpc(
                self,
                dst: *mut pixel_16bpc,
                dst_stride: ptrdiff_t,
                left: const_left_pixel_row_16bpc,
                lpf: *const pixel_16bpc,
                w: libc::c_int,
                h: libc::c_int,
                params: *const LooprestorationParams,
                edges: LrEdgeFlags,
                bitdepth_max: libc::c_int,
            ) {
                match self {
                    $(
                        Self::$rust_var => unsafe {
                            $rust_fn_16bpc(
                                dst,
                                dst_stride,
                                left,
                                lpf,
                                w,
                                h,
                                params,
                                edges,
                                bitdepth_max,
                            )
                        }
                    )*

                    $($(
                        #[cfg(all(feature = "asm", $arch))]
                        Self::$var_name => unsafe {
                            $fn_16bpc(
                                dst,
                                dst_stride,
                                left,
                                lpf,
                                w,
                                h,
                                params,
                                edges,
                                bitdepth_max,
                            )
                        }
                    )*)*
                }
            }

            #[inline]
            pub fn call_8bpc(
                self,
                dst: *mut pixel_8bpc,
                dst_stride: ptrdiff_t,
                left: const_left_pixel_row_8bpc,
                lpf: *const pixel_8bpc,
                w: libc::c_int,
                h: libc::c_int,
                params: *const LooprestorationParams,
                edges: LrEdgeFlags,
            ) {
                match self {
                    $(
                        Self::$rust_var => unsafe {
                            $rust_fn_8bpc(
                                dst,
                                dst_stride,
                                left,
                                lpf,
                                w,
                                h,
                                params,
                                edges,
                            )
                        }
                    )*

                    $($(
                        #[cfg(all(feature = "asm", $arch))]
                        Self::$var_name => unsafe {
                            $fn_8bpc(
                                dst,
                                dst_stride,
                                left,
                                lpf,
                                w,
                                h,
                                params,
                                edges,
                            )
                        }
                    )*)*
                }
            }
        }
    };
}

looprestoration_filter_fn_enum! {
    pub enum LoopRestorationFilterFn;

    {
        Wiener_Rust => { wiener_c_16bpc, wiener_c_8bpc },

        SgrMix_Rust => { sgr_mix_c_16bpc, sgr_mix_c_8bpc },
        Sgr3x3_Rust => { sgr_3x3_c_16bpc, sgr_3x3_c_8bpc },
        Sgr5x5_Rust => { sgr_5x5_c_16bpc, sgr_5x5_c_8bpc },
    }

    #[any(target_arch = "x86", target_arch = "x86_64")]
    {
        Wiener5_Sse2 => { wiener_c_16bpc, dav1d_wiener_filter5_8bpc_sse2 },
        Wiener7_Sse2 => { wiener_c_16bpc, dav1d_wiener_filter7_8bpc_sse2 },
        Wiener5_Sse3 => { dav1d_wiener_filter5_16bpc_ssse3, dav1d_wiener_filter5_8bpc_ssse3 },
        Wiener7_Sse3 => { dav1d_wiener_filter7_16bpc_ssse3, dav1d_wiener_filter7_8bpc_ssse3 },
        Wiener5_Avx2 => { dav1d_wiener_filter5_16bpc_avx2, dav1d_wiener_filter5_8bpc_avx2 },
        Wiener7_Avx2 => { dav1d_wiener_filter7_16bpc_avx2, dav1d_wiener_filter7_8bpc_avx2 },
        Wiener5_Avx512 => { dav1d_wiener_filter5_16bpc_avx512icl, dav1d_wiener_filter7_8bpc_avx512icl },
        Wiener7_Avx512=> { dav1d_wiener_filter7_16bpc_avx512icl, dav1d_wiener_filter7_8bpc_avx512icl },

        SgrMix_Sse3 => { dav1d_sgr_filter_mix_16bpc_ssse3, dav1d_sgr_filter_mix_8bpc_ssse3 },
        Sgr3x3_Sse3 => { dav1d_sgr_filter_3x3_16bpc_ssse3, dav1d_sgr_filter_3x3_8bpc_ssse3 },
        Sgr5x5_Sse3 => { dav1d_sgr_filter_5x5_16bpc_ssse3, dav1d_sgr_filter_5x5_8bpc_ssse3 },
        SgrMix_Avx2 => { dav1d_sgr_filter_mix_16bpc_avx2, dav1d_sgr_filter_mix_8bpc_avx2 },
        Sgr3x3_Avx2 => { dav1d_sgr_filter_3x3_16bpc_avx2, dav1d_sgr_filter_3x3_8bpc_avx2 },
        Sgr5x5_Avx2 => { dav1d_sgr_filter_5x5_16bpc_avx2, dav1d_sgr_filter_5x5_8bpc_avx2 },
        Sgr5x5_Avx512 => { dav1d_sgr_filter_5x5_16bpc_avx512icl, dav1d_sgr_filter_5x5_8bpc_avx512icl },
        Sgr3x3_Avx512 => { dav1d_sgr_filter_3x3_16bpc_avx512icl, dav1d_sgr_filter_3x3_8bpc_avx512icl },
        SgrMix_Avx512 => { dav1d_sgr_filter_mix_16bpc_avx512icl, dav1d_sgr_filter_mix_8bpc_avx512icl },
    }

    #[any(target_arch = "arm", target_arch = "aarch64")]
    {
        Wiener7_Neon => { dav1d_wiener_filter7_16bpc_neon, dav1d_wiener_filter7_8bpc_neon },
        Wiener5_Neon => { dav1d_wiener_filter5_16bpc_neon, dav1d_wiener_filter5_8bpc_neon },

        Sgr5x5_Neon => { sgr_filter_5x5_neon_16bpc, sgr_filter_5x5_neon_8bpc },
        Sgr3x3_Neon => { sgr_filter_3x3_neon_16bpc, sgr_filter_3x3_neon_8bpc },
        SgrMix_Neon => { sgr_filter_mix_neon_16bpc, sgr_filter_mix_neon_8bpc },
    }
}
