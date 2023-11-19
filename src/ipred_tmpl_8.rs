use crate::include::common::bitdepth::BitDepth8;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::ipred::cfl_ac_420_c_erased;
use crate::src::ipred::cfl_ac_422_c_erased;
use crate::src::ipred::cfl_ac_444_c_erased;
use crate::src::ipred::ipred_cfl_128_c_erased;
use crate::src::ipred::ipred_cfl_c_erased;
use crate::src::ipred::ipred_cfl_left_c_erased;
use crate::src::ipred::ipred_cfl_top_c_erased;
use crate::src::ipred::ipred_dc_128_c_erased;
use crate::src::ipred::ipred_dc_c_erased;
use crate::src::ipred::ipred_dc_left_c_erased;
use crate::src::ipred::ipred_dc_top_c_erased;
use crate::src::ipred::ipred_filter_c_erased;
use crate::src::ipred::ipred_h_c_erased;
use crate::src::ipred::ipred_paeth_c_erased;
use crate::src::ipred::ipred_smooth_c_erased;
use crate::src::ipred::ipred_smooth_h_c_erased;
use crate::src::ipred::ipred_smooth_v_c_erased;
use crate::src::ipred::ipred_v_c_erased;
use crate::src::ipred::ipred_z1_c_erased;
use crate::src::ipred::ipred_z2_c_erased;
use crate::src::ipred::ipred_z3_c_erased;
use crate::src::ipred::pal_pred_c_erased;
use crate::src::ipred::Rav1dIntraPredDSPContext;
use crate::src::levels::DC_128_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::HOR_PRED;
use crate::src::levels::LEFT_DC_PRED;
use crate::src::levels::PAETH_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::TOP_DC_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::levels::Z1_PRED;
use crate::src::levels::Z2_PRED;
use crate::src::levels::Z3_PRED;

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cold]
pub unsafe fn rav1d_intra_pred_dsp_init_8bpc(c: *mut Rav1dIntraPredDSPContext) {
    (*c).intra_pred[DC_PRED as usize] = Some(ipred_dc_c_erased::<BitDepth8>);
    (*c).intra_pred[DC_128_PRED as usize] = Some(ipred_dc_128_c_erased::<BitDepth8>);
    (*c).intra_pred[TOP_DC_PRED as usize] = Some(ipred_dc_top_c_erased::<BitDepth8>);
    (*c).intra_pred[LEFT_DC_PRED as usize] = Some(ipred_dc_left_c_erased::<BitDepth8>);
    (*c).intra_pred[HOR_PRED as usize] = Some(ipred_h_c_erased::<BitDepth8>);
    (*c).intra_pred[VERT_PRED as usize] = Some(ipred_v_c_erased::<BitDepth8>);
    (*c).intra_pred[PAETH_PRED as usize] = Some(ipred_paeth_c_erased::<BitDepth8>);
    (*c).intra_pred[SMOOTH_PRED as usize] = Some(ipred_smooth_c_erased::<BitDepth8>);
    (*c).intra_pred[SMOOTH_V_PRED as usize] = Some(ipred_smooth_v_c_erased::<BitDepth8>);
    (*c).intra_pred[SMOOTH_H_PRED as usize] = Some(ipred_smooth_h_c_erased::<BitDepth8>);
    (*c).intra_pred[Z1_PRED as usize] = Some(ipred_z1_c_erased::<BitDepth8>);
    (*c).intra_pred[Z2_PRED as usize] = Some(ipred_z2_c_erased::<BitDepth8>);
    (*c).intra_pred[Z3_PRED as usize] = Some(ipred_z3_c_erased::<BitDepth8>);
    (*c).intra_pred[FILTER_PRED as usize] = Some(ipred_filter_c_erased::<BitDepth8>);

    (*c).cfl_ac[Rav1dPixelLayout::I420 as usize - 1] = cfl_ac_420_c_erased::<BitDepth8>;
    (*c).cfl_ac[Rav1dPixelLayout::I422 as usize - 1] = cfl_ac_422_c_erased::<BitDepth8>;
    (*c).cfl_ac[Rav1dPixelLayout::I444 as usize - 1] = cfl_ac_444_c_erased::<BitDepth8>;
    (*c).cfl_pred[DC_PRED as usize] = ipred_cfl_c_erased::<BitDepth8>;
    (*c).cfl_pred[DC_128_PRED as usize] = ipred_cfl_128_c_erased::<BitDepth8>;
    (*c).cfl_pred[TOP_DC_PRED as usize] = ipred_cfl_top_c_erased::<BitDepth8>;
    (*c).cfl_pred[LEFT_DC_PRED as usize] = ipred_cfl_left_c_erased::<BitDepth8>;

    (*c).pal_pred = pal_pred_c_erased::<BitDepth8>;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            use crate::src::ipred::intra_pred_dsp_init_x86;

            intra_pred_dsp_init_x86::<BitDepth8>(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            use crate::src::ipred::intra_pred_dsp_init_arm;

            intra_pred_dsp_init_arm::<BitDepth8>(c);
        }
    }
}
