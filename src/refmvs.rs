use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use cfg_if::cfg_if;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> libc::c_int;
    static dav1d_block_dimensions: [[uint8_t; 4]; 22];
}

#[cfg(feature = "asm")]
extern "C" {
    static mut dav1d_cpu_flags_mask: libc::c_uint;
    static mut dav1d_cpu_flags: libc::c_uint;
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "x86", target_arch = "x86_64"),
))]
extern "C" {
    fn dav1d_splat_mv_avx512icl(
        rr: *mut *mut refmvs_block,
        rmv: *const refmvs_block,
        bx4: libc::c_int,
        bw4: libc::c_int,
        bh4: libc::c_int,
    );
    fn dav1d_splat_mv_avx2(
        rr: *mut *mut refmvs_block,
        rmv: *const refmvs_block,
        bx4: libc::c_int,
        bw4: libc::c_int,
        bh4: libc::c_int,
    );
    fn dav1d_splat_mv_sse2(
        rr: *mut *mut refmvs_block,
        rmv: *const refmvs_block,
        bx4: libc::c_int,
        bw4: libc::c_int,
        bh4: libc::c_int,
    );
}

#[cfg(all(
    feature = "asm",
    any(target_arch = "arm", target_arch = "aarch64"),
))]
extern "C" {
    fn dav1d_splat_mv_neon(
        rr: *mut *mut refmvs_block,
        rmv: *const refmvs_block,
        bx4: libc::c_int,
        bw4: libc::c_int,
        bh4: libc::c_int,
    );
}
use crate::include::dav1d::headers::DAV1D_WM_TYPE_TRANSLATION;

use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::src::levels::BlockSize;

use crate::src::levels::mv;
use crate::src::intra_edge::EdgeFlags;

