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
use crate::src::levels::VERT_PRED;
use crate::src::levels::Z1_PRED;
use crate::src::levels::Z2_PRED;
use crate::src::levels::Z3_PRED;
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
    return if m == SMOOTH_PRED || m == SMOOTH_H_PRED || m == SMOOTH_V_PRED {
        512
    } else {
        0
    };
}

#[inline]
pub fn sm_uv_flag(b: &BlockContext, idx: usize) -> c_int {
    let m = b.uvmode[idx];
    return if m == SMOOTH_PRED || m == SMOOTH_H_PRED || m == SMOOTH_V_PRED {
        512
    } else {
        0
    };
}

static av1_mode_conv: [[[IntraPredMode; 2 /* have_top */]; 2 /* have_left */]; N_INTRA_PRED_MODES] = {
    let mut a = [[[0; 2]; 2]; N_INTRA_PRED_MODES];
    a[DC_PRED as usize] = [[DC_128_PRED, TOP_DC_PRED], [LEFT_DC_PRED, DC_PRED]];
    a[PAETH_PRED as usize] = [[DC_128_PRED, VERT_PRED], [HOR_PRED, PAETH_PRED]];
    a
};

static av1_mode_to_angle_map: [u8; 8] = [90, 180, 45, 135, 113, 157, 203, 67];

macro_rules! bools_bitfield_struct {
    (
        type Bits = $Bits:ty;

        struct $T:ident {
            $($vis:vis $field:ident: bool: $index:literal,)*
        }
    ) => {
        #[derive(Clone, Copy, Default, PartialEq, Eq)]
        struct $T {
            bits: $Bits,
        }

        impl $T {
            const fn empty() -> Self {
                Self {
                    bits: 0,
                }
            }

            const fn bit(self, index: usize) -> bool {
                ((self.bits >> index) & 1) != 0
            }

            const fn with_bit(self, index: usize, value: bool) -> Self {
                Self {
                    bits: self.bits | (value as u8) << index,
                }
            }
        }

        paste::paste! {
            impl $T {
                $(
                    pub const fn $field(self) -> bool {
                        self.bit($index)
                    }

                    $vis const fn [<with_ $field>](self, value: bool) -> Self {
                        self.with_bit($index, value)
                    }

                    $vis const fn [<set_ $field>](self) -> Self {
                        self.[<with_ $field>](true)
                    }

                    #[allow(dead_code)]
                    $vis const fn [<unset_ $field>](self) -> Self {
                        self.[<with_ $field>](false)
                    }
                )*

                #[allow(dead_code)]
                pub const fn new(
                    $($field: bool),*
                ) -> Self {
                    Self::empty()
                        $(.[<with_ $field>]($field))*
                }
            }
        }
    };
}

bools_bitfield_struct! {
    type Bits = u8;

    struct Needs {
        left: bool: 0,
        top: bool: 1,
        top_left: bool: 2,
        top_right: bool: 3,
        bottom_left: bool: 4,
    }
}

#[derive(Clone, Copy)]
struct av1_intra_prediction_edge {
    pub needs: Needs,
}

static av1_intra_prediction_edges: [av1_intra_prediction_edge; N_IMPL_INTRA_PRED_MODES] = {
    let mut a = [Needs::empty(); N_IMPL_INTRA_PRED_MODES];
    a[DC_PRED as usize] = Needs::empty().set_top().set_left();
    a[VERT_PRED as usize] = Needs::empty().set_top();
    a[HOR_PRED as usize] = Needs::empty().set_left();
    a[LEFT_DC_PRED as usize] = Needs::empty().set_left();
    a[TOP_DC_PRED as usize] = Needs::empty().set_top();
    a[DC_128_PRED as usize] = Needs::empty();
    a[Z1_PRED as usize] = Needs::empty().set_top().set_top_right().set_top_left();
    a[Z2_PRED as usize] = Needs::empty().set_left().set_top().set_top_left();
    a[Z3_PRED as usize] = Needs::empty().set_left().set_bottom_left().set_top_left();
    a[SMOOTH_PRED as usize] = Needs::empty().set_left().set_top();
    a[SMOOTH_V_PRED as usize] = Needs::empty().set_left().set_top();
    a[SMOOTH_H_PRED as usize] = Needs::empty().set_left().set_top();
    a[PAETH_PRED as usize] = Needs::empty().set_left().set_top().set_top_left();
    a[FILTER_PRED as usize] = Needs::empty().set_left().set_top().set_top_left();

    let mut b = [av1_intra_prediction_edge {
        needs: Needs::empty(),
    }; N_IMPL_INTRA_PRED_MODES];
    const_for!(i in 0..N_IMPL_INTRA_PRED_MODES => {
        b[i].needs = a[i];
    });

    b
};

