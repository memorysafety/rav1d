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
}

#[cfg(feature = "asm")]
extern "C" {
    static mut dav1d_cpu_flags_mask: libc::c_uint;
    static mut dav1d_cpu_flags: libc::c_uint;
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
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

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
extern "C" {
    fn dav1d_splat_mv_neon(
        rr: *mut *mut refmvs_block,
        rmv: *const refmvs_block,
        bx4: libc::c_int,
        bw4: libc::c_int,
        bh4: libc::c_int,
    );
}

use crate::src::tables::dav1d_block_dimensions;

use crate::include::dav1d::headers::DAV1D_WM_TYPE_TRANSLATION;

use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::src::levels::BlockSize;

use crate::src::intra_edge::EdgeFlags;
use crate::src::levels::mv;

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

#[derive(Clone, Copy, Default, PartialEq, Eq)]
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
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct refmvs_candidate {
    pub mv: refmvs_mvpair,
    pub weight: libc::c_int,
}
pub type splat_mv_fn = Option<
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

use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;
use crate::src::env::fix_mv_precision;
use crate::src::env::get_poc_diff;

use crate::src::env::get_gmv_2d;
use crate::src::mem::dav1d_freep_aligned;

use crate::src::mem::dav1d_alloc_aligned;

fn add_spatial_candidate(
    mvstack: &mut [refmvs_candidate],
    cnt: &mut usize,
    weight: libc::c_int,
    b: &refmvs_block,
    r#ref: refmvs_refpair,
    gmv: &[mv; 2],
    have_newmv_match: &mut libc::c_int,
    have_refmv_match: &mut libc::c_int,
) {
    if b.mv.mv[0].is_invalid() {
        // intra block, no intrabc
        return;
    }

    let mf_odd = b.mf & 1 != 0;
    if r#ref.r#ref[1] == -1 {
        for n in 0..2 {
            if b.r#ref.r#ref[n] == r#ref.r#ref[0] {
                let cand_mv = if mf_odd && gmv[0] != mv::INVALID {
                    gmv[0]
                } else {
                    b.mv.mv[n]
                };

                *have_refmv_match = 1;
                *have_newmv_match |= b.mf as libc::c_int >> 1;

                let last = *cnt;
                for cand in &mut mvstack[..last] {
                    if cand.mv.mv[0] == cand_mv {
                        cand.weight += weight;
                        return;
                    }
                }

                if last < 8 {
                    let cand = &mut mvstack[last];
                    cand.mv.mv[0] = cand_mv;
                    cand.weight = weight;
                    *cnt = last + 1;
                }
                return;
            }
        }
    } else if b.r#ref == r#ref {
        let cand_mv = refmvs_mvpair {
            mv: [
                if mf_odd && gmv[0] != mv::INVALID {
                    gmv[0]
                } else {
                    b.mv.mv[0]
                },
                if mf_odd && gmv[1] != mv::INVALID {
                    gmv[1]
                } else {
                    b.mv.mv[1]
                },
            ],
        };

        *have_refmv_match = 1;
        *have_newmv_match |= b.mf as libc::c_int >> 1;

        let last = *cnt;
        for cand in &mut mvstack[..last] {
            if cand.mv == cand_mv {
                cand.weight += weight;
                return;
            }
        }

        if last < 8 {
            let cand = &mut mvstack[last];
            cand.mv = cand_mv;
            cand.weight = weight;
            *cnt = last + 1;
        }
    }
}

