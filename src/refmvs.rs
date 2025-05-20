#![deny(unsafe_op_in_unsafe_fn)]

use crate::align::Align16;
use crate::align::AlignedVec64;
use crate::cpu::CpuFlags;
use crate::disjoint_mut::DisjointMut;
use crate::disjoint_mut::DisjointMutArcSlice;
use crate::disjoint_mut::DisjointMutGuard;
use crate::disjoint_mut::DisjointMutSlice;
use crate::env::fix_mv_precision;
use crate::env::get_gmv_2d;
use crate::env::get_poc_diff;
use crate::error::Rav1dResult;
use crate::ffi_safe::FFISafe;
use crate::include::common::intops::apply_sign;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::internal::Bxy;
use crate::intra_edge::EdgeFlags;
use crate::levels::BlockSize;
use crate::levels::Mv;
use crate::wrap_fn_ptr::wrap_fn_ptr;
use std::cmp;
use std::marker::PhantomData;
use std::mem;
use std::mem::MaybeUninit;
use std::ptr;
use std::slice;
use zerocopy::AsBytes;
use zerocopy::FromZeroes;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(C, packed)]
pub struct RefMvsTemporalBlock {
    pub mv: Mv,
    pub r#ref: i8,
}
const _: () = assert!(mem::size_of::<RefMvsTemporalBlock>() == 5);

#[derive(Clone, Copy, Eq, FromZeroes, AsBytes)]
// In C, this is packed and is 2 bytes.
// In Rust, being packed and aligned is tricky
#[repr(C, align(2))]
pub struct RefMvsRefPair {
    pub r#ref: [i8; 2],
}
const _: () = assert!(mem::size_of::<RefMvsRefPair>() == 2);

impl PartialEq for RefMvsRefPair {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        // `#[derive(PartialEq)]` compares per-field with `&&`,
        // which isn't optimized well and isn't coalesced into wider loads.
        // Comparing all of the bytes at once optimizes better with wider loads.
        // See <https://github.com/rust-lang/rust/issues/140167>.
        self.as_bytes() == other.as_bytes()
    }
}

impl From<[i8; 2]> for RefMvsRefPair {
    fn from(from: [i8; 2]) -> Self {
        RefMvsRefPair { r#ref: from }
    }
}

#[derive(Clone, Copy, Default, Eq, FromZeroes, AsBytes)]
#[repr(C)]
#[repr(align(4))] // Is a `union` with a `uint64_t` in C, so `align(8)`, but `align(8)` doesn't allow `align(4)` for `RefMvsBlock`.
pub struct RefMvsMvPair {
    pub mv: [Mv; 2],
}
const _: () = assert!(mem::size_of::<RefMvsMvPair>() == 8);

impl PartialEq for RefMvsMvPair {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        // `#[derive(PartialEq)]` compares per-field with `&&`,
        // which isn't optimized well and isn't coalesced into wider loads.
        // Comparing all of the bytes at once optimizes better with wider loads.
        // See <https://github.com/rust-lang/rust/issues/140167>.
        self.as_bytes() == other.as_bytes()
    }
}

#[derive(Clone, Copy, FromZeroes)]
// In C, this is packed and is 12 bytes.
// In Rust, being packed and aligned is tricky
#[repr(C, align(4))]
pub struct RefMvsBlock {
    pub mv: RefMvsMvPair,
    pub r#ref: RefMvsRefPair,
    pub bs: BlockSize,
    pub mf: u8,
}
const _: () = assert!(mem::size_of::<RefMvsBlock>() == 12);

#[repr(C)]
pub(crate) struct AsmRefMvsFrame<'a> {
    /// This lifetime is for the pointers in this [`refmvs_frame`],
    /// which are borrowed from the parent [`RefMvsFrame`].
    /// Since this is a transient type for asm calls, a lifetime is fine,
    /// and since this is a ZST, the layout stays the same.
    _lifetime: PhantomData<&'a ()>,
    /// A pointer to a [`refmvs_frame`] may be passed to a [`load_tmvs_fn`] function.
    /// However, the [`Self::frm_hdr`] pointer is not accessed in such a function (see [`load_tmvs_c`]).
    /// But we need to keep the layout the same, so we store a `*const ()` null ptr.
    _frm_hdr: *const (),
    pub iw4: i32,
    pub ih4: i32,
    pub iw8: i32,
    pub ih8: i32,
    pub sbsz: i32,
    pub use_ref_frame_mvs: i32,
    pub sign_bias: [u8; 7],
    pub mfmv_sign: [u8; 7],
    pub pocdiff: [i8; 7],
    pub mfmv_ref: [u8; 3],
    pub mfmv_ref2cur: [i32; 3],
    pub mfmv_ref2ref: [[i32; 7]; 3],
    pub n_mfmvs: i32,
    pub n_blocks: i32,
    pub rp: *mut RefMvsTemporalBlock,
    pub rp_ref: *const *mut RefMvsTemporalBlock,
    pub rp_proj: *mut RefMvsTemporalBlock,
    pub rp_stride: isize,
    pub r: *mut RefMvsBlock,
    pub n_tile_threads: i32,
    pub n_frame_threads: i32,
}

const R_PAD: usize = 1; // number of elements added to RefMvsFrame.r to avoid overread

#[derive(Default)]
pub struct RefMvsFrame {
    pub iw4: i32,
    pub ih4: i32,
    pub iw8: i32,
    pub ih8: i32,
    pub sbsz: i32,
    pub use_ref_frame_mvs: i32,
    pub sign_bias: [u8; 7],
    pub mfmv_sign: [u8; 7],
    pub pocdiff: [i8; 7],
    pub mfmv_ref: [u8; 3],
    pub mfmv_ref2cur: [i32; 3],
    pub mfmv_ref2ref: [[i32; 7]; 3],
    pub n_mfmvs: i32,
    pub n_blocks: u32,
    // TODO: The C code uses a single buffer to store `rp_proj` and `r` to minimize
    // the number of allocated buffers.
    pub rp_proj: DisjointMut<AlignedVec64<RefMvsTemporalBlock>>,
    pub rp_stride: u32,
    pub r: DisjointMut<AlignedVec64<RefMvsBlock>>,
    pub n_tile_threads: u32,
    pub n_frame_threads: u32,
}