pub unsafe fn dav1d_prepare_intra_edges<BD: BitDepth>(
    x: c_int,
    have_left: c_int,
    y: c_int,
    have_top: c_int,
    w: c_int,
    h: c_int,
    edge_flags: EdgeFlags,
    dst: *const BD::Pixel,
    stride: ptrdiff_t,
    prefilter_toplevel_sb_edge: *const BD::Pixel,
    mut mode: IntraPredMode,
    angle: *mut c_int,
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
    match mode as c_uint {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
            *angle = av1_mode_to_angle_map
                [(mode as c_uint).wrapping_sub(VERT_PRED as c_int as c_uint) as usize]
                as c_int
                + 3 * *angle;
            if *angle <= 90 {
                mode = (if *angle < 90 && have_top != 0 {
                    Z1_PRED as c_int
                } else {
                    VERT_PRED as c_int
                }) as IntraPredMode;
            } else if *angle < 180 {
                mode = Z2_PRED;
            } else {
                mode = (if *angle > 180 && have_left != 0 {
                    Z3_PRED as c_int
                } else {
                    HOR_PRED as c_int
                }) as IntraPredMode;
            }
        }
        0 | 12 => {
            mode = av1_mode_conv[mode as usize][have_left as usize][have_top as usize]
                as IntraPredMode;
        }
        _ => {}
    }
    let mut dst_top: *const BD::Pixel = 0 as *const BD::Pixel;
    if have_top != 0
        && (av1_intra_prediction_edges[mode as usize].needs.top()
            || av1_intra_prediction_edges[mode as usize].needs.top_left()
            || av1_intra_prediction_edges[mode as usize].needs.left() && have_left == 0)
    {
        if !prefilter_toplevel_sb_edge.is_null() {
            dst_top = &*prefilter_toplevel_sb_edge.offset((x * 4) as isize) as *const BD::Pixel;
        } else {
            dst_top = &*dst.offset(-(BD::pxstride(stride as usize) as isize)) as *const BD::Pixel;
        }
    }
    if av1_intra_prediction_edges[mode as usize].needs.left() {
        let sz = th << 2;
        let left: *mut BD::Pixel = &mut *topleft_out.offset(-sz as isize) as *mut BD::Pixel;
        if have_left != 0 {
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
                if have_top != 0 {
                    *dst_top
                } else {
                    ((1 << bitdepth >> 1) + 1).as_::<BD::Pixel>()
                },
                sz.try_into().unwrap(),
            );
        }
        if av1_intra_prediction_edges[mode as usize]
            .needs
            .bottom_left()
        {
            let have_bottomleft = (if have_left == 0 || y + th >= h {
                0 as c_int as c_uint
            } else {
                edge_flags as c_uint & EDGE_I444_LEFT_HAS_BOTTOM as c_int as c_uint
            }) as c_int;
            if have_bottomleft != 0 {
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
    if av1_intra_prediction_edges[mode as usize].needs.top() {
        let sz_0 = tw << 2;
        let top: *mut BD::Pixel = &mut *topleft_out.offset(1) as *mut BD::Pixel;
        if have_top != 0 {
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
                if have_left != 0 {
                    *dst.offset(-(1 as c_int) as isize)
                } else {
                    ((1 << bitdepth >> 1) - 1).as_::<BD::Pixel>()
                },
                sz_0.try_into().unwrap(),
            );
        }
        if av1_intra_prediction_edges[mode as usize].needs.top_right() {
            let have_topright = (if have_top == 0 || x + tw >= w {
                0 as c_int as c_uint
            } else {
                edge_flags as c_uint & EDGE_I444_TOP_HAS_RIGHT as c_int as c_uint
            }) as c_int;
            if have_topright != 0 {
                let px_have_2 = cmp::min(sz_0, w - x - tw << 2);
                memcpy(
                    top.offset(sz_0 as isize) as *mut c_void,
                    &*dst_top.offset(sz_0 as isize) as *const BD::Pixel as *const c_void,
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
    if av1_intra_prediction_edges[mode as usize].needs.top_left() {
        if have_left != 0 {
            *topleft_out = (if have_top != 0 {
                (*dst_top.offset(-(1 as c_int) as isize)).as_::<c_int>()
            } else {
                (*dst.offset(-(1 as c_int) as isize)).as_::<c_int>()
            })
            .as_::<BD::Pixel>();
        } else {
            *topleft_out = (if have_top != 0 {
                (*dst_top).as_::<c_int>()
            } else {
                (1 as c_int) << bitdepth >> 1
            })
            .as_::<BD::Pixel>();
        }
        if mode as c_uint == Z2_PRED as c_int as c_uint && tw + th >= 6 && filter_edge != 0 {
            *topleft_out = (((*topleft_out.offset(-(1 as c_int) as isize)).as_::<c_int>()
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
