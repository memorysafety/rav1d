use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::src::const_fn::const_for;
use crate::src::env::BlockContext;
use crate::src::intra_edge::EdgeFlags;
use crate::src::levels::IntraPredMode;
use crate::src::levels::DC_128_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::HOR_PRED;
use crate::src::levels::LEFT_DC_PRED;
use crate::src::levels::N_IMPL_INTRA_PRED_MODES;
use crate::src::levels::N_INTRA_PRED_MODES;
use crate::src::levels::PAETH_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::TOP_DC_PRED;
use crate::src::levels::VERT_LEFT_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::levels::Z1_PRED;
use crate::src::levels::Z2_PRED;
use crate::src::levels::Z3_PRED;
use bitflags::bitflags;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;

#[inline]
pub fn sm_flag(b: &BlockContext, idx: usize) -> c_int {
    if *b.intra.index(idx) == 0 {
        return 0;
    }
    let m = *b.mode.index(idx);
    if m == SMOOTH_PRED || m == SMOOTH_H_PRED || m == SMOOTH_V_PRED {
        512
    } else {
        0
    }
}

#[inline]
pub fn sm_uv_flag(b: &BlockContext, idx: usize) -> c_int {
    let m = *b.uvmode.index(idx);
    if m == SMOOTH_PRED || m == SMOOTH_H_PRED || m == SMOOTH_V_PRED {
        512
    } else {
        0
    }
}

static av1_mode_conv: [[[IntraPredMode; 2 /* have_top */]; 2 /* have_left */]; N_INTRA_PRED_MODES] = {
    let mut a = [[[0; 2]; 2]; N_INTRA_PRED_MODES];
    a[DC_PRED as usize] = [[DC_128_PRED, TOP_DC_PRED], [LEFT_DC_PRED, DC_PRED]];
    a[PAETH_PRED as usize] = [[DC_128_PRED, VERT_PRED], [HOR_PRED, PAETH_PRED]];
    a
};

static av1_mode_to_angle_map: [u8; 8] = [90, 180, 45, 135, 113, 157, 203, 67];

bitflags! {
    #[derive(Clone, Copy)]
    struct Needs: u8 {
        const LEFT = 1 << 0;
        const TOP = 1 << 1;
        const TOP_LEFT = 1 << 2;
        const TOP_RIGHT = 1 << 3;
        const BOTTOM_LEFT = 1 << 4;
    }
}

#[derive(Clone, Copy)]
struct av1_intra_prediction_edge {
    pub needs: Needs,
}

static av1_intra_prediction_edges: [av1_intra_prediction_edge; N_IMPL_INTRA_PRED_MODES] = {
    const LEFT: Needs = Needs::LEFT;
    const TOP: Needs = Needs::TOP;
    const TOP_LEFT: Needs = Needs::TOP_LEFT;
    const TOP_RIGHT: Needs = Needs::TOP_RIGHT;
    const BOTTOM_LEFT: Needs = Needs::BOTTOM_LEFT;

    const fn all<const N: usize>(a: [Needs; N]) -> Needs {
        let mut needs = Needs::empty();
        const_for!(i in 0..N => {
            needs = needs.union(a[i]);
        });
        needs
    }

    let mut a = [Needs::empty(); N_IMPL_INTRA_PRED_MODES];
    a[DC_PRED as usize] = all([TOP, LEFT]);
    a[VERT_PRED as usize] = all([TOP]);
    a[HOR_PRED as usize] = all([LEFT]);
    a[LEFT_DC_PRED as usize] = all([LEFT]);
    a[TOP_DC_PRED as usize] = all([TOP]);
    a[DC_128_PRED as usize] = all([]);
    a[Z1_PRED as usize] = all([TOP, TOP_RIGHT, TOP_LEFT]);
    a[Z2_PRED as usize] = all([LEFT, TOP, TOP_LEFT]);
    a[Z3_PRED as usize] = all([LEFT, BOTTOM_LEFT, TOP_LEFT]);
    a[SMOOTH_PRED as usize] = all([LEFT, TOP]);
    a[SMOOTH_V_PRED as usize] = all([LEFT, TOP]);
    a[SMOOTH_H_PRED as usize] = all([LEFT, TOP]);
    a[PAETH_PRED as usize] = all([LEFT, TOP, TOP_LEFT]);
    a[FILTER_PRED as usize] = all([LEFT, TOP, TOP_LEFT]);

    let mut b = [av1_intra_prediction_edge {
        needs: Needs::empty(),
    }; N_IMPL_INTRA_PRED_MODES];
    const_for!(i in 0..N_IMPL_INTRA_PRED_MODES => {
        b[i].needs = a[i];
    });

    b
};

