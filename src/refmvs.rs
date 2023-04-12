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
    fn abs(_: libc::c_int) -> libc::c_int;
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

use crate::include::dav1d::headers::Dav1dWarpedMotionParams;

































































use crate::include::dav1d::headers::Dav1dSequenceHeader;






use crate::include::dav1d::headers::Dav1dFrameHeader;












use crate::src::levels::BlockSize;























use crate::src::levels::mv;
use crate::src::levels::mv_xy;
use crate::src::intra_edge::EdgeFlags;





use crate::src::intra_edge::EDGE_I444_TOP_HAS_RIGHT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refmvs_temporal_block {
    pub mv: mv,
    pub ref_0: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union refmvs_refpair {
    pub ref_0: [int8_t; 2],
    pub pair: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union refmvs_mvpair {
    pub mv: [mv; 2],
    pub n: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct refmvs_block {
    pub mv: refmvs_mvpair,
    pub ref_0: refmvs_refpair,
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
pub const DAV1D_X86_CPU_FLAG_AVX512ICL: CpuFlags = 16;
pub const DAV1D_X86_CPU_FLAG_SSE2: CpuFlags = 1;
pub const DAV1D_X86_CPU_FLAG_AVX2: CpuFlags = 8;
pub type CpuFlags = libc::c_uint;
pub const DAV1D_X86_CPU_FLAG_SLOW_GATHER: CpuFlags = 32;
pub const DAV1D_X86_CPU_FLAG_SSE41: CpuFlags = 4;
pub const DAV1D_X86_CPU_FLAG_SSSE3: CpuFlags = 2;
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn apply_sign(v: libc::c_int, s: libc::c_int) -> libc::c_int {
    return if s < 0 as libc::c_int { -v } else { v };
}
#[inline]
unsafe extern "C" fn get_poc_diff(
    order_hint_n_bits: libc::c_int,
    poc0: libc::c_int,
    poc1: libc::c_int,
) -> libc::c_int {
    if order_hint_n_bits == 0 {
        return 0 as libc::c_int;
    }
    let mask: libc::c_int = (1 as libc::c_int) << order_hint_n_bits - 1 as libc::c_int;
    let diff: libc::c_int = poc0 - poc1;
    return (diff & mask - 1 as libc::c_int) - (diff & mask);
}
#[inline]
unsafe extern "C" fn fix_mv_precision(hdr: *const Dav1dFrameHeader, mv: *mut mv) {
    if (*hdr).force_integer_mv != 0 {
        fix_int_mv_precision(mv);
    } else if (*hdr).hp == 0 {
        (*mv)
            .c2rust_unnamed
            .x = (((*mv).c2rust_unnamed.x as libc::c_int
            - ((*mv).c2rust_unnamed.x as libc::c_int >> 15 as libc::c_int))
            as libc::c_uint & !(1 as libc::c_uint)) as int16_t;
        (*mv)
            .c2rust_unnamed
            .y = (((*mv).c2rust_unnamed.y as libc::c_int
            - ((*mv).c2rust_unnamed.y as libc::c_int >> 15 as libc::c_int))
            as libc::c_uint & !(1 as libc::c_uint)) as int16_t;
    }
}
#[inline]
unsafe extern "C" fn fix_int_mv_precision(mv: *mut mv) {
    (*mv)
        .c2rust_unnamed
        .x = (((*mv).c2rust_unnamed.x as libc::c_int
        - ((*mv).c2rust_unnamed.x as libc::c_int >> 15 as libc::c_int)
        + 3 as libc::c_int) as libc::c_uint & !(7 as libc::c_uint)) as int16_t;
    (*mv)
        .c2rust_unnamed
        .y = (((*mv).c2rust_unnamed.y as libc::c_int
        - ((*mv).c2rust_unnamed.y as libc::c_int >> 15 as libc::c_int)
        + 3 as libc::c_int) as libc::c_uint & !(7 as libc::c_uint)) as int16_t;
}
#[inline]
unsafe extern "C" fn get_gmv_2d(
    gmv: *const Dav1dWarpedMotionParams,
    bx4: libc::c_int,
    by4: libc::c_int,
    bw4: libc::c_int,
    bh4: libc::c_int,
    hdr: *const Dav1dFrameHeader,
) -> mv {
    match (*gmv).type_0 as libc::c_uint {
        2 => {
            if !((*gmv).matrix[5 as libc::c_int as usize]
                == (*gmv).matrix[2 as libc::c_int as usize])
            {
                unreachable!();
            }
            if !((*gmv).matrix[4 as libc::c_int as usize]
                == -(*gmv).matrix[3 as libc::c_int as usize])
            {
                unreachable!();
            }
        }
        1 => {
            let mut res_0: mv = mv {
                c2rust_unnamed: {
                    let mut init = mv_xy {
                        y: ((*gmv).matrix[0 as libc::c_int as usize]
                            >> 13 as libc::c_int) as int16_t,
                        x: ((*gmv).matrix[1 as libc::c_int as usize]
                            >> 13 as libc::c_int) as int16_t,
                    };
                    init
                },
            };
            if (*hdr).force_integer_mv != 0 {
                fix_int_mv_precision(&mut res_0);
            }
            return res_0;
        }
        0 => {
            return mv {
                c2rust_unnamed: {
                    let mut init = mv_xy {
                        y: 0 as libc::c_int as int16_t,
                        x: 0 as libc::c_int as int16_t,
                    };
                    init
                },
            };
        }
        3 | _ => {}
    }
    let x: libc::c_int = bx4 * 4 as libc::c_int + bw4 * 2 as libc::c_int
        - 1 as libc::c_int;
    let y: libc::c_int = by4 * 4 as libc::c_int + bh4 * 2 as libc::c_int
        - 1 as libc::c_int;
    let xc: libc::c_int = ((*gmv).matrix[2 as libc::c_int as usize]
        - ((1 as libc::c_int) << 16 as libc::c_int)) * x
        + (*gmv).matrix[3 as libc::c_int as usize] * y
        + (*gmv).matrix[0 as libc::c_int as usize];
    let yc: libc::c_int = ((*gmv).matrix[5 as libc::c_int as usize]
        - ((1 as libc::c_int) << 16 as libc::c_int)) * y
        + (*gmv).matrix[4 as libc::c_int as usize] * x
        + (*gmv).matrix[1 as libc::c_int as usize];
    let shift: libc::c_int = 16 as libc::c_int
        - (3 as libc::c_int - ((*hdr).hp == 0) as libc::c_int);
    let round: libc::c_int = (1 as libc::c_int) << shift >> 1 as libc::c_int;
    let mut res: mv = mv {
        c2rust_unnamed: {
            let mut init = mv_xy {
                y: apply_sign(
                    abs(yc) + round >> shift << ((*hdr).hp == 0) as libc::c_int,
                    yc,
                ) as int16_t,
                x: apply_sign(
                    abs(xc) + round >> shift << ((*hdr).hp == 0) as libc::c_int,
                    xc,
                ) as int16_t,
            };
            init
        },
    };
    if (*hdr).force_integer_mv != 0 {
        fix_int_mv_precision(&mut res);
    }
    return res;
}
#[inline]
unsafe extern "C" fn dav1d_freep_aligned(mut ptr: *mut libc::c_void) {
    let mut mem: *mut *mut libc::c_void = ptr as *mut *mut libc::c_void;
    if !(*mem).is_null() {
        dav1d_free_aligned(*mem);
        *mem = 0 as *mut libc::c_void;
    }
}
#[inline]
unsafe extern "C" fn dav1d_free_aligned(mut ptr: *mut libc::c_void) {
    free(ptr);
}
#[inline]
unsafe extern "C" fn dav1d_alloc_aligned(
    mut sz: size_t,
    mut align: size_t,
) -> *mut libc::c_void {
    if align & align.wrapping_sub(1) != 0 {
        unreachable!();
    }
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if posix_memalign(&mut ptr, align, sz) != 0 {
        return 0 as *mut libc::c_void;
    }
    return ptr;
}
unsafe extern "C" fn add_spatial_candidate(
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    weight: libc::c_int,
    b: *const refmvs_block,
    ref_0: refmvs_refpair,
    mut gmv: *const mv,
    have_newmv_match: *mut libc::c_int,
    have_refmv_match: *mut libc::c_int,
) {
    if (*b).mv.mv[0 as libc::c_int as usize].n == 0x80008000 as libc::c_uint {
        return;
    }
    if ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int == -(1 as libc::c_int) {
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < 2 as libc::c_int {
            if (*b).ref_0.ref_0[n as usize] as libc::c_int
                == ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
            {
                let cand_mv: mv = if (*b).mf as libc::c_int & 1 as libc::c_int != 0
                    && (*gmv.offset(0 as libc::c_int as isize)).n
                        != 0x80008000 as libc::c_uint
                {
                    *gmv.offset(0 as libc::c_int as isize)
                } else {
                    (*b).mv.mv[n as usize]
                };
                *have_refmv_match = 1 as libc::c_int;
                *have_newmv_match |= (*b).mf as libc::c_int >> 1 as libc::c_int;
                let last: libc::c_int = *cnt;
                let mut m: libc::c_int = 0 as libc::c_int;
                while m < last {
                    if (*mvstack.offset(m as isize)).mv.mv[0 as libc::c_int as usize].n
                        == cand_mv.n
                    {
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
    } else if (*b).ref_0.pair as libc::c_int == ref_0.pair as libc::c_int {
        let cand_mv_0: refmvs_mvpair = refmvs_mvpair {
            mv: [
                if (*b).mf as libc::c_int & 1 as libc::c_int != 0
                    && (*gmv.offset(0 as libc::c_int as isize)).n
                        != 0x80008000 as libc::c_uint
                {
                    *gmv.offset(0 as libc::c_int as isize)
                } else {
                    (*b).mv.mv[0 as libc::c_int as usize]
                },
                if (*b).mf as libc::c_int & 1 as libc::c_int != 0
                    && (*gmv.offset(1 as libc::c_int as isize)).n
                        != 0x80008000 as libc::c_uint
                {
                    *gmv.offset(1 as libc::c_int as isize)
                } else {
                    (*b).mv.mv[1 as libc::c_int as usize]
                },
            ],
        };
        *have_refmv_match = 1 as libc::c_int;
        *have_newmv_match |= (*b).mf as libc::c_int >> 1 as libc::c_int;
        let last_0: libc::c_int = *cnt;
        let mut n_0: libc::c_int = 0 as libc::c_int;
        while n_0 < last_0 {
            if (*mvstack.offset(n_0 as isize)).mv.n == cand_mv_0.n {
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
    ref_0: refmvs_refpair,
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
            ref_0,
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
            ref_0,
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
    ref_0: refmvs_refpair,
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
            ref_0,
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
            ref_0,
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
    let y: libc::c_int = mv.c2rust_unnamed.y as libc::c_int * frac;
    let x: libc::c_int = mv.c2rust_unnamed.x as libc::c_int * frac;
    return mv {
        c2rust_unnamed: {
            let mut init = mv_xy {
                y: iclip(
                    y + 8192 as libc::c_int + (y >> 31 as libc::c_int)
                        >> 14 as libc::c_int,
                    -(0x3fff as libc::c_int),
                    0x3fff as libc::c_int,
                ) as int16_t,
                x: iclip(
                    x + 8192 as libc::c_int + (x >> 31 as libc::c_int)
                        >> 14 as libc::c_int,
                    -(0x3fff as libc::c_int),
                    0x3fff as libc::c_int,
                ) as int16_t,
            };
            init
        },
    };
}
unsafe extern "C" fn add_temporal_candidate(
    rf: *const refmvs_frame,
    mvstack: *mut refmvs_candidate,
    cnt: *mut libc::c_int,
    rb: *const refmvs_temporal_block,
    ref_0: refmvs_refpair,
    globalmv_ctx: *mut libc::c_int,
    mut gmv: *const mv,
) {
    if (*rb).mv.n == 0x80008000 as libc::c_uint {
        return;
    }
    let mut mv: mv = mv_projection(
        (*rb).mv,
        (*rf)
            .pocdiff[(ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
            - 1 as libc::c_int) as usize] as libc::c_int,
        (*rb).ref_0 as libc::c_int,
    );
    fix_mv_precision((*rf).frm_hdr, &mut mv);
    let last: libc::c_int = *cnt;
    if ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int == -(1 as libc::c_int) {
        if !globalmv_ctx.is_null() {
            *globalmv_ctx = (abs(
                mv.c2rust_unnamed.x as libc::c_int
                    - (*gmv.offset(0 as libc::c_int as isize)).c2rust_unnamed.x
                        as libc::c_int,
            )
                | abs(
                    mv.c2rust_unnamed.y as libc::c_int
                        - (*gmv.offset(0 as libc::c_int as isize)).c2rust_unnamed.y
                            as libc::c_int,
                ) >= 16 as libc::c_int) as libc::c_int;
        }
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < last {
            if (*mvstack.offset(n as isize)).mv.mv[0 as libc::c_int as usize].n == mv.n {
                (*mvstack.offset(n as isize)).weight += 2 as libc::c_int;
                return;
            }
            n += 1;
        }
        if last < 8 as libc::c_int {
            (*mvstack.offset(last as isize)).mv.mv[0 as libc::c_int as usize] = mv;
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
                        .pocdiff[(ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int
                        - 1 as libc::c_int) as usize] as libc::c_int,
                    (*rb).ref_0 as libc::c_int,
                ),
            ],
        };
        fix_mv_precision(
            (*rf).frm_hdr,
            &mut *(mvp.mv).as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        let mut n_0: libc::c_int = 0 as libc::c_int;
        while n_0 < last {
            if (*mvstack.offset(n_0 as isize)).mv.n == mvp.n {
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
    ref_0: refmvs_refpair,
    sign_bias: *const uint8_t,
) {
    let diff: *mut refmvs_candidate = &mut *same.offset(2 as libc::c_int as isize)
        as *mut refmvs_candidate;
    let diff_count: *mut libc::c_int = &mut *same_count.offset(2 as libc::c_int as isize)
        as *mut libc::c_int;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < 2 as libc::c_int {
        let cand_ref: libc::c_int = (*cand_b).ref_0.ref_0[n as usize] as libc::c_int;
        if cand_ref <= 0 as libc::c_int {
            break;
        }
        let mut cand_mv: mv = (*cand_b).mv.mv[n as usize];
        if cand_ref == ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int {
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
                        .c2rust_unnamed
                        .y = -(cand_mv.c2rust_unnamed.y as libc::c_int) as int16_t;
                    cand_mv
                        .c2rust_unnamed
                        .x = -(cand_mv.c2rust_unnamed.x as libc::c_int) as int16_t;
                }
                let ref mut fresh2 = *diff_count.offset(1 as libc::c_int as isize);
                let fresh3 = *fresh2;
                *fresh2 = *fresh2 + 1;
                (*diff.offset(fresh3 as isize))
                    .mv
                    .mv[1 as libc::c_int as usize] = cand_mv;
            }
        } else if cand_ref == ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int {
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
                    cand_mv
                        .c2rust_unnamed
                        .y = -(cand_mv.c2rust_unnamed.y as libc::c_int) as int16_t;
                    cand_mv
                        .c2rust_unnamed
                        .x = -(cand_mv.c2rust_unnamed.x as libc::c_int) as int16_t;
                }
                let ref mut fresh6 = *diff_count.offset(0 as libc::c_int as isize);
                let fresh7 = *fresh6;
                *fresh6 = *fresh6 + 1;
                (*diff.offset(fresh7 as isize))
                    .mv
                    .mv[0 as libc::c_int as usize] = cand_mv;
            }
        } else {
            let mut i_cand_mv: mv = mv {
                c2rust_unnamed: {
                    let mut init = mv_xy {
                        y: -(cand_mv.c2rust_unnamed.y as libc::c_int) as int16_t,
                        x: -(cand_mv.c2rust_unnamed.x as libc::c_int) as int16_t,
                    };
                    init
                },
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
        let cand_ref: libc::c_int = (*cand_b).ref_0.ref_0[n as usize] as libc::c_int;
        if cand_ref <= 0 as libc::c_int {
            break;
        }
        let mut cand_mv: mv = (*cand_b).mv.mv[n as usize];
        if sign
            ^ *sign_bias.offset((cand_ref - 1 as libc::c_int) as isize) as libc::c_int
            != 0
        {
            cand_mv
                .c2rust_unnamed
                .y = -(cand_mv.c2rust_unnamed.y as libc::c_int) as int16_t;
            cand_mv
                .c2rust_unnamed
                .x = -(cand_mv.c2rust_unnamed.x as libc::c_int) as int16_t;
        }
        let mut m: libc::c_int = 0;
        let last: libc::c_int = *cnt;
        m = 0 as libc::c_int;
        while m < last {
            if cand_mv.n
                == (*mvstack.offset(m as isize)).mv.mv[0 as libc::c_int as usize].n
            {
                break;
            }
            m += 1;
        }
        if m == last {
            (*mvstack.offset(m as isize)).mv.mv[0 as libc::c_int as usize] = cand_mv;
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
    ref_0: refmvs_refpair,
    bs: BlockSize,
    edge_flags: EdgeFlags,
    by4: libc::c_int,
    bx4: libc::c_int,
) {
    let rf: *const refmvs_frame = (*rt).rf;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: libc::c_int = *b_dim.offset(0 as libc::c_int as isize) as libc::c_int;
    let w4: libc::c_int = imin(imin(bw4, 16 as libc::c_int), (*rt).tile_col.end - bx4);
    let bh4: libc::c_int = *b_dim.offset(1 as libc::c_int as isize) as libc::c_int;
    let h4: libc::c_int = imin(imin(bh4, 16 as libc::c_int), (*rt).tile_row.end - by4);
    let mut gmv: [mv; 2] = [mv {
        c2rust_unnamed: mv_xy { y: 0, x: 0 },
    }; 2];
    let mut tgmv: [mv; 2] = [mv {
        c2rust_unnamed: mv_xy { y: 0, x: 0 },
    }; 2];
    *cnt = 0 as libc::c_int;
    if !(ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int >= 0 as libc::c_int
        && ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int <= 8 as libc::c_int
        && ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int >= -(1 as libc::c_int)
        && ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int <= 8 as libc::c_int)
    {
        unreachable!();
    }
    if ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int {
        tgmv[0 as libc::c_int
            as usize] = get_gmv_2d(
            &*((*(*rf).frm_hdr).gmv)
                .as_ptr()
                .offset(
                    (*(ref_0.ref_0).as_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int - 1 as libc::c_int) as isize,
                ),
            bx4,
            by4,
            bw4,
            bh4,
            (*rf).frm_hdr,
        );
        gmv[0 as libc::c_int
            as usize] = if (*(*rf).frm_hdr)
            .gmv[(ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize]
            .type_0 as libc::c_uint
            > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint
        {
            tgmv[0 as libc::c_int as usize]
        } else {
            mv {
                n: 0x80008000 as libc::c_uint,
            }
        };
    } else {
        tgmv[0 as libc::c_int
            as usize] = mv {
            n: 0 as libc::c_int as uint32_t,
        };
        gmv[0 as libc::c_int
            as usize] = mv {
            n: 0x80008000 as libc::c_uint,
        };
    }
    if ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int {
        tgmv[1 as libc::c_int
            as usize] = get_gmv_2d(
            &*((*(*rf).frm_hdr).gmv)
                .as_ptr()
                .offset(
                    (*(ref_0.ref_0).as_ptr().offset(1 as libc::c_int as isize)
                        as libc::c_int - 1 as libc::c_int) as isize,
                ),
            bx4,
            by4,
            bw4,
            bh4,
            (*rf).frm_hdr,
        );
        gmv[1 as libc::c_int
            as usize] = if (*(*rf).frm_hdr)
            .gmv[(ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize]
            .type_0 as libc::c_uint
            > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint
        {
            tgmv[1 as libc::c_int as usize]
        } else {
            mv {
                n: 0x80008000 as libc::c_uint,
            }
        };
    }
    let mut have_newmv: libc::c_int = 0 as libc::c_int;
    let mut have_col_mvs: libc::c_int = 0 as libc::c_int;
    let mut have_row_mvs: libc::c_int = 0 as libc::c_int;
    let mut max_rows: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n_rows: libc::c_uint = !(0 as libc::c_int) as libc::c_uint;
    let mut b_top: *const refmvs_block = 0 as *const refmvs_block;
    if by4 > (*rt).tile_row.start {
        max_rows = imin(
            by4 - (*rt).tile_row.start + 1 as libc::c_int >> 1 as libc::c_int,
            2 as libc::c_int + (bh4 > 1 as libc::c_int) as libc::c_int,
        ) as libc::c_uint;
        b_top = &mut *(*((*rt).r)
            .as_ptr()
            .offset(
                ((by4 & 31 as libc::c_int) - 1 as libc::c_int + 5 as libc::c_int)
                    as isize,
            ))
            .offset(bx4 as isize) as *mut refmvs_block;
        n_rows = scan_row(
            mvstack,
            cnt,
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            b_top,
            bw4,
            w4,
            max_rows as libc::c_int,
            if bw4 >= 16 as libc::c_int { 4 as libc::c_int } else { 1 as libc::c_int },
            &mut have_newmv,
            &mut have_row_mvs,
        ) as libc::c_uint;
    }
    let mut max_cols: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut n_cols: libc::c_uint = !(0 as libc::c_uint);
    let mut b_left: *const *mut refmvs_block = 0 as *const *mut refmvs_block;
    if bx4 > (*rt).tile_col.start {
        max_cols = imin(
            bx4 - (*rt).tile_col.start + 1 as libc::c_int >> 1 as libc::c_int,
            2 as libc::c_int + (bw4 > 1 as libc::c_int) as libc::c_int,
        ) as libc::c_uint;
        b_left = &*((*rt).r)
            .as_ptr()
            .offset(((by4 & 31 as libc::c_int) + 5 as libc::c_int) as isize)
            as *const *mut refmvs_block;
        n_cols = scan_col(
            mvstack,
            cnt,
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            b_left,
            bh4,
            h4,
            bx4 - 1 as libc::c_int,
            max_cols as libc::c_int,
            if bh4 >= 16 as libc::c_int { 4 as libc::c_int } else { 1 as libc::c_int },
            &mut have_newmv,
            &mut have_col_mvs,
        ) as libc::c_uint;
    }
    if n_rows != !(0 as libc::c_uint)
        && edge_flags as libc::c_uint
            & EDGE_I444_TOP_HAS_RIGHT as libc::c_int as libc::c_uint != 0
        && imax(bw4, bh4) <= 16 as libc::c_int && bw4 + bx4 < (*rt).tile_col.end
    {
        add_spatial_candidate(
            mvstack,
            cnt,
            4 as libc::c_int,
            &*b_top.offset(bw4 as isize),
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            &mut have_newmv,
            &mut have_row_mvs,
        );
    }
    let nearest_match: libc::c_int = have_col_mvs + have_row_mvs;
    let nearest_cnt: libc::c_int = *cnt;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < nearest_cnt {
        (*mvstack.offset(n as isize)).weight += 640 as libc::c_int;
        n += 1;
    }
    let mut globalmv_ctx: libc::c_int = (*(*rf).frm_hdr).use_ref_frame_mvs;
    if (*rf).use_ref_frame_mvs != 0 {
        let stride: ptrdiff_t = (*rf).rp_stride;
        let by8: libc::c_int = by4 >> 1 as libc::c_int;
        let bx8: libc::c_int = bx4 >> 1 as libc::c_int;
        let rbi: *const refmvs_temporal_block = &mut *((*rt).rp_proj)
            .offset((by8 & 15) as isize * stride + bx8 as isize,
            ) as *mut refmvs_temporal_block;
        let mut rb: *const refmvs_temporal_block = rbi;
        let step_h: libc::c_int = if bw4 >= 16 as libc::c_int {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let step_v: libc::c_int = if bh4 >= 16 as libc::c_int {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        };
        let w8: libc::c_int = imin(
            w4 + 1 as libc::c_int >> 1 as libc::c_int,
            8 as libc::c_int,
        );
        let h8: libc::c_int = imin(
            h4 + 1 as libc::c_int >> 1 as libc::c_int,
            8 as libc::c_int,
        );
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < h8 {
            let mut x: libc::c_int = 0 as libc::c_int;
            while x < w8 {
                add_temporal_candidate(
                    rf,
                    mvstack,
                    cnt,
                    &*rb.offset(x as isize),
                    ref_0,
                    if x | y == 0 { &mut globalmv_ctx } else { 0 as *mut libc::c_int },
                    tgmv.as_mut_ptr() as *const mv,
                );
                x += step_h;
            }
            rb = rb.offset(stride * step_v as isize);
            y += step_v;
        }
        if imin(bw4, bh4) >= 2 as libc::c_int && imax(bw4, bh4) < 16 as libc::c_int {
            let bh8: libc::c_int = bh4 >> 1 as libc::c_int;
            let bw8: libc::c_int = bw4 >> 1 as libc::c_int;
            rb = &*rbi.offset(bh8 as isize * stride)
                as *const refmvs_temporal_block;
            let has_bottom: libc::c_int = (by8 + bh8
                < imin(
                    (*rt).tile_row.end >> 1 as libc::c_int,
                    (by8 & !(7 as libc::c_int)) + 8 as libc::c_int,
                )) as libc::c_int;
            if has_bottom != 0
                && bx8 - 1 as libc::c_int
                    >= imax(
                        (*rt).tile_col.start >> 1 as libc::c_int,
                        bx8 & !(7 as libc::c_int),
                    )
            {
                add_temporal_candidate(
                    rf,
                    mvstack,
                    cnt,
                    &*rb.offset(-(1 as libc::c_int) as isize),
                    ref_0,
                    0 as *mut libc::c_int,
                    0 as *const mv,
                );
            }
            if bx8 + bw8
                < imin(
                    (*rt).tile_col.end >> 1 as libc::c_int,
                    (bx8 & !(7 as libc::c_int)) + 8 as libc::c_int,
                )
            {
                if has_bottom != 0 {
                    add_temporal_candidate(
                        rf,
                        mvstack,
                        cnt,
                        &*rb.offset(bw8 as isize),
                        ref_0,
                        0 as *mut libc::c_int,
                        0 as *const mv,
                    );
                }
                if (by8 + bh8 - 1 as libc::c_int)
                    < imin(
                        (*rt).tile_row.end >> 1 as libc::c_int,
                        (by8 & !(7 as libc::c_int)) + 8 as libc::c_int,
                    )
                {
                    add_temporal_candidate(
                        rf,
                        mvstack,
                        cnt,
                        &*rb.offset(bw8 as isize - stride),
                        ref_0,
                        0 as *mut libc::c_int,
                        0 as *const mv,
                    );
                }
            }
        }
    }
    if !(*cnt <= 8 as libc::c_int) {
        unreachable!();
    }
    let mut have_dummy_newmv_match: libc::c_int = 0;
    if n_rows | n_cols != !(0 as libc::c_uint) {
        add_spatial_candidate(
            mvstack,
            cnt,
            4 as libc::c_int,
            &*b_top.offset(-(1 as libc::c_int) as isize),
            ref_0,
            gmv.as_mut_ptr() as *const mv,
            &mut have_dummy_newmv_match,
            &mut have_row_mvs,
        );
    }
    let mut n_0: libc::c_int = 2 as libc::c_int;
    while n_0 <= 3 as libc::c_int {
        if n_0 as libc::c_uint > n_rows && n_0 as libc::c_uint <= max_rows {
            n_rows = n_rows
                .wrapping_add(
                    scan_row(
                        mvstack,
                        cnt,
                        ref_0,
                        gmv.as_mut_ptr() as *const mv,
                        &mut *(*((*rt).r)
                            .as_ptr()
                            .offset(
                                (((by4 & 31 as libc::c_int) - 2 as libc::c_int * n_0
                                    + 1 as libc::c_int | 1 as libc::c_int) + 5 as libc::c_int)
                                    as isize,
                            ))
                            .offset((bx4 | 1 as libc::c_int) as isize),
                        bw4,
                        w4,
                        (1 as libc::c_int as libc::c_uint)
                            .wrapping_add(max_rows)
                            .wrapping_sub(n_0 as libc::c_uint) as libc::c_int,
                        if bw4 >= 16 as libc::c_int {
                            4 as libc::c_int
                        } else {
                            2 as libc::c_int
                        },
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
                        ref_0,
                        gmv.as_mut_ptr() as *const mv,
                        &*((*rt).r)
                            .as_ptr()
                            .offset(
                                ((by4 & 31 as libc::c_int | 1 as libc::c_int)
                                    + 5 as libc::c_int) as isize,
                            ),
                        bh4,
                        h4,
                        bx4 - n_0 * 2 as libc::c_int + 1 as libc::c_int
                            | 1 as libc::c_int,
                        (1 as libc::c_int as libc::c_uint)
                            .wrapping_add(max_cols)
                            .wrapping_sub(n_0 as libc::c_uint) as libc::c_int,
                        if bh4 >= 16 as libc::c_int {
                            4 as libc::c_int
                        } else {
                            2 as libc::c_int
                        },
                        &mut have_dummy_newmv_match,
                        &mut have_col_mvs,
                    ) as libc::c_uint,
                );
        }
        n_0 += 1;
    }
    if !(*cnt <= 8 as libc::c_int) {
        unreachable!();
    }
    let ref_match_count: libc::c_int = have_col_mvs + have_row_mvs;
    let mut refmv_ctx: libc::c_int = 0;
    let mut newmv_ctx: libc::c_int = 0;
    match nearest_match {
        0 => {
            refmv_ctx = imin(2 as libc::c_int, ref_match_count);
            newmv_ctx = (ref_match_count > 0 as libc::c_int) as libc::c_int;
        }
        1 => {
            refmv_ctx = imin(ref_match_count * 3 as libc::c_int, 4 as libc::c_int);
            newmv_ctx = 3 as libc::c_int - have_newmv;
        }
        2 => {
            refmv_ctx = 5 as libc::c_int;
            newmv_ctx = 5 as libc::c_int - have_newmv;
        }
        _ => {}
    }
    let mut len: libc::c_int = nearest_cnt;
    while len != 0 {
        let mut last: libc::c_int = 0 as libc::c_int;
        let mut n_1: libc::c_int = 1 as libc::c_int;
        while n_1 < len {
            if (*mvstack.offset((n_1 - 1 as libc::c_int) as isize)).weight
                < (*mvstack.offset(n_1 as isize)).weight
            {
                let mut tmp: refmvs_candidate = *mvstack
                    .offset((n_1 - 1 as libc::c_int) as isize);
                *mvstack
                    .offset(
                        (n_1 - 1 as libc::c_int) as isize,
                    ) = *mvstack.offset(n_1 as isize);
                *mvstack.offset(n_1 as isize) = tmp;
                last = n_1;
            }
            n_1 += 1;
        }
        len = last;
    }
    len = *cnt;
    while len > nearest_cnt {
        let mut last_0: libc::c_int = nearest_cnt;
        let mut n_2: libc::c_int = nearest_cnt + 1 as libc::c_int;
        while n_2 < len {
            if (*mvstack.offset((n_2 - 1 as libc::c_int) as isize)).weight
                < (*mvstack.offset(n_2 as isize)).weight
            {
                let mut tmp_0: refmvs_candidate = *mvstack
                    .offset((n_2 - 1 as libc::c_int) as isize);
                *mvstack
                    .offset(
                        (n_2 - 1 as libc::c_int) as isize,
                    ) = *mvstack.offset(n_2 as isize);
                *mvstack.offset(n_2 as isize) = tmp_0;
                last_0 = n_2;
            }
            n_2 += 1;
        }
        len = last_0;
    }
    if ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int {
        if *cnt < 2 as libc::c_int {
            let sign0: libc::c_int = (*rf)
                .sign_bias[(ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize] as libc::c_int;
            let sign1: libc::c_int = (*rf)
                .sign_bias[(ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize] as libc::c_int;
            let sz4: libc::c_int = imin(w4, h4);
            let same: *mut refmvs_candidate = &mut *mvstack.offset(*cnt as isize)
                as *mut refmvs_candidate;
            let mut same_count: [libc::c_int; 4] = [0 as libc::c_int, 0, 0, 0];
            if n_rows != !(0 as libc::c_uint) {
                let mut x_0: libc::c_int = 0 as libc::c_int;
                while x_0 < sz4 {
                    let cand_b: *const refmvs_block = &*b_top.offset(x_0 as isize)
                        as *const refmvs_block;
                    add_compound_extended_candidate(
                        same,
                        same_count.as_mut_ptr(),
                        cand_b,
                        sign0,
                        sign1,
                        ref_0,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    x_0
                        += dav1d_block_dimensions[(*cand_b).bs
                            as usize][0 as libc::c_int as usize] as libc::c_int;
                }
            }
            if n_cols != !(0 as libc::c_uint) {
                let mut y_0: libc::c_int = 0 as libc::c_int;
                while y_0 < sz4 {
                    let cand_b_0: *const refmvs_block = &mut *(*b_left
                        .offset(y_0 as isize))
                        .offset((bx4 - 1 as libc::c_int) as isize) as *mut refmvs_block;
                    add_compound_extended_candidate(
                        same,
                        same_count.as_mut_ptr(),
                        cand_b_0,
                        sign0,
                        sign1,
                        ref_0,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    y_0
                        += dav1d_block_dimensions[(*cand_b_0).bs
                            as usize][1 as libc::c_int as usize] as libc::c_int;
                }
            }
            let diff: *mut refmvs_candidate = &mut *same
                .offset(2 as libc::c_int as isize) as *mut refmvs_candidate;
            let diff_count: *const libc::c_int = &mut *same_count
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize) as *mut libc::c_int;
            let mut current_block_118: u64;
            let mut n_3: libc::c_int = 0 as libc::c_int;
            while n_3 < 2 as libc::c_int {
                let mut m: libc::c_int = same_count[n_3 as usize];
                if !(m >= 2 as libc::c_int) {
                    let l: libc::c_int = *diff_count.offset(n_3 as isize);
                    if l != 0 {
                        (*same.offset(m as isize))
                            .mv
                            .mv[n_3
                            as usize] = (*diff.offset(0 as libc::c_int as isize))
                            .mv
                            .mv[n_3 as usize];
                        m += 1;
                        if m == 2 as libc::c_int {
                            current_block_118 = 13740693533991687037;
                        } else if l == 2 as libc::c_int {
                            (*same.offset(1 as libc::c_int as isize))
                                .mv
                                .mv[n_3
                                as usize] = (*diff.offset(1 as libc::c_int as isize))
                                .mv
                                .mv[n_3 as usize];
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
                                (*same.offset(m as isize))
                                    .mv
                                    .mv[n_3 as usize] = tgmv[n_3 as usize];
                                m += 1;
                                if !(m < 2 as libc::c_int) {
                                    break;
                                }
                            }
                        }
                    }
                }
                n_3 += 1;
            }
            let mut n_4: libc::c_int = *cnt;
            if n_4 == 1 as libc::c_int
                && (*mvstack.offset(0 as libc::c_int as isize)).mv.n
                    == (*same.offset(0 as libc::c_int as isize)).mv.n
            {
                (*mvstack.offset(1 as libc::c_int as isize))
                    .mv = (*mvstack.offset(2 as libc::c_int as isize)).mv;
            }
            loop {
                (*mvstack.offset(n_4 as isize)).weight = 2 as libc::c_int;
                n_4 += 1;
                if !(n_4 < 2 as libc::c_int) {
                    break;
                }
            }
            *cnt = 2 as libc::c_int;
        }
        let left: libc::c_int = -(bx4 + bw4 + 4 as libc::c_int) * 4 as libc::c_int
            * 8 as libc::c_int;
        let right: libc::c_int = ((*rf).iw4 - bx4 + 4 as libc::c_int) * 4 as libc::c_int
            * 8 as libc::c_int;
        let top: libc::c_int = -(by4 + bh4 + 4 as libc::c_int) * 4 as libc::c_int
            * 8 as libc::c_int;
        let bottom: libc::c_int = ((*rf).ih4 - by4 + 4 as libc::c_int) * 4 as libc::c_int
            * 8 as libc::c_int;
        let n_refmvs: libc::c_int = *cnt;
        let mut n_5: libc::c_int = 0 as libc::c_int;
        loop {
            (*mvstack.offset(n_5 as isize))
                .mv
                .mv[0 as libc::c_int as usize]
                .c2rust_unnamed
                .x = iclip(
                (*mvstack.offset(n_5 as isize))
                    .mv
                    .mv[0 as libc::c_int as usize]
                    .c2rust_unnamed
                    .x as libc::c_int,
                left,
                right,
            ) as int16_t;
            (*mvstack.offset(n_5 as isize))
                .mv
                .mv[0 as libc::c_int as usize]
                .c2rust_unnamed
                .y = iclip(
                (*mvstack.offset(n_5 as isize))
                    .mv
                    .mv[0 as libc::c_int as usize]
                    .c2rust_unnamed
                    .y as libc::c_int,
                top,
                bottom,
            ) as int16_t;
            (*mvstack.offset(n_5 as isize))
                .mv
                .mv[1 as libc::c_int as usize]
                .c2rust_unnamed
                .x = iclip(
                (*mvstack.offset(n_5 as isize))
                    .mv
                    .mv[1 as libc::c_int as usize]
                    .c2rust_unnamed
                    .x as libc::c_int,
                left,
                right,
            ) as int16_t;
            (*mvstack.offset(n_5 as isize))
                .mv
                .mv[1 as libc::c_int as usize]
                .c2rust_unnamed
                .y = iclip(
                (*mvstack.offset(n_5 as isize))
                    .mv
                    .mv[1 as libc::c_int as usize]
                    .c2rust_unnamed
                    .y as libc::c_int,
                top,
                bottom,
            ) as int16_t;
            n_5 += 1;
            if !(n_5 < n_refmvs) {
                break;
            }
        }
        match refmv_ctx >> 1 as libc::c_int {
            0 => {
                *ctx = imin(newmv_ctx, 1 as libc::c_int);
            }
            1 => {
                *ctx = 1 as libc::c_int + imin(newmv_ctx, 3 as libc::c_int);
            }
            2 => {
                *ctx = iclip(
                    3 as libc::c_int + newmv_ctx,
                    4 as libc::c_int,
                    7 as libc::c_int,
                );
            }
            _ => {}
        }
        return;
    } else {
        if *cnt < 2 as libc::c_int
            && ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int > 0 as libc::c_int
        {
            let sign: libc::c_int = (*rf)
                .sign_bias[(ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
                - 1 as libc::c_int) as usize] as libc::c_int;
            let sz4_0: libc::c_int = imin(w4, h4);
            if n_rows != !(0 as libc::c_uint) {
                let mut x_1: libc::c_int = 0 as libc::c_int;
                while x_1 < sz4_0 && *cnt < 2 as libc::c_int {
                    let cand_b_1: *const refmvs_block = &*b_top.offset(x_1 as isize)
                        as *const refmvs_block;
                    add_single_extended_candidate(
                        mvstack,
                        cnt,
                        cand_b_1,
                        sign,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    x_1
                        += dav1d_block_dimensions[(*cand_b_1).bs
                            as usize][0 as libc::c_int as usize] as libc::c_int;
                }
            }
            if n_cols != !(0 as libc::c_uint) {
                let mut y_1: libc::c_int = 0 as libc::c_int;
                while y_1 < sz4_0 && *cnt < 2 as libc::c_int {
                    let cand_b_2: *const refmvs_block = &mut *(*b_left
                        .offset(y_1 as isize))
                        .offset((bx4 - 1 as libc::c_int) as isize) as *mut refmvs_block;
                    add_single_extended_candidate(
                        mvstack,
                        cnt,
                        cand_b_2,
                        sign,
                        ((*rf).sign_bias).as_ptr(),
                    );
                    y_1
                        += dav1d_block_dimensions[(*cand_b_2).bs
                            as usize][1 as libc::c_int as usize] as libc::c_int;
                }
            }
        }
    }
    if !(*cnt <= 8 as libc::c_int) {
        unreachable!();
    }
    let mut n_refmvs_0: libc::c_int = *cnt;
    if n_refmvs_0 != 0 {
        let left_0: libc::c_int = -(bx4 + bw4 + 4 as libc::c_int) * 4 as libc::c_int
            * 8 as libc::c_int;
        let right_0: libc::c_int = ((*rf).iw4 - bx4 + 4 as libc::c_int)
            * 4 as libc::c_int * 8 as libc::c_int;
        let top_0: libc::c_int = -(by4 + bh4 + 4 as libc::c_int) * 4 as libc::c_int
            * 8 as libc::c_int;
        let bottom_0: libc::c_int = ((*rf).ih4 - by4 + 4 as libc::c_int)
            * 4 as libc::c_int * 8 as libc::c_int;
        let mut n_6: libc::c_int = 0 as libc::c_int;
        loop {
            (*mvstack.offset(n_6 as isize))
                .mv
                .mv[0 as libc::c_int as usize]
                .c2rust_unnamed
                .x = iclip(
                (*mvstack.offset(n_6 as isize))
                    .mv
                    .mv[0 as libc::c_int as usize]
                    .c2rust_unnamed
                    .x as libc::c_int,
                left_0,
                right_0,
            ) as int16_t;
            (*mvstack.offset(n_6 as isize))
                .mv
                .mv[0 as libc::c_int as usize]
                .c2rust_unnamed
                .y = iclip(
                (*mvstack.offset(n_6 as isize))
                    .mv
                    .mv[0 as libc::c_int as usize]
                    .c2rust_unnamed
                    .y as libc::c_int,
                top_0,
                bottom_0,
            ) as int16_t;
            n_6 += 1;
            if !(n_6 < n_refmvs_0) {
                break;
            }
        }
    }
    let mut n_7: libc::c_int = *cnt;
    while n_7 < 2 as libc::c_int {
        (*mvstack.offset(n_7 as isize))
            .mv
            .mv[0 as libc::c_int as usize] = tgmv[0 as libc::c_int as usize];
        n_7 += 1;
    }
    *ctx = refmv_ctx << 4 as libc::c_int | globalmv_ctx << 3 as libc::c_int | newmv_ctx;
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
            (*rp_proj.offset(x as isize)).mv.n = 0x80008000 as libc::c_uint;
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
            let ref_0: libc::c_int = (*rf).mfmv_ref[n as usize] as libc::c_int;
            let ref_sign: libc::c_int = ref_0 - 4 as libc::c_int;
            let mut r: *const refmvs_temporal_block = &mut *(*((*rf).rp_ref)
                .offset(ref_0 as isize))
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
                    let b_ref: libc::c_int = (*rb).ref_0 as libc::c_int;
                    if !(b_ref == 0) {
                        let ref2ref: libc::c_int = (*rf)
                            .mfmv_ref2ref[n
                            as usize][(b_ref - 1 as libc::c_int) as usize];
                        if !(ref2ref == 0) {
                            let b_mv: mv = (*rb).mv;
                            let offset: mv = mv_projection(b_mv, ref2cur, ref2ref);
                            let mut pos_x: libc::c_int = x_0
                                + apply_sign(
                                    abs(offset.c2rust_unnamed.x as libc::c_int)
                                        >> 6 as libc::c_int,
                                    offset.c2rust_unnamed.x as libc::c_int ^ ref_sign,
                                );
                            let pos_y: libc::c_int = y_0
                                + apply_sign(
                                    abs(offset.c2rust_unnamed.y as libc::c_int)
                                        >> 6 as libc::c_int,
                                    offset.c2rust_unnamed.y as libc::c_int ^ ref_sign,
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
                                            .ref_0 = ref2ref as int8_t;
                                    }
                                    x_0 += 1;
                                    if x_0 >= col_end8i {
                                        break;
                                    }
                                    rb = rb.offset(1);
                                    if (*rb).ref_0 as libc::c_int != b_ref
                                        || (*rb).mv.n != b_mv.n
                                    {
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
                                    if (*rb).ref_0 as libc::c_int != b_ref
                                        || (*rb).mv.n != b_mv.n
                                    {
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
            if (*cand_b).ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int
                > 0 as libc::c_int
                && *ref_sign
                    .offset(
                        ((*cand_b).ref_0.ref_0[1 as libc::c_int as usize] as libc::c_int
                            - 1 as libc::c_int) as isize,
                    ) as libc::c_int != 0
                && abs(
                    (*cand_b).mv.mv[1 as libc::c_int as usize].c2rust_unnamed.y
                        as libc::c_int,
                )
                    | abs(
                        (*cand_b).mv.mv[1 as libc::c_int as usize].c2rust_unnamed.x
                            as libc::c_int,
                    ) < 4096 as libc::c_int
            {
                let mut n: libc::c_int = 0 as libc::c_int;
                while n < bw8 {
                    *rp
                        .offset(
                            x as isize,
                        ) = {
                        let mut init = refmvs_temporal_block {
                            mv: (*cand_b).mv.mv[1 as libc::c_int as usize],
                            ref_0: (*cand_b).ref_0.ref_0[1 as libc::c_int as usize],
                        };
                        init
                    };
                    n += 1;
                    x += 1;
                }
            } else if (*cand_b).ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
                > 0 as libc::c_int
                && *ref_sign
                    .offset(
                        ((*cand_b).ref_0.ref_0[0 as libc::c_int as usize] as libc::c_int
                            - 1 as libc::c_int) as isize,
                    ) as libc::c_int != 0
                && abs(
                    (*cand_b).mv.mv[0 as libc::c_int as usize].c2rust_unnamed.y
                        as libc::c_int,
                )
                    | abs(
                        (*cand_b).mv.mv[0 as libc::c_int as usize].c2rust_unnamed.x
                            as libc::c_int,
                    ) < 4096 as libc::c_int
            {
                let mut n_0: libc::c_int = 0 as libc::c_int;
                while n_0 < bw8 {
                    *rp
                        .offset(
                            x as isize,
                        ) = {
                        let mut init = refmvs_temporal_block {
                            mv: (*cand_b).mv.mv[0 as libc::c_int as usize],
                            ref_0: (*cand_b).ref_0.ref_0[0 as libc::c_int as usize],
                        };
                        init
                    };
                    n_0 += 1;
                    x += 1;
                }
            } else {
                let mut n_1: libc::c_int = 0 as libc::c_int;
                while n_1 < bw8 {
                    (*rp.offset(x as isize)).ref_0 = 0 as libc::c_int as int8_t;
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
            if abs(diff1) > 31 as libc::c_int {
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
#[inline(always)]
#[cfg(feature = "asm")]
unsafe extern "C" fn dav1d_get_cpu_flags() -> libc::c_uint {
    let mut flags = dav1d_cpu_flags & dav1d_cpu_flags_mask;
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            flags |= DAV1D_X86_CPU_FLAG_SSE2;
        }
    }
    return flags;
}
#[inline(always)]
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = "asm"))]
unsafe extern "C" fn refmvs_dsp_init_x86(c: *mut Dav1dRefmvsDSPContext) {
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