use crate::src::intra_edge::EDGE_I444_TOP_HAS_RIGHT;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_temporal_block {
    pub mv: mv,
    pub r#ref: int8_t,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct refmvs_refpair {
    pub r#ref: [int8_t; 2],
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct refmvs_mvpair {
    pub mv: [mv; 2],
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct refmvs_block {
    pub mv: refmvs_mvpair,
    pub r#ref: refmvs_refpair,
    pub bs: uint8_t,
    pub mf: uint8_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_frame {
    pub frm_hdr: *const Dav1dFrameHeader,
    pub iw4: libc::c_int,
    pub ih4: libc::c_int,
    pub iw8: libc::c_int,
    pub ih8: libc::c_int,
    pub sbsz: libc::c_int,
    pub use_ref_frame_mvs: libc::c_int,
    pub sign_bias: [uint8_t; 7],
    pub mfmv_sign: [uint8_t; 7],
    pub pocdiff: [int8_t; 7],
    pub mfmv_ref: [uint8_t; 3],
    pub mfmv_ref2cur: [libc::c_int; 3],
    pub mfmv_ref2ref: [[libc::c_int; 7]; 3],
    pub n_mfmvs: libc::c_int,
    pub rp: *mut refmvs_temporal_block,
    pub rp_ref: *const *mut refmvs_temporal_block,
    pub rp_proj: *mut refmvs_temporal_block,
    pub rp_stride: ptrdiff_t,
    pub r: *mut refmvs_block,
    pub r_stride: ptrdiff_t,
    pub n_tile_rows: libc::c_int,
    pub n_tile_threads: libc::c_int,
    pub n_frame_threads: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_tile_range {
    pub start: libc::c_int,
    pub end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_tile {
    pub rf: *const refmvs_frame,
    pub r: [*mut refmvs_block; 37],
    pub rp_proj: *mut refmvs_temporal_block,
    pub tile_col: refmvs_tile_range,
    pub tile_row: refmvs_tile_range,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_candidate {
    pub mv: refmvs_mvpair,
    pub weight: libc::c_int,
}
pub type splat_mv_fn = Option::<
    unsafe extern "C" fn(
        *mut *mut refmvs_block,
        *const refmvs_block,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dRefmvsDSPContext {
    pub splat_mv: splat_mv_fn,
}

use crate::include::common::intops::imax;
use crate::include::common::intops::imin;
use crate::include::common::intops::iclip;
use crate::include::common::intops::apply_sign;
use crate::src::env::get_poc_diff;
use crate::src::env::fix_mv_precision;

use crate::src::env::get_gmv_2d;
use crate::src::mem::dav1d_freep_aligned;

use crate::src::mem::dav1d_alloc_aligned;
unsafe extern "C" fn add_spatial_candidate(
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    weight: libc::c_int,
    b: *const refmvs_block,
    r#ref: refmvs_refpair,
    mut gmv: *const mv,
    have_newmv_match: *mut libc::c_int,
    have_refmv_match: *mut libc::c_int,
) {
    if (*b).mv.mv[0].is_invalid() {
        return;
    }
    if r#ref.r#ref[1 as libc::c_int as usize] as libc::c_int == -(1 as libc::c_int) {
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < 2 as libc::c_int {
            if (*b).r#ref.r#ref[n as usize] as libc::c_int
                == r#ref.r#ref[0 as libc::c_int as usize] as libc::c_int
            {
                let cand_mv: mv = if (*b).mf as libc::c_int & 1 as libc::c_int != 0
                    && (*gmv.offset(0)) != mv::INVALID {
                    *gmv.offset(0)
                } else {
                    (*b).mv.mv[n as usize]
                };
                *have_refmv_match = 1 as libc::c_int;
                *have_newmv_match |= (*b).mf as libc::c_int >> 1 as libc::c_int;
                let last: libc::c_int = *cnt;
                let mut m: libc::c_int = 0 as libc::c_int;
                while m < last {
                    if (*mvstack.offset(m as isize)).mv.mv[0 as libc::c_int as usize] == cand_mv {
                        (*mvstack.offset(m as isize)).weight += weight;
                        return;
                    }
                    m += 1;
                }
                if last < 8 as libc::c_int {
                    (*mvstack.offset(last as isize))
                        .mv
                        .mv[0 as libc::c_int as usize] = cand_mv;
                    (*mvstack.offset(last as isize)).weight = weight;
                    *cnt = last + 1 as libc::c_int;
                }
                return;
            }
            n += 1;
        }
    } else if (*b).r#ref == r#ref {
        let cand_mv_0: refmvs_mvpair = refmvs_mvpair {
            mv: [
                if (*b).mf as libc::c_int & 1 as libc::c_int != 0
                    && (*gmv.offset(0)) != mv::INVALID {
                    *gmv.offset(0)
                } else {
                    (*b).mv.mv[0]
                },
                if (*b).mf as libc::c_int & 1 as libc::c_int != 0
                    && (*gmv.offset(1)) != mv::INVALID {
                    *gmv.offset(1)
                } else {
                    (*b).mv.mv[1]
                },
            ],
        };
        *have_refmv_match = 1 as libc::c_int;
        *have_newmv_match |= (*b).mf as libc::c_int >> 1 as libc::c_int;
        let last_0: libc::c_int = *cnt;
        let mut n_0: libc::c_int = 0 as libc::c_int;
        while n_0 < last_0 {
            if (*mvstack.offset(n_0 as isize)).mv == cand_mv_0 {
                (*mvstack.offset(n_0 as isize)).weight += weight;
                return;
            }
            n_0 += 1;
        }
        if last_0 < 8 as libc::c_int {
            (*mvstack.offset(last_0 as isize)).mv = cand_mv_0;
            (*mvstack.offset(last_0 as isize)).weight = weight;
            *cnt = last_0 + 1 as libc::c_int;
        }
    }
}
unsafe extern "C" fn scan_row(
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    r#ref: refmvs_refpair,
    mut gmv: *const mv,
    mut b: *const refmvs_block,
    bw4: libc::c_int,
    w4: libc::c_int,
    max_rows: libc::c_int,
    step: libc::c_int,
    have_newmv_match: *mut libc::c_int,
    have_refmv_match: *mut libc::c_int,
) -> libc::c_int {
    let mut cand_b: *const refmvs_block = b;
    let first_cand_bs: BlockSize = (*cand_b).bs as BlockSize;
    let first_cand_b_dim: *const uint8_t = (dav1d_block_dimensions[first_cand_bs
        as usize])
        .as_ptr();
    let mut cand_bw4: libc::c_int = *first_cand_b_dim.offset(0 as libc::c_int as isize)
        as libc::c_int;
    let mut len: libc::c_int = imax(step, imin(bw4, cand_bw4));
    if bw4 <= cand_bw4 {
        let weight: libc::c_int = if bw4 == 1 as libc::c_int {
            2 as libc::c_int
        } else {
            imax(
                2 as libc::c_int,
                imin(
                    2 as libc::c_int * max_rows,
                    *first_cand_b_dim.offset(1 as libc::c_int as isize) as libc::c_int,
                ),
            )
        };
        add_spatial_candidate(
            mvstack,
            cnt,
            len * weight,
            cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        return weight >> 1 as libc::c_int;
    }
    let mut x: libc::c_int = 0 as libc::c_int;
    loop {
        add_spatial_candidate(
            mvstack,
            cnt,
            len * 2 as libc::c_int,
            cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        x += len;
        if x >= w4 {
            return 1 as libc::c_int;
        }
        cand_b = &*b.offset(x as isize) as *const refmvs_block;
        cand_bw4 = dav1d_block_dimensions[(*cand_b).bs
            as usize][0 as libc::c_int as usize] as libc::c_int;
        if !(cand_bw4 < bw4) {
            unreachable!();
        }
        len = imax(step, cand_bw4);
    };
}
unsafe extern "C" fn scan_col(
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    r#ref: refmvs_refpair,
    mut gmv: *const mv,
    mut b: *const *mut refmvs_block,
    bh4: libc::c_int,
    h4: libc::c_int,
    bx4: libc::c_int,
    max_cols: libc::c_int,
    step: libc::c_int,
    have_newmv_match: *mut libc::c_int,
    have_refmv_match: *mut libc::c_int,
) -> libc::c_int {
    let mut cand_b: *const refmvs_block = &mut *(*b.offset(0 as libc::c_int as isize))
        .offset(bx4 as isize) as *mut refmvs_block;
    let first_cand_bs: BlockSize = (*cand_b).bs as BlockSize;
    let first_cand_b_dim: *const uint8_t = (dav1d_block_dimensions[first_cand_bs
        as usize])
        .as_ptr();
    let mut cand_bh4: libc::c_int = *first_cand_b_dim.offset(1 as libc::c_int as isize)
        as libc::c_int;
    let mut len: libc::c_int = imax(step, imin(bh4, cand_bh4));
    if bh4 <= cand_bh4 {
        let weight: libc::c_int = if bh4 == 1 as libc::c_int {
            2 as libc::c_int
        } else {
            imax(
                2 as libc::c_int,
                imin(
                    2 as libc::c_int * max_cols,
                    *first_cand_b_dim.offset(0 as libc::c_int as isize) as libc::c_int,
                ),
            )
        };
        add_spatial_candidate(
            mvstack,
            cnt,
            len * weight,
            cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        return weight >> 1 as libc::c_int;
    }
    let mut y: libc::c_int = 0 as libc::c_int;
    loop {
        add_spatial_candidate(
            mvstack,
            cnt,
            len * 2 as libc::c_int,
            cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        y += len;
        if y >= h4 {
            return 1 as libc::c_int;
        }
        cand_b = &mut *(*b.offset(y as isize)).offset(bx4 as isize) as *mut refmvs_block;
        cand_bh4 = dav1d_block_dimensions[(*cand_b).bs
            as usize][1 as libc::c_int as usize] as libc::c_int;
        if !(cand_bh4 < bh4) {
            unreachable!();
        }
        len = imax(step, cand_bh4);
    };
}
#[inline]
unsafe extern "C" fn mv_projection(mv: mv, num: libc::c_int, den: libc::c_int) -> mv {
    static mut div_mult: [uint16_t; 32] = [
        0 as libc::c_int as uint16_t,
        16384 as libc::c_int as uint16_t,
        8192 as libc::c_int as uint16_t,
        5461 as libc::c_int as uint16_t,
        4096 as libc::c_int as uint16_t,
        3276 as libc::c_int as uint16_t,
        2730 as libc::c_int as uint16_t,
        2340 as libc::c_int as uint16_t,
        2048 as libc::c_int as uint16_t,
        1820 as libc::c_int as uint16_t,
        1638 as libc::c_int as uint16_t,
        1489 as libc::c_int as uint16_t,
        1365 as libc::c_int as uint16_t,
        1260 as libc::c_int as uint16_t,
        1170 as libc::c_int as uint16_t,
        1092 as libc::c_int as uint16_t,
        1024 as libc::c_int as uint16_t,
        963 as libc::c_int as uint16_t,
        910 as libc::c_int as uint16_t,
        862 as libc::c_int as uint16_t,
        819 as libc::c_int as uint16_t,
        780 as libc::c_int as uint16_t,
        744 as libc::c_int as uint16_t,
        712 as libc::c_int as uint16_t,
        682 as libc::c_int as uint16_t,
        655 as libc::c_int as uint16_t,
        630 as libc::c_int as uint16_t,
        606 as libc::c_int as uint16_t,
        585 as libc::c_int as uint16_t,
        564 as libc::c_int as uint16_t,
        546 as libc::c_int as uint16_t,
        528 as libc::c_int as uint16_t,
    ];
    if !(den > 0 as libc::c_int && den < 32 as libc::c_int) {
        unreachable!();
    }
    if !(num > -(32 as libc::c_int) && num < 32 as libc::c_int) {
        unreachable!();
    }
    let frac: libc::c_int = num * div_mult[den as usize] as libc::c_int;
    let y: libc::c_int = mv.y as libc::c_int * frac;
    let x: libc::c_int = mv.x as libc::c_int * frac;
    return mv {
        y: iclip(y + 8192 + (y >> 31) >> 14, -0x3fff, 0x3fff) as i16,
        x: iclip(x + 8192 + (x >> 31) >> 14, -0x3fff, 0x3fff) as i16,
    };
}
unsafe extern "C" fn add_temporal_candidate(
    rf: *const refmvs_frame,
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    rb: *const refmvs_temporal_block,
    r#ref: refmvs_refpair,
    globalmv_ctx: *mut libc::c_int,
    mut gmv: *const mv,
) {
    if (*rb).mv == mv::INVALID {
        return;
    }
    let mut mv: mv = mv_projection(
        (*rb).mv,
        (*rf)
            .pocdiff[(r#ref.r#ref[0 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int) as usize] as libc::c_int,
        (*rb).r#ref as libc::c_int,
    );
    fix_mv_precision((*rf).frm_hdr, &mut mv);
    let last: libc::c_int = *cnt;
    if r#ref.r#ref[1 as libc::c_int as usize] as libc::c_int == -(1 as libc::c_int) {
        if !globalmv_ctx.is_null() {
            *globalmv_ctx = ((mv.x as libc::c_int - (*gmv.offset(0)).x as libc::c_int).abs()
                | (mv.y as libc::c_int - (*gmv.offset(0)).y as libc::c_int).abs() >= 16 as libc::c_int) as libc::c_int;
        }
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < last {
            if (*mvstack.offset(n as isize)).mv.mv[0] == mv {
                (*mvstack.offset(n as isize)).weight += 2 as libc::c_int;
                return;
            }
            n += 1;
        }
        if last < 8 as libc::c_int {
            (*mvstack.offset(last as isize)).mv.mv[0] = mv;
            (*mvstack.offset(last as isize)).weight = 2 as libc::c_int;
            *cnt = last + 1 as libc::c_int;
        }
    } else {
        let mut mvp: refmvs_mvpair = refmvs_mvpair {
            mv: [
                mv,
                mv_projection(
                    (*rb).mv,
                    (*rf)
                        .pocdiff[(r#ref.r#ref[1 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int) as usize] as libc::c_int,
                    (*rb).r#ref as libc::c_int,
                ),
            ],
        };
        fix_mv_precision(
            (*rf).frm_hdr,
            &mut *(mvp.mv).as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        let mut n_0: libc::c_int = 0 as libc::c_int;
        while n_0 < last {
            if (*mvstack.offset(n_0 as isize)).mv == mvp {
                (*mvstack.offset(n_0 as isize)).weight += 2 as libc::c_int;
                return;
            }
            n_0 += 1;
        }
        if last < 8 as libc::c_int {
            (*mvstack.offset(last as isize)).mv = mvp;
            (*mvstack.offset(last as isize)).weight = 2 as libc::c_int;
            *cnt = last + 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn add_compound_extended_candidate(
    same: *mut refmvs_candidate,
    same_count: *mut libc::c_int,
    cand_b: *const refmvs_block,
    sign0: libc::c_int,
    sign1: libc::c_int,
    r#ref: refmvs_refpair,
    sign_bias: *const uint8_t,
) {
    let diff: *mut refmvs_candidate = &mut *same.offset(2 as libc::c_int as isize)
        as *mut refmvs_candidate;
    let diff_count: *mut libc::c_int = &mut *same_count.offset(2 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        let cand_ref: libc::c_int = (*cand_b).r#ref.r#ref[n as usize] as libc::c_int;
        if cand_ref <= 0 as libc::c_int {
            break;
        }
        let mut cand_mv: mv = (*cand_b).mv.mv[n as usize];
        if cand_ref == r#ref.r#ref[0 as libc::c_int as usize] as libc::c_int {
            if *same_count.offset(0 as libc::c_int as isize) < 2 as libc::c_int {
                let ref mut fresh0 = *same_count.offset(0 as libc::c_int as isize);
                let fresh1 = *fresh0;
                *fresh0 = *fresh0 + 1;
                (*same.offset(fresh1 as isize))
                    .mv
                    .mv[0 as libc::c_int as usize] = cand_mv;
            }
            if *diff_count.offset(1 as libc::c_int as isize) < 2 as libc::c_int {
                if sign1
                    ^ *sign_bias.offset((cand_ref - 1 as libc::c_int) as isize)
                        as libc::c_int != 0
                {
                    cand_mv
                        .y = -(cand_mv.y as libc::c_int) as int16_t;
                    cand_mv
                        .x = -(cand_mv.x as libc::c_int) as int16_t;
                }
                let ref mut fresh2 = *diff_count.offset(1 as libc::c_int as isize);
                let fresh3 = *fresh2;
                *fresh2 = *fresh2 + 1;
                (*diff.offset(fresh3 as isize))
                    .mv
                    .mv[1 as libc::c_int as usize] = cand_mv;
            }
        } else if cand_ref == r#ref.r#ref[1 as libc::c_int as usize] as libc::c_int {
            if *same_count.offset(1 as libc::c_int as isize) < 2 as libc::c_int {
                let ref mut fresh4 = *same_count.offset(1 as libc::c_int as isize);
                let fresh5 = *fresh4;
                *fresh4 = *fresh4 + 1;
                (*same.offset(fresh5 as isize))
                    .mv
                    .mv[1 as libc::c_int as usize] = cand_mv;
            }
            if *diff_count.offset(0 as libc::c_int as isize) < 2 as libc::c_int {
                if sign0
                    ^ *sign_bias.offset((cand_ref - 1 as libc::c_int) as isize)
                        as libc::c_int != 0
                {
                    cand_mv.y = -cand_mv.y;
                    cand_mv.x = -cand_mv.x;
                }
                let ref mut fresh6 = *diff_count.offset(0 as libc::c_int as isize);
                let fresh7 = *fresh6;
                *fresh6 = *fresh6 + 1;
                (*diff.offset(fresh7 as isize))
                    .mv
                    .mv[0 as libc::c_int as usize] = cand_mv;
            }
        } else {
            let mut i_cand_mv = mv {
                y: -cand_mv.y,
                x: -cand_mv.x,
            };
            if *diff_count.offset(0 as libc::c_int as isize) < 2 as libc::c_int {
                let ref mut fresh8 = *diff_count.offset(0 as libc::c_int as isize);
                let fresh9 = *fresh8;
                *fresh8 = *fresh8 + 1;
                (*diff.offset(fresh9 as isize))
                    .mv
                    .mv[0 as libc::c_int
                    as usize] = if sign0
                    ^ *sign_bias.offset((cand_ref - 1 as libc::c_int) as isize)
                        as libc::c_int != 0
                {
                    i_cand_mv
                } else {
                    cand_mv
                };
            }
            if *diff_count.offset(1 as libc::c_int as isize) < 2 as libc::c_int {
                let ref mut fresh10 = *diff_count.offset(1 as libc::c_int as isize);
                let fresh11 = *fresh10;
                *fresh10 = *fresh10 + 1;
                (*diff.offset(fresh11 as isize))
                    .mv
                    .mv[1 as libc::c_int
                    as usize] = if sign1
                    ^ *sign_bias.offset((cand_ref - 1 as libc::c_int) as isize)
                        as libc::c_int != 0
                {
                    i_cand_mv
                } else {
                    cand_mv
                };
            }
        }
        n += 1;
    }
}
unsafe extern "C" fn add_single_extended_candidate(
    mut mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    cand_b: *const refmvs_block,
    sign: libc::c_int,
    sign_bias: *const uint8_t,
) {
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        let cand_ref: libc::c_int = (*cand_b).r#ref.r#ref[n as usize] as libc::c_int;
        if cand_ref <= 0 as libc::c_int {
            break;
        }
        let mut cand_mv: mv = (*cand_b).mv.mv[n as usize];
        if sign
            ^ *sign_bias.offset((cand_ref - 1 as libc::c_int) as isize) as libc::c_int
            != 0
        {
            cand_mv
                .y = -(cand_mv.y as libc::c_int) as int16_t;
            cand_mv
                .x = -(cand_mv.x as libc::c_int) as int16_t;
        }
        let mut m: libc::c_int = 0;
        let last: libc::c_int = *cnt;
        m = 0 as libc::c_int;
        while m < last {
            if cand_mv == (*mvstack.offset(m as isize)).mv.mv[0] {
                break;
            }
            m += 1;
        }
        if m == last {
            (*mvstack.offset(m as isize)).mv.mv[0] = cand_mv;
            (*mvstack.offset(m as isize)).weight = 2 as libc::c_int;
            *cnt = last + 1 as libc::c_int;
        }
        n += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_find(
    rt: *const refmvs_tile,
    mut mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    ctx: *mut libc::c_int,
    r#ref: refmvs_refpair,
    bs: BlockSize,
    edge_flags: EdgeFlags,
    by4: libc::c_int,
    bx4: libc::c_int,
) {
    let rf = &*(*rt).rf;
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bw4 = b_dim[0] as libc::c_int;
    let w4 = imin(imin(bw4, 16), (*rt).tile_col.end - bx4);
    let bh4 = b_dim[1] as libc::c_int;
    let h4 = imin(imin(bh4, 16), (*rt).tile_row.end - by4);
    let mut gmv = [mv::ZERO; 2];
    let mut tgmv = [mv::ZERO; 2];
    *cnt = 0;
    assert!(r#ref.r#ref[0] >= 0 && r#ref.r#ref[0] <= 8
        && r#ref.r#ref[1] >= -1 && r#ref.r#ref[1] <= 8);
    if r#ref.r#ref[0] > 0 {
        tgmv[0] = get_gmv_2d(
            &(*rf.frm_hdr).gmv[r#ref.r#ref[0] as usize - 1],
            bx4,
            by4,
            bw4,
            bh4,
            rf.frm_hdr,
        );
        gmv[0] = if (*rf.frm_hdr).gmv[r#ref.r#ref[0] as usize - 1].type_0 > DAV1D_WM_TYPE_TRANSLATION {
            tgmv[0]
        } else {
            mv::INVALID
        };
    } else {
        tgmv[0] = mv::ZERO;
        gmv[0] = mv::INVALID;
    }
    if r#ref.r#ref[1] > 0 {
        tgmv[1] = get_gmv_2d(
            &(*rf.frm_hdr).gmv[r#ref.r#ref[1] as usize - 1],
            bx4,
            by4,
            bw4,
            bh4,
            rf.frm_hdr,
        );
        gmv[1] = if (*rf.frm_hdr).gmv[r#ref.r#ref[1] as usize - 1].type_0 > DAV1D_WM_TYPE_TRANSLATION {
            tgmv[1]
        } else {
            mv::INVALID
        };
    }
    let mut have_newmv = 0;
    let mut have_col_mvs = 0;
    let mut have_row_mvs = 0;
    let mut max_rows = 0;
    let mut n_rows = !0;
    let mut b_top = std::ptr::null();
    if by4 > (*rt).tile_row.start {
        max_rows = imin(by4 - (*rt).tile_row.start + 1 >> 1, 2 + (bh4 > 1) as libc::c_int) as libc::c_uint;
        b_top = &mut *(*rt).r[(by4 as usize & 31) + 5 - 1].offset(bx4 as isize);
        n_rows = scan_row(
            mvstack,
            cnt,
            r#ref,
            gmv.as_mut_ptr() as *const mv,
            b_top,
            bw4,
            w4,
            max_rows as libc::c_int,
            if bw4 >= 16 { 4 } else { 1 },
            &mut have_newmv,
            &mut have_row_mvs,
        ) as libc::c_uint;
    }
    let mut max_cols = 0;
    let mut n_cols = !0;
    let mut b_left = std::ptr::null();
    if bx4 > (*rt).tile_col.start {
        max_cols = imin(bx4 - (*rt).tile_col.start + 1 >> 1, 2 + (bw4 > 1) as libc::c_int) as libc::c_uint;
        b_left = &(*rt).r[(by4 as usize & 31) + 5];
        n_cols = scan_col(
            mvstack,
            cnt,
            r#ref,
            gmv.as_mut_ptr() as *const mv,
            b_left,
            bh4,
            h4,
            bx4 - 1,
            max_cols as libc::c_int,
            if bh4 >= 16 { 4 } else { 1 },
            &mut have_newmv,
            &mut have_col_mvs,
        ) as libc::c_uint;
    }
    if n_rows != !0 && edge_flags & EDGE_I444_TOP_HAS_RIGHT != 0
        && imax(bw4, bh4) <= 16 && bw4 + bx4 < (*rt).tile_col.end {
        add_spatial_candidate(
            mvstack,
            cnt,
            4,
            &*b_top.offset(bw4 as isize),
            r#ref,
            gmv.as_mut_ptr() as *const mv,
            &mut have_newmv,
            &mut have_row_mvs,
        );
    }
    let nearest_match = have_col_mvs + have_row_mvs;
    let nearest_cnt = *cnt;
    let mut n = 0;
    while n < nearest_cnt {
        (*mvstack.offset(n as isize)).weight += 640;
        n += 1;
    }
    let mut globalmv_ctx = (*rf.frm_hdr).use_ref_frame_mvs;
    if rf.use_ref_frame_mvs != 0 {
        let stride: ptrdiff_t = rf.rp_stride;
        let by8 = by4 >> 1;
        let bx8 = bx4 >> 1;
        let rbi: *const refmvs_temporal_block = &mut *((*rt).rp_proj)
            .offset((by8 & 15) as isize * stride + bx8 as isize) as *mut refmvs_temporal_block;
        let mut rb = rbi;
        let step_h = if bw4 >= 16 { 2 } else { 1 };
        let step_v = if bh4 >= 16 { 2 } else { 1 };
        let w8 = imin(w4 + 1 >> 1, 8);
        let h8 = imin(h4 + 1 >> 1, 8);
        let mut y = 0;
        while y < h8 {
            let mut x = 0;
            while x < w8 {
                add_temporal_candidate(
                    rf,
                    mvstack,
                    cnt,
                    &*rb.offset(x as isize),
                    r#ref,
                    if x | y == 0 { &mut globalmv_ctx } else { 0 as *mut libc::c_int },
                    tgmv.as_mut_ptr() as *const mv,
                );
                x += step_h;
            }
            rb = rb.offset(stride * step_v as isize);
            y += step_v;
        }
        if imin(bw4, bh4) >= 2 && imax(bw4, bh4) < 16 {
            let bh8 = bh4 >> 1;
            let bw8 = bw4 >> 1;
            rb = &*rbi.offset(bh8 as isize * stride) as *const refmvs_temporal_block;
            let has_bottom = (by8 + bh8 < imin((*rt).tile_row.end >> 1, (by8 & !7) + 8)) as libc::c_int;
            if has_bottom != 0 && bx8 - 1 >= imax((*rt).tile_col.start >> 1, bx8 & !7) {
                add_temporal_candidate(
                    rf,
                    mvstack,
                    cnt,
                    &*rb.offset(-1),
                    r#ref,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                );
            }
            if bx8 + bw8 < imin((*rt).tile_col.end >> 1, (bx8 & !7) + 8) {
                if has_bottom != 0 {
                    add_temporal_candidate(
                        rf,
                        mvstack,
                        cnt,
                        &*rb.offset(bw8 as isize),
                        r#ref,
                        std::ptr::null_mut(),
                        std::ptr::null(),
                    );
                }
                if (by8 + bh8 - 1) < imin((*rt).tile_row.end >> 1, (by8 & !7) + 8) {
                    add_temporal_candidate(
                        rf,
                        mvstack,
                        cnt,
                        &*rb.offset(bw8 as isize - stride),
                        r#ref,
                        std::ptr::null_mut(),
                        std::ptr::null(),
                    );
                }
            }
        }
    }
    assert!(*cnt <= 8);
    let mut have_dummy_newmv_match = 0;
    if n_rows | n_cols != !0 {
        add_spatial_candidate(
            mvstack,
            cnt,
            4,
            &*b_top.offset(-1),
            r#ref,
            gmv.as_mut_ptr() as *const mv,
            &mut have_dummy_newmv_match,
            &mut have_row_mvs,
        );
    }
    let mut n_0 = 2;
    while n_0 <= 3 {
        if n_0 as libc::c_uint > n_rows && n_0 as libc::c_uint <= max_rows {
            n_rows = n_rows
                .wrapping_add(
                    scan_row(
                        mvstack,
                        cnt,
                        r#ref,
                        gmv.as_mut_ptr() as *const mv,
                        &mut *((*rt).r[(((by4 & 31) - 2 * n_0 + 1 | 1) + 5) as usize]).offset(bx4 as isize | 1),
                        bw4,
                        w4,
                        (1 as libc::c_uint).wrapping_add(max_rows).wrapping_sub(n_0 as libc::c_uint) as libc::c_int,
                        if bw4 >= 16 { 4 } else { 2 },
                        &mut have_dummy_newmv_match,
                        &mut have_row_mvs,
                    ) as libc::c_uint,
                );
        }
        if n_0 as libc::c_uint > n_cols && n_0 as libc::c_uint <= max_cols {
            n_cols = n_cols
                .wrapping_add(
                    scan_col(
                        mvstack,
                        cnt,
                        r#ref,
                        gmv.as_mut_ptr() as *const mv,
                        &(*rt).r[(by4 as usize & 31 | 1) + 5],
                        bh4,
                        h4,
                        bx4 - n_0 * 2 + 1 | 1,
                        (1 as libc::c_uint).wrapping_add(max_cols).wrapping_sub(n_0 as libc::c_uint) as libc::c_int,
                        if bh4 >= 16 { 4 } else { 2 },
                        &mut have_dummy_newmv_match,
                        &mut have_col_mvs,
                    ) as libc::c_uint,
                );
        }
        n_0 += 1;
    }
    assert!(*cnt <= 8);
    let ref_match_count = have_col_mvs + have_row_mvs;
    let mut refmv_ctx = 0;
    let mut newmv_ctx = 0;
    match nearest_match {
        0 => {
            refmv_ctx = imin(2, ref_match_count);
            newmv_ctx = (ref_match_count > 0) as libc::c_int;
        }
        1 => {
            refmv_ctx = imin(ref_match_count * 3, 4);
            newmv_ctx = 3 - have_newmv;
        }
        2 => {
            refmv_ctx = 5;
            newmv_ctx = 5 - have_newmv;
        }
        _ => {}
    }
    let mut len = nearest_cnt;
    while len != 0 {
        let mut last = 0;
        let mut n_1 = 1;
        while n_1 < len {
            if (*mvstack.offset(n_1 as isize - 1)).weight < (*mvstack.offset(n_1 as isize)).weight {
                let mut tmp = *mvstack.offset((n_1 - 1) as isize);
                *mvstack.offset(n_1 as isize - 1) = *mvstack.offset(n_1 as isize);
                *mvstack.offset(n_1 as isize) = tmp;
                last = n_1;
            }
            n_1 += 1;
        }
        len = last;
    }
    len = *cnt;
    while len > nearest_cnt {
        let mut last_0 = nearest_cnt;
        let mut n_2 = nearest_cnt + 1;
        while n_2 < len {
            if (*mvstack.offset(n_2 as isize - 1)).weight < (*mvstack.offset(n_2 as isize)).weight {
                let mut tmp_0 = *mvstack.offset(n_2 as isize - 1);
                *mvstack.offset(n_2 as isize - 1) = *mvstack.offset(n_2 as isize);
                *mvstack.offset(n_2 as isize) = tmp_0;
                last_0 = n_2;
            }
            n_2 += 1;
        }
        len = last_0;
    }
    if r#ref.r#ref[1] > 0 {
        if *cnt < 2 {
            let sign0 = rf.sign_bias[r#ref.r#ref[0] as usize - 1] as libc::c_int;
            let sign1 = rf.sign_bias[r#ref.r#ref[1] as usize - 1] as libc::c_int;
            let sz4 = imin(w4, h4);
            let same = &mut *mvstack.offset(*cnt as isize) as *mut refmvs_candidate;
            let mut same_count = [0, 0, 0, 0];
            if n_rows != !0 {
                let mut x_0 = 0;
                while x_0 < sz4 {
                    let cand_b = &*b_top.offset(x_0 as isize) as *const refmvs_block;
                    add_compound_extended_candidate(
                        same,
                        same_count.as_mut_ptr(),
                        cand_b,
                        sign0,
                        sign1,
                        r#ref,
                        rf.sign_bias.as_ptr(),
                    );
                    x_0 += dav1d_block_dimensions[(*cand_b).bs as usize][0] as libc::c_int;
                }
            }
            if n_cols != !0 {
                let mut y_0 = 0;
                while y_0 < sz4 {
                    let cand_b_0: *const refmvs_block = &mut *(*b_left.offset(y_0 as isize))
                        .offset(bx4 as isize - 1) as *mut refmvs_block;
                    add_compound_extended_candidate(
                        same,
                        same_count.as_mut_ptr(),
                        cand_b_0,
                        sign0,
                        sign1,
                        r#ref,
                        rf.sign_bias.as_ptr(),
                    );
                    y_0 += dav1d_block_dimensions[(*cand_b_0).bs as usize][1] as libc::c_int;
                }
            }
            let diff = &mut *same
                .offset(2) as *mut refmvs_candidate;
            let diff_count: *const libc::c_int = &mut *same_count.as_mut_ptr().offset(2) as *mut libc::c_int;
            let mut current_block_118: u64;
            let mut n_3 = 0;
            while n_3 < 2 {
                let mut m = same_count[n_3 as usize];
                if !(m >= 2) {
                    let l = *diff_count.offset(n_3 as isize);
                    if l != 0 {
                        (*same.offset(m as isize)).mv.mv[n_3 as usize] = (*diff.offset(0)).mv.mv[n_3 as usize];
                        m += 1;
                        if m == 2 {
                            current_block_118 = 13740693533991687037;
                        } else if l == 2 {
                            (*same.offset(1)).mv.mv[n_3 as usize] = (*diff.offset(1)).mv.mv[n_3 as usize];
                            current_block_118 = 13740693533991687037;
                        } else {
                            current_block_118 = 9430418855388998878;
                        }
                    } else {
                        current_block_118 = 9430418855388998878;
                    }
                    match current_block_118 {
                        13740693533991687037 => {}
                        _ => {
                            loop {
                                (*same.offset(m as isize)).mv.mv[n_3 as usize] = tgmv[n_3 as usize];
                                m += 1;
                                if !(m < 2) {
                                    break;
                                }
                            }
                        }
                    }
                }
                n_3 += 1;
            }
            let mut n_4 = *cnt;
            if n_4 == 1 && (*mvstack.offset(0)).mv == (*same.offset(0)).mv {
                (*mvstack.offset(1)).mv = (*mvstack.offset(2)).mv;
            }
            loop {
                (*mvstack.offset(n_4 as isize)).weight = 2;
                n_4 += 1;
                if !(n_4 < 2) {
                    break;
                }
            }
            *cnt = 2;
        }
        let left = -(bx4 + bw4 + 4) * 4 * 8;
        let right = (rf.iw4 - bx4 + 4) * 4 * 8;
        let top = -(by4 + bh4 + 4) * 4 * 8;
        let bottom = (rf.ih4 - by4 + 4) * 4 * 8;
        let n_refmvs = *cnt;
        let mut n_5 = 0;
        loop {
            (*mvstack.offset(n_5 as isize)).mv.mv[0].x = 
                iclip((*mvstack.offset(n_5 as isize)).mv.mv[0].x as libc::c_int, left, right) as i16;
            (*mvstack.offset(n_5 as isize)).mv.mv[0].y = 
                iclip((*mvstack.offset(n_5 as isize)).mv.mv[0].y as libc::c_int, top, bottom) as i16;
            (*mvstack.offset(n_5 as isize)).mv.mv[1].x = 
                iclip((*mvstack.offset(n_5 as isize)).mv.mv[1].x as libc::c_int, left, right) as i16;
            (*mvstack.offset(n_5 as isize)).mv.mv[1].y = 
                iclip((*mvstack.offset(n_5 as isize)).mv.mv[1].y as libc::c_int, top, bottom) as i16;
            n_5 += 1;
            if !(n_5 < n_refmvs) {
                break;
            }
        }
        match refmv_ctx >> 1 {
            0 => {
                *ctx = imin(newmv_ctx, 1);
            }
            1 => {
                *ctx = 1 + imin(newmv_ctx, 3);
            }
            2 => {
                *ctx = iclip(3 + newmv_ctx, 4, 7);
            }
            _ => {}
        }
        return;
    } else {
        if *cnt < 2 && r#ref.r#ref[0] > 0 {
            let sign = rf.sign_bias[r#ref.r#ref[0] as usize - 1] as libc::c_int;
            let sz4_0 = imin(w4, h4);
            if n_rows != !0 {
                let mut x_1 = 0;
                while x_1 < sz4_0 && *cnt < 2 {
                    let cand_b_1 = &*b_top.offset(x_1 as isize) as *const refmvs_block;
                    add_single_extended_candidate(
                        mvstack,
                        cnt,
                        cand_b_1,
                        sign,
                        rf.sign_bias.as_ptr(),
                    );
                    x_1 += dav1d_block_dimensions[(*cand_b_1).bs as usize][0] as libc::c_int;
                }
            }
            if n_cols != !0 {
                let mut y_1 = 0;
                while y_1 < sz4_0 && *cnt < 2 {
                    let cand_b_2: *const refmvs_block = &mut *(*b_left.offset(y_1 as isize))
                        .offset(bx4 as isize - 1) as *mut refmvs_block;
                    add_single_extended_candidate(
                        mvstack,
                        cnt,
                        cand_b_2,
                        sign,
                        rf.sign_bias.as_ptr(),
                    );
                    y_1 += dav1d_block_dimensions[(*cand_b_2).bs as usize][1] as libc::c_int;
                }
            }
        }
    }
    assert!(*cnt <= 8);
    let mut n_refmvs_0 = *cnt;
    if n_refmvs_0 != 0 {
        let left_0 = -(bx4 + bw4 + 4) * 4 * 8;
        let right_0 = (rf.iw4 - bx4 + 4) * 4 * 8;
        let top_0 = -(by4 + bh4 + 4) * 4 * 8;
        let bottom_0 = (rf.ih4 - by4 + 4) * 4 * 8;
        let mut n_6 = 0;
        loop {
            (*mvstack.offset(n_6 as isize)).mv.mv[0].x = 
                iclip((*mvstack.offset(n_6 as isize)).mv.mv[0].x as libc::c_int, left_0, right_0) as i16;
            (*mvstack.offset(n_6 as isize)).mv.mv[0].y = 
                iclip((*mvstack.offset(n_6 as isize)).mv.mv[0].y as libc::c_int, top_0, bottom_0) as i16;
            n_6 += 1;
            if !(n_6 < n_refmvs_0) {
                break;
            }
        }
    }
    let mut n_7 = *cnt;
    while n_7 < 2 {
        (*mvstack.offset(n_7 as isize)).mv.mv[0] = tgmv[0];
        n_7 += 1;
    }
    *ctx = refmv_ctx << 4 | globalmv_ctx << 3 | newmv_ctx;
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_tile_sbrow_init(
    rt: *mut refmvs_tile,
    rf: *const refmvs_frame,
    tile_col_start4: libc::c_int,
    tile_col_end4: libc::c_int,
    tile_row_start4: libc::c_int,
    tile_row_end4: libc::c_int,
    sby: libc::c_int,
    mut tile_row_idx: libc::c_int,
    pass: libc::c_int,
) {
    if (*rf).n_tile_threads == 1 as libc::c_int {
        tile_row_idx = 0 as libc::c_int;
    }
    (*rt)
        .rp_proj = &mut *((*rf).rp_proj)
        .offset(16 * (*rf).rp_stride * tile_row_idx as isize,
        ) as *mut refmvs_temporal_block;
    let uses_2pass: libc::c_int = ((*rf).n_tile_threads > 1 as libc::c_int
        && (*rf).n_frame_threads > 1 as libc::c_int) as libc::c_int;
    let pass_off: ptrdiff_t = if uses_2pass != 0 && pass == 2 as libc::c_int {
        35 * (*rf).r_stride * (*rf).n_tile_rows as isize
    } else {
        0
    };
    let mut r: *mut refmvs_block = &mut *((*rf).r)
        .offset(
            35 * (*rf).r_stride * tile_row_idx as isize + pass_off,
        ) as *mut refmvs_block;
    let sbsz: libc::c_int = (*rf).sbsz;
    let off: libc::c_int = sbsz * sby & 16 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < sbsz {
        (*rt).r[(off + 5 as libc::c_int + i) as usize] = r;
        i += 1;
        r = r.offset((*rf).r_stride as isize);
    }
    (*rt).r[(off + 0 as libc::c_int) as usize] = r;
    r = r.offset((*rf).r_stride as isize);
    (*rt).r[(off + 1 as libc::c_int) as usize] = 0 as *mut refmvs_block;
    (*rt).r[(off + 2 as libc::c_int) as usize] = r;
    r = r.offset((*rf).r_stride as isize);
    (*rt).r[(off + 3 as libc::c_int) as usize] = 0 as *mut refmvs_block;
    (*rt).r[(off + 4 as libc::c_int) as usize] = r;
    if sby & 1 as libc::c_int != 0 {
        let tmp: *mut libc::c_void = (*rt).r[(off + 0 as libc::c_int) as usize]
            as *mut libc::c_void;
        (*rt)
            .r[(off + 0 as libc::c_int)
            as usize] = (*rt).r[(off + sbsz + 0 as libc::c_int) as usize];
        (*rt).r[(off + sbsz + 0 as libc::c_int) as usize] = tmp as *mut refmvs_block;
        let tmp_0: *mut libc::c_void = (*rt).r[(off + 2 as libc::c_int) as usize]
            as *mut libc::c_void;
        (*rt)
            .r[(off + 2 as libc::c_int)
            as usize] = (*rt).r[(off + sbsz + 2 as libc::c_int) as usize];
        (*rt).r[(off + sbsz + 2 as libc::c_int) as usize] = tmp_0 as *mut refmvs_block;
        let tmp_1: *mut libc::c_void = (*rt).r[(off + 4 as libc::c_int) as usize]
            as *mut libc::c_void;
        (*rt)
            .r[(off + 4 as libc::c_int)
            as usize] = (*rt).r[(off + sbsz + 4 as libc::c_int) as usize];
        (*rt).r[(off + sbsz + 4 as libc::c_int) as usize] = tmp_1 as *mut refmvs_block;
    }
    (*rt).rf = rf;
    (*rt).tile_row.start = tile_row_start4;
    (*rt).tile_row.end = imin(tile_row_end4, (*rf).ih4);
    (*rt).tile_col.start = tile_col_start4;
    (*rt).tile_col.end = imin(tile_col_end4, (*rf).iw4);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_load_tmvs(
    rf: *const refmvs_frame,
    mut tile_row_idx: libc::c_int,
    col_start8: libc::c_int,
    col_end8: libc::c_int,
    row_start8: libc::c_int,
    mut row_end8: libc::c_int,
) {
    if (*rf).n_tile_threads == 1 as libc::c_int {
        tile_row_idx = 0 as libc::c_int;
    }
    if !(row_start8 >= 0 as libc::c_int) {
        unreachable!();
    }
    if !((row_end8 - row_start8) as libc::c_uint <= 16 as libc::c_uint) {
        unreachable!();
    }
    row_end8 = imin(row_end8, (*rf).ih8);
    let col_start8i: libc::c_int = imax(col_start8 - 8 as libc::c_int, 0 as libc::c_int);
    let col_end8i: libc::c_int = imin(col_end8 + 8 as libc::c_int, (*rf).iw8);
    let stride: ptrdiff_t = (*rf).rp_stride;
    let mut rp_proj: *mut refmvs_temporal_block = &mut *((*rf).rp_proj)
        .offset(
            16  * stride * tile_row_idx as isize
                + (row_start8 & 15) as isize * stride,
        ) as *mut refmvs_temporal_block;
    let mut y: libc::c_int = row_start8;
    while y < row_end8 {
        let mut x: libc::c_int = col_start8;
        while x < col_end8 {
            (*rp_proj.offset(x as isize)).mv = mv::INVALID;
            x += 1;
        }
        rp_proj = rp_proj.offset(stride as isize);
        y += 1;
    }
    rp_proj = &mut *((*rf).rp_proj)
        .offset(16 * stride * tile_row_idx as isize) as *mut refmvs_temporal_block;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < (*rf).n_mfmvs {
        let ref2cur: libc::c_int = (*rf).mfmv_ref2cur[n as usize];
        if !(ref2cur == -(2147483647 as libc::c_int) - 1 as libc::c_int) {
            let r#ref: libc::c_int = (*rf).mfmv_ref[n as usize] as libc::c_int;
            let ref_sign: libc::c_int = r#ref - 4 as libc::c_int;
            let mut r: *const refmvs_temporal_block = &mut *(*((*rf).rp_ref)
                .offset(r#ref as isize))
                .offset(row_start8 as isize * stride)
                as *mut refmvs_temporal_block;
            let mut y_0: libc::c_int = row_start8;
            while y_0 < row_end8 {
                let y_sb_align: libc::c_int = y_0 & !(7 as libc::c_int);
                let y_proj_start: libc::c_int = imax(y_sb_align, row_start8);
                let y_proj_end: libc::c_int = imin(
                    y_sb_align + 8 as libc::c_int,
                    row_end8,
                );
                let mut x_0: libc::c_int = col_start8i;
                while x_0 < col_end8i {
                    let mut rb: *const refmvs_temporal_block = &*r.offset(x_0 as isize)
                        as *const refmvs_temporal_block;
                    let b_ref: libc::c_int = (*rb).r#ref as libc::c_int;
                    if !(b_ref == 0) {
                        let ref2ref: libc::c_int = (*rf)
                            .mfmv_ref2ref[n
                            as usize][(b_ref - 1 as libc::c_int) as usize];
                        if !(ref2ref == 0) {
                            let b_mv: mv = (*rb).mv;
                            let offset: mv = mv_projection(b_mv, ref2cur, ref2ref);
                            let mut pos_x: libc::c_int = x_0
                                + apply_sign(
                                    (offset.x as libc::c_int).abs()
                                        >> 6 as libc::c_int,
                                    offset.x as libc::c_int ^ ref_sign,
                                );
                            let pos_y: libc::c_int = y_0
                                + apply_sign(
                                    (offset.y as libc::c_int).abs()
                                        >> 6 as libc::c_int,
                                    offset.y as libc::c_int ^ ref_sign,
                                );
                            if pos_y >= y_proj_start && pos_y < y_proj_end {
                                let pos: ptrdiff_t = (pos_y & 15) as isize * stride;
                                loop {
                                    let x_sb_align: libc::c_int = x_0 & !(7 as libc::c_int);
                                    if pos_x >= imax(x_sb_align - 8 as libc::c_int, col_start8)
                                        && pos_x < imin(x_sb_align + 16 as libc::c_int, col_end8)
                                    {
                                        (*rp_proj.offset(pos + pos_x as isize))
                                            .mv = (*rb).mv;
                                        (*rp_proj.offset(pos + pos_x as isize))
                                            .r#ref = ref2ref as int8_t;
                                    }
                                    x_0 += 1;
                                    if x_0 >= col_end8i {
                                        break;
                                    }
                                    rb = rb.offset(1);
                                    if (*rb).r#ref as libc::c_int != b_ref || (*rb).mv != b_mv {
                                        break;
                                    }
                                    pos_x += 1;
                                }
                            } else {
                                loop {
                                    x_0 += 1;
                                    if x_0 >= col_end8i {
                                        break;
                                    }
                                    rb = rb.offset(1);
                                    if (*rb).r#ref as libc::c_int != b_ref || (*rb).mv != b_mv {
                                        break;
                                    }
                                }
                            }
                            x_0 -= 1;
                        }
                    }
                    x_0 += 1;
                }
                r = r.offset(stride as isize);
                y_0 += 1;
            }
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_save_tmvs(
    rt: *const refmvs_tile,
    col_start8: libc::c_int,
    mut col_end8: libc::c_int,
    row_start8: libc::c_int,
    mut row_end8: libc::c_int,
) {
    let rf: *const refmvs_frame = (*rt).rf;
    if !(row_start8 >= 0 as libc::c_int) {
        unreachable!();
    }
    if !((row_end8 - row_start8) as libc::c_uint <= 16 as libc::c_uint) {
        unreachable!();
    }
    row_end8 = imin(row_end8, (*rf).ih8);
    col_end8 = imin(col_end8, (*rf).iw8);
    let stride: ptrdiff_t = (*rf).rp_stride;
    let ref_sign: *const uint8_t = ((*rf).mfmv_sign).as_ptr();
    let mut rp: *mut refmvs_temporal_block = &mut *((*rf).rp)
        .offset(row_start8 as isize * stride)
        as *mut refmvs_temporal_block;
    let mut y: libc::c_int = row_start8;
    while y < row_end8 {
        let b: *const refmvs_block = (*rt)
            .r[(6 as libc::c_int + (y & 15 as libc::c_int) * 2 as libc::c_int) as usize];
        let mut x: libc::c_int = col_start8;
        while x < col_end8 {
            let cand_b: *const refmvs_block = &*b
                .offset((x * 2 as libc::c_int + 1 as libc::c_int) as isize)
                as *const refmvs_block;
            let bw8: libc::c_int = dav1d_block_dimensions[(*cand_b).bs
                as usize][0 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int
                >> 1 as libc::c_int;
            if (*cand_b).r#ref.r#ref[1 as libc::c_int as usize] as libc::c_int
                > 0 as libc::c_int
                && *ref_sign
                    .offset(
                        ((*cand_b).r#ref.r#ref[1 as libc::c_int as usize] as libc::c_int
                            - 1 as libc::c_int) as isize,
                    ) as libc::c_int != 0
                && (*cand_b).mv.mv[1].y.abs() | (*cand_b).mv.mv[1].x.abs() < 4096 {
                let mut n: libc::c_int = 0 as libc::c_int;
                while n < bw8 {
                    *rp
                        .offset(
                            x as isize,
                        ) = {
                        let mut init = refmvs_temporal_block {
                            mv: (*cand_b).mv.mv[1 as libc::c_int as usize],
                            r#ref: (*cand_b).r#ref.r#ref[1 as libc::c_int as usize],
                        };
                        init
                    };
                    n += 1;
                    x += 1;
                }
            } else if (*cand_b).r#ref.r#ref[0 as libc::c_int as usize] as libc::c_int
                > 0 as libc::c_int
                && *ref_sign
                    .offset(
                        ((*cand_b).r#ref.r#ref[0 as libc::c_int as usize] as libc::c_int
                            - 1 as libc::c_int) as isize,
                    ) as libc::c_int != 0
                && (*cand_b).mv.mv[0].y.abs() | (*cand_b).mv.mv[0].x.abs() < 4096 {
                let mut n_0: libc::c_int = 0 as libc::c_int;
                while n_0 < bw8 {
                    *rp
                        .offset(
                            x as isize,
                        ) = {
                        let mut init = refmvs_temporal_block {
                            mv: (*cand_b).mv.mv[0 as libc::c_int as usize],
                            r#ref: (*cand_b).r#ref.r#ref[0 as libc::c_int as usize],
                        };
                        init
                    };
                    n_0 += 1;
                    x += 1;
                }
            } else {
                let mut n_1: libc::c_int = 0 as libc::c_int;
                while n_1 < bw8 {
                    (*rp.offset(x as isize)).r#ref = 0 as libc::c_int as int8_t;
                    n_1 += 1;
                    x += 1;
                }
            }
        }
        rp = rp.offset(stride as isize);
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_init_frame(
    rf: *mut refmvs_frame,
    seq_hdr: *const Dav1dSequenceHeader,
    frm_hdr: *const Dav1dFrameHeader,
    mut ref_poc: *const libc::c_uint,
    rp: *mut refmvs_temporal_block,
    mut ref_ref_poc: *const [libc::c_uint; 7],
    mut rp_ref: *const *mut refmvs_temporal_block,
    n_tile_threads: libc::c_int,
    n_frame_threads: libc::c_int,
) -> libc::c_int {
    (*rf).sbsz = (16 as libc::c_int) << (*seq_hdr).sb128;
    (*rf).frm_hdr = frm_hdr;
    (*rf)
        .iw8 = (*frm_hdr).width[0 as libc::c_int as usize] + 7 as libc::c_int
        >> 3 as libc::c_int;
    (*rf).ih8 = (*frm_hdr).height + 7 as libc::c_int >> 3 as libc::c_int;
    (*rf).iw4 = (*rf).iw8 << 1 as libc::c_int;
    (*rf).ih4 = (*rf).ih8 << 1 as libc::c_int;
    let r_stride: ptrdiff_t = (((*frm_hdr).width[0 as libc::c_int as usize]
        + 127 as libc::c_int & !(127 as libc::c_int)) >> 2 as libc::c_int) as ptrdiff_t;
    let n_tile_rows: libc::c_int = if n_tile_threads > 1 as libc::c_int {
        (*frm_hdr).tiling.rows
    } else {
        1 as libc::c_int
    };
    if r_stride != (*rf).r_stride || n_tile_rows != (*rf).n_tile_rows {
        if !((*rf).r).is_null() {
            dav1d_freep_aligned(
                &mut (*rf).r as *mut *mut refmvs_block as *mut libc::c_void,
            );
        }
        let uses_2pass: libc::c_int = (n_tile_threads > 1 as libc::c_int
            && n_frame_threads > 1 as libc::c_int) as libc::c_int;
        (*rf)
            .r = dav1d_alloc_aligned(
            (::core::mem::size_of::<refmvs_block>())
                .wrapping_mul(35 as size_t)
                .wrapping_mul(r_stride as size_t)
                .wrapping_mul(n_tile_rows as size_t)
                .wrapping_mul((1 + uses_2pass) as size_t),
            64 as libc::c_int as size_t,
        ) as *mut refmvs_block;
        if ((*rf).r).is_null() {
            return -(12 as libc::c_int);
        }
        (*rf).r_stride = r_stride;
    }
    let rp_stride: ptrdiff_t = r_stride >> 1 as libc::c_int;
    if rp_stride != (*rf).rp_stride || n_tile_rows != (*rf).n_tile_rows {
        if !((*rf).rp_proj).is_null() {
            dav1d_freep_aligned(
                &mut (*rf).rp_proj as *mut *mut refmvs_temporal_block
                    as *mut libc::c_void,
            );
        }
        (*rf)
            .rp_proj = dav1d_alloc_aligned(
            (::core::mem::size_of::<refmvs_temporal_block>())
                .wrapping_mul(16 as size_t)
                .wrapping_mul(rp_stride as size_t)
                .wrapping_mul(n_tile_rows as size_t),
            64 as size_t,
        ) as *mut refmvs_temporal_block;
        if ((*rf).rp_proj).is_null() {
            return -(12 as libc::c_int);
        }
        (*rf).rp_stride = rp_stride;
    }
    (*rf).n_tile_rows = n_tile_rows;
    (*rf).n_tile_threads = n_tile_threads;
    (*rf).n_frame_threads = n_frame_threads;
    (*rf).rp = rp;
    (*rf).rp_ref = rp_ref;
    let poc: libc::c_uint = (*frm_hdr).frame_offset as libc::c_uint;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        let poc_diff: libc::c_int = get_poc_diff(
            (*seq_hdr).order_hint_n_bits,
            *ref_poc.offset(i as isize) as libc::c_int,
            poc as libc::c_int,
        );
        (*rf)
            .sign_bias[i
            as usize] = (poc_diff > 0 as libc::c_int) as libc::c_int as uint8_t;
        (*rf)
            .mfmv_sign[i
            as usize] = (poc_diff < 0 as libc::c_int) as libc::c_int as uint8_t;
        (*rf)
            .pocdiff[i
            as usize] = iclip(
            get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                poc as libc::c_int,
                *ref_poc.offset(i as isize) as libc::c_int,
            ),
            -(31 as libc::c_int),
            31 as libc::c_int,
        ) as int8_t;
        i += 1;
    }
    (*rf).n_mfmvs = 0 as libc::c_int;
    if (*frm_hdr).use_ref_frame_mvs != 0 && (*seq_hdr).order_hint_n_bits != 0 {
        let mut total: libc::c_int = 2 as libc::c_int;
        if !(*rp_ref.offset(0 as libc::c_int as isize)).is_null()
            && (*ref_ref_poc
                .offset(0 as libc::c_int as isize))[6 as libc::c_int as usize]
                != *ref_poc.offset(3 as libc::c_int as isize)
        {
            let fresh12 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh12 as usize] = 0 as libc::c_int as uint8_t;
            total = 3 as libc::c_int;
        }
        if !(*rp_ref.offset(4 as libc::c_int as isize)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                *ref_poc.offset(4 as libc::c_int as isize) as libc::c_int,
                (*frm_hdr).frame_offset,
            ) > 0 as libc::c_int
        {
            let fresh13 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh13 as usize] = 4 as libc::c_int as uint8_t;
        }
        if !(*rp_ref.offset(5 as libc::c_int as isize)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                *ref_poc.offset(5 as libc::c_int as isize) as libc::c_int,
                (*frm_hdr).frame_offset,
            ) > 0 as libc::c_int
        {
            let fresh14 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh14 as usize] = 5 as libc::c_int as uint8_t;
        }
        if (*rf).n_mfmvs < total
            && !(*rp_ref.offset(6 as libc::c_int as isize)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                *ref_poc.offset(6 as libc::c_int as isize) as libc::c_int,
                (*frm_hdr).frame_offset,
            ) > 0 as libc::c_int
        {
            let fresh15 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh15 as usize] = 6 as libc::c_int as uint8_t;
        }
        if (*rf).n_mfmvs < total
            && !(*rp_ref.offset(1 as libc::c_int as isize)).is_null()
        {
            let fresh16 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh16 as usize] = 1 as libc::c_int as uint8_t;
        }
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < (*rf).n_mfmvs {
            let rpoc: libc::c_uint = *ref_poc
                .offset((*rf).mfmv_ref[n as usize] as isize);
            let diff1: libc::c_int = get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                rpoc as libc::c_int,
                (*frm_hdr).frame_offset,
            );
            if diff1.abs() > 31 as libc::c_int {
                (*rf)
                    .mfmv_ref2cur[n
                    as usize] = -(2147483647 as libc::c_int) - 1 as libc::c_int;
            } else {
                (*rf)
                    .mfmv_ref2cur[n
                    as usize] = if ((*rf).mfmv_ref[n as usize] as libc::c_int)
                    < 4 as libc::c_int
                {
                    -diff1
                } else {
                    diff1
                };
                let mut m: libc::c_int = 0 as libc::c_int;
                while m < 7 as libc::c_int {
                    let rrpoc: libc::c_uint = (*ref_ref_poc
                        .offset((*rf).mfmv_ref[n as usize] as isize))[m as usize];
                    let diff2: libc::c_int = get_poc_diff(
                        (*seq_hdr).order_hint_n_bits,
                        rpoc as libc::c_int,
                        rrpoc as libc::c_int,
                    );
                    (*rf)
                        .mfmv_ref2ref[n
                        as usize][m
                        as usize] = if diff2 as libc::c_uint > 31 as libc::c_uint {
                        0 as libc::c_int
                    } else {
                        diff2
                    };
                    m += 1;
                }
            }
            n += 1;
        }
    }
    (*rf).use_ref_frame_mvs = ((*rf).n_mfmvs > 0 as libc::c_int) as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_init(rf: *mut refmvs_frame) {
    (*rf).r = 0 as *mut refmvs_block;
    (*rf).r_stride = 0 as libc::c_int as ptrdiff_t;
    (*rf).rp_proj = 0 as *mut refmvs_temporal_block;
    (*rf).rp_stride = 0 as libc::c_int as ptrdiff_t;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_refmvs_clear(rf: *mut refmvs_frame) {
    if !((*rf).r).is_null() {
        dav1d_freep_aligned(&mut (*rf).r as *mut *mut refmvs_block as *mut libc::c_void);
    }
    if !((*rf).rp_proj).is_null() {
        dav1d_freep_aligned(
            &mut (*rf).rp_proj as *mut *mut refmvs_temporal_block as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn splat_mv_c(
    mut rr: *mut *mut refmvs_block,
    rmv: *const refmvs_block,
    bx4: libc::c_int,
    bw4: libc::c_int,
    mut bh4: libc::c_int,
) {
    loop {
        let fresh17 = rr;
        rr = rr.offset(1);
        let r: *mut refmvs_block = (*fresh17).offset(bx4 as isize);
        let mut x: libc::c_int = 0 as libc::c_int;
        while x < bw4 {
            *r.offset(x as isize) = *rmv;
            x += 1;
        }
        bh4 -= 1;
        if !(bh4 != 0) {
            break;
        }
    };
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[inline(always)]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "asm"))]
unsafe extern "C" fn refmvs_dsp_init_x86(c: *mut Dav1dRefmvsDSPContext) {
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX512ICL;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_SSE2;
    use crate::src::x86::cpu::DAV1D_X86_CPU_FLAG_AVX2;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    (*c).splat_mv = Some(dav1d_splat_mv_sse2);

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    (*c).splat_mv = Some(dav1d_splat_mv_avx2);

    if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
        return;
    }

    (*c).splat_mv = Some(dav1d_splat_mv_avx512icl);
}

#[inline(always)]
#[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), feature = "asm"))]
unsafe extern "C" fn refmvs_dsp_init_arm(c: *mut Dav1dRefmvsDSPContext) {
    use crate::src::arm::cpu::DAV1D_ARM_CPU_FLAG_NEON;

    let flags: libc::c_uint = dav1d_get_cpu_flags();
    if (flags & DAV1D_ARM_CPU_FLAG_NEON) != 0 {
        (*c).splat_mv = Some(dav1d_splat_mv_neon);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_refmvs_dsp_init(c: *mut Dav1dRefmvsDSPContext) {
    (*c)
        .splat_mv = Some(
        splat_mv_c
            as unsafe extern "C" fn(
                *mut *mut refmvs_block,
                *const refmvs_block,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    cfg_if! {
        if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "asm"))] {
            refmvs_dsp_init_x86(c);
        } else if #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), feature = "asm"))] {
            refmvs_dsp_init_arm(c);
        }
    }
}