pub fn rav1d_prepare_intra_edges<BD: BitDepth>(
    x: c_int,
    have_left: bool,
    y: c_int,
    have_top: bool,
    w: c_int,
    h: c_int,
    edge_flags: EdgeFlags,
    dst: &[BD::Pixel], // contains 4*h first rows of picture, last row in slice contains 4*w samples
    stride: ptrdiff_t,
    prefilter_toplevel_sb_edge: Option<&[BD::Pixel]>,
    mut mode: IntraPredMode,
    angle: &mut c_int,
    tw: c_int,
    th: c_int,
    filter_edge: u8,
    topleft_out: &mut [BD::Pixel],
    topleft_out_offset: usize, // position of top-left sample in `topleft_out`
    bd: BD,
) -> IntraPredMode {
    assert!(y < h && x < w);

    let bitdepth = bd.bitdepth();
    let stride = BD::pxstride(stride);

    let dst_offset = 4 * x as usize
        + (if stride >= 0 { 4 * y } else { 4 * (h - y) - 1 }) as usize * stride.unsigned_abs();

    match mode {
        VERT_PRED..=VERT_LEFT_PRED => {
            *angle = av1_mode_to_angle_map[(mode - VERT_PRED) as usize] as c_int + 3 * *angle;
            if *angle <= 90 {
                mode = if *angle < 90 && have_top {
                    Z1_PRED
                } else {
                    VERT_PRED
                };
            } else if *angle < 180 {
                mode = Z2_PRED;
            } else {
                mode = if *angle > 180 && have_left {
                    Z3_PRED
                } else {
                    HOR_PRED
                };
            }
        }
        DC_PRED | PAETH_PRED => {
            mode = av1_mode_conv[mode as usize][have_left as usize][have_top as usize];
        }
        _ => {}
    }

    // `dst_top` starts with either the top or top-left sample depending on whether have_left is true
    let dst_top = if have_top
        && (av1_intra_prediction_edges[mode as usize]
            .needs
            .contains(Needs::TOP)
            || av1_intra_prediction_edges[mode as usize]
                .needs
                .contains(Needs::TOP_LEFT)
            || av1_intra_prediction_edges[mode as usize]
                .needs
                .contains(Needs::LEFT)
                && !have_left)
    {
        let px_have = cmp::min(8 * tw, 4 * (w - x)) as usize;
        let n = px_have + have_left as usize;
        if let Some(prefilter_toplevel_sb_edge) = prefilter_toplevel_sb_edge {
            let offset = (x * 4) as usize - have_left as usize;
            &prefilter_toplevel_sb_edge[offset..][..n]
        } else {
            &dst[(dst_offset as isize - stride) as usize - have_left as usize..][..n]
        }
    } else {
        &[]
    };

    if av1_intra_prediction_edges[mode as usize]
        .needs
        .contains(Needs::LEFT)
    {
        let sz = 4 * th as usize;
        let left = &mut topleft_out[topleft_out_offset - sz..];
        if have_left {
            let px_have = cmp::min(sz, (h - y << 2) as usize);
            for i in 0..px_have {
                left[sz - 1 - i] = dst[i * stride as usize + dst_offset - 1];
            }
            if px_have < sz {
                BD::pixel_set(left, left[sz - px_have], sz - px_have);
            }
        } else {
            BD::pixel_set(
                left,
                if have_top {
                    dst_top[0] // have_left is always false
                } else {
                    ((1 << bitdepth >> 1) + 1).as_::<BD::Pixel>()
                },
                sz,
            );
        }
        if av1_intra_prediction_edges[mode as usize]
            .needs
            .contains(Needs::BOTTOM_LEFT)
        {
            let bottom_left = &mut topleft_out[topleft_out_offset - 2 * sz..];
            let have_bottomleft = if !have_left || y + th >= h {
                false
            } else {
                edge_flags.contains(EdgeFlags::I444_LEFT_HAS_BOTTOM)
            };
            if have_bottomleft {
                let px_have = cmp::min(sz, (h - y - th << 2) as usize);
                for i in 0..px_have {
                    bottom_left[sz - 1 - i] = dst[(sz + i) * stride as usize + dst_offset - 1];
                }
                if px_have < sz {
                    BD::pixel_set(bottom_left, bottom_left[sz - px_have], sz - px_have);
                }
            } else {
                BD::pixel_set(bottom_left, bottom_left[sz], sz);
            }
        }
    }
    if av1_intra_prediction_edges[mode as usize]
        .needs
        .contains(Needs::TOP)
    {
        let sz = 4 * tw as usize;
        let top = &mut topleft_out[topleft_out_offset + 1..];
        if have_top {
            let px_have = cmp::min(sz, (w - x << 2) as usize);
            BD::pixel_copy(top, &dst_top[have_left as usize..], px_have);
            if px_have < sz {
                let fill_value = top[px_have - 1];
                BD::pixel_set(&mut top[px_have..], fill_value, sz - px_have);
            }
        } else {
            BD::pixel_set(
                top,
                if have_left {
                    dst[dst_offset - 1]
                } else {
                    ((1 << bitdepth >> 1) - 1).as_::<BD::Pixel>()
                },
                sz,
            );
        }
        if av1_intra_prediction_edges[mode as usize]
            .needs
            .contains(Needs::TOP_RIGHT)
        {
            let have_topright = if !have_top || x + tw >= w {
                false
            } else {
                edge_flags.contains(EdgeFlags::I444_TOP_HAS_RIGHT)
            };
            if have_topright {
                let top_right = &mut top[sz..];
                let px_have = cmp::min(sz, (w - x - tw << 2) as usize);
                BD::pixel_copy(top_right, &dst_top[sz + have_left as usize..], px_have);
                if px_have < sz {
                    let fill_value = top_right[px_have - 1];
                    BD::pixel_set(&mut top_right[px_have..], fill_value, sz - px_have);
                }
            } else {
                let fill_value = top[sz - 1];
                BD::pixel_set(&mut top[sz..], fill_value, sz);
            }
        }
    }
    if av1_intra_prediction_edges[mode as usize]
        .needs
        .contains(Needs::TOP_LEFT)
    {
        // top-left sample and immediate neighbours
        let corner =
            <&mut [_; 3]>::try_from(&mut topleft_out[topleft_out_offset - 1..][..3]).unwrap();
        corner[1] = if have_top {
            dst_top[0]
        } else if have_left {
            dst[dst_offset - 1]
        } else {
            (1 << bitdepth >> 1).as_::<BD::Pixel>()
        };
        if mode == Z2_PRED && tw + th >= 6 && filter_edge != 0 {
            corner[1] = ((corner[0].as_::<c_int>() + corner[2].as_::<c_int>()) * 5
                + corner[1].as_::<c_int>() * 6
                + 8
                >> 4)
                .as_::<BD::Pixel>();
        }
    }
    return mode;
}
