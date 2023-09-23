use crate::include::common::attributes::clz;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EDGE_I444_LEFT_HAS_BOTTOM;
use crate::src::intra_edge::EDGE_I444_TOP_HAS_RIGHT;
use crate::src::ipred_prepare::av1_intra_prediction_edge;
use crate::src::ipred_prepare::av1_mode_conv;
use crate::src::ipred_prepare::av1_mode_to_angle_map;
use crate::src::levels::IntraPredMode;
use crate::src::levels::HOR_PRED;
use crate::src::levels::N_IMPL_INTRA_PRED_MODES;
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

pub type pixel = u16;

#[inline]
unsafe extern "C" fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

#[inline]
unsafe extern "C" fn pixel_set(dst: *mut pixel, val: c_int, num: c_int) {
    let mut n = 0;
    while n < num {
        *dst.offset(n as isize) = val as pixel;
        n += 1;
    }
}

static mut av1_intra_prediction_edges: [av1_intra_prediction_edge; 14] =
    [av1_intra_prediction_edge {
        needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
    }; N_IMPL_INTRA_PRED_MODES];

pub unsafe fn dav1d_prepare_intra_edges_16bpc(
    x: c_int,
    have_left: c_int,
    y: c_int,
    have_top: c_int,
    w: c_int,
    h: c_int,
    edge_flags: EdgeFlags,
    dst: *const pixel,
    stride: ptrdiff_t,
    prefilter_toplevel_sb_edge: *const pixel,
    mut mode: IntraPredMode,
    angle: *mut c_int,
    tw: c_int,
    th: c_int,
    filter_edge: c_int,
    topleft_out: *mut pixel,
    bitdepth_max: c_int,
) -> IntraPredMode {
    let bitdepth = 32 - clz(bitdepth_max as c_uint);
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
    let mut dst_top: *const pixel = 0 as *const pixel;
    if have_top != 0
        && ((av1_intra_prediction_edges[mode as usize]).needs_top() as c_int != 0
            || (av1_intra_prediction_edges[mode as usize]).needs_topleft() as c_int != 0
            || (av1_intra_prediction_edges[mode as usize]).needs_left() as c_int != 0
                && have_left == 0)
    {
        if !prefilter_toplevel_sb_edge.is_null() {
            dst_top = &*prefilter_toplevel_sb_edge.offset((x * 4) as isize) as *const pixel;
        } else {
            dst_top = &*dst.offset(-(PXSTRIDE as unsafe extern "C" fn(ptrdiff_t) -> ptrdiff_t)(
                stride,
            ) as isize) as *const pixel;
        }
    }
    if (av1_intra_prediction_edges[mode as usize]).needs_left() != 0 {
        let sz = th << 2;
        let left: *mut pixel = &mut *topleft_out.offset(-sz as isize) as *mut pixel;
        if have_left != 0 {
            let px_have = cmp::min(sz, h - y << 2);
            let mut i = 0;
            while i < px_have {
                *left.offset((sz - 1 - i) as isize) =
                    *dst.offset(PXSTRIDE(stride) * i as isize - 1);
                i += 1;
            }
            if px_have < sz {
                pixel_set(
                    left,
                    *left.offset((sz - px_have) as isize) as c_int,
                    sz - px_have,
                );
            }
        } else {
            pixel_set(
                left,
                if have_top != 0 {
                    *dst_top as c_int
                } else {
                    ((1 as c_int) << bitdepth >> 1) + 1
                },
                sz,
            );
        }
        if (av1_intra_prediction_edges[mode as usize]).needs_bottomleft() != 0 {
            let have_bottomleft = (if have_left == 0 || y + th >= h {
                0 as c_int as c_uint
            } else {
                edge_flags as c_uint & EDGE_I444_LEFT_HAS_BOTTOM as c_int as c_uint
            }) as c_int;
            if have_bottomleft != 0 {
                let px_have_0 = cmp::min(sz, h - y - th << 2);
                let mut i_0 = 0;
                while i_0 < px_have_0 {
                    *left.offset(-(i_0 + 1) as isize) =
                        *dst.offset(((sz + i_0) as isize * PXSTRIDE(stride) - 1 as isize) as isize);
                    i_0 += 1;
                }
                if px_have_0 < sz {
                    pixel_set(
                        left.offset(-(sz as isize)),
                        *left.offset(-px_have_0 as isize) as c_int,
                        sz - px_have_0,
                    );
                }
            } else {
                pixel_set(left.offset(-(sz as isize)), *left.offset(0) as c_int, sz);
            }
        }
    }
    if (av1_intra_prediction_edges[mode as usize]).needs_top() != 0 {
        let sz_0 = tw << 2;
        let top: *mut pixel = &mut *topleft_out.offset(1) as *mut pixel;
        if have_top != 0 {
            let px_have_1 = cmp::min(sz_0, w - x << 2);
            memcpy(
                top as *mut c_void,
                dst_top as *const c_void,
                (px_have_1 << 1) as usize,
            );
            if px_have_1 < sz_0 {
                pixel_set(
                    top.offset(px_have_1 as isize),
                    *top.offset((px_have_1 - 1) as isize) as c_int,
                    sz_0 - px_have_1,
                );
            }
        } else {
            pixel_set(
                top,
                if have_left != 0 {
                    *dst.offset(-(1 as c_int) as isize) as c_int
                } else {
                    ((1 as c_int) << bitdepth >> 1) - 1
                },
                sz_0,
            );
        }
        if (av1_intra_prediction_edges[mode as usize]).needs_topright() != 0 {
            let have_topright = (if have_top == 0 || x + tw >= w {
                0 as c_int as c_uint
            } else {
                edge_flags as c_uint & EDGE_I444_TOP_HAS_RIGHT as c_int as c_uint
            }) as c_int;
            if have_topright != 0 {
                let px_have_2 = cmp::min(sz_0, w - x - tw << 2);
                memcpy(
                    top.offset(sz_0 as isize) as *mut c_void,
                    &*dst_top.offset(sz_0 as isize) as *const pixel as *const c_void,
                    (px_have_2 << 1) as usize,
                );
                if px_have_2 < sz_0 {
                    pixel_set(
                        top.offset(sz_0 as isize).offset(px_have_2 as isize),
                        *top.offset((sz_0 + px_have_2 - 1) as isize) as c_int,
                        sz_0 - px_have_2,
                    );
                }
            } else {
                pixel_set(
                    top.offset(sz_0 as isize),
                    *top.offset((sz_0 - 1) as isize) as c_int,
                    sz_0,
                );
            }
        }
    }
    if (av1_intra_prediction_edges[mode as usize]).needs_topleft() != 0 {
        if have_left != 0 {
            *topleft_out = (if have_top != 0 {
                *dst_top.offset(-(1 as c_int) as isize) as c_int
            } else {
                *dst.offset(-(1 as c_int) as isize) as c_int
            }) as pixel;
        } else {
            *topleft_out = (if have_top != 0 {
                *dst_top as c_int
            } else {
                (1 as c_int) << bitdepth >> 1
            }) as pixel;
        }
        if mode as c_uint == Z2_PRED as c_int as c_uint && tw + th >= 6 && filter_edge != 0 {
            *topleft_out = ((*topleft_out.offset(-(1 as c_int) as isize) as c_int
                + *topleft_out.offset(1) as c_int)
                * 5
                + *topleft_out.offset(0) as c_int * 6
                + 8
                >> 4) as pixel;
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
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
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
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
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
            init.set_needs_left(1 as c_int as u8);
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
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0 as c_int as u8);
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
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(1 as c_int as u8);
            init.set_needs_topright(1 as c_int as u8);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(1 as c_int as u8);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(0);
            init.set_needs_topleft(1 as c_int as u8);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(1 as c_int as u8);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(1 as c_int as u8);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = av1_intra_prediction_edge {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as c_int as u8);
            init.set_needs_top(1 as c_int as u8);
            init.set_needs_topleft(1 as c_int as u8);
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
