use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::DynEntry;
use crate::include::common::bitdepth::DynPixel;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::src::filmgrain::fguv_32x32xn_420_c_erased;
use crate::src::filmgrain::fguv_32x32xn_422_c_erased;
use crate::src::filmgrain::fguv_32x32xn_444_c_erased;
use crate::src::filmgrain::fgy_32x32xn_c_erased;
use crate::src::filmgrain::generate_grain_uv_420_c_erased;
use crate::src::filmgrain::generate_grain_uv_422_c_erased;
use crate::src::filmgrain::generate_grain_uv_444_c_erased;
use crate::src::filmgrain::generate_grain_y_c_erased;
use crate::src::filmgrain::Rav1dFilmGrainDSPContext;
use crate::src::filmgrain::GRAIN_WIDTH;
use libc::intptr_t;
use libc::ptrdiff_t;
use std::ffi::c_int;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
use crate::src::filmgrain::get_random_number;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
use std::ffi::c_uint;

#[cfg(feature = "asm")]
use crate::src::cpu::{rav1d_get_cpu_flags, CpuFlags};

#[cfg(feature = "asm")]
use cfg_if::cfg_if;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64")))]
extern "C" {
    fn dav1d_fguv_32x32xn_i422_16bpc_ssse3(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv_pl: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_uv_444_16bpc_ssse3(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        buf_y: *const [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_uv_422_16bpc_ssse3(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        buf_y: *const [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: c_int,
    );
    fn dav1d_fguv_32x32xn_i420_16bpc_ssse3(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv_pl: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_fgy_32x32xn_16bpc_ssse3(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_uv_420_16bpc_ssse3(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        buf_y: *const [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_y_16bpc_ssse3(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        bitdepth_max: c_int,
    );
    fn dav1d_fguv_32x32xn_i444_16bpc_ssse3(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv_pl: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
    );
}

#[cfg(all(feature = "asm", target_arch = "x86_64"))]
extern "C" {
    fn dav1d_fguv_32x32xn_i422_16bpc_avx512icl(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv_pl: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_uv_444_16bpc_avx2(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        buf_y: *const [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: c_int,
    );
    fn dav1d_fgy_32x32xn_16bpc_avx2(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_fguv_32x32xn_i420_16bpc_avx2(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv_pl: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_fguv_32x32xn_i422_16bpc_avx2(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv_pl: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_fguv_32x32xn_i444_16bpc_avx2(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv_pl: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_fgy_32x32xn_16bpc_avx512icl(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_fguv_32x32xn_i420_16bpc_avx512icl(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv_pl: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_fguv_32x32xn_i444_16bpc_avx512icl(
        dst_row: *mut DynPixel,
        src_row: *const DynPixel,
        stride: ptrdiff_t,
        data: *const Rav1dFilmGrainData,
        pw: usize,
        scaling: *const u8,
        grain_lut: *const [DynEntry; GRAIN_WIDTH],
        bh: c_int,
        row_num: c_int,
        luma_row: *const DynPixel,
        luma_stride: ptrdiff_t,
        uv_pl: c_int,
        is_id: c_int,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_uv_420_16bpc_avx2(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        buf_y: *const [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_y_16bpc_avx2(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_uv_422_16bpc_avx2(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        buf_y: *const [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
extern "C" {
    fn dav1d_fguv_32x32_420_16bpc_neon(
        dst: *mut pixel,
        src: *const pixel,
        stride: ptrdiff_t,
        scaling: *const u8,
        data: *const Rav1dFilmGrainData,
        grain_lut: *const [entry; GRAIN_WIDTH],
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        offsets: *const [c_int; 2],
        h: ptrdiff_t,
        uv: ptrdiff_t,
        is_id: ptrdiff_t,
        type_0: ptrdiff_t,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_uv_422_16bpc_neon(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        buf_y: *const [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_uv_444_16bpc_neon(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        buf_y: *const [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_y_16bpc_neon(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        bitdepth_max: c_int,
    );
    fn dav1d_generate_grain_uv_420_16bpc_neon(
        buf: *mut [DynEntry; GRAIN_WIDTH],
        buf_y: *const [DynEntry; GRAIN_WIDTH],
        data: *const Rav1dFilmGrainData,
        uv: intptr_t,
        bitdepth_max: c_int,
    );
    fn dav1d_fgy_32x32_16bpc_neon(
        dst: *mut pixel,
        src: *const pixel,
        stride: ptrdiff_t,
        scaling: *const u8,
        scaling_shift: c_int,
        grain_lut: *const [entry; GRAIN_WIDTH],
        offsets: *const [c_int; 2],
        h: c_int,
        clip: ptrdiff_t,
        type_0: ptrdiff_t,
        bitdepth_max: c_int,
    );
    fn dav1d_fguv_32x32_422_16bpc_neon(
        dst: *mut pixel,
        src: *const pixel,
        stride: ptrdiff_t,
        scaling: *const u8,
        data: *const Rav1dFilmGrainData,
        grain_lut: *const [entry; GRAIN_WIDTH],
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        offsets: *const [c_int; 2],
        h: ptrdiff_t,
        uv: ptrdiff_t,
        is_id: ptrdiff_t,
        type_0: ptrdiff_t,
        bitdepth_max: c_int,
    );
    fn dav1d_fguv_32x32_444_16bpc_neon(
        dst: *mut pixel,
        src: *const pixel,
        stride: ptrdiff_t,
        scaling: *const u8,
        data: *const Rav1dFilmGrainData,
        grain_lut: *const [entry; GRAIN_WIDTH],
        luma_row: *const pixel,
        luma_stride: ptrdiff_t,
        offsets: *const [c_int; 2],
        h: ptrdiff_t,
        uv: ptrdiff_t,
        is_id: ptrdiff_t,
        type_0: ptrdiff_t,
        bitdepth_max: c_int,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
pub type pixel = u16;

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64")))]
pub type entry = i16;

#[cfg(all(feature = "asm", any(target_arch = "x86", target_arch = "x86_64"),))]
#[inline(always)]
unsafe fn film_grain_dsp_init_x86(c: *mut Rav1dFilmGrainDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::SSSE3) {
        return;
    }

    (*c).generate_grain_y = Some(dav1d_generate_grain_y_16bpc_ssse3);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(dav1d_generate_grain_uv_420_16bpc_ssse3);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(dav1d_generate_grain_uv_422_16bpc_ssse3);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(dav1d_generate_grain_uv_444_16bpc_ssse3);

    (*c).fgy_32x32xn = Some(dav1d_fgy_32x32xn_16bpc_ssse3);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(dav1d_fguv_32x32xn_i420_16bpc_ssse3);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(dav1d_fguv_32x32xn_i422_16bpc_ssse3);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(dav1d_fguv_32x32xn_i444_16bpc_ssse3);

    #[cfg(target_arch = "x86_64")]
    {
        if !flags.contains(CpuFlags::AVX2) {
            return;
        }

        (*c).generate_grain_y = Some(dav1d_generate_grain_y_16bpc_avx2);
        (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
            Some(dav1d_generate_grain_uv_420_16bpc_avx2);
        (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
            Some(dav1d_generate_grain_uv_422_16bpc_avx2);
        (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
            Some(dav1d_generate_grain_uv_444_16bpc_avx2);

        if !flags.contains(CpuFlags::SLOW_GATHER) {
            (*c).fgy_32x32xn = Some(dav1d_fgy_32x32xn_16bpc_avx2);
            (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
                Some(dav1d_fguv_32x32xn_i420_16bpc_avx2);
            (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
                Some(dav1d_fguv_32x32xn_i422_16bpc_avx2);
            (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
                Some(dav1d_fguv_32x32xn_i444_16bpc_avx2);
        }

        if !flags.contains(CpuFlags::AVX512ICL) {
            return;
        }

        (*c).fgy_32x32xn = Some(dav1d_fgy_32x32xn_16bpc_avx512icl);
        (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
            Some(dav1d_fguv_32x32xn_i420_16bpc_avx512icl);
        (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
            Some(dav1d_fguv_32x32xn_i422_16bpc_avx512icl);
        (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
            Some(dav1d_fguv_32x32xn_i444_16bpc_avx512icl);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
#[inline(always)]
unsafe fn film_grain_dsp_init_arm(c: *mut Rav1dFilmGrainDSPContext) {
    let flags = rav1d_get_cpu_flags();

    if !flags.contains(CpuFlags::NEON) {
        return;
    }

    (*c).generate_grain_y = Some(dav1d_generate_grain_y_16bpc_neon);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(dav1d_generate_grain_uv_420_16bpc_neon);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(dav1d_generate_grain_uv_422_16bpc_neon);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(dav1d_generate_grain_uv_444_16bpc_neon);

    (*c).fgy_32x32xn = Some(fgy_32x32xn_neon_erased);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] = Some(fguv_32x32xn_420_neon_erased);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] = Some(fguv_32x32xn_422_neon_erased);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] = Some(fguv_32x32xn_444_neon_erased);
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn fgy_32x32xn_neon_erased(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) {
    fgy_32x32xn_neon(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        bitdepth_max,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe fn fgy_32x32xn_neon(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    bitdepth_max: c_int,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag && row_num != 0 {
            type_0 |= 1 as c_int;
        }
        if (*data).overlap_flag && bx != 0 {
            type_0 |= 2 as c_int;
        }
        dav1d_fgy_32x32_16bpc_neon(
            dst_row.offset(bx as isize),
            src_row.offset(bx as isize),
            stride,
            scaling,
            (*data).scaling_shift,
            grain_lut,
            offsets.as_mut_ptr() as *const [c_int; 2],
            bh,
            (*data).clip_to_restricted_range as ptrdiff_t,
            type_0 as ptrdiff_t,
            bitdepth_max,
        );
        bx = bx.wrapping_add(32 as c_int as c_uint);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn fguv_32x32xn_420_neon_erased(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    fguv_32x32xn_420_neon(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        luma_row.cast(),
        luma_stride,
        uv,
        is_id,
        bitdepth_max,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe fn fguv_32x32xn_420_neon(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag && row_num != 0 {
            type_0 |= 1 as c_int;
        }
        if (*data).overlap_flag && bx != 0 {
            type_0 |= 2 as c_int;
        }
        if (*data).chroma_scaling_from_luma {
            type_0 |= 4 as c_int;
        }
        dav1d_fguv_32x32_420_16bpc_neon(
            dst_row.offset(bx as isize),
            src_row.offset(bx as isize),
            stride,
            scaling,
            data,
            grain_lut,
            luma_row.offset((bx << 1) as isize),
            luma_stride,
            offsets.as_mut_ptr() as *const [c_int; 2],
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            type_0 as ptrdiff_t,
            bitdepth_max,
        );
        bx = bx.wrapping_add((32 >> 1) as c_uint);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn fguv_32x32xn_422_neon_erased(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    fguv_32x32xn_422_neon(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        luma_row.cast(),
        luma_stride,
        uv,
        is_id,
        bitdepth_max,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe fn fguv_32x32xn_422_neon(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag && row_num != 0 {
            type_0 |= 1 as c_int;
        }
        if (*data).overlap_flag && bx != 0 {
            type_0 |= 2 as c_int;
        }
        if (*data).chroma_scaling_from_luma {
            type_0 |= 4 as c_int;
        }
        dav1d_fguv_32x32_422_16bpc_neon(
            dst_row.offset(bx as isize),
            src_row.offset(bx as isize),
            stride,
            scaling,
            data,
            grain_lut,
            luma_row.offset((bx << 1) as isize),
            luma_stride,
            offsets.as_mut_ptr() as *const [c_int; 2],
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            type_0 as ptrdiff_t,
            bitdepth_max,
        );
        bx = bx.wrapping_add((32 >> 1) as c_uint);
    }
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe extern "C" fn fguv_32x32xn_444_neon_erased(
    dst_row: *mut DynPixel,
    src_row: *const DynPixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [DynEntry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const DynPixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    fguv_32x32xn_444_neon(
        dst_row.cast(),
        src_row.cast(),
        stride,
        data,
        pw,
        scaling,
        grain_lut.cast(),
        bh,
        row_num,
        luma_row.cast(),
        luma_stride,
        uv,
        is_id,
        bitdepth_max,
    );
}

#[cfg(all(feature = "asm", any(target_arch = "arm", target_arch = "aarch64"),))]
unsafe fn fguv_32x32xn_444_neon(
    dst_row: *mut pixel,
    src_row: *const pixel,
    stride: ptrdiff_t,
    data: *const Rav1dFilmGrainData,
    pw: usize,
    scaling: *const u8,
    grain_lut: *const [entry; GRAIN_WIDTH],
    bh: c_int,
    row_num: c_int,
    luma_row: *const pixel,
    luma_stride: ptrdiff_t,
    uv: c_int,
    is_id: c_int,
    bitdepth_max: c_int,
) {
    let rows = 1 + ((*data).overlap_flag && row_num > 0) as c_int;
    let mut seed: [c_uint; 2] = [0; 2];
    let mut i = 0;
    while i < rows {
        seed[i as usize] = (*data).seed;
        seed[i as usize] ^= (((row_num - i) * 37 + 178 & 0xff as c_int) << 8) as c_uint;
        seed[i as usize] ^= ((row_num - i) * 173 + 105 & 0xff as c_int) as c_uint;
        i += 1;
    }
    let mut offsets: [[c_int; 2]; 2] = [[0; 2]; 2];
    let mut bx: c_uint = 0 as c_int as c_uint;
    while (bx as usize) < pw {
        if (*data).overlap_flag && bx != 0 {
            let mut i_0 = 0;
            while i_0 < rows {
                offsets[1][i_0 as usize] = offsets[0][i_0 as usize];
                i_0 += 1;
            }
        }
        let mut i_1 = 0;
        while i_1 < rows {
            offsets[0][i_1 as usize] =
                get_random_number(8 as c_int, &mut *seed.as_mut_ptr().offset(i_1 as isize));
            i_1 += 1;
        }
        let mut type_0 = 0;
        if (*data).overlap_flag && row_num != 0 {
            type_0 |= 1 as c_int;
        }
        if (*data).overlap_flag && bx != 0 {
            type_0 |= 2 as c_int;
        }
        if (*data).chroma_scaling_from_luma {
            type_0 |= 4 as c_int;
        }
        dav1d_fguv_32x32_444_16bpc_neon(
            dst_row.offset(bx as isize),
            src_row.offset(bx as isize),
            stride,
            scaling,
            data,
            grain_lut,
            luma_row.offset((bx << 0) as isize),
            luma_stride,
            offsets.as_mut_ptr() as *const [c_int; 2],
            bh as ptrdiff_t,
            uv as ptrdiff_t,
            is_id as ptrdiff_t,
            type_0 as ptrdiff_t,
            bitdepth_max,
        );
        bx = bx.wrapping_add((32 >> 0) as c_uint);
    }
}

#[cold]
pub unsafe fn rav1d_film_grain_dsp_init_16bpc(c: *mut Rav1dFilmGrainDSPContext) {
    (*c).generate_grain_y = Some(generate_grain_y_c_erased::<BitDepth16>);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(generate_grain_uv_420_c_erased::<BitDepth16>);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(generate_grain_uv_422_c_erased::<BitDepth16>);
    (*c).generate_grain_uv[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(generate_grain_uv_444_c_erased::<BitDepth16>);

    (*c).fgy_32x32xn = Some(fgy_32x32xn_c_erased::<BitDepth16>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I420 - 1) as usize] =
        Some(fguv_32x32xn_420_c_erased::<BitDepth16>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I422 - 1) as usize] =
        Some(fguv_32x32xn_422_c_erased::<BitDepth16>);
    (*c).fguv_32x32xn[(RAV1D_PIXEL_LAYOUT_I444 - 1) as usize] =
        Some(fguv_32x32xn_444_c_erased::<BitDepth16>);

    #[cfg(feature = "asm")]
    cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            film_grain_dsp_init_x86(c);
        } else if #[cfg(any(target_arch = "arm", target_arch = "aarch64"))] {
            film_grain_dsp_init_arm(c);
        }
    }
}
