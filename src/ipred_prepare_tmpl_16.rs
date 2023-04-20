use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type pixel = uint16_t;
use crate::src::levels::IntraPredMode;

use crate::src::levels::Z3_PRED;
use crate::src::levels::Z2_PRED;
use crate::src::levels::Z1_PRED;
use crate::src::levels::DC_128_PRED;
use crate::src::levels::TOP_DC_PRED;
use crate::src::levels::LEFT_DC_PRED;
use crate::src::levels::PAETH_PRED;

use crate::src::levels::HOR_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::levels::DC_PRED;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EDGE_I444_LEFT_HAS_BOTTOM;
use crate::src::intra_edge::EDGE_I444_TOP_HAS_RIGHT;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct av1_intra_prediction_edge {
    #[bitfield(name = "needs_left", ty = "uint8_t", bits = "0..=0")]
    #[bitfield(name = "needs_top", ty = "uint8_t", bits = "1..=1")]
    #[bitfield(name = "needs_topleft", ty = "uint8_t", bits = "2..=2")]
    #[bitfield(name = "needs_topright", ty = "uint8_t", bits = "3..=3")]
    #[bitfield(name = "needs_bottomleft", ty = "uint8_t", bits = "4..=4")]
    pub needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [u8; 1],
}
use crate::include::common::attributes::clz;
use crate::include::common::intops::imin;
#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn pixel_set(dst: *mut pixel, val: libc::c_int, num: libc::c_int) {
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < num {
        *dst.offset(n as isize) = val as pixel;
        n += 1;
    }
}
static mut av1_mode_conv: [[[uint8_t; 2]; 2]; 13] = [
    [
        [DC_128_PRED as libc::c_int as uint8_t, TOP_DC_PRED as libc::c_int as uint8_t],
        [LEFT_DC_PRED as libc::c_int as uint8_t, DC_PRED as libc::c_int as uint8_t],
    ],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [
        [DC_128_PRED as libc::c_int as uint8_t, VERT_PRED as libc::c_int as uint8_t],
        [HOR_PRED as libc::c_int as uint8_t, PAETH_PRED as libc::c_int as uint8_t],
    ],
];
static mut av1_mode_to_angle_map: [uint8_t; 8] = [
    90 as libc::c_int as uint8_t,
    180 as libc::c_int as uint8_t,
    45 as libc::c_int as uint8_t,
    135 as libc::c_int as uint8_t,
    113 as libc::c_int as uint8_t,
    157 as libc::c_int as uint8_t,
    203 as libc::c_int as uint8_t,
    67 as libc::c_int as uint8_t,
];
static mut av1_intra_prediction_edges: [av1_intra_prediction_edge; 14] = [av1_intra_prediction_edge {
    needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
}; 14];
#[no_mangle]
pub unsafe extern "C" fn dav1d_prepare_intra_edges_16bpc(
    x: libc::c_int,
    have_left: libc::c_int,
    y: libc::c_int,
    have_top: libc::c_int,
    w: libc::c_int,
    h: libc::c_int,
    edge_flags: EdgeFlags,
    dst: *const pixel,
    stride: ptrdiff_t,
    mut prefilter_toplevel_sb_edge: *const pixel,
    mut mode: IntraPredMode,
    angle: *mut libc::c_int,
    tw: libc::c_int,
    th: libc::c_int,
    filter_edge: libc::c_int,
    topleft_out: *mut pixel,
    bitdepth_max: libc::c_int,
) -> IntraPredMode {
    let bitdepth: libc::c_int = 32 as libc::c_int - clz(bitdepth_max as libc::c_uint);
    if !(y < h && x < w) {
        unreachable!();
    }
    match mode as libc::c_uint {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
            *angle = av1_mode_to_angle_map[(mode as libc::c_uint)
                .wrapping_sub(VERT_PRED as libc::c_int as libc::c_uint) as usize]
                as libc::c_int + 3 as libc::c_int * *angle;
            if *angle <= 90 as libc::c_int {
                mode = (if *angle < 90 as libc::c_int && have_top != 0 {
                    Z1_PRED as libc::c_int
                } else {
                    VERT_PRED as libc::c_int
                }) as IntraPredMode;
            } else if *angle < 180 as libc::c_int {
                mode = Z2_PRED;
            } else {
                mode = (if *angle > 180 as libc::c_int && have_left != 0 {
                    Z3_PRED as libc::c_int
                } else {
                    HOR_PRED as libc::c_int
                }) as IntraPredMode;
            }
        }
        0 | 12 => {
            mode = av1_mode_conv[mode as usize][have_left as usize][have_top as usize]
                as IntraPredMode;
        }
        _ => {}
    }
    let mut dst_top: *const pixel = 0 as *const pixel;
    if have_top != 0
        && ((av1_intra_prediction_edges[mode as usize]).needs_top() as libc::c_int != 0
            || (av1_intra_prediction_edges[mode as usize]).needs_topleft() as libc::c_int
                != 0
            || (av1_intra_prediction_edges[mode as usize]).needs_left() as libc::c_int
                != 0 && have_left == 0)
    {
        if !prefilter_toplevel_sb_edge.is_null() {
            dst_top = &*prefilter_toplevel_sb_edge
                .offset((x * 4 as libc::c_int) as isize) as *const pixel;
        } else {
            dst_top = &*dst
                .offset(
                    -(PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(stride)
                        as isize,
                ) as *const pixel;
        }
    }
    if (av1_intra_prediction_edges[mode as usize]).needs_left() != 0 {
        let sz: libc::c_int = th << 2 as libc::c_int;
        let left: *mut pixel = &mut *topleft_out.offset(-sz as isize) as *mut pixel;
        if have_left != 0 {
            let px_have: libc::c_int = imin(sz, h - y << 2 as libc::c_int);
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < px_have {
                *left
                    .offset(
                        (sz - 1 as libc::c_int - i) as isize,
                    ) = *dst
                    .offset(
                        PXSTRIDE(stride) * i as isize - 1,
                    );
                i += 1;
            }
            if px_have < sz {
                pixel_set(
                    left,
                    *left.offset((sz - px_have) as isize) as libc::c_int,
                    sz - px_have,
                );
            }
        } else {
            pixel_set(
                left,
                if have_top != 0 {
                    *dst_top as libc::c_int
                } else {
                    ((1 as libc::c_int) << bitdepth >> 1 as libc::c_int)
                        + 1 as libc::c_int
                },
                sz,
            );
        }
        if (av1_intra_prediction_edges[mode as usize]).needs_bottomleft() != 0 {
            let have_bottomleft: libc::c_int = (if have_left == 0 || y + th >= h {
                0 as libc::c_int as libc::c_uint
            } else {
                edge_flags as libc::c_uint
                    & EDGE_I444_LEFT_HAS_BOTTOM as libc::c_int as libc::c_uint
            }) as libc::c_int;
            if have_bottomleft != 0 {
                let px_have_0: libc::c_int = imin(sz, h - y - th << 2 as libc::c_int);
                let mut i_0: libc::c_int = 0 as libc::c_int;
                while i_0 < px_have_0 {
                    *left
                        .offset(
                            -(i_0 + 1 as libc::c_int) as isize,
                        ) = *dst
                        .offset(
                            ((sz + i_0) as isize * PXSTRIDE(stride)
                                - 1 as libc::c_int as isize) as isize,
                        );
                    i_0 += 1;
                }
                if px_have_0 < sz {
                    pixel_set(
                        left.offset(-(sz as isize)),
                        *left.offset(-px_have_0 as isize) as libc::c_int,
                        sz - px_have_0,
                    );
                }
            } else {
                pixel_set(
                    left.offset(-(sz as isize)),
                    *left.offset(0) as libc::c_int,
                    sz,
                );
            }
        }
    }
    if (av1_intra_prediction_edges[mode as usize]).needs_top() != 0 {
        let sz_0: libc::c_int = tw << 2 as libc::c_int;
        let top: *mut pixel = &mut *topleft_out.offset(1)
            as *mut pixel;
        if have_top != 0 {
            let px_have_1: libc::c_int = imin(sz_0, w - x << 2 as libc::c_int);
            memcpy(
                top as *mut libc::c_void,
                dst_top as *const libc::c_void,
                (px_have_1 << 1 as libc::c_int) as libc::c_ulong,
            );
            if px_have_1 < sz_0 {
                pixel_set(
                    top.offset(px_have_1 as isize),
                    *top.offset((px_have_1 - 1 as libc::c_int) as isize) as libc::c_int,
                    sz_0 - px_have_1,
                );
            }
        } else {
            pixel_set(
                top,
                if have_left != 0 {
                    *dst.offset(-(1 as libc::c_int) as isize) as libc::c_int
                } else {
                    ((1 as libc::c_int) << bitdepth >> 1 as libc::c_int)
                        - 1 as libc::c_int
                },
                sz_0,
            );
        }
        if (av1_intra_prediction_edges[mode as usize]).needs_topright() != 0 {
            let have_topright: libc::c_int = (if have_top == 0 || x + tw >= w {
                0 as libc::c_int as libc::c_uint
            } else {
                edge_flags as libc::c_uint
                    & EDGE_I444_TOP_HAS_RIGHT as libc::c_int as libc::c_uint
            }) as libc::c_int;
            if have_topright != 0 {
                let px_have_2: libc::c_int = imin(sz_0, w - x - tw << 2 as libc::c_int);
                memcpy(
                    top.offset(sz_0 as isize) as *mut libc::c_void,
                    &*dst_top.offset(sz_0 as isize) as *const pixel
                        as *const libc::c_void,
                    (px_have_2 << 1 as libc::c_int) as libc::c_ulong,
                );
                if px_have_2 < sz_0 {
                    pixel_set(
                        top.offset(sz_0 as isize).offset(px_have_2 as isize),
                        *top.offset((sz_0 + px_have_2 - 1 as libc::c_int) as isize)
                            as libc::c_int,
                        sz_0 - px_have_2,
                    );
                }
            } else {
                pixel_set(
                    top.offset(sz_0 as isize),
                    *top.offset((sz_0 - 1 as libc::c_int) as isize) as libc::c_int,
                    sz_0,
                );
            }
        }
    }
    if (av1_intra_prediction_edges[mode as usize]).needs_topleft() != 0 {
        if have_left != 0 {
            *topleft_out = (if have_top != 0 {
                *dst_top.offset(-(1 as libc::c_int) as isize) as libc::c_int
            } else {
                *dst.offset(-(1 as libc::c_int) as isize) as libc::c_int
            }) as pixel;
        } else {
            *topleft_out = (if have_top != 0 {
                *dst_top as libc::c_int
            } else {
                (1 as libc::c_int) << bitdepth >> 1 as libc::c_int
            }) as pixel;
        }
        if mode as libc::c_uint == Z2_PRED as libc::c_int as libc::c_uint
            && tw + th >= 6 as libc::c_int && filter_edge != 0
        {
            *topleft_out = ((*topleft_out.offset(-(1 as libc::c_int) as isize)
                as libc::c_int
                + *topleft_out.offset(1) as libc::c_int)
                * 5 as libc::c_int
                + *topleft_out.offset(0) as libc::c_int
                    * 6 as libc::c_int + 8 as libc::c_int >> 4 as libc::c_int) as pixel;
        }
    }
    return mode;
}
unsafe extern "C" fn run_static_initializers() {
    av1_intra_prediction_edges = [
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(0);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(0);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0 as libc::c_int as uint8_t);
            init.set_needs_top(0);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(1 as libc::c_int as uint8_t);
            init.set_needs_topright(1 as libc::c_int as uint8_t);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(1 as libc::c_int as uint8_t);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(0);
            init.set_needs_topleft(1 as libc::c_int as uint8_t);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(1 as libc::c_int as uint8_t);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(1 as libc::c_int as uint8_t);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as libc::c_int as uint8_t);
            init.set_needs_top(1 as libc::c_int as uint8_t);
            init.set_needs_topleft(1 as libc::c_int as uint8_t);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
