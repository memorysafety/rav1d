#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
use crate::src::cpu::rav1d_get_cpu_flags;
#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
use crate::src::cpu::CpuFlags;
use cfg_if::cfg_if;
use std::ffi::c_int;
use std::slice;

pub type pal_idx_finish_fn = unsafe extern "C" fn(
    dst: *mut u8,
    src: *const u8,
    bw: c_int,
    bh: c_int,
    w: c_int,
    h: c_int,
) -> ();

#[repr(C)]
pub(crate) struct Rav1dPalDSPContext {
    pub pal_idx_finish: pal_idx_finish_fn,
}

// fill invisible edges and pack to 4-bit (2 pixels per byte)
unsafe extern "C" fn pal_idx_finish_rust(
    dst: *mut u8,
    src: *const u8,
    bw: c_int,
    bh: c_int,
    w: c_int,
    h: c_int,
) {
    assert!(bw >= 4 && bw <= 64 && (bw as u32).is_power_of_two());
    assert!(bh >= 4 && bh <= 64 && (bh as u32).is_power_of_two());
    assert!(w >= 4 && w <= bw && (w & 3) == 0);
    assert!(h >= 4 && h <= bh && (h & 3) == 0);

    let w = w as usize;
    let h = h as usize;
    let bw = bw as usize;
    let bh = bh as usize;
    let dst_w = w / 2;
    let dst_bw = bw / 2;

    let mut dst = slice::from_raw_parts_mut(dst, dst_bw * bh);
    let mut src = slice::from_raw_parts(src, bw * bh);

    for y in 0..h {
        for x in 0..dst_w {
            dst[x] = src[2 * x] | (src[2 * x + 1] << 4)
        }
        if dst_w < dst_bw {
            dst[dst_w..dst_bw].fill(0x11 * src[w]);
        }
        src = &src[bw..];
        if y < h - 1 {
            dst = &mut dst[dst_bw..];
        }
    }

    if h < bh {
        let (last_row, dst) = dst.split_at_mut(dst_bw);

        for row in dst.chunks_exact_mut(dst_bw) {
            row.copy_from_slice(last_row);
        }
    }
}

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
extern "C" {
    fn dav1d_pal_idx_finish_ssse3(
        dst: *mut u8,
        src: *const u8,
        bw: c_int,
        bh: c_int,
        w: c_int,
        h: c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "x86_64"),))]
extern "C" {
    fn dav1d_pal_idx_finish_avx2(
        dst: *mut u8,
        src: *const u8,
        bw: c_int,
        bh: c_int,
        w: c_int,
        h: c_int,
    );

    fn dav1d_pal_idx_finish_avx512icl(
        dst: *mut u8,
        src: *const u8,
        bw: c_int,
        bh: c_int,
        w: c_int,
        h: c_int,
    );
}

#[inline(always)]
#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
unsafe fn pal_dsp_init_x86(c: *mut Rav1dPalDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    (*c).pal_idx_finish = dav1d_pal_idx_finish_ssse3;

    cfg_if! {
        if #[cfg(any(target_arch = "x86_64"))] {
            if !flags.contains(CpuFlags::AVX2) {
                return;
            }

            (*c).pal_idx_finish = dav1d_pal_idx_finish_avx2;

            if !flags.contains(CpuFlags::AVX512ICL) {
                return;
            }

            (*c).pal_idx_finish = dav1d_pal_idx_finish_avx512icl;
        }
    }
}

pub(crate) unsafe fn rav1d_pal_dsp_init(c: *mut Rav1dPalDSPContext) -> () {
    (*c).pal_idx_finish = pal_idx_finish_rust;

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            pal_dsp_init_x86(c);
        }
    }
}