#[derive(Default)]
#[repr(C)]
pub struct RefmvsTileRange {
    pub start: i32,
    pub end: i32,
}

pub struct RefmvsTile {
    /// Unique indices into [`RefMvsFrame::r`].
    /// Out of bounds indices correspond to null pointers.
    ///
    /// # Safety
    ///
    /// These indices, when in bounds, are unique.
    pub r: [usize; 37],

    /// Index into [`RefMvsFrame::rp_proj`].
    pub rp_proj: usize,

    pub tile_col: RefmvsTileRange,
    pub tile_row: RefmvsTileRange,
}

impl Default for RefmvsTile {
    fn default() -> Self {
        Self {
            r: [Default::default(); 37],
            rp_proj: Default::default(),
            tile_col: Default::default(),
            tile_row: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct RefMvsCandidate {
    pub mv: RefMvsMvPair,
    pub weight: i32,
}

wrap_fn_ptr!(pub(crate) unsafe extern "C" fn load_tmvs(
    rf: &AsmRefMvsFrame,
    tile_row_idx: i32,
    col_start8: i32,
    col_end8: i32,
    row_start8: i32,
    row_end8: i32,
    _rp_proj: *const FFISafe<DisjointMut<AlignedVec64<RefMvsTemporalBlock>>>,
    _rp_ref: *const FFISafe<[Option<DisjointMutArcSlice<RefMvsTemporalBlock>>; 7]>,
) -> ());

impl load_tmvs::Fn {
    pub fn call(
        &self,
        rf: &RefMvsFrame,
        rp: &Option<DisjointMutArcSlice<RefMvsTemporalBlock>>,
        rp_ref: &[Option<DisjointMutArcSlice<RefMvsTemporalBlock>>; 7],
        tile_row_idx: i32,
        col_start8: i32,
        col_end8: i32,
        row_start8: i32,
        row_end8: i32,
    ) {
        let RefMvsFrame {
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
            n_blocks,
            ref rp_proj,
            rp_stride,
            ref r,
            n_tile_threads,
            n_frame_threads,
        } = *rf;
        fn mvs_to_dav1d(
            mvs: &Option<DisjointMutArcSlice<RefMvsTemporalBlock>>,
        ) -> *mut RefMvsTemporalBlock {
            mvs.as_ref()
                .map(|rp| rp.inner.as_mut_ptr())
                .unwrap_or_else(ptr::null_mut)
        }
        let rp_ref_dav1d = rp_ref.each_ref().map(mvs_to_dav1d);
        let rf_dav1d = AsmRefMvsFrame {
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
            n_blocks: n_blocks as _,
            rp: mvs_to_dav1d(rp),
            rp_ref: rp_ref_dav1d.as_ptr(),
            rp_proj: rp_proj.as_mut_ptr(),
            rp_stride: rp_stride as _,
            r: r.as_mut_ptr(),
            n_tile_threads: n_tile_threads as _,
            n_frame_threads: n_frame_threads as _,
        };

        let rp_proj = FFISafe::new(&rf.rp_proj);
        let rp_ref = FFISafe::new(rp_ref);
        let rf = &rf_dav1d;
        // SAFETY: Assembly call. Arguments are safe Rust references converted to
        // pointers for use in assembly. For the Rust fallback function the extra args
        // `rp_proj` and `rp_ref` are passed to allow for disjointedness checking.
        unsafe {
            self.get()(
                rf,
                tile_row_idx,
                col_start8,
                col_end8,
                row_start8,
                row_end8,
                rp_proj,
                rp_ref,
            )
        };
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn save_tmvs(
    rp_ptr: *mut RefMvsTemporalBlock,
    stride: isize,
    rr: &[*const RefMvsBlock; 31],
    ref_sign: &[u8; 7],
    col_end8: i32,
    row_end8: i32,
    col_start8: i32,
    row_start8: i32,
    _r: *const FFISafe<DisjointMut<AlignedVec64<RefMvsBlock>>>,
    _ri: &[usize; 31],
    _rp: *const FFISafe<DisjointMutArcSlice<RefMvsTemporalBlock>>,
) -> ());

impl save_tmvs::Fn {
    // cache the current tile/sbrow (or frame/sbrow)'s projectable motion vectors
    // into buffers for use in future frame's temporal MV prediction
    pub fn call(
        &self,
        rt: &RefmvsTile,
        rf: &RefMvsFrame,
        rp: &Option<DisjointMutArcSlice<RefMvsTemporalBlock>>,
        col_start8: i32,
        col_end8: i32,
        row_start8: i32,
        row_end8: i32,
    ) {
        assert!(row_start8 >= 0);
        assert!((row_end8 - row_start8) as u32 <= 16);

        let rp = &*rp.as_ref().unwrap();

        let row_end8 = cmp::min(row_end8, rf.ih8);
        let col_end8 = cmp::min(col_end8, rf.iw8);
        let stride = rf.rp_stride as usize;
        let ref_sign = &rf.mfmv_sign;
        let ri = <&[_; 31]>::try_from(&rt.r[6..]).unwrap();

        // SAFETY: Note that for asm calls, disjointedness is unchecked here,
        // even with `#[cfg(debug_assertions)]`.  This is because the disjointedness
        // is more fine-grained than the pointers passed to asm.
        // For the Rust fallback fn, the extra args `&rf.r` and `ri`
        // are passed to allow for disjointedness checking.
        let rr = &ri.map(|ri| {
            if ri > rf.r.len() - R_PAD {
                return ptr::null();
            }

            const _: () = assert!(mem::size_of::<RefMvsBlock>() * (1 + R_PAD) > 16);
            // SAFETY: `.add` is in-bounds; checked above.
            // Also note that asm may read 12-byte `refmvs_block`s in 16-byte chunks.
            // This is safe because we allocate `rf.r` with an extra `R_PAD` (1) elements.
            // These ptrs are only read, so these overlapping reads are safe
            // (only read is only checked in the fallback Rust `fn`).
            // Furthermore, this is provenance safe because
            // we derive the ptrs from `rf.r.as_mut_ptr()`,
            // as opposed to materializing intermediate references.
            unsafe { rf.r.as_mut_ptr().cast_const().add(ri) }
        });

        // SAFETY: Note that for asm calls, disjointedness is unchecked here,
        // even with `#[cfg(debug_assertions)]`. This is because the disjointedness
        // is more fine-grained than the pointers passed to asm.
        // For the Rust fallback fn, the extra arg `rp`
        // is passed to allow for disjointedness checking.
        let rp_offset = row_start8 as usize * stride;
        assert!(rp_offset <= rp.inner.len());
        // SAFETY: `rp_offset` was just bounds checked.
        let rp_ptr = unsafe { rp.inner.as_mut_ptr().add(rp_offset) };
        let stride = stride as isize;
        let r = FFISafe::new(&rf.r);
        let rp = FFISafe::new(rp);
        // SAFETY: Assembly call. Arguments are safe Rust references converted to
        // pointers for use in assembly.
        unsafe {
            self.get()(
                rp_ptr, stride, rr, ref_sign, col_end8, row_end8, col_start8, row_start8, r, ri, rp,
            )
        };
    }
}

wrap_fn_ptr!(pub unsafe extern "C" fn splat_mv(
    rr: *mut *mut RefMvsBlock,
    rmv: &Align16<RefMvsBlock>,
    bx4: i32,
    bw4: i32,
    bh4: i32,
) -> ());

impl splat_mv::Fn {
    pub fn call(
        &self,
        rf: &RefMvsFrame,
        rt: &RefmvsTile,
        rmv: &Align16<RefMvsBlock>,
        b4: Bxy,
        bw4: usize,
        bh4: usize,
    ) {
        let offset = (b4.y as usize & 31) + 5;
        let len = bh4;
        let bx4 = b4.x as usize;

        type Guard<'a> = DisjointMutGuard<'a, AlignedVec64<RefMvsBlock>, [RefMvsBlock]>;

        let mut r_guards = [const { MaybeUninit::uninit() }; 37];
        let mut r_ptrs = [MaybeUninit::uninit(); 37];

        let r_indices = &rt.r[offset..][..len];
        // SAFETY: `r_guards[i]` will be initialized if `r_ptrs[i]` is non-null.
        let r_guards = &mut r_guards[offset..][..len];
        // SAFETY: This `r_ptrs` slice will be fully initialized.
        let r_ptrs = &mut r_ptrs[offset..][..len];

        for i in 0..len {
            let ri = r_indices[i];
            if ri < rf.r.len() - R_PAD {
                // This is the range that will actually be accessed,
                // but `splat_mv` expects a pointer offset `bx4` backwards.
                let guard = rf.r.index_mut((ri + bx4.., ..bw4));
                r_guards[i].write(guard);
                // SAFETY: We just initialized it directly above.
                let guard = unsafe { r_guards[i].assume_init_mut() };
                // SAFETY: The above `index_mut` starts at `ri + bx4`, so we can safely index `bx4` backwards.
                let ptr = unsafe { guard.as_mut_ptr().sub(bx4) };
                r_ptrs[i].write(ptr);
            } else {
                r_ptrs[i].write(ptr::null_mut());
            }
        }

        /// # Safety
        ///
        /// `slice` must be initialized.
        // TODO use `MaybeUninit::slice_assume_init_mut` once `#![feature(maybe_uninit_slice)]` is stabilized.
        unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
            // SAFETY: `slice` is already initialized and `MaybeUninit` is `#[repr(transparent)]`.
            unsafe { &mut *(ptr::from_mut(slice) as *mut [T]) }
        }

        // SAFETY: The `r_ptrs` slice is fully initialized by the above loop.
        let r_ptrs = unsafe { slice_assume_init_mut(r_ptrs) };

        let rr = r_ptrs.as_mut_ptr();
        let bx4 = b4.x as _;
        let bw4 = bw4 as _;
        let bh4 = bh4 as _;

        // SAFETY: Unsafe asm call. `rr` is `bh4` elements long,
        // and each ptr in `rr` points to at least `bx4 + bw4` elements,
        // which is what will be accessed in `splat_mv`.
        unsafe { self.get()(rr, rmv, bx4, bw4, bh4) };

        if mem::needs_drop::<Guard>() {
            for i in 0..len {
                let ptr = r_ptrs[i];
                if ptr.is_null() {
                    continue;
                }
                // SAFETY: `r_guards[i]` is initialized iff `r_ptrs[i]` is non-null.
                unsafe { r_guards[i].assume_init_drop() };
            }
        }
    }
}

pub struct Rav1dRefmvsDSPContext {
    pub load_tmvs: load_tmvs::Fn,
    pub save_tmvs: save_tmvs::Fn,
    pub splat_mv: splat_mv::Fn,
}

fn add_spatial_candidate(
    mvstack: &mut [RefMvsCandidate],
    cnt: &mut usize,
    weight: i32,
    b: RefMvsBlock,
    r#ref: RefMvsRefPair,
    gmv: &[Mv; 2],
    have_newmv_match: &mut i32,
    have_refmv_match: &mut i32,
) {
    if b.mv.mv[0].is_invalid() {
        // intra block, no intrabc
        return;
    }

    let mf_odd = b.mf & 1 != 0;
    if r#ref.r#ref[1] == -1 {
        for n in 0..2 {
            if b.r#ref.r#ref[n] == r#ref.r#ref[0] {
                let cand_mv = if mf_odd && gmv[0] != Mv::INVALID {
                    gmv[0]
                } else {
                    b.mv.mv[n]
                };

                *have_refmv_match = 1;
                *have_newmv_match |= b.mf as i32 >> 1;

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
        let cand_mv = RefMvsMvPair {
            mv: [
                if mf_odd && gmv[0] != Mv::INVALID {
                    gmv[0]
                } else {
                    b.mv.mv[0]
                },
                if mf_odd && gmv[1] != Mv::INVALID {
                    gmv[1]
                } else {
                    b.mv.mv[1]
                },
            ],
        };

        *have_refmv_match = 1;
        *have_newmv_match |= b.mf as i32 >> 1;

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
    mvstack: &mut [RefMvsCandidate],
    cnt: &mut usize,
    r#ref: RefMvsRefPair,
    gmv: &[Mv; 2],
    r: &DisjointMut<AlignedVec64<RefMvsBlock>>,
    b_offset: usize,
    bw4: i32,
    w4: i32,
    max_rows: i32,
    step: i32,
    have_newmv_match: &mut i32,
    have_refmv_match: &mut i32,
) -> i32 {
    let mut cand_b = *r.index(b_offset);
    let first_cand_bs = cand_b.bs;
    let first_cand_b_dim = first_cand_bs.dimensions();
    let mut cand_bw4 = first_cand_b_dim[0] as i32;
    let mut len = cmp::max(step, cmp::min(bw4, cand_bw4));

    if bw4 <= cand_bw4 {
        // FIXME weight can be higher for odd blocks (bx4 & 1), but then the
        // position of the first block has to be odd already, i.e. not just
        // for row_offset=-3/-5
        // FIXME why can this not be cand_bw4?
        let weight = if bw4 == 1 {
            2
        } else {
            cmp::max(2, cmp::min(2 * max_rows, first_cand_b_dim[1] as i32))
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
        cand_b = *r.index(b_offset + x as usize);
        cand_bw4 = cand_b.bs.dimensions()[0] as i32;
        assert!(cand_bw4 < bw4);
        len = cmp::max(step, cand_bw4);
    }
}

fn scan_col(
    mvstack: &mut [RefMvsCandidate],
    cnt: &mut usize,
    r#ref: RefMvsRefPair,
    gmv: &[Mv; 2],
    r: &DisjointMut<AlignedVec64<RefMvsBlock>>,
    b: &[usize],
    bh4: i32,
    h4: i32,
    bx4: i32,
    max_cols: i32,
    step: i32,
    have_newmv_match: &mut i32,
    have_refmv_match: &mut i32,
) -> i32 {
    let mut cand_b = *r.index(b[0] + bx4 as usize);
    let first_cand_bs = cand_b.bs;
    let first_cand_b_dim = first_cand_bs.dimensions();
    let mut cand_bh4 = first_cand_b_dim[1] as i32;
    let mut len = cmp::max(step, cmp::min(bh4, cand_bh4));

    if bh4 <= cand_bh4 {
        // FIXME weight can be higher for odd blocks (by4 & 1), but then the
        // position of the first block has to be odd already, i.e. not just
        // for col_offset=-3/-5
        // FIXME why can this not be cand_bh4?
        let weight = if bh4 == 1 {
            2
        } else {
            cmp::max(2, cmp::min(2 * max_cols, first_cand_b_dim[0] as i32))
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
        cand_b = *r.index(b[y as usize] + bx4 as usize);
        cand_bh4 = cand_b.bs.dimensions()[1] as i32;
        assert!(cand_bh4 < bh4);
        len = cmp::max(step, cand_bh4);
    }
}

#[inline]
fn mv_projection(mv: Mv, num: i32, den: i32) -> Mv {
    static div_mult: [u16; 32] = [
        0, 16384, 8192, 5461, 4096, 3276, 2730, 2340, 2048, 1820, 1638, 1489, 1365, 1260, 1170,
        1092, 1024, 963, 910, 862, 819, 780, 744, 712, 682, 655, 630, 606, 585, 564, 546, 528,
    ];
    assert!(den > 0 && den < 32);
    assert!(num > -32 && num < 32);
    let frac = num * div_mult[den as usize] as i32;
    let y = mv.y as i32 * frac;
    let x = mv.x as i32 * frac;
    // Round and clip according to AV1 spec section 7.9.3
    let max = (1 << 14) - 1;
    return Mv {
        y: iclip(y + 8192 + (y >> 31) >> 14, -max, max) as i16,
        x: iclip(x + 8192 + (x >> 31) >> 14, -max, max) as i16,
    };
}

fn add_temporal_candidate(
    rf: &RefMvsFrame,
    mvstack: &mut [RefMvsCandidate],
    cnt: &mut usize,
    rb: RefMvsTemporalBlock,
    r#ref: RefMvsRefPair,
    globalmv: Option<(&mut i32, &[Mv; 2])>,
    frame_hdr: &Rav1dFrameHeader,
) {
    if rb.mv.is_invalid() {
        return;
    }

    let mut mv = mv_projection(
        rb.mv,
        rf.pocdiff[r#ref.r#ref[0] as usize - 1] as i32,
        rb.r#ref as i32,
    );
    fix_mv_precision(frame_hdr, &mut mv);

    let last = *cnt;
    if r#ref.r#ref[1] == -1 {
        if let Some((globalmv_ctx, gmv)) = globalmv {
            *globalmv_ctx = ((mv.x - gmv[0].x).abs() | (mv.y - gmv[0].y).abs() >= 16) as i32;
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
        let mut mvp = RefMvsMvPair {
            mv: [
                mv,
                mv_projection(
                    rb.mv,
                    rf.pocdiff[r#ref.r#ref[1] as usize - 1] as i32,
                    rb.r#ref as i32,
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
    same: &mut [RefMvsCandidate],
    same_count: &mut [usize; 4],
    cand_b: RefMvsBlock,
    sign0: u8,
    sign1: u8,
    r#ref: RefMvsRefPair,
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
    mvstack: &mut [RefMvsCandidate; 8],
    cnt: &mut usize,
    cand_b: RefMvsBlock,
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
    rt: &RefmvsTile,
    rf: &RefMvsFrame,
    mvstack: &mut [RefMvsCandidate; 8],
    cnt: &mut usize,
    ctx: &mut i32,
    r#ref: RefMvsRefPair,
    bs: BlockSize,
    edge_flags: EdgeFlags,
    by4: i32,
    bx4: i32,
    frame_hdr: &Rav1dFrameHeader,
) {
    let b_dim = bs.dimensions();
    let bw4 = b_dim[0] as i32;
    let w4 = cmp::min(cmp::min(bw4, 16), rt.tile_col.end - bx4);
    let bh4 = b_dim[1] as i32;
    let h4 = cmp::min(cmp::min(bh4, 16), rt.tile_row.end - by4);
    let mut gmv = [Mv::default(); 2];
    let mut tgmv = [Mv::default(); 2];

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
            Mv::INVALID
        };
    } else {
        tgmv[0] = Mv::ZERO;
        gmv[0] = Mv::INVALID;
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
            Mv::INVALID
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
        max_rows = cmp::min(by4 - rt.tile_row.start + 1 >> 1, 2 + (bh4 > 1) as i32) as u32;
        let i = rt.r[(by4 as usize & 31) + 5 - 1] + bx4 as usize;
        // We can't offset below 0.
        b_top_offset = match i {
            0 => 0,
            _ => 1,
        };
        b_top = i - b_top_offset;
        n_rows = scan_row(
            mvstack,
            cnt,
            r#ref,
            &gmv,
            &rf.r,
            b_top + b_top_offset,
            bw4,
            w4,
            max_rows as i32,
            if bw4 >= 16 { 4 } else { 1 },
            &mut have_newmv,
            &mut have_row_mvs,
        ) as u32;
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
        max_cols = cmp::min(bx4 - rt.tile_col.start + 1 >> 1, 2 + (bw4 > 1) as i32) as u32;
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
            max_cols as i32,
            if bh4 >= 16 { 4 } else { 1 },
            &mut have_newmv,
            &mut have_col_mvs,
        ) as u32;
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
            *rf.r.index(b_top + bw4 as usize + b_top_offset),
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
    let mut globalmv_ctx = frame_hdr.use_ref_frame_mvs as i32;
    if rf.use_ref_frame_mvs != 0 {
        let stride = rf.rp_stride as usize;
        let by8 = by4 >> 1;
        let bx8 = bx4 >> 1;
        let rbi = rt.rp_proj + (by8 as usize & 15) * stride + bx8 as usize;
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
                    *rf.rp_proj.index(rbi + y * stride + x),
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
            let rbi = rbi + bh8 as usize * stride;
            let has_bottom = by8 + bh8 < cmp::min(rt.tile_row.end >> 1, (by8 & !7) + 8);
            if has_bottom && bx8 - 1 >= cmp::max(rt.tile_col.start >> 1, bx8 & !7) {
                let rb = *rf.rp_proj.index(rbi - 1);
                add_temporal_candidate(rf, mvstack, cnt, rb, r#ref, None, frame_hdr);
            }
            if bx8 + bw8 < cmp::min(rt.tile_col.end >> 1, (bx8 & !7) + 8) {
                if has_bottom {
                    let rb = *rf.rp_proj.index(rbi + bw8 as usize);
                    add_temporal_candidate(rf, mvstack, cnt, rb, r#ref, None, frame_hdr);
                }
                if (by8 + bh8 - 1) < cmp::min(rt.tile_row.end >> 1, (by8 & !7) + 8) {
                    let rb = *rf.rp_proj.index(rbi + bw8 as usize - stride);
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
            *rf.r.index(b_top + b_top_offset - 1),
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
            let ri = rt.r[(((by4 & 31) - 2 * n as i32 + 1 | 1) + 5) as usize] + (bx4 as usize | 1);
            n_rows = n_rows.wrapping_add(scan_row(
                mvstack,
                cnt,
                r#ref,
                &gmv,
                &rf.r,
                ri,
                bw4,
                w4,
                (1 + max_rows - n) as _,
                if bw4 >= 16 { 4 } else { 2 },
                &mut have_dummy_newmv_match,
                &mut have_row_mvs,
            ) as u32);
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
                bx4 - n as i32 * 2 + 1 | 1,
                (1 + max_cols - n) as _,
                if bh4 >= 16 { 4 } else { 2 },
                &mut have_dummy_newmv_match,
                &mut have_col_mvs,
            ) as u32);
        }
    }
    assert!(*cnt <= 8);

    let ref_match_count = have_col_mvs + have_row_mvs;

    // context build-up
    let (refmv_ctx, newmv_ctx) = match nearest_match {
        0 => (cmp::min(2, ref_match_count), (ref_match_count > 0) as i32),
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
                    let cand_b = *rf.r.index(b_top + x as usize + b_top_offset);
                    add_compound_extended_candidate(
                        same,
                        &mut same_count,
                        cand_b,
                        sign0,
                        sign1,
                        r#ref,
                        &rf.sign_bias,
                    );
                    x += cand_b.bs.dimensions()[0] as i32;
                }
            }

            // non-self references in left
            if n_cols != !0 {
                let mut y = 0;
                while y < sz4 {
                    let cand_b = *rf.r.index(b_left[y as usize] + bx4 as usize - 1);
                    add_compound_extended_candidate(
                        same,
                        &mut same_count,
                        cand_b,
                        sign0,
                        sign1,
                        r#ref,
                        &rf.sign_bias,
                    );
                    y += cand_b.bs.dimensions()[1] as i32;
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
            mv[0].x = iclip(mv[0].x as i32, left, right) as i16;
            mv[0].y = iclip(mv[0].y as i32, top, bottom) as i16;
            mv[1].x = iclip(mv[1].x as i32, left, right) as i16;
            mv[1].y = iclip(mv[1].y as i32, top, bottom) as i16;
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
                let cand_b = *rf.r.index(b_top + x as usize + b_top_offset);
                add_single_extended_candidate(mvstack, cnt, cand_b, sign, &rf.sign_bias);
                x += cand_b.bs.dimensions()[0] as i32;
            }
        }

        // non-self references in left
        if n_cols != !0 {
            let mut y = 0;
            while y < sz4 && *cnt < 2 {
                let cand_b = *rf.r.index(b_left[y as usize] + bx4 as usize - 1);
                add_single_extended_candidate(mvstack, cnt, cand_b, sign, &rf.sign_bias);
                y += cand_b.bs.dimensions()[1] as i32;
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
            mv[0].x = iclip(mv[0].x as i32, left, right) as i16;
            mv[0].y = iclip(mv[0].y as i32, top, bottom) as i16;
        }
    }

    // Need to use `min` so we don't get a backwards range,
    // which will fail on slicing.
    for cand in &mut mvstack[cmp::min(*cnt, 2)..2] {
        cand.mv.mv[0] = tgmv[0];
    }

    *ctx = refmv_ctx << 4 | globalmv_ctx << 3 | newmv_ctx;
}

pub(crate) fn rav1d_refmvs_tile_sbrow_init(
    rf: &RefMvsFrame,
    tile_col_start4: i32,
    tile_col_end4: i32,
    tile_row_start4: i32,
    tile_row_end4: i32,
    sby: i32,
    mut tile_row_idx: i32,
    pass: i32,
) -> RefmvsTile {
    if rf.n_tile_threads == 1 {
        tile_row_idx = 0;
    }
    let rp_stride = rf.rp_stride as usize;
    let r_stride = rp_stride * 2;
    let rp_proj = 16 * rp_stride * tile_row_idx as usize;
    let pass_off = if rf.n_frame_threads > 1 && pass == 2 {
        35 * 2 * rf.n_blocks as usize
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

    RefmvsTile {
        r: rr,
        rp_proj,
        tile_col: RefmvsTileRange {
            start: tile_col_start4,
            end: cmp::min(tile_col_end4, rf.iw4),
        },
        tile_row: RefmvsTileRange {
            start: tile_row_start4,
            end: cmp::min(tile_row_end4, rf.ih4),
        },
    }
}

/// # Safety
///
/// Must be called by [`load_tmvs::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn load_tmvs_c(
    rf: &AsmRefMvsFrame,
    tile_row_idx: i32,
    col_start8: i32,
    col_end8: i32,
    row_start8: i32,
    row_end8: i32,
    rp_proj: *const FFISafe<DisjointMut<AlignedVec64<RefMvsTemporalBlock>>>,
    rp_ref: *const FFISafe<[Option<DisjointMutArcSlice<RefMvsTemporalBlock>>; 7]>,
) {
    // SAFETY: Was passed as `FFISafe::new(_)` in `load_tmvs::Fn::call`.
    let rp_proj = unsafe { FFISafe::get(rp_proj) };
    // SAFETY: Was passed as `FFISafe::new(_)` in `load_tmvs::Fn::call`.
    let rp_ref = unsafe { FFISafe::get(rp_ref) };
    load_tmvs_rust(
        rf,
        tile_row_idx,
        col_start8,
        col_end8,
        row_start8,
        row_end8,
        rp_proj,
        rp_ref,
    )
}

fn load_tmvs_rust(
    rf: &AsmRefMvsFrame,
    mut tile_row_idx: i32,
    col_start8: i32,
    col_end8: i32,
    row_start8: i32,
    mut row_end8: i32,
    rp_proj: &DisjointMut<AlignedVec64<RefMvsTemporalBlock>>,
    rp_ref: &[Option<DisjointMutArcSlice<RefMvsTemporalBlock>>; 7],
) {
    if rf.n_tile_threads == 1 {
        tile_row_idx = 0;
    }
    assert!(row_start8 >= 0);
    assert!((row_end8 - row_start8) as u32 <= 16);
    row_end8 = cmp::min(row_end8, rf.ih8);
    let col_start8i = cmp::max(col_start8 - 8, 0);
    let col_end8i = cmp::min(col_end8 + 8, rf.iw8);
    let stride = rf.rp_stride as usize;
    let rp_proj_offset = 16 * stride * tile_row_idx as usize;
    for y in row_start8..row_end8 {
        let offset = rp_proj_offset + (y & 15) as usize * stride;
        for rp_proj in
            &mut *rp_proj.index_mut(offset + col_start8 as usize..offset + col_end8 as usize)
        {
            rp_proj.mv = Mv::INVALID;
        }
    }
    for n in 0..rf.n_mfmvs {
        let ref2cur = rf.mfmv_ref2cur[n as usize];
        if ref2cur == i32::MIN {
            continue;
        }
        let r#ref = rf.mfmv_ref[n as usize];
        let ref_sign = r#ref as i32 - 4;
        let r = &*rp_ref[r#ref as usize].as_ref().unwrap().inner;
        for y in row_start8..row_end8 {
            let y_sb_align = y & !7;
            let y_proj_start = cmp::max(y_sb_align, row_start8);
            let y_proj_end = cmp::min(y_sb_align + 8, row_end8);
            let mut x = col_start8i;
            while x < col_end8i {
                let mut rbi = y as usize * stride + x as usize;
                let mut rb = *r.index(rbi);
                if rb.r#ref == 0 {
                    x += 1;
                    continue;
                }
                let ref2ref = rf.mfmv_ref2ref[n as usize][(rb.r#ref - 1) as usize];
                if ref2ref == 0 {
                    x += 1;
                    continue;
                }
                let offset = mv_projection(rb.mv, ref2cur, ref2ref);
                let mut pos_x =
                    x + apply_sign((offset.x as i32).abs() >> 6, offset.x as i32 ^ ref_sign);
                let pos_y =
                    y + apply_sign((offset.y as i32).abs() >> 6, offset.y as i32 ^ ref_sign);
                if pos_y >= y_proj_start && pos_y < y_proj_end {
                    let pos = (pos_y & 15) as usize * stride;
                    loop {
                        let x_sb_align = x & !7;
                        if pos_x >= cmp::max(x_sb_align - 8, col_start8)
                            && pos_x < cmp::min(x_sb_align + 16, col_end8)
                        {
                            *rp_proj.index_mut(
                                rp_proj_offset + (pos as isize + pos_x as isize) as usize,
                            ) = RefMvsTemporalBlock {
                                mv: rb.mv,
                                r#ref: ref2ref as i8,
                            };
                        }
                        x += 1;
                        if x >= col_end8i {
                            break;
                        }
                        let prev_rb = rb;
                        rbi += 1;
                        rb = *r.index(rbi);
                        if rb != prev_rb {
                            break;
                        }
                        pos_x += 1;
                    }
                } else {
                    loop {
                        x += 1;
                        if x >= col_end8i {
                            break;
                        }
                        let prev_rb = rb;
                        rbi += 1;
                        rb = *r.index(rbi);
                        if rb != prev_rb {
                            break;
                        }
                    }
                }
            }
        }
    }
}

/// # Safety
///
/// Must be called by [`save_tmvs::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn save_tmvs_c(
    _rp: *mut RefMvsTemporalBlock,
    stride: isize,
    _rr: &[*const RefMvsBlock; 31],
    ref_sign: &[u8; 7],
    col_end8: i32,
    row_end8: i32,
    col_start8: i32,
    row_start8: i32,
    r: *const FFISafe<DisjointMut<AlignedVec64<RefMvsBlock>>>,
    ri: &[usize; 31],
    rp: *const FFISafe<DisjointMutArcSlice<RefMvsTemporalBlock>>,
) {
    let stride = stride as usize;
    let [col_end8, row_end8, col_start8, row_start8] =
        [col_end8, row_end8, col_start8, row_start8].map(|it| it as usize);
    // SAFETY: Was passed as `FFISafe::new(_)` in `save_tmvs::Fn::call`.
    let r = unsafe { FFISafe::get(r) };
    // SAFETY: Was passed as `FFISafe::new(_)` in `save_tmvs::Fn::call`.
    let rp = unsafe { FFISafe::get(rp) };
    let rp = &*rp.inner;
    save_tmvs_rust(
        stride, ref_sign, col_end8, row_end8, col_start8, row_start8, r, ri, rp,
    )
}

fn save_tmvs_rust(
    stride: usize,
    ref_sign: &[u8; 7],
    col_end8: usize,
    row_end8: usize,
    col_start8: usize,
    row_start8: usize,
    r: &DisjointMut<AlignedVec64<RefMvsBlock>>,
    ri: &[usize; 31],
    rp: &DisjointMutSlice<RefMvsTemporalBlock>,
) {
    for y in row_start8..row_end8 {
        let b = ri[(y & 15) * 2];
        let mut x = col_start8;
        while x < col_end8 {
            let cand_b = *r.index(b + x * 2 + 1);
            let bw8 = cand_b.bs.dimensions()[0] + 1 >> 1;
            let block = |i: usize| {
                let mv = cand_b.mv.mv[i];
                let r#ref = cand_b.r#ref.r#ref[i];
                if r#ref > 0 && ref_sign[r#ref as usize - 1] != 0 && mv.y.abs() | mv.x.abs() < 4096
                {
                    Some(RefMvsTemporalBlock { mv, r#ref })
                } else {
                    None
                }
            };
            let block = block(1).or_else(|| block(0)).unwrap_or_default();
            let offset = y * stride + x;
            rp.index_mut(offset..offset + bw8 as usize).fill(block);
            x += bw8 as usize;
        }
    }
}

pub(crate) fn rav1d_refmvs_init_frame(
    rf: &mut RefMvsFrame,
    seq_hdr: &Rav1dSequenceHeader,
    frm_hdr: &Rav1dFrameHeader,
    ref_poc: &[u32; 7],
    ref_ref_poc: &[[u32; 7]; 7],
    rp_ref: &[Option<DisjointMutArcSlice<RefMvsTemporalBlock>>; 7],
    n_tile_threads: u32,
    n_frame_threads: u32,
) -> Rav1dResult {
    let rp_stride = ((frm_hdr.size.width[0] + 127 & !127) >> 3) as u32;
    let n_tile_rows = if n_tile_threads > 1 {
        frm_hdr.tiling.rows as u32
    } else {
        1
    };
    let n_blocks = rp_stride * n_tile_rows;

    rf.sbsz = 16 << seq_hdr.sb128;
    rf.iw8 = frm_hdr.size.width[0] + 7 >> 3;
    rf.ih8 = frm_hdr.size.height + 7 >> 3;
    rf.iw4 = rf.iw8 << 1;
    rf.ih4 = rf.ih8 << 1;
    rf.rp_stride = rp_stride;
    rf.n_tile_threads = n_tile_threads;
    rf.n_frame_threads = n_frame_threads;

    if n_blocks != rf.n_blocks {
        // `mem::size_of::<RefMvsBlock>() == 12`,
        // but it's accessed using 16-byte unaligned loads in save_tmvs() asm,
        // so add `R_PAD` elements to avoid buffer overreads.
        let r_sz = 35 * 2 * n_blocks as usize * (1 + (n_frame_threads > 1) as usize) + R_PAD;
        let rp_proj_sz = 16 * n_blocks as usize;
        // TODO fallible allocation
        rf.r.resize(r_sz, FromZeroes::new_zeroed());
        rf.rp_proj.resize(rp_proj_sz, Default::default());
        rf.n_blocks = n_blocks;
    }

    let poc = frm_hdr.frame_offset as u32;
    for i in 0..7 {
        let poc_diff = get_poc_diff(seq_hdr.order_hint_n_bits, ref_poc[i] as i32, poc as i32);
        rf.sign_bias[i] = (poc_diff > 0) as u8;
        rf.mfmv_sign[i] = (poc_diff < 0) as u8;
        rf.pocdiff[i] = iclip(
            get_poc_diff(seq_hdr.order_hint_n_bits, poc as i32, ref_poc[i] as i32),
            -31,
            31,
        ) as i8;
    }

    // temporal MV setup
    rf.n_mfmvs = 0;
    if frm_hdr.use_ref_frame_mvs != 0 && seq_hdr.order_hint_n_bits != 0 {
        let mut total = 2;
        if rp_ref[0].is_some() && ref_ref_poc[0][6] != ref_poc[3] {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 0; // last
            rf.n_mfmvs += 1;
            total = 3;
        }
        if rp_ref[4].is_some()
            && get_poc_diff(
                seq_hdr.order_hint_n_bits,
                ref_poc[4] as i32,
                frm_hdr.frame_offset as i32,
            ) > 0
        {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 4; // bwd
            rf.n_mfmvs += 1;
        }
        if rp_ref[5].is_some()
            && get_poc_diff(
                seq_hdr.order_hint_n_bits,
                ref_poc[5] as i32,
                frm_hdr.frame_offset as i32,
            ) > 0
        {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 5; // altref2
            rf.n_mfmvs += 1;
        }
        if rf.n_mfmvs < total
            && rp_ref[6].is_some()
            && get_poc_diff(
                seq_hdr.order_hint_n_bits,
                ref_poc[6] as i32,
                frm_hdr.frame_offset as i32,
            ) > 0
        {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 6; // altref
            rf.n_mfmvs += 1;
        }
        if rf.n_mfmvs < total && rp_ref[1].is_some() {
            rf.mfmv_ref[rf.n_mfmvs as usize] = 1; // last2
            rf.n_mfmvs += 1;
        }

        for n in 0..rf.n_mfmvs as usize {
            let rpoc = ref_poc[rf.mfmv_ref[n] as usize];
            let diff1 = get_poc_diff(
                seq_hdr.order_hint_n_bits,
                rpoc as i32,
                frm_hdr.frame_offset as i32,
            );
            if diff1.abs() > 31 {
                rf.mfmv_ref2cur[n] = i32::MIN;
            } else {
                rf.mfmv_ref2cur[n] = if rf.mfmv_ref[n] < 4 { -diff1 } else { diff1 };
                for m in 0..7 {
                    let rrpoc = ref_ref_poc[rf.mfmv_ref[n] as usize][m];
                    let diff2 = get_poc_diff(seq_hdr.order_hint_n_bits, rpoc as i32, rrpoc as i32);
                    // unsigned comparison also catches the < 0 case
                    rf.mfmv_ref2ref[n][m] = if diff2 as u32 > 31 { 0 } else { diff2 };
                }
            }
        }
    }
    rf.use_ref_frame_mvs = (rf.n_mfmvs > 0) as i32;

    Ok(())
}

/// # Safety
///
/// Must be called by [`splat_mv::Fn::call`].
#[deny(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn splat_mv_c(
    rr: *mut *mut RefMvsBlock,
    rmv: &Align16<RefMvsBlock>,
    bx4: i32,
    bw4: i32,
    bh4: i32,
) {
    let [bx4, bw4, bh4] = [bx4, bw4, bh4].map(|it| it as usize);
    // SAFETY: Length sliced in `splat_mv::Fn::call`.
    let rr = unsafe { slice::from_raw_parts_mut(rr, bh4) };
    let rr = rr.into_iter().map(|&mut r| {
        // SAFETY: `r` is from `rf.r.index_mut((ri + bx4.., ..bw4)).as_mut_ptr().sub(bx4)` in `splat_mv::Fn::call`.
        unsafe { slice::from_raw_parts_mut(r.add(bx4), bw4) }
    });
    splat_mv_rust(rr, rmv)
}

fn splat_mv_rust<'a>(rr: impl Iterator<Item = &'a mut [RefMvsBlock]>, rmv: &Align16<RefMvsBlock>) {
    let rmv = rmv.0;
    for r in rr {
        r.fill_with(|| rmv)
    }
}

impl Rav1dRefmvsDSPContext {
    pub const fn default() -> Self {
        Self {
            load_tmvs: load_tmvs::Fn::new(load_tmvs_c),
            save_tmvs: save_tmvs::Fn::new(save_tmvs_c),
            splat_mv: splat_mv::Fn::new(splat_mv_c),
        }
    }

    #[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline(always)]
    const fn init_x86(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::SSE2) {
            return self;
        }

        self.splat_mv = splat_mv::decl_fn!(fn dav1d_splat_mv_sse2);

        if !flags.contains(CpuFlags::SSSE3) {
            return self;
        }

        self.save_tmvs = save_tmvs::decl_fn!(fn dav1d_save_tmvs_ssse3);

        if !flags.contains(CpuFlags::SSE41) {
            return self;
        }

        #[cfg(target_arch = "x86_64")]
        {
            self.load_tmvs = load_tmvs::decl_fn!(fn dav1d_load_tmvs_sse4);

            if !flags.contains(CpuFlags::AVX2) {
                return self;
            }

            self.save_tmvs = save_tmvs::decl_fn!(fn dav1d_save_tmvs_avx2);
            self.splat_mv = splat_mv::decl_fn!(fn dav1d_splat_mv_avx2);

            if !flags.contains(CpuFlags::AVX512ICL) {
                return self;
            }

            self.save_tmvs = save_tmvs::decl_fn!(fn dav1d_save_tmvs_avx512icl);
            self.splat_mv = splat_mv::decl_fn!(fn dav1d_splat_mv_avx512icl);
        }

        self
    }

    #[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
    #[inline(always)]
    const fn init_arm(mut self, flags: CpuFlags) -> Self {
        if !flags.contains(CpuFlags::NEON) {
            return self;
        }

        #[cfg(target_arch = "aarch64")]
        {
            self.load_tmvs = load_tmvs::decl_fn!(fn dav1d_load_tmvs_neon);
        }

        self.save_tmvs = save_tmvs::decl_fn!(fn dav1d_save_tmvs_neon);
        self.splat_mv = splat_mv::decl_fn!(fn dav1d_splat_mv_neon);

        self
    }

    #[inline(always)]
    const fn init(self, flags: CpuFlags) -> Self {
        #[cfg(feature = "asm")]
        {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                return self.init_x86(flags);
            }
            #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
            {
                return self.init_arm(flags);
            }
        }

        #[allow(unreachable_code)] // Reachable on some #[cfg]s.
        {
            let _ = flags;
            self
        }
    }

    pub const fn new(flags: CpuFlags) -> Self {
        Self::default().init(flags)
    }
}

impl Default for Rav1dRefmvsDSPContext {
    fn default() -> Self {
        Self::default()
    }
}
