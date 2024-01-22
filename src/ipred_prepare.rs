use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::src::const_fn::const_for;
use crate::src::env::BlockContext;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EDGE_I444_LEFT_HAS_BOTTOM;
use crate::src::intra_edge::EDGE_I444_TOP_HAS_RIGHT;
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
use libc::memcpy;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::slice;

#[inline]
pub fn sm_flag(b: &BlockContext, idx: usize) -> c_int {
    if b.intra[idx] == 0 {
        return 0;
    }
    let m = b.mode[idx];
    if m == SMOOTH_PRED || m == SMOOTH_H_PRED || m == SMOOTH_V_PRED {
        512
    } else {
        0
    }
}

#[inline]
pub fn sm_uv_flag(b: &BlockContext, idx: usize) -> c_int {
    let m = b.uvmode[idx];
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

pub unsafe fn rav1d_prepare_intra_edges<BD: BitDepth>(
    x: c_int,
    have_left: bool,
    y: c_int,
    have_top: bool,
    w: c_int,
    h: c_int,
    edge_flags: EdgeFlags,
    dst: *const BD::Pixel,
    stride: ptrdiff_t,
    prefilter_toplevel_sb_edge: *const BD::Pixel,
    mut mode: IntraPredMode,
    angle: &mut c_int,
    tw: c_int,
    th: c_int,
    filter_edge: c_int,
    topleft_out: *mut BD::Pixel,
    bd: BD,
) -> IntraPredMode {
    let bitdepth = bd.bitdepth();
    if !(y < h && x < w) {
        unreachable!();
    }
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
    let mut dst_top: *const BD::Pixel = 0 as *const BD::Pixel;
    if have_top
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
        if !prefilter_toplevel_sb_edge.is_null() {
            dst_top = &*prefilter_toplevel_sb_edge.offset((x * 4) as isize) as *const BD::Pixel;
        } else {
            dst_top = &*dst.offset(-(BD::pxstride(stride as usize) as isize)) as *const BD::Pixel;
        }
    }
    if av1_intra_prediction_edges[mode as usize]
        .needs
        .contains(Needs::LEFT)
    {
        let sz = th << 2;
        let left: *mut BD::Pixel = &mut *topleft_out.offset(-sz as isize) as *mut BD::Pixel;
        if have_left {
            let px_have = cmp::min(sz, h - y << 2);
            let mut i = 0;
            while i < px_have {
                *left.offset((sz - 1 - i) as isize) =
                    *dst.offset(BD::pxstride(stride as usize) as isize * i as isize - 1);
                i += 1;
            }
            if px_have < sz {
                BD::pixel_set(
                    slice::from_raw_parts_mut(left, (sz - px_have).try_into().unwrap()),
                    *left.offset((sz - px_have) as isize),
                    (sz - px_have).try_into().unwrap(),
                );
            }
        } else {
            BD::pixel_set(
                slice::from_raw_parts_mut(left, sz.try_into().unwrap()),
                if have_top {
                    *dst_top
                } else {
                    ((1 << bitdepth >> 1) + 1).as_::<BD::Pixel>()
                },
                sz.try_into().unwrap(),
            );
        }
        if av1_intra_prediction_edges[mode as usize]
            .needs
            .contains(Needs::BOTTOM_LEFT)
        {
            let have_bottomleft = if !have_left || y + th >= h {
                false
            } else {
                (edge_flags & EDGE_I444_LEFT_HAS_BOTTOM) != 0
            };
            if have_bottomleft {
                let px_have_0 = cmp::min(sz, h - y - th << 2);
                let mut i_0 = 0;
                while i_0 < px_have_0 {
                    *left.offset(-(i_0 + 1) as isize) = *dst.offset(
                        ((sz + i_0) as isize * BD::pxstride(stride as usize) as isize - 1 as isize)
                            as isize,
                    );
                    i_0 += 1;
                }
                if px_have_0 < sz {
                    BD::pixel_set(
                        slice::from_raw_parts_mut(
                            left.offset(-(sz as isize)),
                            (sz - px_have_0).try_into().unwrap(),
                        ),
                        *left.offset(-px_have_0 as isize),
                        (sz - px_have_0).try_into().unwrap(),
                    );
                }
            } else {
                BD::pixel_set(
                    slice::from_raw_parts_mut(left.offset(-(sz as isize)), sz.try_into().unwrap()),
                    *left.offset(0),
                    sz.try_into().unwrap(),
                );
            }
        }
    }
    if av1_intra_prediction_edges[mode as usize]
        .needs
        .contains(Needs::TOP)
    {
        let sz_0 = tw << 2;
        let top: *mut BD::Pixel = &mut *topleft_out.offset(1) as *mut BD::Pixel;
        if have_top {
            let px_have_1 = cmp::min(sz_0, w - x << 2);
            memcpy(
                top as *mut c_void,
                dst_top as *const c_void,
                (px_have_1 << 1) as usize,
            );
            if px_have_1 < sz_0 {
                BD::pixel_set(
                    slice::from_raw_parts_mut(
                        top.offset(px_have_1 as isize),
                        (sz_0 - px_have_1).try_into().unwrap(),
                    ),
                    *top.offset((px_have_1 - 1) as isize),
                    (sz_0 - px_have_1).try_into().unwrap(),
                );
            }
        } else {
            BD::pixel_set(
                slice::from_raw_parts_mut(top, sz_0.try_into().unwrap()),
                if have_left {
                    *dst.offset(-1)
                } else {
                    ((1 << bitdepth >> 1) - 1).as_::<BD::Pixel>()
                },
                sz_0.try_into().unwrap(),
            );
        }
        if av1_intra_prediction_edges[mode as usize]
            .needs
            .contains(Needs::TOP_RIGHT)
        {
            let have_topright = if !have_top || x + tw >= w {
                false
            } else {
                (edge_flags & EDGE_I444_TOP_HAS_RIGHT) != 0
            };
            if have_topright {
                let px_have_2 = cmp::min(sz_0, w - x - tw << 2);
                memcpy(
                    top.offset(sz_0 as isize) as *mut c_void,
                    dst_top.offset(sz_0 as isize) as *const c_void,
                    (px_have_2 << 1) as usize,
                );
                if px_have_2 < sz_0 {
                    BD::pixel_set(
                        slice::from_raw_parts_mut(
                            top.offset(sz_0 as isize).offset(px_have_2 as isize),
                            (sz_0 - px_have_2).try_into().unwrap(),
                        ),
                        *top.offset((sz_0 + px_have_2 - 1) as isize),
                        (sz_0 - px_have_2).try_into().unwrap(),
                    );
                }
            } else {
                BD::pixel_set(
                    slice::from_raw_parts_mut(top.offset(sz_0 as isize), sz_0.try_into().unwrap()),
                    *top.offset((sz_0 - 1) as isize),
                    sz_0.try_into().unwrap(),
                );
            }
        }
    }
    if av1_intra_prediction_edges[mode as usize]
        .needs
        .contains(Needs::TOP_LEFT)
    {
        if have_left {
            *topleft_out = if have_top {
                *dst_top.offset(-1)
            } else {
                *dst.offset(-1)
            };
        } else {
            *topleft_out = if have_top {
                *dst_top
            } else {
                (1 << bitdepth >> 1).as_::<BD::Pixel>()
            };
        }
        if mode == Z2_PRED && tw + th >= 6 && filter_edge != 0 {
            *topleft_out = (((*topleft_out.offset(-1)).as_::<c_int>()
                + (*topleft_out.offset(1)).as_::<c_int>())
                * 5
                + (*topleft_out.offset(0)).as_::<c_int>() * 6
                + 8
                >> 4)
                .as_::<BD::Pixel>();
        }
    }
    return mode;
}