unsafe fn scan_row(
    mvstack: &mut [refmvs_candidate],
    cnt: &mut usize,
    r#ref: refmvs_refpair,
    gmv: &[mv; 2],
    b: *const refmvs_block,
    bw4: libc::c_int,
    w4: libc::c_int,
    max_rows: libc::c_int,
    step: libc::c_int,
    have_newmv_match: &mut libc::c_int,
    have_refmv_match: &mut libc::c_int,
) -> libc::c_int {
    let mut cand_b = &*b;
    let first_cand_bs = cand_b.bs as BlockSize;
    let first_cand_b_dim = &dav1d_block_dimensions[first_cand_bs as usize];
    let mut cand_bw4 = first_cand_b_dim[0] as libc::c_int;
    let mut len = imax(step, imin(bw4, cand_bw4));

    if bw4 <= cand_bw4 {
        // FIXME weight can be higher for odd blocks (bx4 & 1), but then the
        // position of the first block has to be odd already, i.e. not just
        // for row_offset=-3/-5
        // FIXME why can this not be cand_bw4?
        let weight = if bw4 == 1 {
            2
        } else {
            imax(2, imin(2 * max_rows, first_cand_b_dim[1] as libc::c_int))
        };
        add_spatial_candidate(
            mvstack,
            cnt,
            len * weight,
            &*cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        return weight >> 1;
    }

    let mut x = 0;
    loop {
        // FIXME if we overhang above, we could fill a bitmask so we don't have
        // to repeat the add_spatial_candidate() for the next row, but just increase
        // the weight here
        add_spatial_candidate(
            mvstack,
            cnt,
            len * 2,
            &*cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        x += len;
        if x >= w4 {
            return 1;
        }
        cand_b = &*b.offset(x as isize);
        cand_bw4 = dav1d_block_dimensions[cand_b.bs as usize][0] as libc::c_int;
        assert!(cand_bw4 < bw4);
        len = imax(step, cand_bw4);
    }
}

unsafe fn scan_col(
    mvstack: &mut [refmvs_candidate],
    cnt: &mut usize,
    r#ref: refmvs_refpair,
    gmv: &[mv; 2],
    b: *const *mut refmvs_block,
    bh4: libc::c_int,
    h4: libc::c_int,
    bx4: libc::c_int,
    max_cols: libc::c_int,
    step: libc::c_int,
    have_newmv_match: &mut libc::c_int,
    have_refmv_match: &mut libc::c_int,
) -> libc::c_int {
    let mut cand_b = &*(*b.offset(0)).offset(bx4 as isize);
    let first_cand_bs = cand_b.bs as BlockSize;
    let first_cand_b_dim = &dav1d_block_dimensions[first_cand_bs as usize];
    let mut cand_bh4 = first_cand_b_dim[1] as libc::c_int;
    let mut len = imax(step, imin(bh4, cand_bh4));

    if bh4 <= cand_bh4 {
        // FIXME weight can be higher for odd blocks (by4 & 1), but then the
        // position of the first block has to be odd already, i.e. not just
        // for col_offset=-3/-5
        // FIXME why can this not be cand_bh4?
        let weight = if bh4 == 1 {
            2
        } else {
            imax(2, imin(2 * max_cols, first_cand_b_dim[0] as libc::c_int))
        };
        add_spatial_candidate(
            mvstack,
            cnt,
            len * weight,
            &*cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        return weight >> 1;
    }

    let mut y = 0;
    loop {
        // FIXME if we overhang above, we could fill a bitmask so we don't have
        // to repeat the add_spatial_candidate() for the next row, but just increase
        // the weight here
        add_spatial_candidate(
            mvstack,
            cnt,
            len * 2,
            &*cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        y += len;
        if y >= h4 {
            return 1;
        }
        cand_b = &*(*b.offset(y as isize)).offset(bx4 as isize);
        cand_bh4 = dav1d_block_dimensions[cand_b.bs as usize][1] as libc::c_int;
        assert!(cand_bh4 < bh4);
        len = imax(step, cand_bh4);
    }
}

#[inline]
fn mv_projection(mv: mv, num: libc::c_int, den: libc::c_int) -> mv {
    static div_mult: [u16; 32] = [
        0, 16384, 8192, 5461, 4096, 3276, 2730, 2340, 2048, 1820, 1638, 1489, 1365, 1260, 1170,
        1092, 1024, 963, 910, 862, 819, 780, 744, 712, 682, 655, 630, 606, 585, 564, 546, 528,
    ];
    assert!(den > 0 && den < 32);
    assert!(num > -32 && num < 32);
    let frac = num * div_mult[den as usize] as libc::c_int;
    let y = mv.y as libc::c_int * frac;
    let x = mv.x as libc::c_int * frac;
    // Round and clip according to AV1 spec section 7.9.3
    let max = (1 << 14) - 1;
    return mv {
        y: iclip(y + 8192 + (y >> 31) >> 14, -max, max) as i16,
        x: iclip(x + 8192 + (x >> 31) >> 14, -max, max) as i16,
    };
}

unsafe fn add_temporal_candidate(
    rf: &refmvs_frame,
    mvstack: &mut [refmvs_candidate],
    cnt: &mut usize,
    rb: &refmvs_temporal_block,
    r#ref: refmvs_refpair,
    globalmv: Option<(&mut libc::c_int, &[mv; 2])>,
) {
    if rb.mv == mv::INVALID {
        return;
    }

    let mut mv = mv_projection(
        rb.mv,
        rf.pocdiff[r#ref.r#ref[0] as usize - 1] as libc::c_int,
        rb.r#ref as libc::c_int,
    );
    fix_mv_precision(&*rf.frm_hdr, &mut mv);

    let last = *cnt;
    if r#ref.r#ref[1] == -1 {
        if let Some((globalmv_ctx, gmv)) = globalmv {
            *globalmv_ctx =
                ((mv.x - gmv[0].x).abs() | (mv.y - gmv[0].y).abs() >= 16) as libc::c_int;
        }

        for cand in &mut mvstack[..last] {
            if cand.mv.mv[0] == mv {
                cand.weight += 2;
                return;
            }
        }
        if last < 8 {
            let cand = &mut mvstack[last];
            cand.mv.mv[0] = mv;
            cand.weight = 2;
            *cnt = last + 1;
        }
    } else {
        let mut mvp = refmvs_mvpair {
            mv: [
                mv,
                mv_projection(
                    rb.mv,
                    rf.pocdiff[r#ref.r#ref[1] as usize - 1] as libc::c_int,
                    rb.r#ref as libc::c_int,
                ),
            ],
        };
        fix_mv_precision(&*rf.frm_hdr, &mut mvp.mv[1]);

        for cand in &mut mvstack[..last] {
            if cand.mv == mvp {
                cand.weight += 2;
                return;
            }
        }
        if last < 8 {
            let cand = &mut mvstack[last];
            cand.mv = mvp;
            cand.weight = 2;
            *cnt = last + 1;
        }
    };
}

fn add_compound_extended_candidate(
    same: &mut [refmvs_candidate],
    same_count: &mut [usize; 4],
    cand_b: &refmvs_block,
    sign0: u8,
    sign1: u8,
    r#ref: refmvs_refpair,
    sign_bias: &[u8; 7],
) {
    let (same, diff) = same.split_at_mut(2);
    let (same_count, diff_count) = same_count.split_at_mut(2);

    for n in 0..2 {
        let cand_ref = cand_b.r#ref.r#ref[n];

        if cand_ref <= 0 {
            break;
        }

        let sign_bias = sign_bias[cand_ref as usize - 1];
        let mut cand_mv = cand_b.mv.mv[n];
        if cand_ref == r#ref.r#ref[0] {
            if same_count[0] < 2 {
                same[same_count[0]].mv.mv[0] = cand_mv;
                same_count[0] += 1;
            }
            if diff_count[1] < 2 {
                if (sign1 ^ sign_bias) != 0 {
                    cand_mv = -cand_mv;
                }
                diff[diff_count[1]].mv.mv[1] = cand_mv;
                diff_count[1] += 1;
            }
        } else if cand_ref == r#ref.r#ref[1] {
            if same_count[1] < 2 {
                same[same_count[1]].mv.mv[1] = cand_mv;
                same_count[1] += 1;
            }
            if diff_count[0] < 2 {
                if (sign0 ^ sign_bias) != 0 {
                    cand_mv = -cand_mv;
                }
                diff[diff_count[0]].mv.mv[0] = cand_mv;
                diff_count[0] += 1;
            }
        } else {
            let i_cand_mv = -cand_mv;

            if diff_count[0] < 2 {
                diff[diff_count[0]].mv.mv[0] = if (sign0 ^ sign_bias) != 0 {
                    i_cand_mv
                } else {
                    cand_mv
                };
                diff_count[0] += 1;
            }

            if diff_count[1] < 2 {
                diff[diff_count[1]].mv.mv[1] = if (sign1 ^ sign_bias) != 0 {
                    i_cand_mv
                } else {
                    cand_mv
                };
                diff_count[1] += 1;
            }
        }
    }
}

fn add_single_extended_candidate(
    mvstack: &mut [refmvs_candidate; 8],
    cnt: &mut usize,
    cand_b: &refmvs_block,
    sign: u8,
    sign_bias: &[u8; 7],
) {
    for n in 0..2 {
        let cand_ref = cand_b.r#ref.r#ref[n];

        if cand_ref <= 0 {
            // we need to continue even if cand_ref == ref.ref[0], since
            // the candidate could have been added as a globalmv variant,
            // which changes the value
            // FIXME if scan_{row,col}() returned a mask for the nearest
            // edge, we could skip the appropriate ones here
            break;
        }

        let mut cand_mv = cand_b.mv.mv[n];
        if (sign ^ sign_bias[cand_ref as usize - 1]) != 0 {
            cand_mv = -cand_mv;
        }

        let last = *cnt;
        let mut broke_early = false;
        for cand in &mut mvstack[..last] {
            if cand_mv == cand.mv.mv[0] {
                broke_early = true;
                break;
            }
        }
        if !broke_early {
            mvstack[last].mv.mv[0] = cand_mv;
            mvstack[last].weight = 2; // "minimal"
            *cnt = last + 1;
        }
    }
}

/// refmvs_frame allocates memory for one sbrow (32 blocks high, whole frame
/// wide) of 4x4-resolution refmvs_block entries for spatial MV referencing.
/// mvrefs_tile[] keeps a list of 35 (32 + 3 above) pointers into this memory,
/// and each sbrow, the bottom entries (y=27/29/31) are exchanged with the top
/// (-5/-3/-1) pointers by calling dav1d_refmvs_tile_sbrow_init() at the start
/// of each tile/sbrow.
///
/// For temporal MV referencing, we call dav1d_refmvs_save_tmvs() at the end of
/// each tile/sbrow (when tile column threading is enabled), or at the start of
/// each interleaved sbrow (i.e. once for all tile columns together, when tile
/// column threading is disabled). This will copy the 4x4-resolution spatial MVs
/// into 8x8-resolution refmvs_temporal_block structures. Then, for subsequent
/// frames, at the start of each tile/sbrow (when tile column threading is
/// enabled) or at the start of each interleaved sbrow (when tile column
/// threading is disabled), we call load_tmvs(), which will project the MVs to
/// their respective position in the current frame.
pub unsafe fn dav1d_refmvs_find(
    rt: &refmvs_tile,
    mvstack: &mut [refmvs_candidate; 8],
    cnt: &mut usize,
    ctx: &mut libc::c_int,
    r#ref: refmvs_refpair,
    bs: BlockSize,
    edge_flags: EdgeFlags,
    by4: libc::c_int,
    bx4: libc::c_int,
) {
    let rf = &*rt.rf;
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bw4 = b_dim[0] as libc::c_int;
    let w4 = imin(imin(bw4, 16), rt.tile_col.end - bx4);
    let bh4 = b_dim[1] as libc::c_int;
    let h4 = imin(imin(bh4, 16), rt.tile_row.end - by4);
    let mut gmv = [mv::default(); 2];
    let mut tgmv = [mv::default(); 2];

    *cnt = 0;
    assert!(
        r#ref.r#ref[0] >= 0 && r#ref.r#ref[0] <= 8 && r#ref.r#ref[1] >= -1 && r#ref.r#ref[1] <= 8
    );
    if r#ref.r#ref[0] > 0 {
        tgmv[0] = get_gmv_2d(
            &(*rf.frm_hdr).gmv[r#ref.r#ref[0] as usize - 1],
            bx4,
            by4,
            bw4,
            bh4,
            &*rf.frm_hdr,
        );
        gmv[0] =
            if (*rf.frm_hdr).gmv[r#ref.r#ref[0] as usize - 1].type_0 > DAV1D_WM_TYPE_TRANSLATION {
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
            &*rf.frm_hdr,
        );
        gmv[1] =
            if (*rf.frm_hdr).gmv[r#ref.r#ref[1] as usize - 1].type_0 > DAV1D_WM_TYPE_TRANSLATION {
                tgmv[1]
            } else {
                mv::INVALID
            };
    }

    // top
    let mut have_newmv = 0;
    let mut have_col_mvs = 0;
    let mut have_row_mvs = 0;
    let mut max_rows = 0;
    let mut n_rows = !0;
    let mut b_top = std::ptr::null();
    if by4 > rt.tile_row.start {
        max_rows = imin(
            by4 - rt.tile_row.start + 1 >> 1,
            2 + (bh4 > 1) as libc::c_int,
        ) as libc::c_uint;
        b_top = &mut *rt.r[(by4 as usize & 31) + 5 - 1].offset(bx4 as isize);
        n_rows = scan_row(
            mvstack,
            cnt,
            r#ref,
            &gmv,
            b_top,
            bw4,
            w4,
            max_rows as libc::c_int,
            if bw4 >= 16 { 4 } else { 1 },
            &mut have_newmv,
            &mut have_row_mvs,
        ) as libc::c_uint;
    }

    // left
    let mut max_cols = 0;
    let mut n_cols = !0;
    let mut b_left = std::ptr::null();
    if bx4 > rt.tile_col.start {
        max_cols = imin(
            bx4 - rt.tile_col.start + 1 >> 1,
            2 + (bw4 > 1) as libc::c_int,
        ) as libc::c_uint;
        b_left = &rt.r[(by4 as usize & 31) + 5];
        n_cols = scan_col(
            mvstack,
            cnt,
            r#ref,
            &gmv,
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

    // top/right
    if n_rows != !0
        && edge_flags & EDGE_I444_TOP_HAS_RIGHT != 0
        && imax(bw4, bh4) <= 16
        && bw4 + bx4 < rt.tile_col.end
    {
        add_spatial_candidate(
            mvstack,
            cnt,
            4,
            &*b_top.offset(bw4 as isize),
            r#ref,
            &gmv,
            &mut have_newmv,
            &mut have_row_mvs,
        );
    }

    let nearest_match = have_col_mvs + have_row_mvs;
    let nearest_cnt = *cnt;
    for cand in &mut mvstack[..nearest_cnt] {
        cand.weight += 640;
    }

    // temporal
    let mut globalmv_ctx = (*rf.frm_hdr).use_ref_frame_mvs;
    if rf.use_ref_frame_mvs != 0 {
        let stride: ptrdiff_t = rf.rp_stride;
        let by8 = by4 >> 1;
        let bx8 = bx4 >> 1;
        let rbi: *const refmvs_temporal_block = &mut *(rt.rp_proj)
            .offset((by8 & 15) as isize * stride + bx8 as isize)
            as *mut refmvs_temporal_block;
        let mut rb = rbi;
        let step_h = if bw4 >= 16 { 2 } else { 1 };
        let step_v = if bh4 >= 16 { 2 } else { 1 };
        let w8 = imin(w4 + 1 >> 1, 8);
        let h8 = imin(h4 + 1 >> 1, 8);
        for y in (0..h8).step_by(step_v) {
            for x in (0..w8).step_by(step_h) {
                add_temporal_candidate(
                    rf,
                    mvstack,
                    cnt,
                    &*rb.offset(x as isize),
                    r#ref,
                    if x | y == 0 {
                        Some((&mut globalmv_ctx, &tgmv))
                    } else {
                        None
                    },
                );
            }
            rb = rb.offset(stride * step_v as isize);
        }
        if imin(bw4, bh4) >= 2 && imax(bw4, bh4) < 16 {
            let bh8 = bh4 >> 1;
            let bw8 = bw4 >> 1;
            rb = &*rbi.offset(bh8 as isize * stride) as *const refmvs_temporal_block;
            let has_bottom =
                (by8 + bh8 < imin(rt.tile_row.end >> 1, (by8 & !7) + 8)) as libc::c_int;
            if has_bottom != 0 && bx8 - 1 >= imax(rt.tile_col.start >> 1, bx8 & !7) {
                add_temporal_candidate(rf, mvstack, cnt, &*rb.offset(-1), r#ref, None);
            }
            if bx8 + bw8 < imin(rt.tile_col.end >> 1, (bx8 & !7) + 8) {
                if has_bottom != 0 {
                    add_temporal_candidate(
                        rf,
                        mvstack,
                        cnt,
                        &*rb.offset(bw8 as isize),
                        r#ref,
                        None,
                    );
                }
                if (by8 + bh8 - 1) < imin(rt.tile_row.end >> 1, (by8 & !7) + 8) {
                    add_temporal_candidate(
                        rf,
                        mvstack,
                        cnt,
                        &*rb.offset(bw8 as isize - stride),
                        r#ref,
                        None,
                    );
                }
            }
        }
    }
    assert!(*cnt <= 8);

    // top/left (which, confusingly, is part of "secondary" references)
    let mut have_dummy_newmv_match = 0;
    if n_rows | n_cols != !0 {
        add_spatial_candidate(
            mvstack,
            cnt,
            4,
            &*b_top.offset(-1),
            r#ref,
            &gmv,
            &mut have_dummy_newmv_match,
            &mut have_row_mvs,
        );
    }

    // "secondary" (non-direct neighbour) top & left edges
    // what is different about secondary is that everything is now in 8x8 resolution
    for n in 2..=3 {
        if n as libc::c_uint > n_rows && n as libc::c_uint <= max_rows {
            n_rows = n_rows.wrapping_add(scan_row(
                mvstack,
                cnt,
                r#ref,
                &gmv,
                &mut *(rt.r[(((by4 & 31) - 2 * n + 1 | 1) + 5) as usize]).offset(bx4 as isize | 1),
                bw4,
                w4,
                (1 as libc::c_uint)
                    .wrapping_add(max_rows)
                    .wrapping_sub(n as libc::c_uint) as libc::c_int,
                if bw4 >= 16 { 4 } else { 2 },
                &mut have_dummy_newmv_match,
                &mut have_row_mvs,
            ) as libc::c_uint);
        }
        if n as libc::c_uint > n_cols && n as libc::c_uint <= max_cols {
            n_cols = n_cols.wrapping_add(scan_col(
                mvstack,
                cnt,
                r#ref,
                &gmv,
                &rt.r[(by4 as usize & 31 | 1) + 5],
                bh4,
                h4,
                bx4 - n * 2 + 1 | 1,
                (1 as libc::c_uint)
                    .wrapping_add(max_cols)
                    .wrapping_sub(n as libc::c_uint) as libc::c_int,
                if bh4 >= 16 { 4 } else { 2 },
                &mut have_dummy_newmv_match,
                &mut have_col_mvs,
            ) as libc::c_uint);
        }
    }
    assert!(*cnt <= 8);

    let ref_match_count = have_col_mvs + have_row_mvs;

    // context build-up
    let (refmv_ctx, newmv_ctx) = match nearest_match {
        0 => (
            imin(2, ref_match_count),
            (ref_match_count > 0) as libc::c_int,
        ),
        1 => (imin(ref_match_count * 3, 4), 3 - have_newmv),
        2 => (5, 5 - have_newmv),
        _ => (0, 0),
    };

    // sorting (nearest, then "secondary")
    // Previously used bubble sort; now we use Rust's stable sort,
    // which for small slices is insertion sort.
    mvstack[..nearest_cnt].sort_by_key(|cand| -cand.weight);
    mvstack[nearest_cnt..*cnt].sort_by_key(|cand| -cand.weight);

    if r#ref.r#ref[1] > 0 {
        if *cnt < 2 {
            let sign0 = rf.sign_bias[r#ref.r#ref[0] as usize - 1];
            let sign1 = rf.sign_bias[r#ref.r#ref[1] as usize - 1];
            let sz4 = imin(w4, h4);
            let cur_cnt = *cnt;
            let same = &mut mvstack[cur_cnt..];
            let mut same_count = [0; 4];

            // non-self references in top
            if n_rows != !0 {
                let mut x = 0;
                while x < sz4 {
                    let cand_b = &*b_top.offset(x as isize);
                    add_compound_extended_candidate(
                        same,
                        &mut same_count,
                        cand_b,
                        sign0,
                        sign1,
                        r#ref,
                        &rf.sign_bias,
                    );
                    x += dav1d_block_dimensions[cand_b.bs as usize][0] as libc::c_int;
                }
            }

            // non-self references in left
            if n_cols != !0 {
                let mut y = 0;
                while y < sz4 {
                    let cand_b = &*(*b_left.offset(y as isize)).offset(bx4 as isize - 1);
                    add_compound_extended_candidate(
                        same,
                        &mut same_count,
                        cand_b,
                        sign0,
                        sign1,
                        r#ref,
                        &rf.sign_bias,
                    );
                    y += dav1d_block_dimensions[cand_b.bs as usize][1] as libc::c_int;
                }
            }

            // Below, `same` will only be accessed by `m`, which is `< 2`,
            // so this `same.split_at_mut(2).0` won't be accessed out of bounds.
            let (same, diff) = same.split_at_mut(2);
            let diff = &diff[..]; // not &mut
            let diff_count = &same_count[2..];

            // merge together
            for n in 0..2 {
                let mut m = same_count[n];

                if m >= 2 {
                    continue;
                }

                let l = diff_count[n];
                if l != 0 {
                    same[m].mv.mv[n] = diff[0].mv.mv[n];
                    m += 1;
                    if m == 2 {
                        continue;
                    }
                    if l == 2 {
                        same[1].mv.mv[n] = diff[1].mv.mv[n];
                        continue;
                    }
                }
                for mut cand in &mut same[m..2] {
                    cand.mv.mv[n] = tgmv[n];
                }
            }

            // if the first extended was the same as the non-extended one,
            // then replace it with the second extended one
            let n = *cnt;
            let same = &mvstack[cur_cnt..]; // need to reborrow to get a &, not &mut
            if n == 1 && mvstack[0].mv == same[0].mv {
                mvstack[1].mv = mvstack[2].mv;
            }
            for cand in &mut mvstack[n..2] {
                cand.weight = 2;
            }
            *cnt = 2;
        }

        // clamping
        let left = -(bx4 + bw4 + 4) * 4 * 8;
        let right = (rf.iw4 - bx4 + 4) * 4 * 8;
        let top = -(by4 + bh4 + 4) * 4 * 8;
        let bottom = (rf.ih4 - by4 + 4) * 4 * 8;

        let n_refmvs = *cnt;

        for cand in &mut mvstack[..n_refmvs] {
            let mv = &mut cand.mv.mv;
            mv[0].x = iclip(mv[0].x as libc::c_int, left, right) as i16;
            mv[0].y = iclip(mv[0].y as libc::c_int, top, bottom) as i16;
            mv[1].x = iclip(mv[1].x as libc::c_int, left, right) as i16;
            mv[1].y = iclip(mv[1].y as libc::c_int, top, bottom) as i16;
        }

        *ctx = match refmv_ctx >> 1 {
            0 => imin(newmv_ctx, 1),
            1 => 1 + imin(newmv_ctx, 3),
            2 => iclip(3 + newmv_ctx, 4, 7),
            _ => *ctx,
        };

        return;
    } else if *cnt < 2 && r#ref.r#ref[0] > 0 {
        let sign = rf.sign_bias[r#ref.r#ref[0] as usize - 1];
        let sz4 = imin(w4, h4);

        // non-self references in top
        if n_rows != !0 {
            let mut x = 0;
            while x < sz4 && *cnt < 2 {
                let cand_b = &*b_top.offset(x as isize);
                add_single_extended_candidate(mvstack, cnt, cand_b, sign, &rf.sign_bias);
                x += dav1d_block_dimensions[cand_b.bs as usize][0] as libc::c_int;
            }
        }

        // non-self references in left
        if n_cols != !0 {
            let mut y = 0;
            while y < sz4 && *cnt < 2 {
                let cand_b = &*(*b_left.offset(y as isize)).offset(bx4 as isize - 1);
                add_single_extended_candidate(mvstack, cnt, cand_b, sign, &rf.sign_bias);
                y += dav1d_block_dimensions[cand_b.bs as usize][1] as libc::c_int;
            }
        }
    }
    assert!(*cnt <= 8);

    // clamping
    let mut n_refmvs = *cnt;
    if n_refmvs != 0 {
        let left = -(bx4 + bw4 + 4) * 4 * 8;
        let right = (rf.iw4 - bx4 + 4) * 4 * 8;
        let top = -(by4 + bh4 + 4) * 4 * 8;
        let bottom = (rf.ih4 - by4 + 4) * 4 * 8;

        for cand in &mut mvstack[..n_refmvs] {
            let mv = &mut cand.mv.mv;
            mv[0].x = iclip(mv[0].x as libc::c_int, left, right) as i16;
            mv[0].y = iclip(mv[0].y as libc::c_int, top, bottom) as i16;
        }
    }

    // Need to use `min` so we don't get a backwards range,
    // which will fail on slicing.
    for cand in &mut mvstack[std::cmp::min(*cnt, 2)..2] {
        cand.mv.mv[0] = tgmv[0];
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
    if (*rf).n_tile_threads == 1 {
        tile_row_idx = 0 as libc::c_int;
    }
    (*rt).rp_proj = &mut *((*rf).rp_proj).offset(16 * (*rf).rp_stride * tile_row_idx as isize)
        as *mut refmvs_temporal_block;
    let uses_2pass = ((*rf).n_tile_threads > 1 && (*rf).n_frame_threads > 1) as libc::c_int;
    let pass_off: ptrdiff_t = if uses_2pass != 0 && pass == 2 {
        35 * (*rf).r_stride * (*rf).n_tile_rows as isize
    } else {
        0
    };
    let mut r: *mut refmvs_block = &mut *((*rf).r)
        .offset(35 * (*rf).r_stride * tile_row_idx as isize + pass_off)
        as *mut refmvs_block;
    let sbsz = (*rf).sbsz;
    let off = sbsz * sby & 16;
    let mut i = 0;
    while i < sbsz {
        (*rt).r[(off + 5 + i) as usize] = r;
        i += 1;
        r = r.offset((*rf).r_stride as isize);
    }
    (*rt).r[(off + 0) as usize] = r;
    r = r.offset((*rf).r_stride as isize);
    (*rt).r[(off + 1) as usize] = 0 as *mut refmvs_block;
    (*rt).r[(off + 2) as usize] = r;
    r = r.offset((*rf).r_stride as isize);
    (*rt).r[(off + 3) as usize] = 0 as *mut refmvs_block;
    (*rt).r[(off + 4) as usize] = r;
    if sby & 1 != 0 {
        let tmp: *mut libc::c_void = (*rt).r[(off + 0) as usize] as *mut libc::c_void;
        (*rt).r[(off + 0) as usize] = (*rt).r[(off + sbsz + 0) as usize];
        (*rt).r[(off + sbsz + 0) as usize] = tmp as *mut refmvs_block;
        let tmp_0: *mut libc::c_void = (*rt).r[(off + 2) as usize] as *mut libc::c_void;
        (*rt).r[(off + 2) as usize] = (*rt).r[(off + sbsz + 2) as usize];
        (*rt).r[(off + sbsz + 2) as usize] = tmp_0 as *mut refmvs_block;
        let tmp_1: *mut libc::c_void = (*rt).r[(off + 4) as usize] as *mut libc::c_void;
        (*rt).r[(off + 4) as usize] = (*rt).r[(off + sbsz + 4) as usize];
        (*rt).r[(off + sbsz + 4) as usize] = tmp_1 as *mut refmvs_block;
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
    if (*rf).n_tile_threads == 1 {
        tile_row_idx = 0 as libc::c_int;
    }
    if !(row_start8 >= 0) {
        unreachable!();
    }
    if !((row_end8 - row_start8) as libc::c_uint <= 16 as libc::c_uint) {
        unreachable!();
    }
    row_end8 = imin(row_end8, (*rf).ih8);
    let col_start8i = imax(col_start8 - 8, 0 as libc::c_int);
    let col_end8i = imin(col_end8 + 8, (*rf).iw8);
    let stride: ptrdiff_t = (*rf).rp_stride;
    let mut rp_proj: *mut refmvs_temporal_block = &mut *((*rf).rp_proj)
        .offset(16 * stride * tile_row_idx as isize + (row_start8 & 15) as isize * stride)
        as *mut refmvs_temporal_block;
    let mut y = row_start8;
    while y < row_end8 {
        let mut x = col_start8;
        while x < col_end8 {
            (*rp_proj.offset(x as isize)).mv = mv::INVALID;
            x += 1;
        }
        rp_proj = rp_proj.offset(stride as isize);
        y += 1;
    }
    rp_proj = &mut *((*rf).rp_proj).offset(16 * stride * tile_row_idx as isize)
        as *mut refmvs_temporal_block;
    let mut n = 0;
    while n < (*rf).n_mfmvs {
        let ref2cur = (*rf).mfmv_ref2cur[n as usize];
        if !(ref2cur == -(2147483647 as libc::c_int) - 1) {
            let r#ref = (*rf).mfmv_ref[n as usize] as libc::c_int;
            let ref_sign = r#ref - 4;
            let mut r: *const refmvs_temporal_block = &mut *(*((*rf).rp_ref).offset(r#ref as isize))
                .offset(row_start8 as isize * stride)
                as *mut refmvs_temporal_block;
            let mut y_0 = row_start8;
            while y_0 < row_end8 {
                let y_sb_align = y_0 & !(7 as libc::c_int);
                let y_proj_start = imax(y_sb_align, row_start8);
                let y_proj_end = imin(y_sb_align + 8, row_end8);
                let mut x_0 = col_start8i;
                while x_0 < col_end8i {
                    let mut rb: *const refmvs_temporal_block =
                        &*r.offset(x_0 as isize) as *const refmvs_temporal_block;
                    let b_ref = (*rb).r#ref as libc::c_int;
                    if !(b_ref == 0) {
                        let ref2ref = (*rf).mfmv_ref2ref[n as usize][(b_ref - 1) as usize];
                        if !(ref2ref == 0) {
                            let b_mv: mv = (*rb).mv;
                            let offset: mv = mv_projection(b_mv, ref2cur, ref2ref);
                            let mut pos_x = x_0
                                + apply_sign(
                                    (offset.x as libc::c_int).abs() >> 6,
                                    offset.x as libc::c_int ^ ref_sign,
                                );
                            let pos_y = y_0
                                + apply_sign(
                                    (offset.y as libc::c_int).abs() >> 6,
                                    offset.y as libc::c_int ^ ref_sign,
                                );
                            if pos_y >= y_proj_start && pos_y < y_proj_end {
                                let pos: ptrdiff_t = (pos_y & 15) as isize * stride;
                                loop {
                                    let x_sb_align = x_0 & !(7 as libc::c_int);
                                    if pos_x >= imax(x_sb_align - 8, col_start8)
                                        && pos_x < imin(x_sb_align + 16, col_end8)
                                    {
                                        (*rp_proj.offset(pos + pos_x as isize)).mv = (*rb).mv;
                                        (*rp_proj.offset(pos + pos_x as isize)).r#ref =
                                            ref2ref as int8_t;
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
    if !(row_start8 >= 0) {
        unreachable!();
    }
    if !((row_end8 - row_start8) as libc::c_uint <= 16 as libc::c_uint) {
        unreachable!();
    }
    row_end8 = imin(row_end8, (*rf).ih8);
    col_end8 = imin(col_end8, (*rf).iw8);
    let stride: ptrdiff_t = (*rf).rp_stride;
    let ref_sign: *const uint8_t = ((*rf).mfmv_sign).as_ptr();
    let mut rp: *mut refmvs_temporal_block =
        &mut *((*rf).rp).offset(row_start8 as isize * stride) as *mut refmvs_temporal_block;
    let mut y = row_start8;
    while y < row_end8 {
        let b: *const refmvs_block = (*rt).r[(6 + (y & 15) * 2) as usize];
        let mut x = col_start8;
        while x < col_end8 {
            let cand_b: *const refmvs_block =
                &*b.offset((x * 2 + 1) as isize) as *const refmvs_block;
            let bw8 = dav1d_block_dimensions[(*cand_b).bs as usize][0] as libc::c_int + 1 >> 1;
            if (*cand_b).r#ref.r#ref[1] as libc::c_int > 0
                && *ref_sign.offset(((*cand_b).r#ref.r#ref[1] as libc::c_int - 1) as isize)
                    as libc::c_int
                    != 0
                && (*cand_b).mv.mv[1].y.abs() | (*cand_b).mv.mv[1].x.abs() < 4096
            {
                let mut n = 0;
                while n < bw8 {
                    *rp.offset(x as isize) = {
                        let mut init = refmvs_temporal_block {
                            mv: (*cand_b).mv.mv[1],
                            r#ref: (*cand_b).r#ref.r#ref[1],
                        };
                        init
                    };
                    n += 1;
                    x += 1;
                }
            } else if (*cand_b).r#ref.r#ref[0] as libc::c_int > 0
                && *ref_sign.offset(((*cand_b).r#ref.r#ref[0] as libc::c_int - 1) as isize)
                    as libc::c_int
                    != 0
                && (*cand_b).mv.mv[0].y.abs() | (*cand_b).mv.mv[0].x.abs() < 4096
            {
                let mut n_0 = 0;
                while n_0 < bw8 {
                    *rp.offset(x as isize) = {
                        let mut init = refmvs_temporal_block {
                            mv: (*cand_b).mv.mv[0],
                            r#ref: (*cand_b).r#ref.r#ref[0],
                        };
                        init
                    };
                    n_0 += 1;
                    x += 1;
                }
            } else {
                let mut n_1 = 0;
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
    (*rf).iw8 = (*frm_hdr).width[0] + 7 >> 3;
    (*rf).ih8 = (*frm_hdr).height + 7 >> 3;
    (*rf).iw4 = (*rf).iw8 << 1;
    (*rf).ih4 = (*rf).ih8 << 1;
    let r_stride: ptrdiff_t =
        (((*frm_hdr).width[0] + 127 & !(127 as libc::c_int)) >> 2) as ptrdiff_t;
    let n_tile_rows = if n_tile_threads > 1 {
        (*frm_hdr).tiling.rows
    } else {
        1 as libc::c_int
    };
    if r_stride != (*rf).r_stride || n_tile_rows != (*rf).n_tile_rows {
        if !((*rf).r).is_null() {
            dav1d_freep_aligned(&mut (*rf).r as *mut *mut refmvs_block as *mut libc::c_void);
        }
        let uses_2pass = (n_tile_threads > 1 && n_frame_threads > 1) as libc::c_int;
        (*rf).r = dav1d_alloc_aligned(
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
    let rp_stride: ptrdiff_t = r_stride >> 1;
    if rp_stride != (*rf).rp_stride || n_tile_rows != (*rf).n_tile_rows {
        if !((*rf).rp_proj).is_null() {
            dav1d_freep_aligned(
                &mut (*rf).rp_proj as *mut *mut refmvs_temporal_block as *mut libc::c_void,
            );
        }
        (*rf).rp_proj = dav1d_alloc_aligned(
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
    let mut i = 0;
    while i < 7 {
        let poc_diff = get_poc_diff(
            (*seq_hdr).order_hint_n_bits,
            *ref_poc.offset(i as isize) as libc::c_int,
            poc as libc::c_int,
        );
        (*rf).sign_bias[i as usize] = (poc_diff > 0) as libc::c_int as uint8_t;
        (*rf).mfmv_sign[i as usize] = (poc_diff < 0) as libc::c_int as uint8_t;
        (*rf).pocdiff[i as usize] = iclip(
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
        let mut total = 2;
        if !(*rp_ref.offset(0)).is_null() && (*ref_ref_poc.offset(0))[6] != *ref_poc.offset(3) {
            let fresh12 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh12 as usize] = 0 as libc::c_int as uint8_t;
            total = 3 as libc::c_int;
        }
        if !(*rp_ref.offset(4)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                *ref_poc.offset(4) as libc::c_int,
                (*frm_hdr).frame_offset,
            ) > 0
        {
            let fresh13 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh13 as usize] = 4 as libc::c_int as uint8_t;
        }
        if !(*rp_ref.offset(5)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                *ref_poc.offset(5) as libc::c_int,
                (*frm_hdr).frame_offset,
            ) > 0
        {
            let fresh14 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh14 as usize] = 5 as libc::c_int as uint8_t;
        }
        if (*rf).n_mfmvs < total
            && !(*rp_ref.offset(6)).is_null()
            && get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                *ref_poc.offset(6) as libc::c_int,
                (*frm_hdr).frame_offset,
            ) > 0
        {
            let fresh15 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh15 as usize] = 6 as libc::c_int as uint8_t;
        }
        if (*rf).n_mfmvs < total && !(*rp_ref.offset(1)).is_null() {
            let fresh16 = (*rf).n_mfmvs;
            (*rf).n_mfmvs = (*rf).n_mfmvs + 1;
            (*rf).mfmv_ref[fresh16 as usize] = 1 as libc::c_int as uint8_t;
        }
        let mut n = 0;
        while n < (*rf).n_mfmvs {
            let rpoc: libc::c_uint = *ref_poc.offset((*rf).mfmv_ref[n as usize] as isize);
            let diff1 = get_poc_diff(
                (*seq_hdr).order_hint_n_bits,
                rpoc as libc::c_int,
                (*frm_hdr).frame_offset,
            );
            if diff1.abs() > 31 {
                (*rf).mfmv_ref2cur[n as usize] = -(2147483647 as libc::c_int) - 1;
            } else {
                (*rf).mfmv_ref2cur[n as usize] = if ((*rf).mfmv_ref[n as usize] as libc::c_int) < 4
                {
                    -diff1
                } else {
                    diff1
                };
                let mut m = 0;
                while m < 7 {
                    let rrpoc: libc::c_uint =
                        (*ref_ref_poc.offset((*rf).mfmv_ref[n as usize] as isize))[m as usize];
                    let diff2 = get_poc_diff(
                        (*seq_hdr).order_hint_n_bits,
                        rpoc as libc::c_int,
                        rrpoc as libc::c_int,
                    );
                    (*rf).mfmv_ref2ref[n as usize][m as usize] =
                        if diff2 as libc::c_uint > 31 as libc::c_uint {
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
    (*rf).use_ref_frame_mvs = ((*rf).n_mfmvs > 0) as libc::c_int;
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
        let mut x = 0;
        while x < bw4 {
            *r.offset(x as isize) = *rmv;
            x += 1;
        }
        bh4 -= 1;
        if !(bh4 != 0) {
            break;
        }
    }
}

#[cfg(feature = "asm")]
use crate::src::cpu::dav1d_get_cpu_flags;

#[inline(always)]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "asm"))]
unsafe extern "C" fn refmvs_dsp_init_x86(c: *mut Dav1dRefmvsDSPContext) {
    use crate::src::x86::cpu::*;

    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    (*c).splat_mv = Some(dav1d_splat_mv_sse2);

    #[cfg(target_arch = "x86_64")]
    {
        if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
            return;
        }

        (*c).splat_mv = Some(dav1d_splat_mv_avx2);

        if flags & DAV1D_X86_CPU_FLAG_AVX512ICL == 0 {
            return;
        }

        (*c).splat_mv = Some(dav1d_splat_mv_avx512icl);
    }
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
    (*c).splat_mv = Some(
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
