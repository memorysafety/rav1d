use crate::include::common::bitdepth::BitDepth16;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::src::filmgrain::fguv_32x32xn_420_c_erased;
use crate::src::filmgrain::fguv_32x32xn_422_c_erased;
use crate::src::filmgrain::fguv_32x32xn_444_c_erased;
use crate::src::filmgrain::fgy_32x32xn_c_erased;
use crate::src::filmgrain::generate_grain_uv_420_c_erased;
use crate::src::filmgrain::generate_grain_uv_422_c_erased;
use crate::src::filmgrain::generate_grain_uv_444_c_erased;
use crate::src::filmgrain::generate_grain_y_c_erased;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::src::filmgrain::film_grain_dsp_init_x86;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
use crate::{
    src::filmgrain::dav1d_generate_grain_uv_420_16bpc_neon,
    src::filmgrain::dav1d_generate_grain_uv_422_16bpc_neon,
    src::filmgrain::dav1d_generate_grain_uv_444_16bpc_neon,
    src::filmgrain::dav1d_generate_grain_y_16bpc_neon,
    src::filmgrain::fguv_32x32xn_420_neon_erased, src::filmgrain::fguv_32x32xn_422_neon_erased,
    src::filmgrain::fguv_32x32xn_444_neon_erased, src::filmgrain::fgy_32x32xn_neon_erased,
};

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
#[inline(always)]
unsafe fn film_grain_dsp_init_arm(c: *mut Rav1dFilmGrainDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    (*c).generate_grain_y = Some(dav1d_generate_grain_y_16bpc_neon);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(dav1d_generate_grain_uv_420_16bpc_neon);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(dav1d_generate_grain_uv_422_16bpc_neon);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(dav1d_generate_grain_uv_444_16bpc_neon);

    (*c).fgy_32x32xn = Some(fgy_32x32xn_neon_erased::<BitDepth16>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(fguv_32x32xn_420_neon_erased::<BitDepth16>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(fguv_32x32xn_422_neon_erased::<BitDepth16>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(fguv_32x32xn_444_neon_erased::<BitDepth16>);
}

#[cold]
pub unsafe fn rav1d_film_grain_dsp_init_16bpc(c: *mut Rav1dFilmGrainDSPContext) {
    (*c).generate_grain_y = Some(generate_grain_y_c_erased::<BitDepth16>);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(generate_grain_uv_420_c_erased::<BitDepth16>);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(generate_grain_uv_422_c_erased::<BitDepth16>);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(generate_grain_uv_444_c_erased::<BitDepth16>);

    (*c).fgy_32x32xn = Some(fgy_32x32xn_c_erased::<BitDepth16>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(fguv_32x32xn_420_c_erased::<BitDepth16>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(fguv_32x32xn_422_c_erased::<BitDepth16>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(fguv_32x32xn_444_c_erased::<BitDepth16>);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            film_grain_dsp_init_x86::<BitDepth16>(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            film_grain_dsp_init_arm(c);
        }
    }
}
