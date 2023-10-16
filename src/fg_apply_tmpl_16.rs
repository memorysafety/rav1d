use crate::include::common::bitdepth::BitDepth16;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::align::Align1;
use crate::src::align::Align16;
use crate::src::fg_apply::rav1d_apply_grain_row;
use crate::src::fg_apply::rav1d_prep_grain;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;

pub type entry = i16;

pub(crate) unsafe fn rav1d_apply_grain_16bpc(
    dsp: *const Rav1dFilmGrainDSPContext,
    out: *mut Rav1dPicture,
    in_0: *const Rav1dPicture,
) {
    let mut grain_lut = Align16([[[0; 82]; 74]; 3]);
    let mut scaling = Align1([[0; 4096]; 3]);
    let rows = (*out).p.h + 31 >> 5;
    rav1d_prep_grain::<BitDepth16>(
        dsp,
        out,
        in_0,
        scaling.0.as_mut_ptr(),
        grain_lut.0.as_mut_ptr(),
    );
    let mut row = 0;
    while row < rows {
        rav1d_apply_grain_row::<BitDepth16>(
            dsp,
            out,
            in_0,
            scaling.0.as_mut_ptr() as *const [u8; 4096],
            grain_lut.0.as_mut_ptr() as *const [[entry; 82]; 74],
            row,
        );
        row += 1;
    }
}
