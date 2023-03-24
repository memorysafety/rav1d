use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    static dav1d_block_dimensions: [[uint8_t; 4]; 22];
    static dav1d_txfm_dimensions: [TxfmInfo; 19];
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ptrdiff_t = libc::c_long;

#[repr(C)]
#[derive(Copy, Clone)]
pub union alias64 {
    pub u64_0: uint64_t,
    pub u8_0: [uint8_t; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union alias32 {
    pub u32_0: uint32_t,
    pub u8_0: [uint8_t; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union alias16 {
    pub u16_0: uint16_t,
    pub u8_0: [uint8_t; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union alias8 {
    pub u8_0: uint8_t,
}
pub type Dav1dTxfmMode = libc::c_uint;
pub const DAV1D_N_TX_MODES: Dav1dTxfmMode = 3;
pub const DAV1D_TX_SWITCHABLE: Dav1dTxfmMode = 2;
pub const DAV1D_TX_LARGEST: Dav1dTxfmMode = 1;
pub const DAV1D_TX_4X4_ONLY: Dav1dTxfmMode = 0;
pub type Dav1dFilterMode = libc::c_uint;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub const DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
pub type Dav1dRestorationType = libc::c_uint;
pub const DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType = 3;
pub const DAV1D_RESTORATION_WIENER: Dav1dRestorationType = 2;
pub const DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType = 1;
pub const DAV1D_RESTORATION_NONE: Dav1dRestorationType = 0;
pub type Dav1dWarpedMotionType = libc::c_uint;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    pub p: C2RustUnnamed_0,
    pub abcd: [int16_t; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}
pub type Dav1dPixelLayout = libc::c_uint;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
pub type Dav1dFrameType = libc::c_uint;
pub const DAV1D_FRAME_TYPE_SWITCH: Dav1dFrameType = 3;
pub const DAV1D_FRAME_TYPE_INTRA: Dav1dFrameType = 2;
pub const DAV1D_FRAME_TYPE_INTER: Dav1dFrameType = 1;
pub const DAV1D_FRAME_TYPE_KEY: Dav1dFrameType = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dSegmentationData {
    pub delta_q: libc::c_int,
    pub delta_lf_y_v: libc::c_int,
    pub delta_lf_y_h: libc::c_int,
    pub delta_lf_u: libc::c_int,
    pub delta_lf_v: libc::c_int,
    pub ref_0: libc::c_int,
    pub skip: libc::c_int,
    pub globalmv: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; 8],
    pub preskip: libc::c_int,
    pub last_active_segid: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [libc::c_int; 2],
    pub ref_delta: [libc::c_int; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dFilmGrainData {
    pub seed: libc::c_uint,
    pub num_y_points: libc::c_int,
    pub y_points: [[uint8_t; 2]; 14],
    pub chroma_scaling_from_luma: libc::c_int,
    pub num_uv_points: [libc::c_int; 2],
    pub uv_points: [[[uint8_t; 2]; 10]; 2],
    pub scaling_shift: libc::c_int,
    pub ar_coeff_lag: libc::c_int,
    pub ar_coeffs_y: [int8_t; 24],
    pub ar_coeffs_uv: [[int8_t; 28]; 2],
    pub ar_coeff_shift: uint64_t,
    pub grain_scale_shift: libc::c_int,
    pub uv_mult: [libc::c_int; 2],
    pub uv_luma_mult: [libc::c_int; 2],
    pub uv_offset: [libc::c_int; 2],
    pub overlap_flag: libc::c_int,
    pub clip_to_restricted_range: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dFrameHeader {
    pub film_grain: C2RustUnnamed_11,
    pub frame_type: Dav1dFrameType,
    pub width: [libc::c_int; 2],
    pub height: libc::c_int,
    pub frame_offset: libc::c_int,
    pub temporal_id: libc::c_int,
    pub spatial_id: libc::c_int,
    pub show_existing_frame: libc::c_int,
    pub existing_frame_idx: libc::c_int,
    pub frame_id: libc::c_int,
    pub frame_presentation_delay: libc::c_int,
    pub show_frame: libc::c_int,
    pub showable_frame: libc::c_int,
    pub error_resilient_mode: libc::c_int,
    pub disable_cdf_update: libc::c_int,
    pub allow_screen_content_tools: libc::c_int,
    pub force_integer_mv: libc::c_int,
    pub frame_size_override: libc::c_int,
    pub primary_ref_frame: libc::c_int,
    pub buffer_removal_time_present: libc::c_int,
    pub operating_points: [Dav1dFrameHeaderOperatingPoint; 32],
    pub refresh_frame_flags: libc::c_int,
    pub render_width: libc::c_int,
    pub render_height: libc::c_int,
    pub super_res: C2RustUnnamed_10,
    pub have_render_size: libc::c_int,
    pub allow_intrabc: libc::c_int,
    pub frame_ref_short_signaling: libc::c_int,
    pub refidx: [libc::c_int; 7],
    pub hp: libc::c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: libc::c_int,
    pub use_ref_frame_mvs: libc::c_int,
    pub refresh_context: libc::c_int,
    pub tiling: C2RustUnnamed_9,
    pub quant: C2RustUnnamed_8,
    pub segmentation: C2RustUnnamed_7,
    pub delta: C2RustUnnamed_4,
    pub all_lossless: libc::c_int,
    pub loopfilter: C2RustUnnamed_3,
    pub cdef: C2RustUnnamed_2,
    pub restoration: C2RustUnnamed_1,
    pub txfm_mode: Dav1dTxfmMode,
    pub switchable_comp_refs: libc::c_int,
    pub skip_mode_allowed: libc::c_int,
    pub skip_mode_enabled: libc::c_int,
    pub skip_mode_refs: [libc::c_int; 2],
    pub warp_motion: libc::c_int,
    pub reduced_txtp_set: libc::c_int,
    pub gmv: [Dav1dWarpedMotionParams; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub type_0: [Dav1dRestorationType; 3],
    pub unit_size: [libc::c_int; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_2 {
    pub damping: libc::c_int,
    pub n_bits: libc::c_int,
    pub y_strength: [libc::c_int; 8],
    pub uv_strength: [libc::c_int; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_3 {
    pub level_y: [libc::c_int; 2],
    pub level_u: libc::c_int,
    pub level_v: libc::c_int,
    pub mode_ref_delta_enabled: libc::c_int,
    pub mode_ref_delta_update: libc::c_int,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_4 {
    pub q: C2RustUnnamed_6,
    pub lf: C2RustUnnamed_5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_5 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
    pub multi: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_6 {
    pub present: libc::c_int,
    pub res_log2: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_7 {
    pub enabled: libc::c_int,
    pub update_map: libc::c_int,
    pub temporal: libc::c_int,
    pub update_data: libc::c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [libc::c_int; 8],
    pub qidx: [libc::c_int; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_8 {
    pub yac: libc::c_int,
    pub ydc_delta: libc::c_int,
    pub udc_delta: libc::c_int,
    pub uac_delta: libc::c_int,
    pub vdc_delta: libc::c_int,
    pub vac_delta: libc::c_int,
    pub qm: libc::c_int,
    pub qm_y: libc::c_int,
    pub qm_u: libc::c_int,
    pub qm_v: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_9 {
    pub uniform: libc::c_int,
    pub n_bytes: libc::c_uint,
    pub min_log2_cols: libc::c_int,
    pub max_log2_cols: libc::c_int,
    pub log2_cols: libc::c_int,
    pub cols: libc::c_int,
    pub min_log2_rows: libc::c_int,
    pub max_log2_rows: libc::c_int,
    pub log2_rows: libc::c_int,
    pub rows: libc::c_int,
    pub col_start_sb: [uint16_t; 65],
    pub row_start_sb: [uint16_t; 65],
    pub update: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_10 {
    pub width_scale_denominator: libc::c_int,
    pub enabled: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_11 {
    pub data: Dav1dFilmGrainData,
    pub present: libc::c_int,
    pub update: libc::c_int,
}
pub type TxfmSize = libc::c_uint;
pub const N_TX_SIZES: TxfmSize = 5;
pub const TX_64X64: TxfmSize = 4;
pub const TX_32X32: TxfmSize = 3;
pub const TX_16X16: TxfmSize = 2;
pub const TX_8X8: TxfmSize = 1;
pub const TX_4X4: TxfmSize = 0;
pub type RectTxfmSize = libc::c_uint;
pub const N_RECT_TX_SIZES: RectTxfmSize = 19;
pub const RTX_64X16: RectTxfmSize = 18;
pub const RTX_16X64: RectTxfmSize = 17;
pub const RTX_32X8: RectTxfmSize = 16;
pub const RTX_8X32: RectTxfmSize = 15;
pub const RTX_16X4: RectTxfmSize = 14;
pub const RTX_4X16: RectTxfmSize = 13;
pub const RTX_64X32: RectTxfmSize = 12;
pub const RTX_32X64: RectTxfmSize = 11;
pub const RTX_32X16: RectTxfmSize = 10;
pub const RTX_16X32: RectTxfmSize = 9;
pub const RTX_16X8: RectTxfmSize = 8;
pub const RTX_8X16: RectTxfmSize = 7;
pub const RTX_8X4: RectTxfmSize = 6;
pub const RTX_4X8: RectTxfmSize = 5;
pub type BlockSize = libc::c_uint;
pub const N_BS_SIZES: BlockSize = 22;
pub const BS_4x4: BlockSize = 21;
pub const BS_4x8: BlockSize = 20;
pub const BS_4x16: BlockSize = 19;
pub const BS_8x4: BlockSize = 18;
pub const BS_8x8: BlockSize = 17;
pub const BS_8x16: BlockSize = 16;
pub const BS_8x32: BlockSize = 15;
pub const BS_16x4: BlockSize = 14;
pub const BS_16x8: BlockSize = 13;
pub const BS_16x16: BlockSize = 12;
pub const BS_16x32: BlockSize = 11;
pub const BS_16x64: BlockSize = 10;
pub const BS_32x8: BlockSize = 9;
pub const BS_32x16: BlockSize = 8;
pub const BS_32x32: BlockSize = 7;
pub const BS_32x64: BlockSize = 6;
pub const BS_64x16: BlockSize = 5;
pub const BS_64x32: BlockSize = 4;
pub const BS_64x64: BlockSize = 3;
pub const BS_64x128: BlockSize = 2;
pub const BS_128x64: BlockSize = 1;
pub const BS_128x128: BlockSize = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Av1FilterLUT {
    pub e: [uint8_t; 64],
    pub i: [uint8_t; 64],
    pub sharp: [uint64_t; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Av1Filter {
    pub filter_y: [[[[uint16_t; 2]; 3]; 32]; 2],
    pub filter_uv: [[[[uint16_t; 2]; 2]; 32]; 2],
    pub cdef_idx: [int8_t; 4],
    pub noskip_mask: [[uint16_t; 2]; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TxfmInfo {
    pub w: uint8_t,
    pub h: uint8_t,
    pub lw: uint8_t,
    pub lh: uint8_t,
    pub min: uint8_t,
    pub max: uint8_t,
    pub sub: uint8_t,
    pub ctx: uint8_t,
}
#[inline]
unsafe extern "C" fn iclip(v: libc::c_int, min: libc::c_int, max: libc::c_int) -> libc::c_int {
    return if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
}
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
unsafe extern "C" fn decomp_tx(
    txa: *mut [[[uint8_t; 32]; 32]; 2],
    from: RectTxfmSize,
    depth: libc::c_int,
    y_off: libc::c_int,
    x_off: libc::c_int,
    tx_masks: *const uint16_t,
) {
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(from as isize) as *const TxfmInfo;
    let is_split: libc::c_int = if from == TX_4X4 || depth > 1i32 {
        0i32
    } else {
        *tx_masks.offset(depth as isize) as libc::c_int >> y_off * 4i32 + x_off & 1i32
    };
    if is_split != 0 {
        let sub: RectTxfmSize = (*t_dim).sub as RectTxfmSize;
        let htw4: libc::c_int = (*t_dim).w as libc::c_int >> 1i32;
        let hth4: libc::c_int = (*t_dim).h as libc::c_int >> 1i32;
        decomp_tx(
            txa,
            sub,
            depth + 1i32,
            y_off * 2i32 + 0i32,
            x_off * 2i32 + 0i32,
            tx_masks,
        );
        if (*t_dim).w as libc::c_int >= (*t_dim).h as libc::c_int {
            decomp_tx(
                &mut *(*(*(*txa.offset(0isize)).as_mut_ptr().offset(0isize))
                    .as_mut_ptr()
                    .offset(0isize))
                .as_mut_ptr()
                .offset(htw4 as isize) as *mut uint8_t
                    as *mut [[[uint8_t; 32]; 32]; 2],
                sub,
                depth + 1i32,
                y_off * 2i32 + 0i32,
                x_off * 2i32 + 1i32,
                tx_masks,
            );
        }
        if (*t_dim).h as libc::c_int >= (*t_dim).w as libc::c_int {
            decomp_tx(
                &mut *(*(*(*txa.offset(0isize)).as_mut_ptr().offset(0isize))
                    .as_mut_ptr()
                    .offset(hth4 as isize))
                .as_mut_ptr()
                .offset(0isize) as *mut uint8_t as *mut [[[uint8_t; 32]; 32]; 2],
                sub,
                depth + 1i32,
                y_off * 2i32 + 1i32,
                x_off * 2i32 + 0i32,
                tx_masks,
            );
            if (*t_dim).w as libc::c_int >= (*t_dim).h as libc::c_int {
                decomp_tx(
                    &mut *(*(*(*txa.offset(0isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(hth4 as isize))
                    .as_mut_ptr()
                    .offset(htw4 as isize) as *mut uint8_t
                        as *mut [[[uint8_t; 32]; 32]; 2],
                    sub,
                    depth + 1i32,
                    y_off * 2i32 + 1i32,
                    x_off * 2i32 + 1i32,
                    tx_masks,
                );
            }
        }
    } else {
        let lw: libc::c_int = imin(2i32, (*t_dim).lw as libc::c_int);
        let lh: libc::c_int = imin(2i32, (*t_dim).lh as libc::c_int);
        match (*t_dim).w as libc::c_int {
            1 => {
                let mut y: libc::c_int = 0i32;
                while y < (*t_dim).h as libc::c_int {
                    (*(&mut *(*(*(*txa.offset(0isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y as isize))
                    .as_mut_ptr()
                    .offset(0isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1i32 * lw) as uint8_t;
                    (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y as isize))
                    .as_mut_ptr()
                    .offset(0isize) as *mut uint8_t as *mut alias8))
                        .u8_0 = (0x1i32 * lh) as uint8_t;
                    (*txa.offset(0isize))[1usize][y as usize][0usize] = (*t_dim).w;
                    y += 1;
                }
            }
            2 => {
                let mut y_0: libc::c_int = 0i32;
                while y_0 < (*t_dim).h as libc::c_int {
                    (*(&mut *(*(*(*txa.offset(0isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_0 as isize))
                    .as_mut_ptr()
                    .offset(0isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101i32 * lw) as uint16_t;
                    (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_0 as isize))
                    .as_mut_ptr()
                    .offset(0isize) as *mut uint8_t as *mut alias16))
                        .u16_0 = (0x101i32 * lh) as uint16_t;
                    (*txa.offset(0isize))[1usize][y_0 as usize][0usize] = (*t_dim).w;
                    y_0 += 1;
                }
            }
            4 => {
                let mut y_1: libc::c_int = 0i32;
                while y_1 < (*t_dim).h as libc::c_int {
                    (*(&mut *(*(*(*txa.offset(0isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_1 as isize))
                    .as_mut_ptr()
                    .offset(0isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101u32).wrapping_mul(lw as libc::c_uint);
                    (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_1 as isize))
                    .as_mut_ptr()
                    .offset(0isize) as *mut uint8_t as *mut alias32))
                        .u32_0 = (0x1010101u32).wrapping_mul(lh as libc::c_uint);
                    (*txa.offset(0isize))[1usize][y_1 as usize][0usize] = (*t_dim).w;
                    y_1 += 1;
                }
            }
            8 => {
                let mut y_2: libc::c_int = 0i32;
                while y_2 < (*t_dim).h as libc::c_int {
                    (*(&mut *(*(*(*txa.offset(0isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_2 as isize))
                    .as_mut_ptr()
                    .offset(0isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101u64).wrapping_mul(lw as libc::c_ulonglong);
                    (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_2 as isize))
                    .as_mut_ptr()
                    .offset(0isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = (0x101010101010101u64).wrapping_mul(lh as libc::c_ulonglong);
                    (*txa.offset(0isize))[1usize][y_2 as usize][0usize] = (*t_dim).w;
                    y_2 += 1;
                }
            }
            16 => {
                let mut y_3: libc::c_int = 0i32;
                while y_3 < (*t_dim).h as libc::c_int {
                    let const_val: uint64_t =
                        (0x101010101010101u64).wrapping_mul(lw as libc::c_ulonglong);
                    (*(&mut *(*(*(*txa.offset(0isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_3 as isize))
                    .as_mut_ptr()
                    .offset((0i32 + 0i32) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val;
                    (*(&mut *(*(*(*txa.offset(0isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_3 as isize))
                    .as_mut_ptr()
                    .offset((0i32 + 8i32) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val;
                    let const_val_0: uint64_t =
                        (0x101010101010101u64).wrapping_mul(lh as libc::c_ulonglong);
                    (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_3 as isize))
                    .as_mut_ptr()
                    .offset((0i32 + 0i32) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_0;
                    (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(0isize))
                        .as_mut_ptr()
                        .offset(y_3 as isize))
                    .as_mut_ptr()
                    .offset((0i32 + 8i32) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_0;
                    (*txa.offset(0isize))[1usize][y_3 as usize][0usize] = (*t_dim).w;
                    y_3 += 1;
                }
            }
            _ => {}
        }
        match (*t_dim).w as libc::c_int {
            1 => {
                (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(1isize))
                    .as_mut_ptr()
                    .offset(0isize))
                .as_mut_ptr()
                .offset(0isize) as *mut uint8_t as *mut alias8))
                    .u8_0 = (0x1i32 * (*t_dim).h as libc::c_int) as uint8_t;
            }
            2 => {
                (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(1isize))
                    .as_mut_ptr()
                    .offset(0isize))
                .as_mut_ptr()
                .offset(0isize) as *mut uint8_t as *mut alias16))
                    .u16_0 = (0x101i32 * (*t_dim).h as libc::c_int) as uint16_t;
            }
            4 => {
                (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(1isize))
                    .as_mut_ptr()
                    .offset(0isize))
                .as_mut_ptr()
                .offset(0isize) as *mut uint8_t as *mut alias32))
                    .u32_0 = (0x1010101u32).wrapping_mul((*t_dim).h as libc::c_uint);
            }
            8 => {
                (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(1isize))
                    .as_mut_ptr()
                    .offset(0isize))
                .as_mut_ptr()
                .offset(0isize) as *mut uint8_t as *mut alias64))
                    .u64_0 = (0x101010101010101u64).wrapping_mul((*t_dim).h as libc::c_ulonglong);
            }
            16 => {
                let const_val_1: uint64_t =
                    (0x101010101010101u64).wrapping_mul((*t_dim).h as libc::c_ulonglong);
                (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(1isize))
                    .as_mut_ptr()
                    .offset(0isize))
                .as_mut_ptr()
                .offset((0i32 + 0i32) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_1;
                (*(&mut *(*(*(*txa.offset(1isize)).as_mut_ptr().offset(1isize))
                    .as_mut_ptr()
                    .offset(0isize))
                .as_mut_ptr()
                .offset((0i32 + 8i32) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_1;
            }
            _ => {}
        }
    };
}
#[inline]
unsafe extern "C" fn mask_edges_inter(
    masks: *mut [[[uint16_t; 2]; 3]; 32],
    by4: libc::c_int,
    bx4: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
    skip: libc::c_int,
    max_tx: RectTxfmSize,
    tx_masks: *const uint16_t,
    a: *mut uint8_t,
    l: *mut uint8_t,
) {
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(max_tx as isize) as *const TxfmInfo;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut txa: [[[[uint8_t; 32]; 32]; 2]; 2] = [[[[0; 32]; 32]; 2]; 2];
    let mut y_off: libc::c_int = 0i32;
    let mut y_0: libc::c_int = 0i32;
    while y_0 < h4 {
        let mut x_off: libc::c_int = 0i32;
        let mut x_0: libc::c_int = 0i32;
        while x_0 < w4 {
            decomp_tx(
                &mut *(*(*(*txa.as_mut_ptr().offset(0isize))
                    .as_mut_ptr()
                    .offset(0isize))
                .as_mut_ptr()
                .offset(y_0 as isize))
                .as_mut_ptr()
                .offset(x_0 as isize) as *mut uint8_t
                    as *mut [[[uint8_t; 32]; 32]; 2],
                max_tx,
                0i32,
                y_off,
                x_off,
                tx_masks,
            );
            x_0 += (*t_dim).w as libc::c_int;
            x_off += 1;
        }
        y_0 += (*t_dim).h as libc::c_int;
        y_off += 1;
    }
    let mut mask: libc::c_uint = (1u32) << by4;
    y = 0i32;
    while y < h4 {
        let sidx: libc::c_int = (mask >= 0x10000u32) as libc::c_int;
        let smask: libc::c_uint = mask >> (sidx << 4i32);
        let ref mut fresh0 = (*masks.offset(0isize))[bx4 as usize][imin(
            txa[0usize][0usize][y as usize][0usize] as libc::c_int,
            *l.offset(y as isize) as libc::c_int,
        ) as usize][sidx as usize];
        *fresh0 = (*fresh0 as libc::c_uint | smask) as uint16_t;
        y += 1;
        mask <<= 1i32;
    }
    x = 0i32;
    mask = (1u32) << bx4;
    while x < w4 {
        let sidx_0: libc::c_int = (mask >= 0x10000u32) as libc::c_int;
        let smask_0: libc::c_uint = mask >> (sidx_0 << 4i32);
        let ref mut fresh1 = (*masks.offset(1isize))[by4 as usize][imin(
            txa[1usize][0usize][0usize][x as usize] as libc::c_int,
            *a.offset(x as isize) as libc::c_int,
        ) as usize][sidx_0 as usize];
        *fresh1 = (*fresh1 as libc::c_uint | smask_0) as uint16_t;
        x += 1;
        mask <<= 1i32;
    }
    if skip == 0 {
        y = 0i32;
        mask = (1u32) << by4;
        while y < h4 {
            let sidx_1: libc::c_int = (mask >= 0x10000u32) as libc::c_int;
            let smask_1: libc::c_uint = mask >> (sidx_1 << 4i32);
            let mut ltx: libc::c_int = txa[0usize][0usize][y as usize][0usize] as libc::c_int;
            let mut step: libc::c_int = txa[0usize][1usize][y as usize][0usize] as libc::c_int;
            x = step;
            while x < w4 {
                let rtx: libc::c_int = txa[0usize][0usize][y as usize][x as usize] as libc::c_int;
                let ref mut fresh2 = (*masks.offset(0isize))[(bx4 + x) as usize]
                    [imin(rtx, ltx) as usize][sidx_1 as usize];
                *fresh2 = (*fresh2 as libc::c_uint | smask_1) as uint16_t;
                ltx = rtx;
                step = txa[0usize][1usize][y as usize][x as usize] as libc::c_int;
                x += step;
            }
            y += 1;
            mask <<= 1i32;
        }
        x = 0i32;
        mask = (1u32) << bx4;
        while x < w4 {
            let sidx_2: libc::c_int = (mask >= 0x10000u32) as libc::c_int;
            let smask_2: libc::c_uint = mask >> (sidx_2 << 4i32);
            let mut ttx: libc::c_int = txa[1usize][0usize][0usize][x as usize] as libc::c_int;
            let mut step_0: libc::c_int = txa[1usize][1usize][0usize][x as usize] as libc::c_int;
            y = step_0;
            while y < h4 {
                let btx: libc::c_int = txa[1usize][0usize][y as usize][x as usize] as libc::c_int;
                let ref mut fresh3 = (*masks.offset(1isize))[(by4 + y) as usize]
                    [imin(ttx, btx) as usize][sidx_2 as usize];
                *fresh3 = (*fresh3 as libc::c_uint | smask_2) as uint16_t;
                ttx = btx;
                step_0 = txa[1usize][1usize][y as usize][x as usize] as libc::c_int;
                y += step_0;
            }
            x += 1;
            mask <<= 1i32;
        }
    }
    y = 0i32;
    while y < h4 {
        *l.offset(y as isize) = txa[0usize][0usize][y as usize][(w4 - 1i32) as usize];
        y += 1;
    }
    memcpy(
        a as *mut libc::c_void,
        (txa[1usize][0usize][(h4 - 1i32) as usize]).as_mut_ptr() as *const libc::c_void,
        w4 as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn mask_edges_intra(
    masks: *mut [[[uint16_t; 2]; 3]; 32],
    by4: libc::c_int,
    bx4: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
    tx: RectTxfmSize,
    a: *mut uint8_t,
    l: *mut uint8_t,
) {
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize) as *const TxfmInfo;
    let twl4: libc::c_int = (*t_dim).lw as libc::c_int;
    let thl4: libc::c_int = (*t_dim).lh as libc::c_int;
    let twl4c: libc::c_int = imin(2i32, twl4);
    let thl4c: libc::c_int = imin(2i32, thl4);
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut mask: libc::c_uint = (1u32) << by4;
    y = 0i32;
    while y < h4 {
        let sidx: libc::c_int = (mask >= 0x10000u32) as libc::c_int;
        let smask: libc::c_uint = mask >> (sidx << 4i32);
        let ref mut fresh4 = (*masks.offset(0isize))[bx4 as usize]
            [imin(twl4c, *l.offset(y as isize) as libc::c_int) as usize][sidx as usize];
        *fresh4 = (*fresh4 as libc::c_uint | smask) as uint16_t;
        y += 1;
        mask <<= 1i32;
    }
    x = 0i32;
    mask = (1u32) << bx4;
    while x < w4 {
        let sidx_0: libc::c_int = (mask >= 0x10000u32) as libc::c_int;
        let smask_0: libc::c_uint = mask >> (sidx_0 << 4i32);
        let ref mut fresh5 = (*masks.offset(1isize))[by4 as usize]
            [imin(thl4c, *a.offset(x as isize) as libc::c_int) as usize][sidx_0 as usize];
        *fresh5 = (*fresh5 as libc::c_uint | smask_0) as uint16_t;
        x += 1;
        mask <<= 1i32;
    }
    let hstep: libc::c_int = (*t_dim).w as libc::c_int;
    let mut t: libc::c_uint = (1u32) << by4;
    let mut inner: libc::c_uint =
        ((t as uint64_t) << h4).wrapping_sub(t as libc::c_ulong) as libc::c_uint;
    let mut inner1: libc::c_uint = inner & 0xffffu32;
    let mut inner2: libc::c_uint = inner >> 16i32;
    x = hstep;
    while x < w4 {
        if inner1 != 0 {
            let ref mut fresh6 =
                (*masks.offset(0isize))[(bx4 + x) as usize][twl4c as usize][0usize];
            *fresh6 = (*fresh6 as libc::c_uint | inner1) as uint16_t;
        }
        if inner2 != 0 {
            let ref mut fresh7 =
                (*masks.offset(0isize))[(bx4 + x) as usize][twl4c as usize][1usize];
            *fresh7 = (*fresh7 as libc::c_uint | inner2) as uint16_t;
        }
        x += hstep;
    }
    let vstep: libc::c_int = (*t_dim).h as libc::c_int;
    t = (1u32) << bx4;
    inner = ((t as uint64_t) << w4).wrapping_sub(t as libc::c_ulong) as libc::c_uint;
    inner1 = inner & 0xffffu32;
    inner2 = inner >> 16i32;
    y = vstep;
    while y < h4 {
        if inner1 != 0 {
            let ref mut fresh8 =
                (*masks.offset(1isize))[(by4 + y) as usize][thl4c as usize][0usize];
            *fresh8 = (*fresh8 as libc::c_uint | inner1) as uint16_t;
        }
        if inner2 != 0 {
            let ref mut fresh9 =
                (*masks.offset(1isize))[(by4 + y) as usize][thl4c as usize][1usize];
            *fresh9 = (*fresh9 as libc::c_uint | inner2) as uint16_t;
        }
        y += vstep;
    }
    match w4 {
        1 => {
            (*(&mut *a.offset(0isize) as *mut uint8_t as *mut alias8)).u8_0 =
                (0x1i32 * thl4c) as uint8_t;
        }
        2 => {
            (*(&mut *a.offset(0isize) as *mut uint8_t as *mut alias16)).u16_0 =
                (0x101i32 * thl4c) as uint16_t;
        }
        4 => {
            (*(&mut *a.offset(0isize) as *mut uint8_t as *mut alias32)).u32_0 =
                (0x1010101u32).wrapping_mul(thl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *a.offset(0isize) as *mut uint8_t as *mut alias64)).u64_0 =
                (0x101010101010101u64).wrapping_mul(thl4c as libc::c_ulonglong);
        }
        16 => {
            let const_val: uint64_t =
                (0x101010101010101u64).wrapping_mul(thl4c as libc::c_ulonglong);
            (*(&mut *a.offset((0i32 + 0i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val;
            (*(&mut *a.offset((0i32 + 8i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val;
        }
        32 => {
            let const_val_0: uint64_t =
                (0x101010101010101u64).wrapping_mul(thl4c as libc::c_ulonglong);
            (*(&mut *a.offset((0i32 + 0i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_0;
            (*(&mut *a.offset((0i32 + 8i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_0;
            (*(&mut *a.offset((0i32 + 16i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_0;
            (*(&mut *a.offset((0i32 + 24i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_0;
        }
        _ => {
            memset(a as *mut libc::c_void, thl4c, w4 as libc::c_ulong);
        }
    }
    match h4 {
        1 => {
            (*(&mut *l.offset(0isize) as *mut uint8_t as *mut alias8)).u8_0 =
                (0x1i32 * twl4c) as uint8_t;
        }
        2 => {
            (*(&mut *l.offset(0isize) as *mut uint8_t as *mut alias16)).u16_0 =
                (0x101i32 * twl4c) as uint16_t;
        }
        4 => {
            (*(&mut *l.offset(0isize) as *mut uint8_t as *mut alias32)).u32_0 =
                (0x1010101u32).wrapping_mul(twl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *l.offset(0isize) as *mut uint8_t as *mut alias64)).u64_0 =
                (0x101010101010101u64).wrapping_mul(twl4c as libc::c_ulonglong);
        }
        16 => {
            let const_val_1: uint64_t =
                (0x101010101010101u64).wrapping_mul(twl4c as libc::c_ulonglong);
            (*(&mut *l.offset((0i32 + 0i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_1;
            (*(&mut *l.offset((0i32 + 8i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_1;
        }
        32 => {
            let const_val_2: uint64_t =
                (0x101010101010101u64).wrapping_mul(twl4c as libc::c_ulonglong);
            (*(&mut *l.offset((0i32 + 0i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_2;
            (*(&mut *l.offset((0i32 + 8i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_2;
            (*(&mut *l.offset((0i32 + 16i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_2;
            (*(&mut *l.offset((0i32 + 24i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_2;
        }
        _ => {
            memset(l as *mut libc::c_void, twl4c, h4 as libc::c_ulong);
        }
    };
}
unsafe extern "C" fn mask_edges_chroma(
    masks: *mut [[[uint16_t; 2]; 2]; 32],
    cby4: libc::c_int,
    cbx4: libc::c_int,
    cw4: libc::c_int,
    ch4: libc::c_int,
    skip_inter: libc::c_int,
    tx: RectTxfmSize,
    a: *mut uint8_t,
    l: *mut uint8_t,
    ss_hor: libc::c_int,
    ss_ver: libc::c_int,
) {
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize) as *const TxfmInfo;
    let twl4: libc::c_int = (*t_dim).lw as libc::c_int;
    let thl4: libc::c_int = (*t_dim).lh as libc::c_int;
    let twl4c: libc::c_int = (twl4 != 0) as libc::c_int;
    let thl4c: libc::c_int = (thl4 != 0) as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let vbits: libc::c_int = 4i32 - ss_ver;
    let hbits: libc::c_int = 4i32 - ss_hor;
    let vmask: libc::c_int = 16i32 >> ss_ver;
    let hmask: libc::c_int = 16i32 >> ss_hor;
    let vmax: libc::c_uint = ((1i32) << vmask) as libc::c_uint;
    let hmax: libc::c_uint = ((1i32) << hmask) as libc::c_uint;
    let mut mask: libc::c_uint = (1u32) << cby4;
    y = 0i32;
    while y < ch4 {
        let sidx: libc::c_int = (mask >= vmax) as libc::c_int;
        let smask: libc::c_uint = mask >> (sidx << vbits);
        let ref mut fresh10 = (*masks.offset(0isize))[cbx4 as usize]
            [imin(twl4c, *l.offset(y as isize) as libc::c_int) as usize][sidx as usize];
        *fresh10 = (*fresh10 as libc::c_uint | smask) as uint16_t;
        y += 1;
        mask <<= 1i32;
    }
    x = 0i32;
    mask = (1u32) << cbx4;
    while x < cw4 {
        let sidx_0: libc::c_int = (mask >= hmax) as libc::c_int;
        let smask_0: libc::c_uint = mask >> (sidx_0 << hbits);
        let ref mut fresh11 = (*masks.offset(1isize))[cby4 as usize]
            [imin(thl4c, *a.offset(x as isize) as libc::c_int) as usize][sidx_0 as usize];
        *fresh11 = (*fresh11 as libc::c_uint | smask_0) as uint16_t;
        x += 1;
        mask <<= 1i32;
    }
    if skip_inter == 0 {
        let hstep: libc::c_int = (*t_dim).w as libc::c_int;
        let mut t: libc::c_uint = (1u32) << cby4;
        let mut inner: libc::c_uint =
            ((t as uint64_t) << ch4).wrapping_sub(t as libc::c_ulong) as libc::c_uint;
        let mut inner1: libc::c_uint = inner & (((1i32) << vmask) - 1i32) as libc::c_uint;
        let mut inner2: libc::c_uint = inner >> vmask;
        x = hstep;
        while x < cw4 {
            if inner1 != 0 {
                let ref mut fresh12 =
                    (*masks.offset(0isize))[(cbx4 + x) as usize][twl4c as usize][0usize];
                *fresh12 = (*fresh12 as libc::c_uint | inner1) as uint16_t;
            }
            if inner2 != 0 {
                let ref mut fresh13 =
                    (*masks.offset(0isize))[(cbx4 + x) as usize][twl4c as usize][1usize];
                *fresh13 = (*fresh13 as libc::c_uint | inner2) as uint16_t;
            }
            x += hstep;
        }
        let vstep: libc::c_int = (*t_dim).h as libc::c_int;
        t = (1u32) << cbx4;
        inner = ((t as uint64_t) << cw4).wrapping_sub(t as libc::c_ulong) as libc::c_uint;
        inner1 = inner & (((1i32) << hmask) - 1i32) as libc::c_uint;
        inner2 = inner >> hmask;
        y = vstep;
        while y < ch4 {
            if inner1 != 0 {
                let ref mut fresh14 =
                    (*masks.offset(1isize))[(cby4 + y) as usize][thl4c as usize][0usize];
                *fresh14 = (*fresh14 as libc::c_uint | inner1) as uint16_t;
            }
            if inner2 != 0 {
                let ref mut fresh15 =
                    (*masks.offset(1isize))[(cby4 + y) as usize][thl4c as usize][1usize];
                *fresh15 = (*fresh15 as libc::c_uint | inner2) as uint16_t;
            }
            y += vstep;
        }
    }
    match cw4 {
        1 => {
            (*(&mut *a.offset(0isize) as *mut uint8_t as *mut alias8)).u8_0 =
                (0x1i32 * thl4c) as uint8_t;
        }
        2 => {
            (*(&mut *a.offset(0isize) as *mut uint8_t as *mut alias16)).u16_0 =
                (0x101i32 * thl4c) as uint16_t;
        }
        4 => {
            (*(&mut *a.offset(0isize) as *mut uint8_t as *mut alias32)).u32_0 =
                (0x1010101u32).wrapping_mul(thl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *a.offset(0isize) as *mut uint8_t as *mut alias64)).u64_0 =
                (0x101010101010101u64).wrapping_mul(thl4c as libc::c_ulonglong);
        }
        16 => {
            let const_val: uint64_t =
                (0x101010101010101u64).wrapping_mul(thl4c as libc::c_ulonglong);
            (*(&mut *a.offset((0i32 + 0i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val;
            (*(&mut *a.offset((0i32 + 8i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val;
        }
        32 => {
            let const_val_0: uint64_t =
                (0x101010101010101u64).wrapping_mul(thl4c as libc::c_ulonglong);
            (*(&mut *a.offset((0i32 + 0i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_0;
            (*(&mut *a.offset((0i32 + 8i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_0;
            (*(&mut *a.offset((0i32 + 16i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_0;
            (*(&mut *a.offset((0i32 + 24i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_0;
        }
        _ => {
            memset(a as *mut libc::c_void, thl4c, cw4 as libc::c_ulong);
        }
    }
    match ch4 {
        1 => {
            (*(&mut *l.offset(0isize) as *mut uint8_t as *mut alias8)).u8_0 =
                (0x1i32 * twl4c) as uint8_t;
        }
        2 => {
            (*(&mut *l.offset(0isize) as *mut uint8_t as *mut alias16)).u16_0 =
                (0x101i32 * twl4c) as uint16_t;
        }
        4 => {
            (*(&mut *l.offset(0isize) as *mut uint8_t as *mut alias32)).u32_0 =
                (0x1010101u32).wrapping_mul(twl4c as libc::c_uint);
        }
        8 => {
            (*(&mut *l.offset(0isize) as *mut uint8_t as *mut alias64)).u64_0 =
                (0x101010101010101u64).wrapping_mul(twl4c as libc::c_ulonglong);
        }
        16 => {
            let const_val_1: uint64_t =
                (0x101010101010101u64).wrapping_mul(twl4c as libc::c_ulonglong);
            (*(&mut *l.offset((0i32 + 0i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_1;
            (*(&mut *l.offset((0i32 + 8i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_1;
        }
        32 => {
            let const_val_2: uint64_t =
                (0x101010101010101u64).wrapping_mul(twl4c as libc::c_ulonglong);
            (*(&mut *l.offset((0i32 + 0i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_2;
            (*(&mut *l.offset((0i32 + 8i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_2;
            (*(&mut *l.offset((0i32 + 16i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_2;
            (*(&mut *l.offset((0i32 + 24i32) as isize) as *mut uint8_t as *mut alias64)).u64_0 =
                const_val_2;
        }
        _ => {
            memset(l as *mut libc::c_void, twl4c, ch4 as libc::c_ulong);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_create_lf_mask_intra(
    lflvl: *mut Av1Filter,
    level_cache: *mut [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mut filter_level: *const [[uint8_t; 2]; 8],
    bx: libc::c_int,
    by: libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    bs: BlockSize,
    ytx: RectTxfmSize,
    uvtx: RectTxfmSize,
    layout: Dav1dPixelLayout,
    ay: *mut uint8_t,
    ly: *mut uint8_t,
    auv: *mut uint8_t,
    luv: *mut uint8_t,
) {
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: libc::c_int = imin(iw - bx, *b_dim.offset(0isize) as libc::c_int);
    let bh4: libc::c_int = imin(ih - by, *b_dim.offset(1isize) as libc::c_int);
    let bx4: libc::c_int = bx & 31i32;
    let by4: libc::c_int = by & 31i32;
    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr: *mut [uint8_t; 4] = level_cache
            .offset((by as libc::c_long * b4_stride) as isize)
            .offset(bx as isize);
        let mut y: libc::c_int = 0i32;
        while y < bh4 {
            let mut x: libc::c_int = 0i32;
            while x < bw4 {
                (*level_cache_ptr.offset(x as isize))[0usize] =
                    (*filter_level.offset(0isize))[0usize][0usize];
                (*level_cache_ptr.offset(x as isize))[1usize] =
                    (*filter_level.offset(1isize))[0usize][0usize];
                x += 1;
            }
            level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
            y += 1;
        }
        mask_edges_intra(
            ((*lflvl).filter_y).as_mut_ptr(),
            by4,
            bx4,
            bw4,
            bh4,
            ytx,
            ay,
            ly,
        );
    }
    if auv.is_null() {
        return;
    }
    let ss_ver: libc::c_int = (layout == DAV1D_PIXEL_LAYOUT_I420) as libc::c_int;
    let ss_hor: libc::c_int = (layout != DAV1D_PIXEL_LAYOUT_I444) as libc::c_int;
    let cbw4: libc::c_int = imin(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        *b_dim.offset(0isize) as libc::c_int + ss_hor >> ss_hor,
    );
    let cbh4: libc::c_int = imin(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        *b_dim.offset(1isize) as libc::c_int + ss_ver >> ss_ver,
    );
    if cbw4 == 0 || cbh4 == 0 {
        return;
    }
    let cbx4: libc::c_int = bx4 >> ss_hor;
    let cby4: libc::c_int = by4 >> ss_ver;
    let mut level_cache_ptr_0: *mut [uint8_t; 4] = level_cache
        .offset(((by >> ss_ver) as libc::c_long * b4_stride) as isize)
        .offset((bx >> ss_hor) as isize);
    let mut y_0: libc::c_int = 0i32;
    while y_0 < cbh4 {
        let mut x_0: libc::c_int = 0i32;
        while x_0 < cbw4 {
            (*level_cache_ptr_0.offset(x_0 as isize))[2usize] =
                (*filter_level.offset(2isize))[0usize][0usize];
            (*level_cache_ptr_0.offset(x_0 as isize))[3usize] =
                (*filter_level.offset(3isize))[0usize][0usize];
            x_0 += 1;
        }
        level_cache_ptr_0 = level_cache_ptr_0.offset(b4_stride as isize);
        y_0 += 1;
    }
    mask_edges_chroma(
        ((*lflvl).filter_uv).as_mut_ptr(),
        cby4,
        cbx4,
        cbw4,
        cbh4,
        0i32,
        uvtx,
        auv,
        luv,
        ss_hor,
        ss_ver,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_create_lf_mask_inter(
    lflvl: *mut Av1Filter,
    level_cache: *mut [uint8_t; 4],
    b4_stride: ptrdiff_t,
    mut filter_level: *const [[uint8_t; 2]; 8],
    bx: libc::c_int,
    by: libc::c_int,
    iw: libc::c_int,
    ih: libc::c_int,
    skip: libc::c_int,
    bs: BlockSize,
    max_ytx: RectTxfmSize,
    tx_masks: *const uint16_t,
    uvtx: RectTxfmSize,
    layout: Dav1dPixelLayout,
    ay: *mut uint8_t,
    ly: *mut uint8_t,
    auv: *mut uint8_t,
    luv: *mut uint8_t,
) {
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4: libc::c_int = imin(iw - bx, *b_dim.offset(0isize) as libc::c_int);
    let bh4: libc::c_int = imin(ih - by, *b_dim.offset(1isize) as libc::c_int);
    let bx4: libc::c_int = bx & 31i32;
    let by4: libc::c_int = by & 31i32;
    if bw4 != 0 && bh4 != 0 {
        let mut level_cache_ptr: *mut [uint8_t; 4] = level_cache
            .offset((by as libc::c_long * b4_stride) as isize)
            .offset(bx as isize);
        let mut y: libc::c_int = 0i32;
        while y < bh4 {
            let mut x: libc::c_int = 0i32;
            while x < bw4 {
                (*level_cache_ptr.offset(x as isize))[0usize] =
                    (*filter_level.offset(0isize))[0usize][0usize];
                (*level_cache_ptr.offset(x as isize))[1usize] =
                    (*filter_level.offset(1isize))[0usize][0usize];
                x += 1;
            }
            level_cache_ptr = level_cache_ptr.offset(b4_stride as isize);
            y += 1;
        }
        mask_edges_inter(
            ((*lflvl).filter_y).as_mut_ptr(),
            by4,
            bx4,
            bw4,
            bh4,
            skip,
            max_ytx,
            tx_masks,
            ay,
            ly,
        );
    }
    if auv.is_null() {
        return;
    }
    let ss_ver: libc::c_int = (layout == DAV1D_PIXEL_LAYOUT_I420) as libc::c_int;
    let ss_hor: libc::c_int = (layout != DAV1D_PIXEL_LAYOUT_I444) as libc::c_int;
    let cbw4: libc::c_int = imin(
        (iw + ss_hor >> ss_hor) - (bx >> ss_hor),
        *b_dim.offset(0isize) as libc::c_int + ss_hor >> ss_hor,
    );
    let cbh4: libc::c_int = imin(
        (ih + ss_ver >> ss_ver) - (by >> ss_ver),
        *b_dim.offset(1isize) as libc::c_int + ss_ver >> ss_ver,
    );
    if cbw4 == 0 || cbh4 == 0 {
        return;
    }
    let cbx4: libc::c_int = bx4 >> ss_hor;
    let cby4: libc::c_int = by4 >> ss_ver;
    let mut level_cache_ptr_0: *mut [uint8_t; 4] = level_cache
        .offset(((by >> ss_ver) as libc::c_long * b4_stride) as isize)
        .offset((bx >> ss_hor) as isize);
    let mut y_0: libc::c_int = 0i32;
    while y_0 < cbh4 {
        let mut x_0: libc::c_int = 0i32;
        while x_0 < cbw4 {
            (*level_cache_ptr_0.offset(x_0 as isize))[2usize] =
                (*filter_level.offset(2isize))[0usize][0usize];
            (*level_cache_ptr_0.offset(x_0 as isize))[3usize] =
                (*filter_level.offset(3isize))[0usize][0usize];
            x_0 += 1;
        }
        level_cache_ptr_0 = level_cache_ptr_0.offset(b4_stride as isize);
        y_0 += 1;
    }
    mask_edges_chroma(
        ((*lflvl).filter_uv).as_mut_ptr(),
        cby4,
        cbx4,
        cbw4,
        cbh4,
        skip,
        uvtx,
        auv,
        luv,
        ss_hor,
        ss_ver,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_calc_eih(lim_lut: *mut Av1FilterLUT, filter_sharpness: libc::c_int) {
    let sharp: libc::c_int = filter_sharpness;
    let mut level: libc::c_int = 0i32;
    while level < 64i32 {
        let mut limit: libc::c_int = level;
        if sharp > 0i32 {
            limit >>= sharp + 3i32 >> 2i32;
            limit = imin(limit, 9i32 - sharp);
        }
        limit = imax(limit, 1i32);
        (*lim_lut).i[level as usize] = limit as uint8_t;
        (*lim_lut).e[level as usize] = (2i32 * (level + 2i32) + limit) as uint8_t;
        level += 1;
    }
    (*lim_lut).sharp[0usize] = (sharp + 3i32 >> 2i32) as uint64_t;
    (*lim_lut).sharp[1usize] = (if sharp != 0 { 9i32 - sharp } else { 0xffi32 }) as uint64_t;
}
unsafe extern "C" fn calc_lf_value(
    lflvl_values: *mut [uint8_t; 2],
    base_lvl: libc::c_int,
    lf_delta: libc::c_int,
    seg_delta: libc::c_int,
    mr_delta: *const Dav1dLoopfilterModeRefDeltas,
) {
    let base: libc::c_int = iclip(
        iclip(base_lvl + lf_delta, 0i32, 63i32) + seg_delta,
        0i32,
        63i32,
    );
    if mr_delta.is_null() {
        memset(
            lflvl_values as *mut libc::c_void,
            base,
            (8i32 * 2i32) as libc::c_ulong,
        );
    } else {
        let sh: libc::c_int = (base >= 32i32) as libc::c_int;
        let ref mut fresh16 = (*lflvl_values.offset(0isize))[1usize];
        *fresh16 = iclip(
            base + (*mr_delta).ref_delta[0usize] * ((1i32) << sh),
            0i32,
            63i32,
        ) as uint8_t;
        (*lflvl_values.offset(0isize))[0usize] = *fresh16;
        let mut r: libc::c_int = 1i32;
        while r < 8i32 {
            let mut m: libc::c_int = 0i32;
            while m < 2i32 {
                let delta: libc::c_int =
                    (*mr_delta).mode_delta[m as usize] + (*mr_delta).ref_delta[r as usize];
                (*lflvl_values.offset(r as isize))[m as usize] =
                    iclip(base + delta * ((1i32) << sh), 0i32, 63i32) as uint8_t;
                m += 1;
            }
            r += 1;
        }
    };
}
#[inline]
unsafe extern "C" fn calc_lf_value_chroma(
    lflvl_values: *mut [uint8_t; 2],
    base_lvl: libc::c_int,
    lf_delta: libc::c_int,
    seg_delta: libc::c_int,
    mr_delta: *const Dav1dLoopfilterModeRefDeltas,
) {
    if base_lvl == 0 {
        memset(
            lflvl_values as *mut libc::c_void,
            0i32,
            (8i32 * 2i32) as libc::c_ulong,
        );
    } else {
        calc_lf_value(lflvl_values, base_lvl, lf_delta, seg_delta, mr_delta);
    };
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_calc_lf_values(
    lflvl_values: *mut [[[uint8_t; 2]; 8]; 4],
    hdr: *const Dav1dFrameHeader,
    mut lf_delta: *const int8_t,
) {
    let n_seg: libc::c_int = if (*hdr).segmentation.enabled != 0 {
        8i32
    } else {
        1i32
    };
    if (*hdr).loopfilter.level_y[0usize] == 0 && (*hdr).loopfilter.level_y[1usize] == 0 {
        memset(
            lflvl_values as *mut libc::c_void,
            0i32,
            (8i32 * 4i32 * 2i32 * n_seg) as libc::c_ulong,
        );
        return;
    }
    let mr_deltas: *const Dav1dLoopfilterModeRefDeltas =
        if (*hdr).loopfilter.mode_ref_delta_enabled != 0 {
            &(*hdr).loopfilter.mode_ref_deltas
        } else {
            0 as *const Dav1dLoopfilterModeRefDeltas
        };
    let mut s: libc::c_int = 0i32;
    while s < n_seg {
        let segd: *const Dav1dSegmentationData = if (*hdr).segmentation.enabled != 0 {
            &*((*hdr).segmentation.seg_data.d).as_ptr().offset(s as isize)
                as *const Dav1dSegmentationData
        } else {
            0 as *const Dav1dSegmentationData
        };
        calc_lf_value(
            ((*lflvl_values.offset(s as isize))[0usize]).as_mut_ptr(),
            (*hdr).loopfilter.level_y[0usize],
            *lf_delta.offset(0isize) as libc::c_int,
            if !segd.is_null() {
                (*segd).delta_lf_y_v
            } else {
                0i32
            },
            mr_deltas,
        );
        calc_lf_value(
            ((*lflvl_values.offset(s as isize))[1usize]).as_mut_ptr(),
            (*hdr).loopfilter.level_y[1usize],
            *lf_delta.offset(
                (if (*hdr).delta.lf.multi != 0 {
                    1i32
                } else {
                    0i32
                }) as isize,
            ) as libc::c_int,
            if !segd.is_null() {
                (*segd).delta_lf_y_h
            } else {
                0i32
            },
            mr_deltas,
        );
        calc_lf_value_chroma(
            ((*lflvl_values.offset(s as isize))[2usize]).as_mut_ptr(),
            (*hdr).loopfilter.level_u,
            *lf_delta.offset(
                (if (*hdr).delta.lf.multi != 0 {
                    2i32
                } else {
                    0i32
                }) as isize,
            ) as libc::c_int,
            if !segd.is_null() {
                (*segd).delta_lf_u
            } else {
                0i32
            },
            mr_deltas,
        );
        calc_lf_value_chroma(
            ((*lflvl_values.offset(s as isize))[3usize]).as_mut_ptr(),
            (*hdr).loopfilter.level_v,
            *lf_delta.offset(
                (if (*hdr).delta.lf.multi != 0 {
                    3i32
                } else {
                    0i32
                }) as isize,
            ) as libc::c_int,
            if !segd.is_null() {
                (*segd).delta_lf_v
            } else {
                0i32
            },
            mr_deltas,
        );
        s += 1;
    }
}
