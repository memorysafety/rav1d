use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::src::align::Align16;
use crate::src::align::AlignedVec64;
use crate::src::env::fix_mv_precision;
use crate::src::env::get_gmv_2d;
use crate::src::env::get_poc_diff;
use crate::src::error::Rav1dResult;
use crate::src::internal::Bxy;
use crate::src::intra_edge::EdgeFlags;
use crate::src::levels::mv;
use crate::src::levels::BlockSize;
use crate::src::tables::dav1d_block_dimensions;
use cfg_if::cfg_if;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use zerocopy::FromZeroes;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern "C" {
    fn dav1d_splat_mv_sse2(
        rr: *mut *mut refmvs_block,
        rmv: *const refmvs_block,
        bx4: c_int,
        bw4: c_int,
        bh4: c_int,
        _rr_len: usize,
    );
    fn dav1d_save_tmvs_ssse3(
        rp: *mut refmvs_temporal_block,
        stride: ptrdiff_t,
        rr: *const [*const refmvs_block; 31],
        ref_sign: *const [u8; 7],
        col_end8: c_int,
        row_end8: c_int,
        col_start8: c_int,
        row_start8: c_int,
    );
}

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
extern "C" {
    fn dav1d_load_tmvs_sse4(
        rf: *const refmvs_frame,
        tile_row_idx: c_int,
        col_start8: c_int,
        col_end8: c_int,
        row_start8: c_int,
        row_end8: c_int,
    );
}

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
extern "C" {
    fn dav1d_splat_mv_avx512icl(
        rr: *mut *mut refmvs_block,
        rmv: *const refmvs_block,
        bx4: c_int,
        bw4: c_int,
        bh4: c_int,
        _rr_len: usize,
    );
    fn dav1d_splat_mv_avx2(
        rr: *mut *mut refmvs_block,
        rmv: *const refmvs_block,
        bx4: c_int,
        bw4: c_int,
        bh4: c_int,
        _rr_len: usize,
    );
    fn dav1d_save_tmvs_avx2(
        rp: *mut refmvs_temporal_block,
        stride: ptrdiff_t,
        rr: *const [*const refmvs_block; 31],
        ref_sign: *const [u8; 7],
        col_end8: c_int,
        row_end8: c_int,
        col_start8: c_int,
        row_start8: c_int,
    );
    fn dav1d_save_tmvs_avx512icl(
        rp: *mut refmvs_temporal_block,
        stride: ptrdiff_t,
        rr: *const [*const refmvs_block; 31],
        ref_sign: *const [u8; 7],
        col_end8: c_int,
        row_end8: c_int,
        col_start8: c_int,
        row_start8: c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
extern "C" {
    fn dav1d_splat_mv_neon(
        rr: *mut *mut refmvs_block,
        rmv: *const refmvs_block,
        bx4: c_int,
        bw4: c_int,
        bh4: c_int,
        _rr_len: usize,
    );
}

#[derive(Clone, Copy, Default)]
#[repr(C, packed)]
pub struct refmvs_temporal_block {
    pub mv: mv,
    pub r#ref: i8,
}

#[derive(Clone, Copy, PartialEq, Eq, FromZeroes)]
#[repr(C)]
pub struct refmvs_refpair {
    pub r#ref: [i8; 2],
}

impl From<[i8; 2]> for refmvs_refpair {
    fn from(from: [i8; 2]) -> Self {
        refmvs_refpair { r#ref: from }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, FromZeroes)]
#[repr(C)]
pub struct refmvs_mvpair {
    pub mv: [mv; 2],
}

/// For why this unaligned, see the aligned [`refmvs_block`] below.
#[derive(Clone, Copy, FromZeroes)]
#[repr(C, packed)]
pub struct refmvs_block_unaligned {
    pub mv: refmvs_mvpair,
    pub r#ref: refmvs_refpair,
    pub bs: BlockSize,
    pub mf: u8,
}

/// In C, `struct refmvs_block` is both aligned and packed,
/// but this (aligned types within a packed type) is not yet allowed in Rust
/// (see [rust-lang/rust#59154](https://github.com/rust-lang/rust/issues/59154)),
/// as different C compilers treat this differently.
/// To get around this limitation, we split `struct refmvs_block`
/// into an inner packed [`refmvs_block_unaligned`]
/// and an outer aligned [`refmvs_block`]
/// that is just a wrapper over the real [`refmvs_block_unaligned`].
#[derive(Clone, Copy, FromZeroes)]
#[repr(C, align(4))]
pub struct refmvs_block(pub refmvs_block_unaligned);

#[repr(C)]
pub(crate) struct refmvs_frame<'a> {
    /// This lifetime is for the pointers in this [`refmvs_frame`],
    /// which are borrowed from the parent [`RefMvsFrame`].
    /// Since this is a transient type for asm calls, a lifetime is fine,
    /// and since this is a ZST, the layout stays the same.
    _lifetime: PhantomData<&'a ()>,
    /// A pointer to a [`refmvs_frame`] may be passed to a [`load_tmvs_fn`] function.
    /// However, the [`Self::frm_hdr`] pointer is not accessed in such a function (see [`load_tmvs_c`]).
    /// But we need to keep the layout the same, so we store a `*const ()` null ptr.
    _frm_hdr: *const (),
    pub iw4: c_int,
    pub ih4: c_int,
    pub iw8: c_int,
    pub ih8: c_int,
    pub sbsz: c_int,
    pub use_ref_frame_mvs: c_int,
    pub sign_bias: [u8; 7],
    pub mfmv_sign: [u8; 7],
    pub pocdiff: [i8; 7],
    pub mfmv_ref: [u8; 3],
    pub mfmv_ref2cur: [c_int; 3],
    pub mfmv_ref2ref: [[c_int; 7]; 3],
    pub n_mfmvs: c_int,
    pub rp: *mut refmvs_temporal_block,
    pub rp_ref: *const *mut refmvs_temporal_block,
    pub rp_proj: *mut refmvs_temporal_block,
    pub rp_stride: ptrdiff_t,
    pub r: *mut refmvs_block,
    pub r_stride: ptrdiff_t,
    pub n_tile_rows: c_int,
    pub n_tile_threads: c_int,
    pub n_frame_threads: c_int,
}

pub(crate) struct RefMvsFrame {
    pub iw4: c_int,
    pub ih4: c_int,
    pub iw8: c_int,
    pub ih8: c_int,
    pub sbsz: c_int,
    pub use_ref_frame_mvs: c_int,
    pub sign_bias: [u8; 7],
    pub mfmv_sign: [u8; 7],
    pub pocdiff: [i8; 7],
    pub mfmv_ref: [u8; 3],
    pub mfmv_ref2cur: [c_int; 3],
    pub mfmv_ref2ref: [[c_int; 7]; 3],
    pub n_mfmvs: c_int,
    pub rp: *mut refmvs_temporal_block,
    pub rp_ref: *const *mut refmvs_temporal_block,
    pub rp_proj: AlignedVec64<refmvs_temporal_block>,
    pub rp_stride: u32,
    pub r: AlignedVec64<refmvs_block>,
    pub r_stride: u32,
    pub n_tile_rows: u32,
    pub n_tile_threads: u32,
    pub n_frame_threads: u32,
}

impl RefMvsFrame {
    pub fn as_dav1d<'a>(&'a mut self) -> refmvs_frame<'a> {
        let Self {
            iw4,
            ih4,
            iw8,
            ih8,
            sbsz,
            use_ref_frame_mvs,
            sign_bias,
            mfmv_sign,
            pocdiff,
            mfmv_ref,
            mfmv_ref2cur,
            mfmv_ref2ref,
            n_mfmvs,
            rp,
            rp_ref,
            ref mut rp_proj,
            rp_stride,
            ref mut r,
            r_stride,
            n_tile_rows,
            n_tile_threads,
            n_frame_threads,
        } = *self;
        refmvs_frame {
            _lifetime: PhantomData,
            _frm_hdr: ptr::null(), // never used
            iw4,
            ih4,
            iw8,
            ih8,
            sbsz,
            use_ref_frame_mvs,
            sign_bias,
            mfmv_sign,
            pocdiff,
            mfmv_ref,
            mfmv_ref2cur,
            mfmv_ref2ref,
            n_mfmvs,
            rp,
            rp_ref,
            rp_proj: rp_proj.as_mut_ptr(),
            rp_stride: rp_stride as _,
            r: r.as_mut_ptr(),
            r_stride: r_stride as _,
            n_tile_rows: n_tile_rows as _,
            n_tile_threads: n_tile_threads as _,
            n_frame_threads: n_frame_threads as _,
        }
    }
}

#[repr(C)]
pub struct refmvs_tile_range {
    pub start: c_int,
    pub end: c_int,
}

pub(crate) struct refmvs_tile {
    /// Unique indices into [`RefMvsFrame::r`].
    /// Out of bounds indices correspond to null pointers.
    ///
    /// # Safety
    ///
    /// These indices, when in bounds, are unique.
    pub r: [usize; 37],

    /// Index into [`RefMvsFrame::rp_proj`].
    pub rp_proj: usize,

    pub tile_col: refmvs_tile_range,
    pub tile_row: refmvs_tile_range,
}

impl refmvs_tile {
    pub fn r_ptrs(&self, r: &[refmvs_block]) -> [*const refmvs_block; 37] {
        self.r
            .map(|i| r.get(i).map_or_else(ptr::null, |r| r as *const _))
    }

    pub fn r_ptrs_mut(&self, r: &mut [refmvs_block]) -> [*mut refmvs_block; 37] {
        self.r
            .map(|i| r.get_mut(i).map_or_else(ptr::null_mut, |r| r as *mut _))
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct refmvs_candidate {
    pub mv: refmvs_mvpair,
    pub weight: c_int,
}

pub(crate) type load_tmvs_fn = unsafe extern "C" fn(
    rf: *const refmvs_frame,
    tile_row_idx: c_int,
    col_start8: c_int,
    col_end8: c_int,
    row_start8: c_int,
    row_end8: c_int,
) -> ();

pub type save_tmvs_fn = unsafe extern "C" fn(
    rp: *mut refmvs_temporal_block,
    stride: ptrdiff_t,
    rr: *const [*const refmvs_block; 31],
    ref_sign: *const [u8; 7],
    col_end8: c_int,
    row_end8: c_int,
    col_start8: c_int,
    row_start8: c_int,
) -> ();

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
extern "C" {
    fn dav1d_save_tmvs_neon(
        rp: *mut refmvs_temporal_block,
        stride: ptrdiff_t,
        rr: *const [*const refmvs_block; 31],
        ref_sign: *const [u8; 7],
        col_end8: c_int,
        row_end8: c_int,
        col_start8: c_int,
        row_start8: c_int,
    );
}

pub type splat_mv_fn = unsafe extern "C" fn(
    rr: *mut *mut refmvs_block,
    rmv: *const refmvs_block,
    bx4: c_int,
    bw4: c_int,
    bh4: c_int,
    // Extra args, unused by asm.F
    rr_len: usize,
) -> ();

#[repr(C)]
pub(crate) struct Rav1dRefmvsDSPContext {
    pub load_tmvs: load_tmvs_fn,
    pub save_tmvs: save_tmvs_fn,
    pub splat_mv: splat_mv_fn,
}

impl Rav1dRefmvsDSPContext {
    pub unsafe fn splat_mv(
        &self,
        r: &mut [refmvs_block],
        rt: &refmvs_tile,
        rmv: &Align16<refmvs_block>,
        b4: Bxy,
        bw4: usize,
        bh4: usize,
    ) {
        let mut r = rt.r_ptrs_mut(r);
        let rr = &mut r[(b4.y as usize & 31) + 5..];
        let rmv = &rmv.0;
        let bx4 = b4.x as _;
        let bw4 = bw4 as _;
        let bh4 = bh4 as _;
        (self.splat_mv)(rr.as_mut_ptr(), rmv, bx4, bw4, bh4, rr.len());
    }
}

fn add_spatial_candidate(
    mvstack: &mut [refmvs_candidate],
    cnt: &mut usize,
    weight: c_int,
    b: refmvs_block_unaligned,
    r#ref: refmvs_refpair,
    gmv: &[mv; 2],
    have_newmv_match: &mut c_int,
    have_refmv_match: &mut c_int,
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
                *have_newmv_match |= b.mf as c_int >> 1;

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
        *have_newmv_match |= b.mf as c_int >> 1;

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

fn scan_row(
    mvstack: &mut [refmvs_candidate],
    cnt: &mut usize,
    r#ref: refmvs_refpair,
    gmv: &[mv; 2],
    b: &[refmvs_block],
    bw4: c_int,
    w4: c_int,
    max_rows: c_int,
    step: c_int,
    have_newmv_match: &mut c_int,
    have_refmv_match: &mut c_int,
) -> c_int {
    let mut cand_b = b[0].0;
    let first_cand_bs = cand_b.bs;
    let first_cand_b_dim = &dav1d_block_dimensions[first_cand_bs as usize];
    let mut cand_bw4 = first_cand_b_dim[0] as c_int;
    let mut len = cmp::max(step, cmp::min(bw4, cand_bw4));

    if bw4 <= cand_bw4 {
        // FIXME weight can be higher for odd blocks (bx4 & 1), but then the
        // position of the first block has to be odd already, i.e. not just
        // for row_offset=-3/-5
        // FIXME why can this not be cand_bw4?
        let weight = if bw4 == 1 {
            2
        } else {
            cmp::max(2, cmp::min(2 * max_rows, first_cand_b_dim[1] as c_int))
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
            cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        x += len;
        if x >= w4 {
            return 1;
        }
        cand_b = b[x as usize].0;
        cand_bw4 = dav1d_block_dimensions[cand_b.bs as usize][0] as c_int;
        assert!(cand_bw4 < bw4);
        len = cmp::max(step, cand_bw4);
    }
}

fn scan_col(
    mvstack: &mut [refmvs_candidate],
    cnt: &mut usize,
    r#ref: refmvs_refpair,
    gmv: &[mv; 2],
    r: &[refmvs_block],
    b: &[usize],
    bh4: c_int,
    h4: c_int,
    bx4: c_int,
    max_cols: c_int,
    step: c_int,
    have_newmv_match: &mut c_int,
    have_refmv_match: &mut c_int,
) -> c_int {
    let mut cand_b = r[b[0] + bx4 as usize].0;
    let first_cand_bs = cand_b.bs;
    let first_cand_b_dim = &dav1d_block_dimensions[first_cand_bs as usize];
    let mut cand_bh4 = first_cand_b_dim[1] as c_int;
    let mut len = cmp::max(step, cmp::min(bh4, cand_bh4));

    if bh4 <= cand_bh4 {
        // FIXME weight can be higher for odd blocks (by4 & 1), but then the
        // position of the first block has to be odd already, i.e. not just
        // for col_offset=-3/-5
        // FIXME why can this not be cand_bh4?
        let weight = if bh4 == 1 {
            2
        } else {
            cmp::max(2, cmp::min(2 * max_cols, first_cand_b_dim[0] as c_int))
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
            cand_b,
            r#ref,
            gmv,
            have_newmv_match,
            have_refmv_match,
        );
        y += len;
        if y >= h4 {
            return 1;
        }
        cand_b = r[b[y as usize] + bx4 as usize].0;
        cand_bh4 = dav1d_block_dimensions[cand_b.bs as usize][1] as c_int;
        assert!(cand_bh4 < bh4);
        len = cmp::max(step, cand_bh4);
    }
}

#[inline]
fn mv_projection(mv: mv, num: c_int, den: c_int) -> mv {
    static div_mult: [u16; 32] = [
        0, 16384, 8192, 5461, 4096, 3276, 2730, 2340, 2048, 1820, 1638, 1489, 1365, 1260, 1170,
        1092, 1024, 963, 910, 862, 819, 780, 744, 712, 682, 655, 630, 606, 585, 564, 546, 528,
    ];
    assert!(den > 0 && den < 32);
    assert!(num > -32 && num < 32);
    let frac = num * div_mult[den as usize] as c_int;
    let y = mv.y as c_int * frac;
    let x = mv.x as c_int * frac;
    // Round and clip according to AV1 spec section 7.9.3
    let max = (1 << 14) - 1;
    return mv {
        y: iclip(y + 8192 + (y >> 31) >> 14, -max, max) as i16,
        x: iclip(x + 8192 + (x >> 31) >> 14, -max, max) as i16,
    };
}

fn add_temporal_candidate(
    rf: &RefMvsFrame,
    mvstack: &mut [refmvs_candidate],
    cnt: &mut usize,
    rb: &refmvs_temporal_block,
    r#ref: refmvs_refpair,
    globalmv: Option<(&mut c_int, &[mv; 2])>,
    frame_hdr: &Rav1dFrameHeader,
) {
    if rb.mv.is_invalid() {
        return;
    }

    let mut mv = mv_projection(
        rb.mv,
        rf.pocdiff[r#ref.r#ref[0] as usize - 1] as c_int,
        rb.r#ref as c_int,
    );
    fix_mv_precision(frame_hdr, &mut mv);

    let last = *cnt;
    if r#ref.r#ref[1] == -1 {
        if let Some((globalmv_ctx, gmv)) = globalmv {
            *globalmv_ctx = ((mv.x - gmv[0].x).abs() | (mv.y - gmv[0].y).abs() >= 16) as c_int;
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
                    rf.pocdiff[r#ref.r#ref[1] as usize - 1] as c_int,
                    rb.r#ref as c_int,
                ),
            ],
        };
        fix_mv_precision(frame_hdr, &mut mvp.mv[1]);

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
    cand_b: refmvs_block_unaligned,
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
    cand_b: refmvs_block_unaligned,
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
pub(crate) fn rav1d_refmvs_find(
    rt: &refmvs_tile,
    rf: &RefMvsFrame,
    mvstack: &mut [refmvs_candidate; 8],
    cnt: &mut usize,
    ctx: &mut c_int,
    r#ref: refmvs_refpair,
    bs: BlockSize,
    edge_flags: EdgeFlags,
    by4: c_int,
    bx4: c_int,
    frame_hdr: &Rav1dFrameHeader,
) {
    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bw4 = b_dim[0] as c_int;
    let w4 = cmp::min(cmp::min(bw4, 16), rt.tile_col.end - bx4);
    let bh4 = b_dim[1] as c_int;
    let h4 = cmp::min(cmp::min(bh4, 16), rt.tile_row.end - by4);
    let mut gmv = [mv::default(); 2];
    let mut tgmv = [mv::default(); 2];

    *cnt = 0;
    assert!(
        r#ref.r#ref[0] >= 0 && r#ref.r#ref[0] <= 8 && r#ref.r#ref[1] >= -1 && r#ref.r#ref[1] <= 8
    );
    if r#ref.r#ref[0] > 0 {
        tgmv[0] = get_gmv_2d(
            &frame_hdr.gmv[r#ref.r#ref[0] as usize - 1],
            bx4,
            by4,
            bw4,
            bh4,
            frame_hdr,
        );

        gmv[0] = if frame_hdr.gmv[r#ref.r#ref[0] as usize - 1].r#type
            > Rav1dWarpedMotionType::Translation
        {
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
            &frame_hdr.gmv[r#ref.r#ref[1] as usize - 1],
            bx4,
            by4,
            bw4,
            bh4,
            frame_hdr,
        );
        gmv[1] = if frame_hdr.gmv[r#ref.r#ref[1] as usize - 1].r#type
            > Rav1dWarpedMotionType::Translation
        {
            tgmv[1]
        } else {
            mv::INVALID
        };
    }

    // top
    let mut have_newmv = 0;
    let mut have_col_mvs = 0;
    let mut have_row_mvs = 0;
    let max_rows;
    let n_rows;
    let b_top;
    let b_top_offset;
    if by4 > rt.tile_row.start {
        max_rows = cmp::min(by4 - rt.tile_row.start + 1 >> 1, 2 + (bh4 > 1) as c_int) as c_uint;
        let i = rt.r[(by4 as usize & 31) + 5 - 1] + bx4 as usize;
        // We can't offset below 0.
        b_top_offset = match i {
            0 => 0,
            _ => 1,
        };
        b_top = &rf.r[i - b_top_offset..];
        n_rows = scan_row(
            mvstack,
            cnt,
            r#ref,
            &gmv,
            &b_top[b_top_offset..],
            bw4,
            w4,
            max_rows as c_int,
            if bw4 >= 16 { 4 } else { 1 },
            &mut have_newmv,
            &mut have_row_mvs,
        ) as c_uint;
    } else {
        max_rows = 0;
        n_rows = !0;
        b_top = Default::default(); // Never actually used
        b_top_offset = 0;
    }

    // left
    let max_cols;
    let n_cols;
    let b_left;
    if bx4 > rt.tile_col.start {
        max_cols = cmp::min(bx4 - rt.tile_col.start + 1 >> 1, 2 + (bw4 > 1) as c_int) as c_uint;
        b_left = &rt.r[(by4 as usize & 31) + 5..];
        n_cols = scan_col(
            mvstack,
            cnt,
            r#ref,
            &gmv,
            &rf.r,
            b_left,
            bh4,
            h4,
            bx4 - 1,
            max_cols as c_int,
            if bh4 >= 16 { 4 } else { 1 },
            &mut have_newmv,
            &mut have_col_mvs,
        ) as c_uint;
    } else {
        max_cols = 0;
        n_cols = !0;
        b_left = Default::default(); // Never actually used
    }

    // top/right
    if n_rows != !0
        && edge_flags.contains(EdgeFlags::I444_TOP_HAS_RIGHT)
        && cmp::max(bw4, bh4) <= 16
        && bw4 + bx4 < rt.tile_col.end
    {
        add_spatial_candidate(
            mvstack,
            cnt,
            4,
            b_top[bw4 as usize + b_top_offset].0,
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
    let mut globalmv_ctx = frame_hdr.use_ref_frame_mvs;
    if rf.use_ref_frame_mvs != 0 {
        let stride = rf.rp_stride as usize;
        let by8 = by4 >> 1;
        let bx8 = bx4 >> 1;
        let rbi = &rf.rp_proj[rt.rp_proj + (by8 as usize & 15) * stride + bx8 as usize..];
        let step_h = if bw4 >= 16 { 2 } else { 1 };
        let step_v = if bh4 >= 16 { 2 } else { 1 };
        let w8 = cmp::min(w4 + 1 >> 1, 8) as usize;
        let h8 = cmp::min(h4 + 1 >> 1, 8) as usize;
        for y in (0..h8).step_by(step_v) {
            for x in (0..w8).step_by(step_h) {
                add_temporal_candidate(
                    rf,
                    mvstack,
                    cnt,
                    &rbi[y * stride + x],
                    r#ref,
                    if x | y == 0 {
                        Some((&mut globalmv_ctx, &tgmv))
                    } else {
                        None
                    },
                    frame_hdr,
                );
            }
        }
        if cmp::min(bw4, bh4) >= 2 && cmp::max(bw4, bh4) < 16 {
            let bh8 = bh4 >> 1;
            let bw8 = bw4 >> 1;
            let offset = bh8 as usize * stride;
            let has_bottom = by8 + bh8 < cmp::min(rt.tile_row.end >> 1, (by8 & !7) + 8);
            if has_bottom && bx8 - 1 >= cmp::max(rt.tile_col.start >> 1, bx8 & !7) {
                let rb = &rbi[offset - 1];
                add_temporal_candidate(rf, mvstack, cnt, rb, r#ref, None, frame_hdr);
            }
            if bx8 + bw8 < cmp::min(rt.tile_col.end >> 1, (bx8 & !7) + 8) {
                if has_bottom {
                    let rb = &rbi[offset + bw8 as usize];
                    add_temporal_candidate(rf, mvstack, cnt, rb, r#ref, None, frame_hdr);
                }
                if (by8 + bh8 - 1) < cmp::min(rt.tile_row.end >> 1, (by8 & !7) + 8) {
                    let rb = &rbi[offset + bw8 as usize - stride];
                    add_temporal_candidate(rf, mvstack, cnt, rb, r#ref, None, frame_hdr);
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
            b_top[b_top_offset - 1].0,
            r#ref,
            &gmv,
            &mut have_dummy_newmv_match,
            &mut have_row_mvs,
        );
    }

    // "secondary" (non-direct neighbour) top & left edges
    // what is different about secondary is that everything is now in 8x8 resolution
    let mut n_rows = n_rows;
    let mut n_cols = n_cols;
    for n in 2..=3 {
        if n > n_rows && n <= max_rows {
            n_rows = n_rows.wrapping_add(scan_row(
                mvstack,
                cnt,
                r#ref,
                &gmv,
                &rf.r[rt.r[(((by4 & 31) - 2 * n as c_int + 1 | 1) + 5) as usize]
                    + (bx4 as usize | 1)..],
                bw4,
                w4,
                (1 + max_rows - n) as _,
                if bw4 >= 16 { 4 } else { 2 },
                &mut have_dummy_newmv_match,
                &mut have_row_mvs,
            ) as c_uint);
        }
        if n > n_cols && n <= max_cols {
            n_cols = n_cols.wrapping_add(scan_col(
                mvstack,
                cnt,
                r#ref,
                &gmv,
                &rf.r,
                &rt.r[(by4 as usize & 31 | 1) + 5..],
                bh4,
                h4,
                bx4 - n as c_int * 2 + 1 | 1,
                (1 + max_cols - n) as _,
                if bh4 >= 16 { 4 } else { 2 },
                &mut have_dummy_newmv_match,
                &mut have_col_mvs,
            ) as c_uint);
        }
    }
    assert!(*cnt <= 8);

    let ref_match_count = have_col_mvs + have_row_mvs;

    // context build-up
    let (refmv_ctx, newmv_ctx) = match nearest_match {
        0 => (cmp::min(2, ref_match_count), (ref_match_count > 0) as c_int),
        1 => (cmp::min(ref_match_count * 3, 4), 3 - have_newmv),
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
            let sz4 = cmp::min(w4, h4);
            let cur_cnt = *cnt;
            let same = &mut mvstack[cur_cnt..];
            let mut same_count = [0; 4];

            // non-self references in top
            if n_rows != !0 {
                let mut x = 0;
                while x < sz4 {
                    let cand_b = b_top[x as usize + b_top_offset].0;
                    add_compound_extended_candidate(
                        same,
                        &mut same_count,
                        cand_b,
                        sign0,
                        sign1,
                        r#ref,
                        &rf.sign_bias,
                    );
                    x += dav1d_block_dimensions[cand_b.bs as usize][0] as c_int;
                }
            }

            // non-self references in left
            if n_cols != !0 {
                let mut y = 0;
                while y < sz4 {
                    let cand_b = rf.r[b_left[y as usize] + bx4 as usize - 1].0;
                    add_compound_extended_candidate(
                        same,
                        &mut same_count,
                        cand_b,
                        sign0,
                        sign1,
                        r#ref,
                        &rf.sign_bias,
                    );
                    y += dav1d_block_dimensions[cand_b.bs as usize][1] as c_int;
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
                for cand in &mut same[m..2] {
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
            mv[0].x = iclip(mv[0].x as c_int, left, right) as i16;
            mv[0].y = iclip(mv[0].y as c_int, top, bottom) as i16;
            mv[1].x = iclip(mv[1].x as c_int, left, right) as i16;
            mv[1].y = iclip(mv[1].y as c_int, top, bottom) as i16;
        }

        *ctx = match refmv_ctx >> 1 {
            0 => cmp::min(newmv_ctx, 1),
            1 => 1 + cmp::min(newmv_ctx, 3),
            2 => iclip(3 + newmv_ctx, 4, 7),
            _ => *ctx,
        };

        return;
    } else if *cnt < 2 && r#ref.r#ref[0] > 0 {
        let sign = rf.sign_bias[r#ref.r#ref[0] as usize - 1];
        let sz4 = cmp::min(w4, h4);

        // non-self references in top
        if n_rows != !0 {
            let mut x = 0;
            while x < sz4 && *cnt < 2 {
                let cand_b = b_top[x as usize + b_top_offset].0;
                add_single_extended_candidate(mvstack, cnt, cand_b, sign, &rf.sign_bias);
                x += dav1d_block_dimensions[cand_b.bs as usize][0] as c_int;
            }
        }

        // non-self references in left
        if n_cols != !0 {
            let mut y = 0;
            while y < sz4 && *cnt < 2 {
                let cand_b = rf.r[b_left[y as usize] + bx4 as usize - 1].0;
                add_single_extended_candidate(mvstack, cnt, cand_b, sign, &rf.sign_bias);
                y += dav1d_block_dimensions[cand_b.bs as usize][1] as c_int;
            }
        }
    }
    assert!(*cnt <= 8);

    // clamping
    let n_refmvs = *cnt;
    if n_refmvs != 0 {
        let left = -(bx4 + bw4 + 4) * 4 * 8;
        let right = (rf.iw4 - bx4 + 4) * 4 * 8;
        let top = -(by4 + bh4 + 4) * 4 * 8;
        let bottom = (rf.ih4 - by4 + 4) * 4 * 8;

        for cand in &mut mvstack[..n_refmvs] {
            let mv = &mut cand.mv.mv;
            mv[0].x = iclip(mv[0].x as c_int, left, right) as i16;
            mv[0].y = iclip(mv[0].y as c_int, top, bottom) as i16;
        }
    }

    // Need to use `min` so we don't get a backwards range,
    // which will fail on slicing.
    for cand in &mut mvstack[cmp::min(*cnt, 2)..2] {
        cand.mv.mv[0] = tgmv[0];
    }

    *ctx = refmv_ctx << 4 | globalmv_ctx << 3 | newmv_ctx;
}

// cache the current tile/sbrow (or frame/sbrow)'s projectable motion vectors
// into buffers for use in future frame's temporal MV prediction
pub(crate) unsafe fn rav1d_refmvs_save_tmvs(
    dsp: &Rav1dRefmvsDSPContext,
    rt: &refmvs_tile,
    rf: &RefMvsFrame,
    col_start8: c_int,
    col_end8: c_int,
    row_start8: c_int,
    row_end8: c_int,
) {
    assert!(row_start8 >= 0);
    assert!((row_end8 - row_start8) as c_uint <= 16);
    let row_end8 = cmp::min(row_end8, rf.ih8);
    let col_end8 = cmp::min(col_end8, rf.iw8);
    let stride = rf.rp_stride as isize;
    let ref_sign = &rf.mfmv_sign;
    let rp = rf.rp.offset(row_start8 as isize * stride);
    let r = rt.r_ptrs(&rf.r);
    let rr = <&[_; 31]>::try_from(&r[6..]).unwrap();

    (dsp.save_tmvs)(
        rp, stride, rr, ref_sign, col_end8, row_end8, col_start8, row_start8,
    );
}

pub(crate) fn rav1d_refmvs_tile_sbrow_init(
    rf: &RefMvsFrame,
    tile_col_start4: c_int,
    tile_col_end4: c_int,
    tile_row_start4: c_int,
    tile_row_end4: c_int,
    sby: c_int,
    mut tile_row_idx: c_int,
    pass: c_int,
) -> refmvs_tile {
    if rf.n_tile_threads == 1 {
        tile_row_idx = 0;
    }
    let rp_stride = rf.rp_stride as usize;
    let r_stride = rf.r_stride as usize;
    let rp_proj = 16 * rp_stride * tile_row_idx as usize;
    let uses_2pass = rf.n_tile_threads > 1 && rf.n_frame_threads > 1;
    let pass_off = if uses_2pass && pass == 2 {
        35 * r_stride * rf.n_tile_rows as usize
    } else {
        0
    };
    let mut r = 35 * r_stride * tile_row_idx as usize + pass_off;
    let sbsz = rf.sbsz;
    let off = sbsz * sby & 16;
    let invalid_r = usize::MAX;
    let mut rr = [invalid_r; 37];
    // SAFETY: All of the valid `r`s that are set are unique, as we add `rf.r_stride` every time.
    for i in 0..sbsz {
        rr[(off + 5 + i) as usize] = r;
        r += r_stride;
    }
    rr[(off + 0) as usize] = r;
    r += r_stride;
    rr[(off + 1) as usize] = invalid_r;
    rr[(off + 2) as usize] = r;
    r += r_stride;
    rr[(off + 3) as usize] = invalid_r;
    rr[(off + 4) as usize] = r;
    if sby & 1 != 0 {
        for i in [0, 2, 4] {
            // SAFETY: Swapping doesn't affect uniqueness.
            rr.swap((off + i) as usize, (off + sbsz + i) as usize);
        }
    }

    refmvs_tile {
        r: rr,
        rp_proj,
        tile_col: refmvs_tile_range {
            start: tile_col_start4,
            end: cmp::min(tile_col_end4, rf.iw4),
        },
        tile_row: refmvs_tile_range {
            start: tile_row_start4,
            end: cmp::min(tile_row_end4, rf.ih4),
        },
    }
}

unsafe extern "C" fn load_tmvs_c(
    rf: *const refmvs_frame,
    mut tile_row_idx: c_int,
    col_start8: c_int,
    col_end8: c_int,
    row_start8: c_int,
    mut row_end8: c_int,
) {
    if (*rf).n_tile_threads == 1 {
        tile_row_idx = 0 as c_int;
    }
    if !(row_start8 >= 0) {
        unreachable!();
    }
    if !((row_end8 - row_start8) as c_uint <= 16 as c_uint) {
        unreachable!();
    }
    row_end8 = cmp::min(row_end8, (*rf).ih8);
    let col_start8i = cmp::max(col_start8 - 8, 0 as c_int);
    let col_end8i = cmp::min(col_end8 + 8, (*rf).iw8);
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
        if !(ref2cur == i32::MIN) {
            let r#ref = (*rf).mfmv_ref[n as usize] as c_int;
            let ref_sign = r#ref - 4;
            let mut r: *const refmvs_temporal_block = &mut *(*((*rf).rp_ref).offset(r#ref as isize))
                .offset(row_start8 as isize * stride)
                as *mut refmvs_temporal_block;
            let mut y_0 = row_start8;
            while y_0 < row_end8 {
                let y_sb_align = y_0 & !(7 as c_int);
                let y_proj_start = cmp::max(y_sb_align, row_start8);
                let y_proj_end = cmp::min(y_sb_align + 8, row_end8);
                let mut x_0 = col_start8i;
                while x_0 < col_end8i {
                    let mut rb: *const refmvs_temporal_block =
                        &*r.offset(x_0 as isize) as *const refmvs_temporal_block;
                    let b_ref = (*rb).r#ref as c_int;
                    if !(b_ref == 0) {
                        let ref2ref = (*rf).mfmv_ref2ref[n as usize][(b_ref - 1) as usize];
                        if !(ref2ref == 0) {
                            let b_mv: mv = (*rb).mv;
                            let offset: mv = mv_projection(b_mv, ref2cur, ref2ref);
                            let mut pos_x = x_0
                                + apply_sign(
                                    (offset.x as c_int).abs() >> 6,
                                    offset.x as c_int ^ ref_sign,
                                );
                            let pos_y = y_0
                                + apply_sign(
                                    (offset.y as c_int).abs() >> 6,
                                    offset.y as c_int ^ ref_sign,
                                );
                            if pos_y >= y_proj_start && pos_y < y_proj_end {
                                let pos: ptrdiff_t = (pos_y & 15) as isize * stride;
                                loop {
                                    let x_sb_align = x_0 & !(7 as c_int);
                                    if pos_x >= cmp::max(x_sb_align - 8, col_start8)
                                        && pos_x < cmp::min(x_sb_align + 16, col_end8)
                                    {
                                        (*rp_proj.offset(pos + pos_x as isize)).mv = (*rb).mv;
                                        (*rp_proj.offset(pos + pos_x as isize)).r#ref =
                                            ref2ref as i8;
                                    }
                                    x_0 += 1;
                                    if x_0 >= col_end8i {
                                        break;
                                    }
                                    rb = rb.offset(1);
                                    let rb_mv = (*rb).mv;
                                    if (*rb).r#ref as c_int != b_ref || rb_mv != b_mv {
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
                                    let rb_mv = (*rb).mv;
                                    if (*rb).r#ref as c_int != b_ref || rb_mv != b_mv {
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

unsafe extern "C" fn save_tmvs_c(
    mut rp: *mut refmvs_temporal_block,
    stride: ptrdiff_t,
    rr: *const [*const refmvs_block; 31],
    ref_sign: *const [u8; 7],
    col_end8: c_int,
    row_end8: c_int,
    col_start8: c_int,
    row_start8: c_int,
) {
    let rr = &*rr;
    let ref_sign = &*ref_sign;
    let mut y = row_start8;
    while y < row_end8 {
        let b: *const refmvs_block = rr[((y & 15) * 2) as usize];
        let mut x = col_start8;
        while x < col_end8 {
            let cand_b = (*b.offset((x * 2 + 1) as isize)).0;
            let bw8 = dav1d_block_dimensions[cand_b.bs as usize][0] as c_int + 1 >> 1;
            if cand_b.r#ref.r#ref[1] as c_int > 0
                && ref_sign[(cand_b.r#ref.r#ref[1] as c_int - 1) as usize] as c_int != 0
                && cand_b.mv.mv[1].y.abs() | cand_b.mv.mv[1].x.abs() < 4096
            {
                let mut n = 0;
                while n < bw8 {
                    *rp.offset(x as isize) = {
                        let init = refmvs_temporal_block {
                            mv: cand_b.mv.mv[1],
                            r#ref: cand_b.r#ref.r#ref[1],
                        };
                        init
                    };
                    n += 1;
                    x += 1;
                }
            } else if cand_b.r#ref.r#ref[0] as c_int > 0
                && ref_sign[(cand_b.r#ref.r#ref[0] as c_int - 1) as usize] as c_int != 0
                && cand_b.mv.mv[0].y.abs() | cand_b.mv.mv[0].x.abs() < 4096
            {
                let mut n = 0;
                while n < bw8 {
                    *rp.offset(x as isize) = {
                        let init = refmvs_temporal_block {
                            mv: cand_b.mv.mv[0],
                            r#ref: cand_b.r#ref.r#ref[0],
                        };
                        init
                    };
                    n += 1;
                    x += 1;
                }
            } else {
                let mut n = 0;
                while n < bw8 {
                    *rp.offset(x as isize) = refmvs_temporal_block {
                        mv: mv { x: 0, y: 0 },
                        r#ref: 0,
                    };
                    n += 1;
                    x += 1;
                }
            }
        }
        rp = rp.offset(stride as isize);
        y += 1;
    }
}

pub(crate) fn rav1d_refmvs_init_frame(
    rf: &mut RefMvsFrame,
    seq_hdr: &Rav1dSequenceHeader,
    frm_hdr: &Rav1dFrameHeader,
    ref_poc: &[c_uint; 7],
    rp: *mut refmvs_temporal_block,
    ref_ref_poc: &[[c_uint; 7]; 7],
    rp_ref: &[*mut refmvs_temporal_block; 7],
    n_tile_threads: u32,
    n_frame_threads: u32,
) -> Rav1dResult {
    rf.sbsz = 16 << seq_hdr.sb128;
    rf.iw8 = frm_hdr.size.width[0] + 7 >> 3;
    rf.ih8 = frm_hdr.size.height + 7 >> 3;
    rf.iw4 = rf.iw8 << 1;
    rf.ih4 = rf.ih8 << 1;

    let r_stride = ((frm_hdr.size.width[0] + 127 & !127) >> 2) as u32;
    let n_tile_rows = if n_tile_threads > 1 {
        frm_hdr.tiling.rows as u32
    } else {
        1
    };
    let uses_2pass = (n_tile_threads > 1 && n_frame_threads > 1) as usize;
    // TODO fallible allocation
    rf.r.resize(
        35 * r_stride as usize * n_tile_rows as usize * (1 + uses_2pass),
        FromZeroes::new_zeroed(),
    );
    rf.r_stride = r_stride;

    let rp_stride = r_stride >> 1;
    // TODO fallible allocation
    rf.rp_proj.resize(
        16 * rp_stride as usize * n_tile_rows as usize,
        Default::default(),
    );
    rf.rp_stride = rp_stride;

    rf.n_tile_rows = n_tile_rows;
    rf.n_tile_threads = n_tile_threads;
    rf.n_frame_threads = n_frame_threads;
    rf.rp = rp;
    rf.rp_ref = rp_ref.as_ptr();
    let poc = frm_hdr.frame_offset as c_uint;
    for i in 0..7 {
        let poc_diff = get_poc_diff(seq_hdr.order_hint_n_bits, ref_poc[i] as c_int, poc as c_int);
        rf.sign_bias[i] = (poc_diff > 0) as u8;
        rf.mfmv_sign[i] = (poc_diff < 0) as u8;
        rf.pocdiff[i] = iclip(
            get_poc_diff(seq_hdr.order_hint_n_bits, poc as c_int, ref_poc[i] as c_int),
            -31,
            31,
        ) as i8;
    }

    // temporal MV setup
    rf.n_mfmvs = 0;
    if frm_hdr.use_ref_frame_mvs != 0 && seq_hdr.order_hint_n_bits != 0 {
        let mut total = 2;
        if !rp_ref[0].is_null() && ref_ref_poc[0][6] != ref_poc[3] {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 0; // last
            rf.n_mfmvs += 1;
            total = 3;
        }
        if !rp_ref[4].is_null()
            && get_poc_diff(
                seq_hdr.order_hint_n_bits,
                ref_poc[4] as c_int,
                frm_hdr.frame_offset,
            ) > 0
        {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 4; // bwd
            rf.n_mfmvs += 1;
        }
        if !rp_ref[5].is_null()
            && get_poc_diff(
                seq_hdr.order_hint_n_bits,
                ref_poc[5] as c_int,
                frm_hdr.frame_offset,
            ) > 0
        {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 5; // altref2
            rf.n_mfmvs += 1;
        }
        if rf.n_mfmvs < total
            && !rp_ref[6].is_null()
            && get_poc_diff(
                seq_hdr.order_hint_n_bits,
                ref_poc[6] as c_int,
                frm_hdr.frame_offset,
            ) > 0
        {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 6; // altref
            rf.n_mfmvs += 1;
        }
        if rf.n_mfmvs < total && !rp_ref[1].is_null() {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 1; // last2
            rf.n_mfmvs += 1;
        }

        for n in 0..rf.n_mfmvs as usize {
            let rpoc = ref_poc[rf.mfmv_ref[n] as usize];
            let diff1 = get_poc_diff(
                seq_hdr.order_hint_n_bits,
                rpoc as c_int,
                frm_hdr.frame_offset,
            );
            if diff1.abs() > 31 {
                rf.mfmv_ref2cur[n] = i32::MIN;
            } else {
                rf.mfmv_ref2cur[n] = if rf.mfmv_ref[n] < 4 { -diff1 } else { diff1 };
                for m in 0..7 {
                    let rrpoc = ref_ref_poc[rf.mfmv_ref[n] as usize][m];
                    let diff2 =
                        get_poc_diff(seq_hdr.order_hint_n_bits, rpoc as c_int, rrpoc as c_int);
                    // unsigned comparison also catches the < 0 case
                    rf.mfmv_ref2ref[n][m] = if diff2 as c_uint > 31 { 0 } else { diff2 };
                }
            }
        }
    }
    rf.use_ref_frame_mvs = (rf.n_mfmvs > 0) as c_int;

    Ok(())
}

pub(crate) fn rav1d_refmvs_init(rf: &mut RefMvsFrame) {
    rf.r = Default::default();
    rf.r_stride = 0;
    rf.rp_proj = Default::default();
    rf.rp_stride = 0;
}

pub(crate) fn rav1d_refmvs_clear(rf: &mut RefMvsFrame) {
    let _ = mem::take(&mut rf.r);
    let _ = mem::take(&mut rf.rp_proj);
}

unsafe extern "C" fn splat_mv_rust(
    rr: *mut *mut refmvs_block,
    rmv: *const refmvs_block,
    bx4: c_int,
    bw4: c_int,
    bh4: c_int,
    rr_len: usize,
) {
    let rmv = &*rmv;
    let [bx4, bw4, bh4] = [bx4, bw4, bh4].map(|it| it as usize);

    // Safety: `rr` and `rr_len` are the raw parts of a slice in [`Dav1dRefmvsDSPContext::splat_mv`].
    let rr = unsafe { std::slice::from_raw_parts_mut(rr, rr_len) };

    for r in &mut rr[..bh4] {
        std::slice::from_raw_parts_mut(*r, bx4 + bw4)[bx4..].fill_with(|| rmv.clone())
    }
}

#[inline(always)]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "asm"))]
fn refmvs_dsp_init_x86(c: &mut Rav1dRefmvsDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSE2) {
        return;
    }

    c.splat_mv = dav1d_splat_mv_sse2;

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    c.save_tmvs = dav1d_save_tmvs_ssse3;

    if !flags.contains(CpuFlags::SSE41) {
        return;
    }

    #[cfg(target_arch = "x86_64")]
    {
        c.load_tmvs = dav1d_load_tmvs_sse4;

        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        c.save_tmvs = dav1d_save_tmvs_avx2;
        c.splat_mv = dav1d_splat_mv_avx2;

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        c.save_tmvs = dav1d_save_tmvs_avx512icl;
        c.splat_mv = dav1d_splat_mv_avx512icl;
    }
}

#[inline(always)]
#[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), feature = "asm"))]
fn refmvs_dsp_init_arm(c: &mut Rav1dRefmvsDSPContext) {
    let flags = rav1d_get_cpu_flags();
    if flags.contains(CpuFlags::NEON) {
        c.save_tmvs = dav1d_save_tmvs_neon;
        c.splat_mv = dav1d_splat_mv_neon;
    }
}

#[cold]
pub(crate) fn rav1d_refmvs_dsp_init(c: &mut Rav1dRefmvsDSPContext) {
    c.load_tmvs = load_tmvs_c;
    c.save_tmvs = save_tmvs_c;
    c.splat_mv = splat_mv_rust;
    cfg_if! {
        if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "asm"))] {
            refmvs_dsp_init_x86(c);
        } else if #[cfg(all(any(target_arch = "arm", target_arch = "aarch64"), feature = "asm"))] {
            refmvs_dsp_init_arm(c);
        }
    }
}
