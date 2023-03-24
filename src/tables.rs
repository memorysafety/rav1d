use ::libc;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type Dav1dFilterMode = libc::c_uint;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub const DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
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
pub type TxfmType = libc::c_uint;
pub const N_TX_TYPES_PLUS_LL: TxfmType = 17;
pub const WHT_WHT: TxfmType = 16;
pub const N_TX_TYPES: TxfmType = 16;
pub const H_FLIPADST: TxfmType = 15;
pub const V_FLIPADST: TxfmType = 14;
pub const H_ADST: TxfmType = 13;
pub const V_ADST: TxfmType = 12;
pub const H_DCT: TxfmType = 11;
pub const V_DCT: TxfmType = 10;
pub const IDTX: TxfmType = 9;
pub const FLIPADST_ADST: TxfmType = 8;
pub const ADST_FLIPADST: TxfmType = 7;
pub const FLIPADST_FLIPADST: TxfmType = 6;
pub const DCT_FLIPADST: TxfmType = 5;
pub const FLIPADST_DCT: TxfmType = 4;
pub const ADST_ADST: TxfmType = 3;
pub const DCT_ADST: TxfmType = 2;
pub const ADST_DCT: TxfmType = 1;
pub const DCT_DCT: TxfmType = 0;
pub type TxClass = libc::c_uint;
pub const TX_CLASS_V: TxClass = 2;
pub const TX_CLASS_H: TxClass = 1;
pub const TX_CLASS_2D: TxClass = 0;
pub type IntraPredMode = libc::c_uint;
pub const FILTER_PRED: IntraPredMode = 13;
pub const Z3_PRED: IntraPredMode = 8;
pub const Z2_PRED: IntraPredMode = 7;
pub const Z1_PRED: IntraPredMode = 6;
pub const DC_128_PRED: IntraPredMode = 5;
pub const TOP_DC_PRED: IntraPredMode = 4;
pub const LEFT_DC_PRED: IntraPredMode = 3;
pub const N_IMPL_INTRA_PRED_MODES: IntraPredMode = 14;
pub const N_UV_INTRA_PRED_MODES: IntraPredMode = 14;
pub const CFL_PRED: IntraPredMode = 13;
pub const N_INTRA_PRED_MODES: IntraPredMode = 13;
pub const PAETH_PRED: IntraPredMode = 12;
pub const SMOOTH_H_PRED: IntraPredMode = 11;
pub const SMOOTH_V_PRED: IntraPredMode = 10;
pub const SMOOTH_PRED: IntraPredMode = 9;
pub const VERT_LEFT_PRED: IntraPredMode = 8;
pub const HOR_UP_PRED: IntraPredMode = 7;
pub const HOR_DOWN_PRED: IntraPredMode = 6;
pub const VERT_RIGHT_PRED: IntraPredMode = 5;
pub const DIAG_DOWN_RIGHT_PRED: IntraPredMode = 4;
pub const DIAG_DOWN_LEFT_PRED: IntraPredMode = 3;
pub const HOR_PRED: IntraPredMode = 2;
pub const VERT_PRED: IntraPredMode = 1;
pub const DC_PRED: IntraPredMode = 0;
pub type BlockPartition = libc::c_uint;
pub const N_SUB8X8_PARTITIONS: BlockPartition = 4;
pub const N_PARTITIONS: BlockPartition = 10;
pub const PARTITION_V4: BlockPartition = 9;
pub const PARTITION_H4: BlockPartition = 8;
pub const PARTITION_T_RIGHT_SPLIT: BlockPartition = 7;
pub const PARTITION_T_LEFT_SPLIT: BlockPartition = 6;
pub const PARTITION_T_BOTTOM_SPLIT: BlockPartition = 5;
pub const PARTITION_T_TOP_SPLIT: BlockPartition = 4;
pub const PARTITION_SPLIT: BlockPartition = 3;
pub const PARTITION_V: BlockPartition = 2;
pub const PARTITION_H: BlockPartition = 1;
pub const PARTITION_NONE: BlockPartition = 0;
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
pub type Filter2d = libc::c_uint;
pub const N_2D_FILTERS: Filter2d = 10;
pub const FILTER_2D_BILINEAR: Filter2d = 9;
pub const FILTER_2D_8TAP_SMOOTH_SHARP: Filter2d = 8;
pub const FILTER_2D_8TAP_SMOOTH: Filter2d = 7;
pub const FILTER_2D_8TAP_SMOOTH_REGULAR: Filter2d = 6;
pub const FILTER_2D_8TAP_SHARP: Filter2d = 5;
pub const FILTER_2D_8TAP_SHARP_SMOOTH: Filter2d = 4;
pub const FILTER_2D_8TAP_SHARP_REGULAR: Filter2d = 3;
pub const FILTER_2D_8TAP_REGULAR_SHARP: Filter2d = 2;
pub const FILTER_2D_8TAP_REGULAR_SMOOTH: Filter2d = 1;
pub const FILTER_2D_8TAP_REGULAR: Filter2d = 0;
pub type InterPredMode = libc::c_uint;
pub const N_INTER_PRED_MODES: InterPredMode = 4;
pub const NEWMV: InterPredMode = 3;
pub const GLOBALMV: InterPredMode = 2;
pub const NEARMV: InterPredMode = 1;
pub const NEARESTMV: InterPredMode = 0;

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
#[no_mangle]
pub static mut dav1d_al_part_ctx: [[[uint8_t; 10]; 5]; 2] = [
    [
        [
            0u8,
            0u8,
            0x10u8,
            -(1i32) as uint8_t,
            0u8,
            0x10u8,
            0x10u8,
            0x10u8,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
        ],
        [
            0x10u8,
            0x10u8,
            0x18u8,
            -(1i32) as uint8_t,
            0x10u8,
            0x18u8,
            0x18u8,
            0x18u8,
            0x10u8,
            0x1cu8,
        ],
        [
            0x18u8,
            0x18u8,
            0x1cu8,
            -(1i32) as uint8_t,
            0x18u8,
            0x1cu8,
            0x1cu8,
            0x1cu8,
            0x18u8,
            0x1eu8,
        ],
        [
            0x1cu8,
            0x1cu8,
            0x1eu8,
            -(1i32) as uint8_t,
            0x1cu8,
            0x1eu8,
            0x1eu8,
            0x1eu8,
            0x1cu8,
            0x1fu8,
        ],
        [
            0x1eu8,
            0x1eu8,
            0x1fu8,
            0x1fu8,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
        ],
    ],
    [
        [
            0u8,
            0x10u8,
            0u8,
            -(1i32) as uint8_t,
            0x10u8,
            0x10u8,
            0u8,
            0x10u8,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
        ],
        [
            0x10u8,
            0x18u8,
            0x10u8,
            -(1i32) as uint8_t,
            0x18u8,
            0x18u8,
            0x10u8,
            0x18u8,
            0x1cu8,
            0x10u8,
        ],
        [
            0x18u8,
            0x1cu8,
            0x18u8,
            -(1i32) as uint8_t,
            0x1cu8,
            0x1cu8,
            0x18u8,
            0x1cu8,
            0x1eu8,
            0x18u8,
        ],
        [
            0x1cu8,
            0x1eu8,
            0x1cu8,
            -(1i32) as uint8_t,
            0x1eu8,
            0x1eu8,
            0x1cu8,
            0x1eu8,
            0x1fu8,
            0x1cu8,
        ],
        [
            0x1eu8,
            0x1fu8,
            0x1eu8,
            0x1fu8,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
            -(1i32) as uint8_t,
        ],
    ],
];
#[no_mangle]
pub static mut dav1d_block_sizes: [[[uint8_t; 2]; 10]; 5] = [
    [
        [BS_128x128 as uint8_t, 0],
        [BS_128x64 as uint8_t, 0],
        [BS_64x128 as uint8_t, 0],
        [0; 2],
        [BS_64x64 as uint8_t, BS_128x64 as uint8_t],
        [BS_128x64 as uint8_t, BS_64x64 as uint8_t],
        [BS_64x64 as uint8_t, BS_64x128 as uint8_t],
        [BS_64x128 as uint8_t, BS_64x64 as uint8_t],
        [0; 2],
        [0; 2],
    ],
    [
        [BS_64x64 as uint8_t, 0],
        [BS_64x32 as uint8_t, 0],
        [BS_32x64 as uint8_t, 0],
        [0; 2],
        [BS_32x32 as uint8_t, BS_64x32 as uint8_t],
        [BS_64x32 as uint8_t, BS_32x32 as uint8_t],
        [BS_32x32 as uint8_t, BS_32x64 as uint8_t],
        [BS_32x64 as uint8_t, BS_32x32 as uint8_t],
        [BS_64x16 as uint8_t, 0],
        [BS_16x64 as uint8_t, 0],
    ],
    [
        [BS_32x32 as uint8_t, 0],
        [BS_32x16 as uint8_t, 0],
        [BS_16x32 as uint8_t, 0],
        [0; 2],
        [BS_16x16 as uint8_t, BS_32x16 as uint8_t],
        [BS_32x16 as uint8_t, BS_16x16 as uint8_t],
        [BS_16x16 as uint8_t, BS_16x32 as uint8_t],
        [BS_16x32 as uint8_t, BS_16x16 as uint8_t],
        [BS_32x8 as uint8_t, 0],
        [BS_8x32 as uint8_t, 0],
    ],
    [
        [BS_16x16 as uint8_t, 0],
        [BS_16x8 as uint8_t, 0],
        [BS_8x16 as uint8_t, 0],
        [0; 2],
        [BS_8x8 as uint8_t, BS_16x8 as uint8_t],
        [BS_16x8 as uint8_t, BS_8x8 as uint8_t],
        [BS_8x8 as uint8_t, BS_8x16 as uint8_t],
        [BS_8x16 as uint8_t, BS_8x8 as uint8_t],
        [BS_16x4 as uint8_t, 0],
        [BS_4x16 as uint8_t, 0],
    ],
    [
        [BS_8x8 as uint8_t, 0],
        [BS_8x4 as uint8_t, 0],
        [BS_4x8 as uint8_t, 0],
        [BS_4x4 as uint8_t, 0],
        [0; 2],
        [0; 2],
        [0; 2],
        [0; 2],
        [0; 2],
        [0; 2],
    ],
];
#[no_mangle]
pub static mut dav1d_block_dimensions: [[uint8_t; 4]; 22] = [
    [32u8, 32u8, 5u8, 5u8],
    [32u8, 16u8, 5u8, 4u8],
    [16u8, 32u8, 4u8, 5u8],
    [16u8, 16u8, 4u8, 4u8],
    [16u8, 8u8, 4u8, 3u8],
    [16u8, 4u8, 4u8, 2u8],
    [8u8, 16u8, 3u8, 4u8],
    [8u8, 8u8, 3u8, 3u8],
    [8u8, 4u8, 3u8, 2u8],
    [8u8, 2u8, 3u8, 1u8],
    [4u8, 16u8, 2u8, 4u8],
    [4u8, 8u8, 2u8, 3u8],
    [4u8, 4u8, 2u8, 2u8],
    [4u8, 2u8, 2u8, 1u8],
    [4u8, 1u8, 2u8, 0u8],
    [2u8, 8u8, 1u8, 3u8],
    [2u8, 4u8, 1u8, 2u8],
    [2u8, 2u8, 1u8, 1u8],
    [2u8, 1u8, 1u8, 0u8],
    [1u8, 4u8, 0u8, 2u8],
    [1u8, 2u8, 0u8, 1u8],
    [1u8, 1u8, 0u8, 0u8],
];
#[no_mangle]
pub static mut dav1d_txfm_dimensions: [TxfmInfo; 19] = [
    {
        let mut init = TxfmInfo {
            w: 1u8,
            h: 1u8,
            lw: 0u8,
            lh: 0u8,
            min: 0u8,
            max: 0u8,
            sub: 0,
            ctx: 0u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 2u8,
            h: 2u8,
            lw: 1u8,
            lh: 1u8,
            min: 1u8,
            max: 1u8,
            sub: TX_4X4 as uint8_t,
            ctx: 1u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 4u8,
            h: 4u8,
            lw: 2u8,
            lh: 2u8,
            min: 2u8,
            max: 2u8,
            sub: TX_8X8 as uint8_t,
            ctx: 2u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 8u8,
            h: 8u8,
            lw: 3u8,
            lh: 3u8,
            min: 3u8,
            max: 3u8,
            sub: TX_16X16 as uint8_t,
            ctx: 3u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 16u8,
            h: 16u8,
            lw: 4u8,
            lh: 4u8,
            min: 4u8,
            max: 4u8,
            sub: TX_32X32 as uint8_t,
            ctx: 4u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 1u8,
            h: 2u8,
            lw: 0u8,
            lh: 1u8,
            min: 0u8,
            max: 1u8,
            sub: TX_4X4 as uint8_t,
            ctx: 1u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 2u8,
            h: 1u8,
            lw: 1u8,
            lh: 0u8,
            min: 0u8,
            max: 1u8,
            sub: TX_4X4 as uint8_t,
            ctx: 1u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 2u8,
            h: 4u8,
            lw: 1u8,
            lh: 2u8,
            min: 1u8,
            max: 2u8,
            sub: TX_8X8 as uint8_t,
            ctx: 2u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 4u8,
            h: 2u8,
            lw: 2u8,
            lh: 1u8,
            min: 1u8,
            max: 2u8,
            sub: TX_8X8 as uint8_t,
            ctx: 2u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 4u8,
            h: 8u8,
            lw: 2u8,
            lh: 3u8,
            min: 2u8,
            max: 3u8,
            sub: TX_16X16 as uint8_t,
            ctx: 3u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 8u8,
            h: 4u8,
            lw: 3u8,
            lh: 2u8,
            min: 2u8,
            max: 3u8,
            sub: TX_16X16 as uint8_t,
            ctx: 3u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 8u8,
            h: 16u8,
            lw: 3u8,
            lh: 4u8,
            min: 3u8,
            max: 4u8,
            sub: TX_32X32 as uint8_t,
            ctx: 4u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 16u8,
            h: 8u8,
            lw: 4u8,
            lh: 3u8,
            min: 3u8,
            max: 4u8,
            sub: TX_32X32 as uint8_t,
            ctx: 4u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 1u8,
            h: 4u8,
            lw: 0u8,
            lh: 2u8,
            min: 0u8,
            max: 2u8,
            sub: RTX_4X8 as uint8_t,
            ctx: 1u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 4u8,
            h: 1u8,
            lw: 2u8,
            lh: 0u8,
            min: 0u8,
            max: 2u8,
            sub: RTX_8X4 as uint8_t,
            ctx: 1u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 2u8,
            h: 8u8,
            lw: 1u8,
            lh: 3u8,
            min: 1u8,
            max: 3u8,
            sub: RTX_8X16 as uint8_t,
            ctx: 2u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 8u8,
            h: 2u8,
            lw: 3u8,
            lh: 1u8,
            min: 1u8,
            max: 3u8,
            sub: RTX_16X8 as uint8_t,
            ctx: 2u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 4u8,
            h: 16u8,
            lw: 2u8,
            lh: 4u8,
            min: 2u8,
            max: 4u8,
            sub: RTX_16X32 as uint8_t,
            ctx: 3u8,
        };
        init
    },
    {
        let mut init = TxfmInfo {
            w: 16u8,
            h: 4u8,
            lw: 4u8,
            lh: 2u8,
            min: 2u8,
            max: 4u8,
            sub: RTX_32X16 as uint8_t,
            ctx: 3u8,
        };
        init
    },
];
#[no_mangle]
pub static mut dav1d_max_txfm_size_for_bs: [[uint8_t; 4]; 22] = [
    [
        TX_64X64 as uint8_t,
        TX_32X32 as uint8_t,
        TX_32X32 as uint8_t,
        TX_32X32 as uint8_t,
    ],
    [
        TX_64X64 as uint8_t,
        TX_32X32 as uint8_t,
        TX_32X32 as uint8_t,
        TX_32X32 as uint8_t,
    ],
    [
        TX_64X64 as uint8_t,
        TX_32X32 as uint8_t,
        0u8,
        TX_32X32 as uint8_t,
    ],
    [
        TX_64X64 as uint8_t,
        TX_32X32 as uint8_t,
        TX_32X32 as uint8_t,
        TX_32X32 as uint8_t,
    ],
    [
        RTX_64X32 as uint8_t,
        RTX_32X16 as uint8_t,
        TX_32X32 as uint8_t,
        TX_32X32 as uint8_t,
    ],
    [
        RTX_64X16 as uint8_t,
        RTX_32X8 as uint8_t,
        RTX_32X16 as uint8_t,
        RTX_32X16 as uint8_t,
    ],
    [
        RTX_32X64 as uint8_t,
        RTX_16X32 as uint8_t,
        0u8,
        TX_32X32 as uint8_t,
    ],
    [
        TX_32X32 as uint8_t,
        TX_16X16 as uint8_t,
        RTX_16X32 as uint8_t,
        TX_32X32 as uint8_t,
    ],
    [
        RTX_32X16 as uint8_t,
        RTX_16X8 as uint8_t,
        TX_16X16 as uint8_t,
        RTX_32X16 as uint8_t,
    ],
    [
        RTX_32X8 as uint8_t,
        RTX_16X4 as uint8_t,
        RTX_16X8 as uint8_t,
        RTX_32X8 as uint8_t,
    ],
    [
        RTX_16X64 as uint8_t,
        RTX_8X32 as uint8_t,
        0u8,
        RTX_16X32 as uint8_t,
    ],
    [
        RTX_16X32 as uint8_t,
        RTX_8X16 as uint8_t,
        0u8,
        RTX_16X32 as uint8_t,
    ],
    [
        TX_16X16 as uint8_t,
        TX_8X8 as uint8_t,
        RTX_8X16 as uint8_t,
        TX_16X16 as uint8_t,
    ],
    [
        RTX_16X8 as uint8_t,
        RTX_8X4 as uint8_t,
        TX_8X8 as uint8_t,
        RTX_16X8 as uint8_t,
    ],
    [
        RTX_16X4 as uint8_t,
        RTX_8X4 as uint8_t,
        RTX_8X4 as uint8_t,
        RTX_16X4 as uint8_t,
    ],
    [
        RTX_8X32 as uint8_t,
        RTX_4X16 as uint8_t,
        0u8,
        RTX_8X32 as uint8_t,
    ],
    [
        RTX_8X16 as uint8_t,
        RTX_4X8 as uint8_t,
        0u8,
        RTX_8X16 as uint8_t,
    ],
    [
        TX_8X8 as uint8_t,
        TX_4X4 as uint8_t,
        RTX_4X8 as uint8_t,
        TX_8X8 as uint8_t,
    ],
    [
        RTX_8X4 as uint8_t,
        TX_4X4 as uint8_t,
        TX_4X4 as uint8_t,
        RTX_8X4 as uint8_t,
    ],
    [
        RTX_4X16 as uint8_t,
        RTX_4X8 as uint8_t,
        0u8,
        RTX_4X16 as uint8_t,
    ],
    [
        RTX_4X8 as uint8_t,
        TX_4X4 as uint8_t,
        0u8,
        RTX_4X8 as uint8_t,
    ],
    [
        TX_4X4 as uint8_t,
        TX_4X4 as uint8_t,
        TX_4X4 as uint8_t,
        TX_4X4 as uint8_t,
    ],
];
#[no_mangle]
pub static mut dav1d_txtp_from_uvmode: [uint8_t; 14] = [
    DCT_DCT as uint8_t,
    ADST_DCT as uint8_t,
    DCT_ADST as uint8_t,
    DCT_DCT as uint8_t,
    ADST_ADST as uint8_t,
    ADST_DCT as uint8_t,
    DCT_ADST as uint8_t,
    DCT_ADST as uint8_t,
    ADST_DCT as uint8_t,
    ADST_ADST as uint8_t,
    ADST_DCT as uint8_t,
    DCT_ADST as uint8_t,
    ADST_ADST as uint8_t,
    0,
];
#[no_mangle]
pub static mut dav1d_comp_inter_pred_modes: [[uint8_t; 2]; 8] = [
    [NEARESTMV as uint8_t, NEARESTMV as uint8_t],
    [NEARMV as uint8_t, NEARMV as uint8_t],
    [NEARESTMV as uint8_t, NEWMV as uint8_t],
    [NEWMV as uint8_t, NEARESTMV as uint8_t],
    [NEARMV as uint8_t, NEWMV as uint8_t],
    [NEWMV as uint8_t, NEARMV as uint8_t],
    [GLOBALMV as uint8_t, GLOBALMV as uint8_t],
    [NEWMV as uint8_t, NEWMV as uint8_t],
];
#[no_mangle]
pub static mut dav1d_partition_type_count: [uint8_t; 5] = [
    (N_PARTITIONS as libc::c_int - 3i32) as uint8_t,
    (N_PARTITIONS as libc::c_int - 1i32) as uint8_t,
    (N_PARTITIONS as libc::c_int - 1i32) as uint8_t,
    (N_PARTITIONS as libc::c_int - 1i32) as uint8_t,
    (N_SUB8X8_PARTITIONS as libc::c_int - 1i32) as uint8_t,
];
#[no_mangle]
pub static mut dav1d_tx_types_per_set: [uint8_t; 40] = [
    IDTX as uint8_t,
    DCT_DCT as uint8_t,
    ADST_ADST as uint8_t,
    ADST_DCT as uint8_t,
    DCT_ADST as uint8_t,
    IDTX as uint8_t,
    DCT_DCT as uint8_t,
    V_DCT as uint8_t,
    H_DCT as uint8_t,
    ADST_ADST as uint8_t,
    ADST_DCT as uint8_t,
    DCT_ADST as uint8_t,
    IDTX as uint8_t,
    V_DCT as uint8_t,
    H_DCT as uint8_t,
    DCT_DCT as uint8_t,
    ADST_DCT as uint8_t,
    DCT_ADST as uint8_t,
    FLIPADST_DCT as uint8_t,
    DCT_FLIPADST as uint8_t,
    ADST_ADST as uint8_t,
    FLIPADST_FLIPADST as uint8_t,
    ADST_FLIPADST as uint8_t,
    FLIPADST_ADST as uint8_t,
    IDTX as uint8_t,
    V_DCT as uint8_t,
    H_DCT as uint8_t,
    V_ADST as uint8_t,
    H_ADST as uint8_t,
    V_FLIPADST as uint8_t,
    H_FLIPADST as uint8_t,
    DCT_DCT as uint8_t,
    ADST_DCT as uint8_t,
    DCT_ADST as uint8_t,
    FLIPADST_DCT as uint8_t,
    DCT_FLIPADST as uint8_t,
    ADST_ADST as uint8_t,
    FLIPADST_FLIPADST as uint8_t,
    ADST_FLIPADST as uint8_t,
    FLIPADST_ADST as uint8_t,
];
#[no_mangle]
pub static mut dav1d_ymode_size_context: [uint8_t; 22] = [
    3u8, 3u8, 3u8, 3u8, 3u8, 2u8, 3u8, 3u8, 2u8, 1u8, 2u8, 2u8, 2u8, 1u8, 0u8, 1u8, 1u8, 1u8, 0u8,
    0u8, 0u8, 0u8,
];
#[no_mangle]
pub static mut dav1d_lo_ctx_offsets: [[[uint8_t; 5]; 5]; 3] = [
    [
        [0u8, 1u8, 6u8, 6u8, 21u8],
        [1u8, 6u8, 6u8, 21u8, 21u8],
        [6u8, 6u8, 21u8, 21u8, 21u8],
        [6u8, 21u8, 21u8, 21u8, 21u8],
        [21u8, 21u8, 21u8, 21u8, 21u8],
    ],
    [
        [0u8, 16u8, 6u8, 6u8, 21u8],
        [16u8, 16u8, 6u8, 21u8, 21u8],
        [16u8, 16u8, 21u8, 21u8, 21u8],
        [16u8, 16u8, 21u8, 21u8, 21u8],
        [16u8, 16u8, 21u8, 21u8, 21u8],
    ],
    [
        [0u8, 11u8, 11u8, 11u8, 11u8],
        [11u8, 11u8, 11u8, 11u8, 11u8],
        [6u8, 6u8, 21u8, 21u8, 21u8],
        [6u8, 21u8, 21u8, 21u8, 21u8],
        [21u8, 21u8, 21u8, 21u8, 21u8],
    ],
];
#[no_mangle]
pub static mut dav1d_skip_ctx: [[uint8_t; 5]; 5] = [
    [1u8, 2u8, 2u8, 2u8, 3u8],
    [2u8, 4u8, 4u8, 4u8, 5u8],
    [2u8, 4u8, 4u8, 4u8, 5u8],
    [2u8, 4u8, 4u8, 4u8, 5u8],
    [3u8, 5u8, 5u8, 5u8, 6u8],
];
#[no_mangle]
pub static mut dav1d_tx_type_class: [uint8_t; 17] = [
    TX_CLASS_2D as uint8_t,
    TX_CLASS_2D as uint8_t,
    TX_CLASS_2D as uint8_t,
    TX_CLASS_2D as uint8_t,
    TX_CLASS_2D as uint8_t,
    TX_CLASS_2D as uint8_t,
    TX_CLASS_2D as uint8_t,
    TX_CLASS_2D as uint8_t,
    TX_CLASS_2D as uint8_t,
    TX_CLASS_2D as uint8_t,
    TX_CLASS_V as uint8_t,
    TX_CLASS_H as uint8_t,
    TX_CLASS_V as uint8_t,
    TX_CLASS_H as uint8_t,
    TX_CLASS_V as uint8_t,
    TX_CLASS_H as uint8_t,
    TX_CLASS_2D as uint8_t,
];
#[no_mangle]
pub static mut dav1d_filter_2d: [[uint8_t; 4]; 4] = [
    [
        FILTER_2D_8TAP_REGULAR as uint8_t,
        FILTER_2D_8TAP_REGULAR_SMOOTH as uint8_t,
        FILTER_2D_8TAP_REGULAR_SHARP as uint8_t,
        0,
    ],
    [
        FILTER_2D_8TAP_SMOOTH_REGULAR as uint8_t,
        FILTER_2D_8TAP_SMOOTH as uint8_t,
        FILTER_2D_8TAP_SMOOTH_SHARP as uint8_t,
        0,
    ],
    [
        FILTER_2D_8TAP_SHARP_REGULAR as uint8_t,
        FILTER_2D_8TAP_SHARP_SMOOTH as uint8_t,
        FILTER_2D_8TAP_SHARP as uint8_t,
        0,
    ],
    [0, 0, 0, FILTER_2D_BILINEAR as uint8_t],
];
#[no_mangle]
pub static mut dav1d_filter_dir: [[uint8_t; 2]; 10] = [
    [
        DAV1D_FILTER_8TAP_REGULAR as uint8_t,
        DAV1D_FILTER_8TAP_REGULAR as uint8_t,
    ],
    [
        DAV1D_FILTER_8TAP_SMOOTH as uint8_t,
        DAV1D_FILTER_8TAP_REGULAR as uint8_t,
    ],
    [
        DAV1D_FILTER_8TAP_SHARP as uint8_t,
        DAV1D_FILTER_8TAP_REGULAR as uint8_t,
    ],
    [
        DAV1D_FILTER_8TAP_REGULAR as uint8_t,
        DAV1D_FILTER_8TAP_SHARP as uint8_t,
    ],
    [
        DAV1D_FILTER_8TAP_SMOOTH as uint8_t,
        DAV1D_FILTER_8TAP_SHARP as uint8_t,
    ],
    [
        DAV1D_FILTER_8TAP_SHARP as uint8_t,
        DAV1D_FILTER_8TAP_SHARP as uint8_t,
    ],
    [
        DAV1D_FILTER_8TAP_REGULAR as uint8_t,
        DAV1D_FILTER_8TAP_SMOOTH as uint8_t,
    ],
    [
        DAV1D_FILTER_8TAP_SMOOTH as uint8_t,
        DAV1D_FILTER_8TAP_SMOOTH as uint8_t,
    ],
    [
        DAV1D_FILTER_8TAP_SHARP as uint8_t,
        DAV1D_FILTER_8TAP_SMOOTH as uint8_t,
    ],
    [
        DAV1D_FILTER_BILINEAR as uint8_t,
        DAV1D_FILTER_BILINEAR as uint8_t,
    ],
];
#[no_mangle]
pub static mut dav1d_filter_mode_to_y_mode: [uint8_t; 5] = [
    DC_PRED as uint8_t,
    VERT_PRED as uint8_t,
    HOR_PRED as uint8_t,
    HOR_DOWN_PRED as uint8_t,
    DC_PRED as uint8_t,
];
#[no_mangle]
pub static mut dav1d_intra_mode_context: [uint8_t; 13] = [
    0u8, 1u8, 2u8, 3u8, 4u8, 4u8, 4u8, 4u8, 3u8, 0u8, 1u8, 2u8, 0u8,
];
#[no_mangle]
pub static mut dav1d_wedge_ctx_lut: [uint8_t; 22] = [
    0, 0, 0, 0, 0, 0, 0, 6u8, 5u8, 8u8, 0, 4u8, 3u8, 2u8, 0, 7u8, 1u8, 0u8, 0, 0, 0, 0,
];
#[no_mangle]
pub static mut dav1d_default_wm_params: Dav1dWarpedMotionParams = {
    let mut init = Dav1dWarpedMotionParams {
        type_0: DAV1D_WM_TYPE_IDENTITY,
        matrix: [0i32, 0i32, (1i32) << 16i32, 0i32, 0i32, (1i32) << 16i32],
        u: C2RustUnnamed {
            p: {
                let mut init = C2RustUnnamed_0 {
                    alpha: 0i16,
                    beta: 0i16,
                    gamma: 0i16,
                    delta: 0i16,
                };
                init
            },
        },
    };
    init
};
#[no_mangle]
pub static mut dav1d_cdef_directions: [[int8_t; 2]; 12] = [
    [
        (1i32 * 12i32 + 0i32) as int8_t,
        (2i32 * 12i32 + 0i32) as int8_t,
    ],
    [
        (1i32 * 12i32 + 0i32) as int8_t,
        (2i32 * 12i32 - 1i32) as int8_t,
    ],
    [
        (-(1i32) * 12i32 + 1i32) as int8_t,
        (-(2i32) * 12i32 + 2i32) as int8_t,
    ],
    [
        (0i32 * 12i32 + 1i32) as int8_t,
        (-(1i32) * 12i32 + 2i32) as int8_t,
    ],
    [
        (0i32 * 12i32 + 1i32) as int8_t,
        (0i32 * 12i32 + 2i32) as int8_t,
    ],
    [
        (0i32 * 12i32 + 1i32) as int8_t,
        (1i32 * 12i32 + 2i32) as int8_t,
    ],
    [
        (1i32 * 12i32 + 1i32) as int8_t,
        (2i32 * 12i32 + 2i32) as int8_t,
    ],
    [
        (1i32 * 12i32 + 0i32) as int8_t,
        (2i32 * 12i32 + 1i32) as int8_t,
    ],
    [
        (1i32 * 12i32 + 0i32) as int8_t,
        (2i32 * 12i32 + 0i32) as int8_t,
    ],
    [
        (1i32 * 12i32 + 0i32) as int8_t,
        (2i32 * 12i32 - 1i32) as int8_t,
    ],
    [
        (-(1i32) * 12i32 + 1i32) as int8_t,
        (-(2i32) * 12i32 + 2i32) as int8_t,
    ],
    [
        (0i32 * 12i32 + 1i32) as int8_t,
        (-(1i32) * 12i32 + 2i32) as int8_t,
    ],
];
#[no_mangle]
pub static mut dav1d_sgr_params: [[uint16_t; 2]; 16] = [
    [140u16, 3236u16],
    [112u16, 2158u16],
    [93u16, 1618u16],
    [80u16, 1438u16],
    [70u16, 1295u16],
    [58u16, 1177u16],
    [47u16, 1079u16],
    [37u16, 996u16],
    [30u16, 925u16],
    [25u16, 863u16],
    [0u16, 2589u16],
    [0u16, 1618u16],
    [0u16, 1177u16],
    [0u16, 925u16],
    [56u16, 0u16],
    [22u16, 0u16],
];
#[no_mangle]
pub static mut dav1d_sgr_x_by_x: [uint8_t; 256] = [
    255u8, 128u8, 85u8, 64u8, 51u8, 43u8, 37u8, 32u8, 28u8, 26u8, 23u8, 21u8, 20u8, 18u8, 17u8,
    16u8, 15u8, 14u8, 13u8, 13u8, 12u8, 12u8, 11u8, 11u8, 10u8, 10u8, 9u8, 9u8, 9u8, 9u8, 8u8, 8u8,
    8u8, 8u8, 7u8, 7u8, 7u8, 7u8, 7u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 6u8, 5u8, 5u8, 5u8, 5u8, 5u8,
    5u8, 5u8, 5u8, 5u8, 5u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8, 4u8,
    4u8, 4u8, 4u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8,
    3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 3u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8,
    2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8,
    2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8,
    2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8, 2u8,
    2u8, 2u8, 2u8, 2u8, 2u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
    1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
    1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
    1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8,
    1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 1u8, 0u8,
];
#[no_mangle]
pub static mut dav1d_mc_subpel_filters: [[[int8_t; 8]; 15]; 6] = [
    [
        [0i8, 1i8, -3i8, 63i8, 4i8, -1i8, 0i8, 0i8],
        [0i8, 1i8, -5i8, 61i8, 9i8, -2i8, 0i8, 0i8],
        [0i8, 1i8, -6i8, 58i8, 14i8, -4i8, 1i8, 0i8],
        [0i8, 1i8, -7i8, 55i8, 19i8, -5i8, 1i8, 0i8],
        [0i8, 1i8, -7i8, 51i8, 24i8, -6i8, 1i8, 0i8],
        [0i8, 1i8, -8i8, 47i8, 29i8, -6i8, 1i8, 0i8],
        [0i8, 1i8, -7i8, 42i8, 33i8, -6i8, 1i8, 0i8],
        [0i8, 1i8, -7i8, 38i8, 38i8, -7i8, 1i8, 0i8],
        [0i8, 1i8, -6i8, 33i8, 42i8, -7i8, 1i8, 0i8],
        [0i8, 1i8, -6i8, 29i8, 47i8, -8i8, 1i8, 0i8],
        [0i8, 1i8, -6i8, 24i8, 51i8, -7i8, 1i8, 0i8],
        [0i8, 1i8, -5i8, 19i8, 55i8, -7i8, 1i8, 0i8],
        [0i8, 1i8, -4i8, 14i8, 58i8, -6i8, 1i8, 0i8],
        [0i8, 0i8, -2i8, 9i8, 61i8, -5i8, 1i8, 0i8],
        [0i8, 0i8, -1i8, 4i8, 63i8, -3i8, 1i8, 0i8],
    ],
    [
        [0i8, 1i8, 14i8, 31i8, 17i8, 1i8, 0i8, 0i8],
        [0i8, 0i8, 13i8, 31i8, 18i8, 2i8, 0i8, 0i8],
        [0i8, 0i8, 11i8, 31i8, 20i8, 2i8, 0i8, 0i8],
        [0i8, 0i8, 10i8, 30i8, 21i8, 3i8, 0i8, 0i8],
        [0i8, 0i8, 9i8, 29i8, 22i8, 4i8, 0i8, 0i8],
        [0i8, 0i8, 8i8, 28i8, 23i8, 5i8, 0i8, 0i8],
        [0i8, -1i8, 8i8, 27i8, 24i8, 6i8, 0i8, 0i8],
        [0i8, -1i8, 7i8, 26i8, 26i8, 7i8, -1i8, 0i8],
        [0i8, 0i8, 6i8, 24i8, 27i8, 8i8, -1i8, 0i8],
        [0i8, 0i8, 5i8, 23i8, 28i8, 8i8, 0i8, 0i8],
        [0i8, 0i8, 4i8, 22i8, 29i8, 9i8, 0i8, 0i8],
        [0i8, 0i8, 3i8, 21i8, 30i8, 10i8, 0i8, 0i8],
        [0i8, 0i8, 2i8, 20i8, 31i8, 11i8, 0i8, 0i8],
        [0i8, 0i8, 2i8, 18i8, 31i8, 13i8, 0i8, 0i8],
        [0i8, 0i8, 1i8, 17i8, 31i8, 14i8, 1i8, 0i8],
    ],
    [
        [-1i8, 1i8, -3i8, 63i8, 4i8, -1i8, 1i8, 0i8],
        [-1i8, 3i8, -6i8, 62i8, 8i8, -3i8, 2i8, -1i8],
        [-1i8, 4i8, -9i8, 60i8, 13i8, -5i8, 3i8, -1i8],
        [-2i8, 5i8, -11i8, 58i8, 19i8, -7i8, 3i8, -1i8],
        [-2i8, 5i8, -11i8, 54i8, 24i8, -9i8, 4i8, -1i8],
        [-2i8, 5i8, -12i8, 50i8, 30i8, -10i8, 4i8, -1i8],
        [-2i8, 5i8, -12i8, 45i8, 35i8, -11i8, 5i8, -1i8],
        [-2i8, 6i8, -12i8, 40i8, 40i8, -12i8, 6i8, -2i8],
        [-1i8, 5i8, -11i8, 35i8, 45i8, -12i8, 5i8, -2i8],
        [-1i8, 4i8, -10i8, 30i8, 50i8, -12i8, 5i8, -2i8],
        [-1i8, 4i8, -9i8, 24i8, 54i8, -11i8, 5i8, -2i8],
        [-1i8, 3i8, -7i8, 19i8, 58i8, -11i8, 5i8, -2i8],
        [-1i8, 3i8, -5i8, 13i8, 60i8, -9i8, 4i8, -1i8],
        [-1i8, 2i8, -3i8, 8i8, 62i8, -6i8, 3i8, -1i8],
        [0i8, 1i8, -1i8, 4i8, 63i8, -3i8, 1i8, -1i8],
    ],
    [
        [0i8, 0i8, -2i8, 63i8, 4i8, -1i8, 0i8, 0i8],
        [0i8, 0i8, -4i8, 61i8, 9i8, -2i8, 0i8, 0i8],
        [0i8, 0i8, -5i8, 58i8, 14i8, -3i8, 0i8, 0i8],
        [0i8, 0i8, -6i8, 55i8, 19i8, -4i8, 0i8, 0i8],
        [0i8, 0i8, -6i8, 51i8, 24i8, -5i8, 0i8, 0i8],
        [0i8, 0i8, -7i8, 47i8, 29i8, -5i8, 0i8, 0i8],
        [0i8, 0i8, -6i8, 42i8, 33i8, -5i8, 0i8, 0i8],
        [0i8, 0i8, -6i8, 38i8, 38i8, -6i8, 0i8, 0i8],
        [0i8, 0i8, -5i8, 33i8, 42i8, -6i8, 0i8, 0i8],
        [0i8, 0i8, -5i8, 29i8, 47i8, -7i8, 0i8, 0i8],
        [0i8, 0i8, -5i8, 24i8, 51i8, -6i8, 0i8, 0i8],
        [0i8, 0i8, -4i8, 19i8, 55i8, -6i8, 0i8, 0i8],
        [0i8, 0i8, -3i8, 14i8, 58i8, -5i8, 0i8, 0i8],
        [0i8, 0i8, -2i8, 9i8, 61i8, -4i8, 0i8, 0i8],
        [0i8, 0i8, -1i8, 4i8, 63i8, -2i8, 0i8, 0i8],
    ],
    [
        [0i8, 0i8, 15i8, 31i8, 17i8, 1i8, 0i8, 0i8],
        [0i8, 0i8, 13i8, 31i8, 18i8, 2i8, 0i8, 0i8],
        [0i8, 0i8, 11i8, 31i8, 20i8, 2i8, 0i8, 0i8],
        [0i8, 0i8, 10i8, 30i8, 21i8, 3i8, 0i8, 0i8],
        [0i8, 0i8, 9i8, 29i8, 22i8, 4i8, 0i8, 0i8],
        [0i8, 0i8, 8i8, 28i8, 23i8, 5i8, 0i8, 0i8],
        [0i8, 0i8, 7i8, 27i8, 24i8, 6i8, 0i8, 0i8],
        [0i8, 0i8, 6i8, 26i8, 26i8, 6i8, 0i8, 0i8],
        [0i8, 0i8, 6i8, 24i8, 27i8, 7i8, 0i8, 0i8],
        [0i8, 0i8, 5i8, 23i8, 28i8, 8i8, 0i8, 0i8],
        [0i8, 0i8, 4i8, 22i8, 29i8, 9i8, 0i8, 0i8],
        [0i8, 0i8, 3i8, 21i8, 30i8, 10i8, 0i8, 0i8],
        [0i8, 0i8, 2i8, 20i8, 31i8, 11i8, 0i8, 0i8],
        [0i8, 0i8, 2i8, 18i8, 31i8, 13i8, 0i8, 0i8],
        [0i8, 0i8, 1i8, 17i8, 31i8, 15i8, 0i8, 0i8],
    ],
    [
        [0i8, 0i8, 0i8, 60i8, 4i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 56i8, 8i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 52i8, 12i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 48i8, 16i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 44i8, 20i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 40i8, 24i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 36i8, 28i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 32i8, 32i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 28i8, 36i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 24i8, 40i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 20i8, 44i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 16i8, 48i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 12i8, 52i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 8i8, 56i8, 0i8, 0i8, 0i8],
        [0i8, 0i8, 0i8, 4i8, 60i8, 0i8, 0i8, 0i8],
    ],
];
#[no_mangle]
pub static mut dav1d_mc_warp_filter: [[int8_t; 8]; 193] = [
    [0i8, 0i8, 127i8, 1i8, 0i8, 0i8, 0i8, 0i8],
    [0i8, -1i8, 127i8, 2i8, 0i8, 0i8, 0i8, 0i8],
    [1i8, -3i8, 127i8, 4i8, -1i8, 0i8, 0i8, 0i8],
    [1i8, -4i8, 126i8, 6i8, -2i8, 1i8, 0i8, 0i8],
    [1i8, -5i8, 126i8, 8i8, -3i8, 1i8, 0i8, 0i8],
    [1i8, -6i8, 125i8, 11i8, -4i8, 1i8, 0i8, 0i8],
    [1i8, -7i8, 124i8, 13i8, -4i8, 1i8, 0i8, 0i8],
    [2i8, -8i8, 123i8, 15i8, -5i8, 1i8, 0i8, 0i8],
    [2i8, -9i8, 122i8, 18i8, -6i8, 1i8, 0i8, 0i8],
    [2i8, -10i8, 121i8, 20i8, -6i8, 1i8, 0i8, 0i8],
    [2i8, -11i8, 120i8, 22i8, -7i8, 2i8, 0i8, 0i8],
    [2i8, -12i8, 119i8, 25i8, -8i8, 2i8, 0i8, 0i8],
    [3i8, -13i8, 117i8, 27i8, -8i8, 2i8, 0i8, 0i8],
    [3i8, -13i8, 116i8, 29i8, -9i8, 2i8, 0i8, 0i8],
    [3i8, -14i8, 114i8, 32i8, -10i8, 3i8, 0i8, 0i8],
    [3i8, -15i8, 113i8, 35i8, -10i8, 2i8, 0i8, 0i8],
    [3i8, -15i8, 111i8, 37i8, -11i8, 3i8, 0i8, 0i8],
    [3i8, -16i8, 109i8, 40i8, -11i8, 3i8, 0i8, 0i8],
    [3i8, -16i8, 108i8, 42i8, -12i8, 3i8, 0i8, 0i8],
    [4i8, -17i8, 106i8, 45i8, -13i8, 3i8, 0i8, 0i8],
    [4i8, -17i8, 104i8, 47i8, -13i8, 3i8, 0i8, 0i8],
    [4i8, -17i8, 102i8, 50i8, -14i8, 3i8, 0i8, 0i8],
    [4i8, -17i8, 100i8, 52i8, -14i8, 3i8, 0i8, 0i8],
    [4i8, -18i8, 98i8, 55i8, -15i8, 4i8, 0i8, 0i8],
    [4i8, -18i8, 96i8, 58i8, -15i8, 3i8, 0i8, 0i8],
    [4i8, -18i8, 94i8, 60i8, -16i8, 4i8, 0i8, 0i8],
    [4i8, -18i8, 91i8, 63i8, -16i8, 4i8, 0i8, 0i8],
    [4i8, -18i8, 89i8, 65i8, -16i8, 4i8, 0i8, 0i8],
    [4i8, -18i8, 87i8, 68i8, -17i8, 4i8, 0i8, 0i8],
    [4i8, -18i8, 85i8, 70i8, -17i8, 4i8, 0i8, 0i8],
    [4i8, -18i8, 82i8, 73i8, -17i8, 4i8, 0i8, 0i8],
    [4i8, -18i8, 80i8, 75i8, -17i8, 4i8, 0i8, 0i8],
    [4i8, -18i8, 78i8, 78i8, -18i8, 4i8, 0i8, 0i8],
    [4i8, -17i8, 75i8, 80i8, -18i8, 4i8, 0i8, 0i8],
    [4i8, -17i8, 73i8, 82i8, -18i8, 4i8, 0i8, 0i8],
    [4i8, -17i8, 70i8, 85i8, -18i8, 4i8, 0i8, 0i8],
    [4i8, -17i8, 68i8, 87i8, -18i8, 4i8, 0i8, 0i8],
    [4i8, -16i8, 65i8, 89i8, -18i8, 4i8, 0i8, 0i8],
    [4i8, -16i8, 63i8, 91i8, -18i8, 4i8, 0i8, 0i8],
    [4i8, -16i8, 60i8, 94i8, -18i8, 4i8, 0i8, 0i8],
    [3i8, -15i8, 58i8, 96i8, -18i8, 4i8, 0i8, 0i8],
    [4i8, -15i8, 55i8, 98i8, -18i8, 4i8, 0i8, 0i8],
    [3i8, -14i8, 52i8, 100i8, -17i8, 4i8, 0i8, 0i8],
    [3i8, -14i8, 50i8, 102i8, -17i8, 4i8, 0i8, 0i8],
    [3i8, -13i8, 47i8, 104i8, -17i8, 4i8, 0i8, 0i8],
    [3i8, -13i8, 45i8, 106i8, -17i8, 4i8, 0i8, 0i8],
    [3i8, -12i8, 42i8, 108i8, -16i8, 3i8, 0i8, 0i8],
    [3i8, -11i8, 40i8, 109i8, -16i8, 3i8, 0i8, 0i8],
    [3i8, -11i8, 37i8, 111i8, -15i8, 3i8, 0i8, 0i8],
    [2i8, -10i8, 35i8, 113i8, -15i8, 3i8, 0i8, 0i8],
    [3i8, -10i8, 32i8, 114i8, -14i8, 3i8, 0i8, 0i8],
    [2i8, -9i8, 29i8, 116i8, -13i8, 3i8, 0i8, 0i8],
    [2i8, -8i8, 27i8, 117i8, -13i8, 3i8, 0i8, 0i8],
    [2i8, -8i8, 25i8, 119i8, -12i8, 2i8, 0i8, 0i8],
    [2i8, -7i8, 22i8, 120i8, -11i8, 2i8, 0i8, 0i8],
    [1i8, -6i8, 20i8, 121i8, -10i8, 2i8, 0i8, 0i8],
    [1i8, -6i8, 18i8, 122i8, -9i8, 2i8, 0i8, 0i8],
    [1i8, -5i8, 15i8, 123i8, -8i8, 2i8, 0i8, 0i8],
    [1i8, -4i8, 13i8, 124i8, -7i8, 1i8, 0i8, 0i8],
    [1i8, -4i8, 11i8, 125i8, -6i8, 1i8, 0i8, 0i8],
    [1i8, -3i8, 8i8, 126i8, -5i8, 1i8, 0i8, 0i8],
    [1i8, -2i8, 6i8, 126i8, -4i8, 1i8, 0i8, 0i8],
    [0i8, -1i8, 4i8, 127i8, -3i8, 1i8, 0i8, 0i8],
    [0i8, 0i8, 2i8, 127i8, -1i8, 0i8, 0i8, 0i8],
    [0i8, 0i8, 0i8, 127i8, 1i8, 0i8, 0i8, 0i8],
    [0i8, 0i8, -1i8, 127i8, 2i8, 0i8, 0i8, 0i8],
    [0i8, 1i8, -3i8, 127i8, 4i8, -2i8, 1i8, 0i8],
    [0i8, 1i8, -5i8, 127i8, 6i8, -2i8, 1i8, 0i8],
    [0i8, 2i8, -6i8, 126i8, 8i8, -3i8, 1i8, 0i8],
    [-1i8, 2i8, -7i8, 126i8, 11i8, -4i8, 2i8, -1i8],
    [-1i8, 3i8, -8i8, 125i8, 13i8, -5i8, 2i8, -1i8],
    [-1i8, 3i8, -10i8, 124i8, 16i8, -6i8, 3i8, -1i8],
    [-1i8, 4i8, -11i8, 123i8, 18i8, -7i8, 3i8, -1i8],
    [-1i8, 4i8, -12i8, 122i8, 20i8, -7i8, 3i8, -1i8],
    [-1i8, 4i8, -13i8, 121i8, 23i8, -8i8, 3i8, -1i8],
    [-2i8, 5i8, -14i8, 120i8, 25i8, -9i8, 4i8, -1i8],
    [-1i8, 5i8, -15i8, 119i8, 27i8, -10i8, 4i8, -1i8],
    [-1i8, 5i8, -16i8, 118i8, 30i8, -11i8, 4i8, -1i8],
    [-2i8, 6i8, -17i8, 116i8, 33i8, -12i8, 5i8, -1i8],
    [-2i8, 6i8, -17i8, 114i8, 35i8, -12i8, 5i8, -1i8],
    [-2i8, 6i8, -18i8, 113i8, 38i8, -13i8, 5i8, -1i8],
    [-2i8, 7i8, -19i8, 111i8, 41i8, -14i8, 6i8, -2i8],
    [-2i8, 7i8, -19i8, 110i8, 43i8, -15i8, 6i8, -2i8],
    [-2i8, 7i8, -20i8, 108i8, 46i8, -15i8, 6i8, -2i8],
    [-2i8, 7i8, -20i8, 106i8, 49i8, -16i8, 6i8, -2i8],
    [-2i8, 7i8, -21i8, 104i8, 51i8, -16i8, 7i8, -2i8],
    [-2i8, 7i8, -21i8, 102i8, 54i8, -17i8, 7i8, -2i8],
    [-2i8, 8i8, -21i8, 100i8, 56i8, -18i8, 7i8, -2i8],
    [-2i8, 8i8, -22i8, 98i8, 59i8, -18i8, 7i8, -2i8],
    [-2i8, 8i8, -22i8, 96i8, 62i8, -19i8, 7i8, -2i8],
    [-2i8, 8i8, -22i8, 94i8, 64i8, -19i8, 7i8, -2i8],
    [-2i8, 8i8, -22i8, 91i8, 67i8, -20i8, 8i8, -2i8],
    [-2i8, 8i8, -22i8, 89i8, 69i8, -20i8, 8i8, -2i8],
    [-2i8, 8i8, -22i8, 87i8, 72i8, -21i8, 8i8, -2i8],
    [-2i8, 8i8, -21i8, 84i8, 74i8, -21i8, 8i8, -2i8],
    [-2i8, 8i8, -22i8, 82i8, 77i8, -21i8, 8i8, -2i8],
    [-2i8, 8i8, -21i8, 79i8, 79i8, -21i8, 8i8, -2i8],
    [-2i8, 8i8, -21i8, 77i8, 82i8, -22i8, 8i8, -2i8],
    [-2i8, 8i8, -21i8, 74i8, 84i8, -21i8, 8i8, -2i8],
    [-2i8, 8i8, -21i8, 72i8, 87i8, -22i8, 8i8, -2i8],
    [-2i8, 8i8, -20i8, 69i8, 89i8, -22i8, 8i8, -2i8],
    [-2i8, 8i8, -20i8, 67i8, 91i8, -22i8, 8i8, -2i8],
    [-2i8, 7i8, -19i8, 64i8, 94i8, -22i8, 8i8, -2i8],
    [-2i8, 7i8, -19i8, 62i8, 96i8, -22i8, 8i8, -2i8],
    [-2i8, 7i8, -18i8, 59i8, 98i8, -22i8, 8i8, -2i8],
    [-2i8, 7i8, -18i8, 56i8, 100i8, -21i8, 8i8, -2i8],
    [-2i8, 7i8, -17i8, 54i8, 102i8, -21i8, 7i8, -2i8],
    [-2i8, 7i8, -16i8, 51i8, 104i8, -21i8, 7i8, -2i8],
    [-2i8, 6i8, -16i8, 49i8, 106i8, -20i8, 7i8, -2i8],
    [-2i8, 6i8, -15i8, 46i8, 108i8, -20i8, 7i8, -2i8],
    [-2i8, 6i8, -15i8, 43i8, 110i8, -19i8, 7i8, -2i8],
    [-2i8, 6i8, -14i8, 41i8, 111i8, -19i8, 7i8, -2i8],
    [-1i8, 5i8, -13i8, 38i8, 113i8, -18i8, 6i8, -2i8],
    [-1i8, 5i8, -12i8, 35i8, 114i8, -17i8, 6i8, -2i8],
    [-1i8, 5i8, -12i8, 33i8, 116i8, -17i8, 6i8, -2i8],
    [-1i8, 4i8, -11i8, 30i8, 118i8, -16i8, 5i8, -1i8],
    [-1i8, 4i8, -10i8, 27i8, 119i8, -15i8, 5i8, -1i8],
    [-1i8, 4i8, -9i8, 25i8, 120i8, -14i8, 5i8, -2i8],
    [-1i8, 3i8, -8i8, 23i8, 121i8, -13i8, 4i8, -1i8],
    [-1i8, 3i8, -7i8, 20i8, 122i8, -12i8, 4i8, -1i8],
    [-1i8, 3i8, -7i8, 18i8, 123i8, -11i8, 4i8, -1i8],
    [-1i8, 3i8, -6i8, 16i8, 124i8, -10i8, 3i8, -1i8],
    [-1i8, 2i8, -5i8, 13i8, 125i8, -8i8, 3i8, -1i8],
    [-1i8, 2i8, -4i8, 11i8, 126i8, -7i8, 2i8, -1i8],
    [0i8, 1i8, -3i8, 8i8, 126i8, -6i8, 2i8, 0i8],
    [0i8, 1i8, -2i8, 6i8, 127i8, -5i8, 1i8, 0i8],
    [0i8, 1i8, -2i8, 4i8, 127i8, -3i8, 1i8, 0i8],
    [0i8, 0i8, 0i8, 2i8, 127i8, -1i8, 0i8, 0i8],
    [0i8, 0i8, 0i8, 1i8, 127i8, 0i8, 0i8, 0i8],
    [0i8, 0i8, 0i8, -1i8, 127i8, 2i8, 0i8, 0i8],
    [0i8, 0i8, 1i8, -3i8, 127i8, 4i8, -1i8, 0i8],
    [0i8, 0i8, 1i8, -4i8, 126i8, 6i8, -2i8, 1i8],
    [0i8, 0i8, 1i8, -5i8, 126i8, 8i8, -3i8, 1i8],
    [0i8, 0i8, 1i8, -6i8, 125i8, 11i8, -4i8, 1i8],
    [0i8, 0i8, 1i8, -7i8, 124i8, 13i8, -4i8, 1i8],
    [0i8, 0i8, 2i8, -8i8, 123i8, 15i8, -5i8, 1i8],
    [0i8, 0i8, 2i8, -9i8, 122i8, 18i8, -6i8, 1i8],
    [0i8, 0i8, 2i8, -10i8, 121i8, 20i8, -6i8, 1i8],
    [0i8, 0i8, 2i8, -11i8, 120i8, 22i8, -7i8, 2i8],
    [0i8, 0i8, 2i8, -12i8, 119i8, 25i8, -8i8, 2i8],
    [0i8, 0i8, 3i8, -13i8, 117i8, 27i8, -8i8, 2i8],
    [0i8, 0i8, 3i8, -13i8, 116i8, 29i8, -9i8, 2i8],
    [0i8, 0i8, 3i8, -14i8, 114i8, 32i8, -10i8, 3i8],
    [0i8, 0i8, 3i8, -15i8, 113i8, 35i8, -10i8, 2i8],
    [0i8, 0i8, 3i8, -15i8, 111i8, 37i8, -11i8, 3i8],
    [0i8, 0i8, 3i8, -16i8, 109i8, 40i8, -11i8, 3i8],
    [0i8, 0i8, 3i8, -16i8, 108i8, 42i8, -12i8, 3i8],
    [0i8, 0i8, 4i8, -17i8, 106i8, 45i8, -13i8, 3i8],
    [0i8, 0i8, 4i8, -17i8, 104i8, 47i8, -13i8, 3i8],
    [0i8, 0i8, 4i8, -17i8, 102i8, 50i8, -14i8, 3i8],
    [0i8, 0i8, 4i8, -17i8, 100i8, 52i8, -14i8, 3i8],
    [0i8, 0i8, 4i8, -18i8, 98i8, 55i8, -15i8, 4i8],
    [0i8, 0i8, 4i8, -18i8, 96i8, 58i8, -15i8, 3i8],
    [0i8, 0i8, 4i8, -18i8, 94i8, 60i8, -16i8, 4i8],
    [0i8, 0i8, 4i8, -18i8, 91i8, 63i8, -16i8, 4i8],
    [0i8, 0i8, 4i8, -18i8, 89i8, 65i8, -16i8, 4i8],
    [0i8, 0i8, 4i8, -18i8, 87i8, 68i8, -17i8, 4i8],
    [0i8, 0i8, 4i8, -18i8, 85i8, 70i8, -17i8, 4i8],
    [0i8, 0i8, 4i8, -18i8, 82i8, 73i8, -17i8, 4i8],
    [0i8, 0i8, 4i8, -18i8, 80i8, 75i8, -17i8, 4i8],
    [0i8, 0i8, 4i8, -18i8, 78i8, 78i8, -18i8, 4i8],
    [0i8, 0i8, 4i8, -17i8, 75i8, 80i8, -18i8, 4i8],
    [0i8, 0i8, 4i8, -17i8, 73i8, 82i8, -18i8, 4i8],
    [0i8, 0i8, 4i8, -17i8, 70i8, 85i8, -18i8, 4i8],
    [0i8, 0i8, 4i8, -17i8, 68i8, 87i8, -18i8, 4i8],
    [0i8, 0i8, 4i8, -16i8, 65i8, 89i8, -18i8, 4i8],
    [0i8, 0i8, 4i8, -16i8, 63i8, 91i8, -18i8, 4i8],
    [0i8, 0i8, 4i8, -16i8, 60i8, 94i8, -18i8, 4i8],
    [0i8, 0i8, 3i8, -15i8, 58i8, 96i8, -18i8, 4i8],
    [0i8, 0i8, 4i8, -15i8, 55i8, 98i8, -18i8, 4i8],
    [0i8, 0i8, 3i8, -14i8, 52i8, 100i8, -17i8, 4i8],
    [0i8, 0i8, 3i8, -14i8, 50i8, 102i8, -17i8, 4i8],
    [0i8, 0i8, 3i8, -13i8, 47i8, 104i8, -17i8, 4i8],
    [0i8, 0i8, 3i8, -13i8, 45i8, 106i8, -17i8, 4i8],
    [0i8, 0i8, 3i8, -12i8, 42i8, 108i8, -16i8, 3i8],
    [0i8, 0i8, 3i8, -11i8, 40i8, 109i8, -16i8, 3i8],
    [0i8, 0i8, 3i8, -11i8, 37i8, 111i8, -15i8, 3i8],
    [0i8, 0i8, 2i8, -10i8, 35i8, 113i8, -15i8, 3i8],
    [0i8, 0i8, 3i8, -10i8, 32i8, 114i8, -14i8, 3i8],
    [0i8, 0i8, 2i8, -9i8, 29i8, 116i8, -13i8, 3i8],
    [0i8, 0i8, 2i8, -8i8, 27i8, 117i8, -13i8, 3i8],
    [0i8, 0i8, 2i8, -8i8, 25i8, 119i8, -12i8, 2i8],
    [0i8, 0i8, 2i8, -7i8, 22i8, 120i8, -11i8, 2i8],
    [0i8, 0i8, 1i8, -6i8, 20i8, 121i8, -10i8, 2i8],
    [0i8, 0i8, 1i8, -6i8, 18i8, 122i8, -9i8, 2i8],
    [0i8, 0i8, 1i8, -5i8, 15i8, 123i8, -8i8, 2i8],
    [0i8, 0i8, 1i8, -4i8, 13i8, 124i8, -7i8, 1i8],
    [0i8, 0i8, 1i8, -4i8, 11i8, 125i8, -6i8, 1i8],
    [0i8, 0i8, 1i8, -3i8, 8i8, 126i8, -5i8, 1i8],
    [0i8, 0i8, 1i8, -2i8, 6i8, 126i8, -4i8, 1i8],
    [0i8, 0i8, 0i8, -1i8, 4i8, 127i8, -3i8, 1i8],
    [0i8, 0i8, 0i8, 0i8, 2i8, 127i8, -1i8, 0i8],
    [0i8, 0i8, 0i8, 0i8, 2i8, 127i8, -1i8, 0i8],
];
#[no_mangle]
pub static mut dav1d_resize_filter: [[int8_t; 8]; 64] = [
    [0i8, 0i8, 0i8, -(128i32) as int8_t, 0i8, 0i8, 0i8, 0i8],
    [0i8, 0i8, 1i8, -(128i32) as int8_t, -2i8, 1i8, 0i8, 0i8],
    [0i8, -1i8, 3i8, -127i8, -4i8, 2i8, -1i8, 0i8],
    [0i8, -1i8, 4i8, -127i8, -6i8, 3i8, -1i8, 0i8],
    [0i8, -2i8, 6i8, -126i8, -8i8, 3i8, -1i8, 0i8],
    [0i8, -2i8, 7i8, -125i8, -11i8, 4i8, -1i8, 0i8],
    [1i8, -2i8, 8i8, -125i8, -13i8, 5i8, -2i8, 0i8],
    [1i8, -3i8, 9i8, -124i8, -15i8, 6i8, -2i8, 0i8],
    [1i8, -3i8, 10i8, -123i8, -18i8, 6i8, -2i8, 1i8],
    [1i8, -3i8, 11i8, -122i8, -20i8, 7i8, -3i8, 1i8],
    [1i8, -4i8, 12i8, -121i8, -22i8, 8i8, -3i8, 1i8],
    [1i8, -4i8, 13i8, -120i8, -25i8, 9i8, -3i8, 1i8],
    [1i8, -4i8, 14i8, -118i8, -28i8, 9i8, -3i8, 1i8],
    [1i8, -4i8, 15i8, -117i8, -30i8, 10i8, -4i8, 1i8],
    [1i8, -5i8, 16i8, -116i8, -32i8, 11i8, -4i8, 1i8],
    [1i8, -5i8, 16i8, -114i8, -35i8, 12i8, -4i8, 1i8],
    [1i8, -5i8, 17i8, -112i8, -38i8, 12i8, -4i8, 1i8],
    [1i8, -5i8, 18i8, -111i8, -40i8, 13i8, -5i8, 1i8],
    [1i8, -5i8, 18i8, -109i8, -43i8, 14i8, -5i8, 1i8],
    [1i8, -6i8, 19i8, -107i8, -45i8, 14i8, -5i8, 1i8],
    [1i8, -6i8, 19i8, -105i8, -48i8, 15i8, -5i8, 1i8],
    [1i8, -6i8, 19i8, -103i8, -51i8, 16i8, -5i8, 1i8],
    [1i8, -6i8, 20i8, -101i8, -53i8, 16i8, -6i8, 1i8],
    [1i8, -6i8, 20i8, -99i8, -56i8, 17i8, -6i8, 1i8],
    [1i8, -6i8, 20i8, -97i8, -58i8, 17i8, -6i8, 1i8],
    [1i8, -6i8, 20i8, -95i8, -61i8, 18i8, -6i8, 1i8],
    [2i8, -7i8, 20i8, -93i8, -64i8, 18i8, -6i8, 2i8],
    [2i8, -7i8, 20i8, -91i8, -66i8, 19i8, -6i8, 1i8],
    [2i8, -7i8, 20i8, -88i8, -69i8, 19i8, -6i8, 1i8],
    [2i8, -7i8, 20i8, -86i8, -71i8, 19i8, -6i8, 1i8],
    [2i8, -7i8, 20i8, -84i8, -74i8, 20i8, -7i8, 2i8],
    [2i8, -7i8, 20i8, -81i8, -76i8, 20i8, -7i8, 1i8],
    [2i8, -7i8, 20i8, -79i8, -79i8, 20i8, -7i8, 2i8],
    [1i8, -7i8, 20i8, -76i8, -81i8, 20i8, -7i8, 2i8],
    [2i8, -7i8, 20i8, -74i8, -84i8, 20i8, -7i8, 2i8],
    [1i8, -6i8, 19i8, -71i8, -86i8, 20i8, -7i8, 2i8],
    [1i8, -6i8, 19i8, -69i8, -88i8, 20i8, -7i8, 2i8],
    [1i8, -6i8, 19i8, -66i8, -91i8, 20i8, -7i8, 2i8],
    [2i8, -6i8, 18i8, -64i8, -93i8, 20i8, -7i8, 2i8],
    [1i8, -6i8, 18i8, -61i8, -95i8, 20i8, -6i8, 1i8],
    [1i8, -6i8, 17i8, -58i8, -97i8, 20i8, -6i8, 1i8],
    [1i8, -6i8, 17i8, -56i8, -99i8, 20i8, -6i8, 1i8],
    [1i8, -6i8, 16i8, -53i8, -101i8, 20i8, -6i8, 1i8],
    [1i8, -5i8, 16i8, -51i8, -103i8, 19i8, -6i8, 1i8],
    [1i8, -5i8, 15i8, -48i8, -105i8, 19i8, -6i8, 1i8],
    [1i8, -5i8, 14i8, -45i8, -107i8, 19i8, -6i8, 1i8],
    [1i8, -5i8, 14i8, -43i8, -109i8, 18i8, -5i8, 1i8],
    [1i8, -5i8, 13i8, -40i8, -111i8, 18i8, -5i8, 1i8],
    [1i8, -4i8, 12i8, -38i8, -112i8, 17i8, -5i8, 1i8],
    [1i8, -4i8, 12i8, -35i8, -114i8, 16i8, -5i8, 1i8],
    [1i8, -4i8, 11i8, -32i8, -116i8, 16i8, -5i8, 1i8],
    [1i8, -4i8, 10i8, -30i8, -117i8, 15i8, -4i8, 1i8],
    [1i8, -3i8, 9i8, -28i8, -118i8, 14i8, -4i8, 1i8],
    [1i8, -3i8, 9i8, -25i8, -120i8, 13i8, -4i8, 1i8],
    [1i8, -3i8, 8i8, -22i8, -121i8, 12i8, -4i8, 1i8],
    [1i8, -3i8, 7i8, -20i8, -122i8, 11i8, -3i8, 1i8],
    [1i8, -2i8, 6i8, -18i8, -123i8, 10i8, -3i8, 1i8],
    [0i8, -2i8, 6i8, -15i8, -124i8, 9i8, -3i8, 1i8],
    [0i8, -2i8, 5i8, -13i8, -125i8, 8i8, -2i8, 1i8],
    [0i8, -1i8, 4i8, -11i8, -125i8, 7i8, -2i8, 0i8],
    [0i8, -1i8, 3i8, -8i8, -126i8, 6i8, -2i8, 0i8],
    [0i8, -1i8, 3i8, -6i8, -127i8, 4i8, -1i8, 0i8],
    [0i8, -1i8, 2i8, -4i8, -127i8, 3i8, -1i8, 0i8],
    [0i8, 0i8, 1i8, -2i8, -(128i32) as int8_t, 1i8, 0i8, 0i8],
];
#[no_mangle]
pub static mut dav1d_sm_weights: [uint8_t; 128] = [
    0u8, 0u8, 255u8, 128u8, 255u8, 149u8, 85u8, 64u8, 255u8, 197u8, 146u8, 105u8, 73u8, 50u8, 37u8,
    32u8, 255u8, 225u8, 196u8, 170u8, 145u8, 123u8, 102u8, 84u8, 68u8, 54u8, 43u8, 33u8, 26u8,
    20u8, 17u8, 16u8, 255u8, 240u8, 225u8, 210u8, 196u8, 182u8, 169u8, 157u8, 145u8, 133u8, 122u8,
    111u8, 101u8, 92u8, 83u8, 74u8, 66u8, 59u8, 52u8, 45u8, 39u8, 34u8, 29u8, 25u8, 21u8, 17u8,
    14u8, 12u8, 10u8, 9u8, 8u8, 8u8, 255u8, 248u8, 240u8, 233u8, 225u8, 218u8, 210u8, 203u8, 196u8,
    189u8, 182u8, 176u8, 169u8, 163u8, 156u8, 150u8, 144u8, 138u8, 133u8, 127u8, 121u8, 116u8,
    111u8, 106u8, 101u8, 96u8, 91u8, 86u8, 82u8, 77u8, 73u8, 69u8, 65u8, 61u8, 57u8, 54u8, 50u8,
    47u8, 44u8, 41u8, 38u8, 35u8, 32u8, 29u8, 27u8, 25u8, 22u8, 20u8, 18u8, 16u8, 15u8, 13u8, 12u8,
    10u8, 9u8, 8u8, 7u8, 6u8, 6u8, 5u8, 5u8, 4u8, 4u8, 4u8,
];
#[no_mangle]
pub static mut dav1d_dr_intra_derivative: [uint16_t; 44] = [
    0u16, 1023u16, 0u16, 547u16, 372u16, 0u16, 0u16, 273u16, 215u16, 0u16, 178u16, 151u16, 0u16,
    132u16, 116u16, 0u16, 102u16, 0u16, 90u16, 80u16, 0u16, 71u16, 64u16, 0u16, 57u16, 51u16, 0u16,
    45u16, 0u16, 40u16, 35u16, 0u16, 31u16, 27u16, 0u16, 23u16, 19u16, 0u16, 15u16, 0u16, 11u16,
    0u16, 7u16, 3u16,
];
#[no_mangle]
pub static mut dav1d_filter_intra_taps: [[int8_t; 64]; 5] = [
    [
        -6i8, 10i8, -5i8, 2i8, -3i8, 1i8, -3i8, 1i8, -4i8, 6i8, -3i8, 2i8, -3i8, 2i8, -3i8, 1i8,
        0i8, 0i8, 10i8, 0i8, 1i8, 10i8, 1i8, 2i8, 0i8, 0i8, 6i8, 0i8, 2i8, 6i8, 2i8, 2i8, 0i8,
        12i8, 0i8, 9i8, 0i8, 7i8, 10i8, 5i8, 0i8, 2i8, 0i8, 2i8, 0i8, 2i8, 6i8, 3i8, 0i8, 0, 0i8,
        0, 0i8, 0, 0i8, 0, 12i8, 0, 9i8, 0, 7i8, 0, 5i8, 0,
    ],
    [
        -10i8, 16i8, -6i8, 0i8, -4i8, 0i8, -2i8, 0i8, -10i8, 16i8, -6i8, 0i8, -4i8, 0i8, -2i8, 0i8,
        0i8, 0i8, 16i8, 0i8, 0i8, 16i8, 0i8, 0i8, 0i8, 0i8, 16i8, 0i8, 0i8, 16i8, 0i8, 0i8, 0i8,
        10i8, 0i8, 6i8, 0i8, 4i8, 16i8, 2i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 16i8, 0i8, 0i8, 0, 0i8,
        0, 0i8, 0, 0i8, 0, 10i8, 0, 6i8, 0, 4i8, 0, 2i8, 0,
    ],
    [
        -8i8, 8i8, -8i8, 0i8, -8i8, 0i8, -8i8, 0i8, -4i8, 4i8, -4i8, 0i8, -4i8, 0i8, -4i8, 0i8,
        0i8, 0i8, 8i8, 0i8, 0i8, 8i8, 0i8, 0i8, 0i8, 0i8, 4i8, 0i8, 0i8, 4i8, 0i8, 0i8, 0i8, 16i8,
        0i8, 16i8, 0i8, 16i8, 8i8, 16i8, 0i8, 0i8, 0i8, 0i8, 0i8, 0i8, 4i8, 0i8, 0i8, 0, 0i8, 0,
        0i8, 0, 0i8, 0, 16i8, 0, 16i8, 0, 16i8, 0, 16i8, 0,
    ],
    [
        -2i8, 8i8, -1i8, 3i8, -1i8, 2i8, 0i8, 1i8, -1i8, 4i8, -1i8, 3i8, -1i8, 2i8, -1i8, 2i8, 0i8,
        0i8, 8i8, 0i8, 3i8, 8i8, 2i8, 3i8, 0i8, 0i8, 4i8, 0i8, 3i8, 4i8, 2i8, 3i8, 0i8, 10i8, 0i8,
        6i8, 0i8, 4i8, 8i8, 2i8, 0i8, 3i8, 0i8, 4i8, 0i8, 4i8, 4i8, 3i8, 0i8, 0, 0i8, 0, 0i8, 0,
        0i8, 0, 10i8, 0, 6i8, 0, 4i8, 0, 3i8, 0,
    ],
    [
        -12i8, 14i8, -10i8, 0i8, -9i8, 0i8, -8i8, 0i8, -10i8, 12i8, -9i8, 1i8, -8i8, 0i8, -7i8,
        0i8, 0i8, 0i8, 14i8, 0i8, 0i8, 14i8, 0i8, 0i8, 0i8, 0i8, 12i8, 0i8, 0i8, 12i8, 0i8, 1i8,
        0i8, 14i8, 0i8, 12i8, 0i8, 11i8, 14i8, 10i8, 0i8, 0i8, 0i8, 0i8, 0i8, 1i8, 12i8, 1i8, 0i8,
        0, 0i8, 0, 0i8, 0, 0i8, 0, 14i8, 0, 12i8, 0, 11i8, 0, 9i8, 0,
    ],
];
#[no_mangle]
pub static mut dav1d_obmc_masks: [uint8_t; 64] = [
    0u8, 0u8, 19u8, 0u8, 25u8, 14u8, 5u8, 0u8, 28u8, 22u8, 16u8, 11u8, 7u8, 3u8, 0u8, 0u8, 30u8,
    27u8, 24u8, 21u8, 18u8, 15u8, 12u8, 10u8, 8u8, 6u8, 4u8, 3u8, 0u8, 0u8, 0u8, 0u8, 31u8, 29u8,
    28u8, 26u8, 24u8, 23u8, 21u8, 20u8, 19u8, 17u8, 16u8, 14u8, 13u8, 12u8, 11u8, 9u8, 8u8, 7u8,
    6u8, 5u8, 4u8, 4u8, 3u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
];
#[no_mangle]
pub static mut dav1d_gaussian_sequence: [int16_t; 2048] = [
    56i16, 568i16, -180i16, 172i16, 124i16, -84i16, 172i16, -64i16, -900i16, 24i16, 820i16, 224i16,
    1248i16, 996i16, 272i16, -8i16, -916i16, -388i16, -732i16, -104i16, -188i16, 800i16, 112i16,
    -652i16, -320i16, -376i16, 140i16, -252i16, 492i16, -168i16, 44i16, -788i16, 588i16, -584i16,
    500i16, -228i16, 12i16, 680i16, 272i16, -476i16, 972i16, -100i16, 652i16, 368i16, 432i16,
    -196i16, -720i16, -192i16, 1000i16, -332i16, 652i16, -136i16, -552i16, -604i16, -4i16, 192i16,
    -220i16, -136i16, 1000i16, -52i16, 372i16, -96i16, -624i16, 124i16, -24i16, 396i16, 540i16,
    -12i16, -104i16, 640i16, 464i16, 244i16, -208i16, -84i16, 368i16, -528i16, -740i16, 248i16,
    -968i16, -848i16, 608i16, 376i16, -60i16, -292i16, -40i16, -156i16, 252i16, -292i16, 248i16,
    224i16, -280i16, 400i16, -244i16, 244i16, -60i16, 76i16, -80i16, 212i16, 532i16, 340i16,
    128i16, -36i16, 824i16, -352i16, -60i16, -264i16, -96i16, -612i16, 416i16, -704i16, 220i16,
    -204i16, 640i16, -160i16, 1220i16, -408i16, 900i16, 336i16, 20i16, -336i16, -96i16, -792i16,
    304i16, 48i16, -28i16, -1232i16, -1172i16, -448i16, 104i16, -292i16, -520i16, 244i16, 60i16,
    -948i16, 0i16, -708i16, 268i16, 108i16, 356i16, -548i16, 488i16, -344i16, -136i16, 488i16,
    -196i16, -224i16, 656i16, -236i16, -1128i16, 60i16, 4i16, 140i16, 276i16, -676i16, -376i16,
    168i16, -108i16, 464i16, 8i16, 564i16, 64i16, 240i16, 308i16, -300i16, -400i16, -456i16,
    -136i16, 56i16, 120i16, -408i16, -116i16, 436i16, 504i16, -232i16, 328i16, 844i16, -164i16,
    -84i16, 784i16, -168i16, 232i16, -224i16, 348i16, -376i16, 128i16, 568i16, 96i16, -1244i16,
    -288i16, 276i16, 848i16, 832i16, -360i16, 656i16, 464i16, -384i16, -332i16, -356i16, 728i16,
    -388i16, 160i16, -192i16, 468i16, 296i16, 224i16, 140i16, -776i16, -100i16, 280i16, 4i16,
    196i16, 44i16, -36i16, -648i16, 932i16, 16i16, 1428i16, 28i16, 528i16, 808i16, 772i16, 20i16,
    268i16, 88i16, -332i16, -284i16, 124i16, -384i16, -448i16, 208i16, -228i16, -1044i16, -328i16,
    660i16, 380i16, -148i16, -300i16, 588i16, 240i16, 540i16, 28i16, 136i16, -88i16, -436i16,
    256i16, 296i16, -1000i16, 1400i16, 0i16, -48i16, 1056i16, -136i16, 264i16, -528i16, -1108i16,
    632i16, -484i16, -592i16, -344i16, 796i16, 124i16, -668i16, -768i16, 388i16, 1296i16, -232i16,
    -188i16, -200i16, -288i16, -4i16, 308i16, 100i16, -168i16, 256i16, -500i16, 204i16, -508i16,
    648i16, -136i16, 372i16, -272i16, -120i16, -1004i16, -552i16, -548i16, -384i16, 548i16,
    -296i16, 428i16, -108i16, -8i16, -912i16, -324i16, -224i16, -88i16, -112i16, -220i16, -100i16,
    996i16, -796i16, 548i16, 360i16, -216i16, 180i16, 428i16, -200i16, -212i16, 148i16, 96i16,
    148i16, 284i16, 216i16, -412i16, -320i16, 120i16, -300i16, -384i16, -604i16, -572i16, -332i16,
    -8i16, -180i16, -176i16, 696i16, 116i16, -88i16, 628i16, 76i16, 44i16, -516i16, 240i16,
    -208i16, -40i16, 100i16, -592i16, 344i16, -308i16, -452i16, -228i16, 20i16, 916i16, -1752i16,
    -136i16, -340i16, -804i16, 140i16, 40i16, 512i16, 340i16, 248i16, 184i16, -492i16, 896i16,
    -156i16, 932i16, -628i16, 328i16, -688i16, -448i16, -616i16, -752i16, -100i16, 560i16,
    -1020i16, 180i16, -800i16, -64i16, 76i16, 576i16, 1068i16, 396i16, 660i16, 552i16, -108i16,
    -28i16, 320i16, -628i16, 312i16, -92i16, -92i16, -472i16, 268i16, 16i16, 560i16, 516i16,
    -672i16, -52i16, 492i16, -100i16, 260i16, 384i16, 284i16, 292i16, 304i16, -148i16, 88i16,
    -152i16, 1012i16, 1064i16, -228i16, 164i16, -376i16, -684i16, 592i16, -392i16, 156i16, 196i16,
    -524i16, -64i16, -884i16, 160i16, -176i16, 636i16, 648i16, 404i16, -396i16, -436i16, 864i16,
    424i16, -728i16, 988i16, -604i16, 904i16, -592i16, 296i16, -224i16, 536i16, -176i16, -920i16,
    436i16, -48i16, 1176i16, -884i16, 416i16, -776i16, -824i16, -884i16, 524i16, -548i16, -564i16,
    -68i16, -164i16, -96i16, 692i16, 364i16, -692i16, -1012i16, -68i16, 260i16, -480i16, 876i16,
    -1116i16, 452i16, -332i16, -352i16, 892i16, -1088i16, 1220i16, -676i16, 12i16, -292i16, 244i16,
    496i16, 372i16, -32i16, 280i16, 200i16, 112i16, -440i16, -96i16, 24i16, -644i16, -184i16,
    56i16, -432i16, 224i16, -980i16, 272i16, -260i16, 144i16, -436i16, 420i16, 356i16, 364i16,
    -528i16, 76i16, 172i16, -744i16, -368i16, 404i16, -752i16, -416i16, 684i16, -688i16, 72i16,
    540i16, 416i16, 92i16, 444i16, 480i16, -72i16, -1416i16, 164i16, -1172i16, -68i16, 24i16,
    424i16, 264i16, 1040i16, 128i16, -912i16, -524i16, -356i16, 64i16, 876i16, -12i16, 4i16,
    -88i16, 532i16, 272i16, -524i16, 320i16, 276i16, -508i16, 940i16, 24i16, -400i16, -120i16,
    756i16, 60i16, 236i16, -412i16, 100i16, 376i16, -484i16, 400i16, -100i16, -740i16, -108i16,
    -260i16, 328i16, -268i16, 224i16, -200i16, -416i16, 184i16, -604i16, -564i16, -20i16, 296i16,
    60i16, 892i16, -888i16, 60i16, 164i16, 68i16, -760i16, 216i16, -296i16, 904i16, -336i16,
    -28i16, 404i16, -356i16, -568i16, -208i16, -1480i16, -512i16, 296i16, 328i16, -360i16, -164i16,
    -1560i16, -776i16, 1156i16, -428i16, 164i16, -504i16, -112i16, 120i16, -216i16, -148i16,
    -264i16, 308i16, 32i16, 64i16, -72i16, 72i16, 116i16, 176i16, -64i16, -272i16, 460i16, -536i16,
    -784i16, -280i16, 348i16, 108i16, -752i16, -132i16, 524i16, -540i16, -776i16, 116i16, -296i16,
    -1196i16, -288i16, -560i16, 1040i16, -472i16, 116i16, -848i16, -1116i16, 116i16, 636i16,
    696i16, 284i16, -176i16, 1016i16, 204i16, -864i16, -648i16, -248i16, 356i16, 972i16, -584i16,
    -204i16, 264i16, 880i16, 528i16, -24i16, -184i16, 116i16, 448i16, -144i16, 828i16, 524i16,
    212i16, -212i16, 52i16, 12i16, 200i16, 268i16, -488i16, -404i16, -880i16, 824i16, -672i16,
    -40i16, 908i16, -248i16, 500i16, 716i16, -576i16, 492i16, -576i16, 16i16, 720i16, -108i16,
    384i16, 124i16, 344i16, 280i16, 576i16, -500i16, 252i16, 104i16, -308i16, 196i16, -188i16,
    -8i16, 1268i16, 296i16, 1032i16, -1196i16, 436i16, 316i16, 372i16, -432i16, -200i16, -660i16,
    704i16, -224i16, 596i16, -132i16, 268i16, 32i16, -452i16, 884i16, 104i16, -1008i16, 424i16,
    -1348i16, -280i16, 4i16, -1168i16, 368i16, 476i16, 696i16, 300i16, -8i16, 24i16, 180i16,
    -592i16, -196i16, 388i16, 304i16, 500i16, 724i16, -160i16, 244i16, -84i16, 272i16, -256i16,
    -420i16, 320i16, 208i16, -144i16, -156i16, 156i16, 364i16, 452i16, 28i16, 540i16, 316i16,
    220i16, -644i16, -248i16, 464i16, 72i16, 360i16, 32i16, -388i16, 496i16, -680i16, -48i16,
    208i16, -116i16, -408i16, 60i16, -604i16, -392i16, 548i16, -840i16, 784i16, -460i16, 656i16,
    -544i16, -388i16, -264i16, 908i16, -800i16, -628i16, -612i16, -568i16, 572i16, -220i16, 164i16,
    288i16, -16i16, -308i16, 308i16, -112i16, -636i16, -760i16, 280i16, -668i16, 432i16, 364i16,
    240i16, -196i16, 604i16, 340i16, 384i16, 196i16, 592i16, -44i16, -500i16, 432i16, -580i16,
    -132i16, 636i16, -76i16, 392i16, 4i16, -412i16, 540i16, 508i16, 328i16, -356i16, -36i16, 16i16,
    -220i16, -64i16, -248i16, -60i16, 24i16, -192i16, 368i16, 1040i16, 92i16, -24i16, -1044i16,
    -32i16, 40i16, 104i16, 148i16, 192i16, -136i16, -520i16, 56i16, -816i16, -224i16, 732i16,
    392i16, 356i16, 212i16, -80i16, -424i16, -1008i16, -324i16, 588i16, -1496i16, 576i16, 460i16,
    -816i16, -848i16, 56i16, -580i16, -92i16, -1372i16, -112i16, -496i16, 200i16, 364i16, 52i16,
    -140i16, 48i16, -48i16, -60i16, 84i16, 72i16, 40i16, 132i16, -356i16, -268i16, -104i16,
    -284i16, -404i16, 732i16, -520i16, 164i16, -304i16, -540i16, 120i16, 328i16, -76i16, -460i16,
    756i16, 388i16, 588i16, 236i16, -436i16, -72i16, -176i16, -404i16, -316i16, -148i16, 716i16,
    -604i16, 404i16, -72i16, -88i16, -888i16, -68i16, 944i16, 88i16, -220i16, -344i16, 960i16,
    472i16, 460i16, -232i16, 704i16, 120i16, 832i16, -228i16, 692i16, -508i16, 132i16, -476i16,
    844i16, -748i16, -364i16, -44i16, 1116i16, -1104i16, -1056i16, 76i16, 428i16, 552i16, -692i16,
    60i16, 356i16, 96i16, -384i16, -188i16, -612i16, -576i16, 736i16, 508i16, 892i16, 352i16,
    -1132i16, 504i16, -24i16, -352i16, 324i16, 332i16, -600i16, -312i16, 292i16, 508i16, -144i16,
    -8i16, 484i16, 48i16, 284i16, -260i16, -240i16, 256i16, -100i16, -292i16, -204i16, -44i16,
    472i16, -204i16, 908i16, -188i16, -1000i16, -256i16, 92i16, 1164i16, -392i16, 564i16, 356i16,
    652i16, -28i16, -884i16, 256i16, 484i16, -192i16, 760i16, -176i16, 376i16, -524i16, -452i16,
    -436i16, 860i16, -736i16, 212i16, 124i16, 504i16, -476i16, 468i16, 76i16, -472i16, 552i16,
    -692i16, -944i16, -620i16, 740i16, -240i16, 400i16, 132i16, 20i16, 192i16, -196i16, 264i16,
    -668i16, -1012i16, -60i16, 296i16, -316i16, -828i16, 76i16, -156i16, 284i16, -768i16, -448i16,
    -832i16, 148i16, 248i16, 652i16, 616i16, 1236i16, 288i16, -328i16, -400i16, -124i16, 588i16,
    220i16, 520i16, -696i16, 1032i16, 768i16, -740i16, -92i16, -272i16, 296i16, 448i16, -464i16,
    412i16, -200i16, 392i16, 440i16, -200i16, 264i16, -152i16, -260i16, 320i16, 1032i16, 216i16,
    320i16, -8i16, -64i16, 156i16, -1016i16, 1084i16, 1172i16, 536i16, 484i16, -432i16, 132i16,
    372i16, -52i16, -256i16, 84i16, 116i16, -352i16, 48i16, 116i16, 304i16, -384i16, 412i16,
    924i16, -300i16, 528i16, 628i16, 180i16, 648i16, 44i16, -980i16, -220i16, 1320i16, 48i16,
    332i16, 748i16, 524i16, -268i16, -720i16, 540i16, -276i16, 564i16, -344i16, -208i16, -196i16,
    436i16, 896i16, 88i16, -392i16, 132i16, 80i16, -964i16, -288i16, 568i16, 56i16, -48i16,
    -456i16, 888i16, 8i16, 552i16, -156i16, -292i16, 948i16, 288i16, 128i16, -716i16, -292i16,
    1192i16, -152i16, 876i16, 352i16, -600i16, -260i16, -812i16, -468i16, -28i16, -120i16, -32i16,
    -44i16, 1284i16, 496i16, 192i16, 464i16, 312i16, -76i16, -516i16, -380i16, -456i16, -1012i16,
    -48i16, 308i16, -156i16, 36i16, 492i16, -156i16, -808i16, 188i16, 1652i16, 68i16, -120i16,
    -116i16, 316i16, 160i16, -140i16, 352i16, 808i16, -416i16, 592i16, 316i16, -480i16, 56i16,
    528i16, -204i16, -568i16, 372i16, -232i16, 752i16, -344i16, 744i16, -4i16, 324i16, -416i16,
    -600i16, 768i16, 268i16, -248i16, -88i16, -132i16, -420i16, -432i16, 80i16, -288i16, 404i16,
    -316i16, -1216i16, -588i16, 520i16, -108i16, 92i16, -320i16, 368i16, -480i16, -216i16, -92i16,
    1688i16, -300i16, 180i16, 1020i16, -176i16, 820i16, -68i16, -228i16, -260i16, 436i16, -904i16,
    20i16, 40i16, -508i16, 440i16, -736i16, 312i16, 332i16, 204i16, 760i16, -372i16, 728i16, 96i16,
    -20i16, -632i16, -520i16, -560i16, 336i16, 1076i16, -64i16, -532i16, 776i16, 584i16, 192i16,
    396i16, -728i16, -520i16, 276i16, -188i16, 80i16, -52i16, -612i16, -252i16, -48i16, 648i16,
    212i16, -688i16, 228i16, -52i16, -260i16, 428i16, -412i16, -272i16, -404i16, 180i16, 816i16,
    -796i16, 48i16, 152i16, 484i16, -88i16, -216i16, 988i16, 696i16, 188i16, -528i16, 648i16,
    -116i16, -180i16, 316i16, 476i16, 12i16, -564i16, 96i16, 476i16, -252i16, -364i16, -376i16,
    -392i16, 556i16, -256i16, -576i16, 260i16, -352i16, 120i16, -16i16, -136i16, -260i16, -492i16,
    72i16, 556i16, 660i16, 580i16, 616i16, 772i16, 436i16, 424i16, -32i16, -324i16, -1268i16,
    416i16, -324i16, -80i16, 920i16, 160i16, 228i16, 724i16, 32i16, -516i16, 64i16, 384i16, 68i16,
    -128i16, 136i16, 240i16, 248i16, -204i16, -68i16, 252i16, -932i16, -120i16, -480i16, -628i16,
    -84i16, 192i16, 852i16, -404i16, -288i16, -132i16, 204i16, 100i16, 168i16, -68i16, -196i16,
    -868i16, 460i16, 1080i16, 380i16, -80i16, 244i16, 0i16, 484i16, -888i16, 64i16, 184i16, 352i16,
    600i16, 460i16, 164i16, 604i16, -196i16, 320i16, -64i16, 588i16, -184i16, 228i16, 12i16,
    372i16, 48i16, -848i16, -344i16, 224i16, 208i16, -200i16, 484i16, 128i16, -20i16, 272i16,
    -468i16, -840i16, 384i16, 256i16, -720i16, -520i16, -464i16, -580i16, 112i16, -120i16, 644i16,
    -356i16, -208i16, -608i16, -528i16, 704i16, 560i16, -424i16, 392i16, 828i16, 40i16, 84i16,
    200i16, -152i16, 0i16, -144i16, 584i16, 280i16, -120i16, 80i16, -556i16, -972i16, -196i16,
    -472i16, 724i16, 80i16, 168i16, -32i16, 88i16, 160i16, -688i16, 0i16, 160i16, 356i16, 372i16,
    -776i16, 740i16, -128i16, 676i16, -248i16, -480i16, 4i16, -364i16, 96i16, 544i16, 232i16,
    -1032i16, 956i16, 236i16, 356i16, 20i16, -40i16, 300i16, 24i16, -676i16, -596i16, 132i16,
    1120i16, -104i16, 532i16, -1096i16, 568i16, 648i16, 444i16, 508i16, 380i16, 188i16, -376i16,
    -604i16, 1488i16, 424i16, 24i16, 756i16, -220i16, -192i16, 716i16, 120i16, 920i16, 688i16,
    168i16, 44i16, -460i16, 568i16, 284i16, 1144i16, 1160i16, 600i16, 424i16, 888i16, 656i16,
    -356i16, -320i16, 220i16, 316i16, -176i16, -724i16, -188i16, -816i16, -628i16, -348i16,
    -228i16, -380i16, 1012i16, -452i16, -660i16, 736i16, 928i16, 404i16, -696i16, -72i16, -268i16,
    -892i16, 128i16, 184i16, -344i16, -780i16, 360i16, 336i16, 400i16, 344i16, 428i16, 548i16,
    -112i16, 136i16, -228i16, -216i16, -820i16, -516i16, 340i16, 92i16, -136i16, 116i16, -300i16,
    376i16, -244i16, 100i16, -316i16, -520i16, -284i16, -12i16, 824i16, 164i16, -548i16, -180i16,
    -128i16, 116i16, -924i16, -828i16, 268i16, -368i16, -580i16, 620i16, 192i16, 160i16, 0i16,
    -1676i16, 1068i16, 424i16, -56i16, -360i16, 468i16, -156i16, 720i16, 288i16, -528i16, 556i16,
    -364i16, 548i16, -148i16, 504i16, 316i16, 152i16, -648i16, -620i16, -684i16, -24i16, -376i16,
    -384i16, -108i16, -920i16, -1032i16, 768i16, 180i16, -264i16, -508i16, -1268i16, -260i16,
    -60i16, 300i16, -240i16, 988i16, 724i16, -376i16, -576i16, -212i16, -736i16, 556i16, 192i16,
    1092i16, -620i16, -880i16, 376i16, -56i16, -4i16, -216i16, -32i16, 836i16, 268i16, 396i16,
    1332i16, 864i16, -600i16, 100i16, 56i16, -412i16, -92i16, 356i16, 180i16, 884i16, -468i16,
    -436i16, 292i16, -388i16, -804i16, -704i16, -840i16, 368i16, -348i16, 140i16, -724i16, 1536i16,
    940i16, 372i16, 112i16, -372i16, 436i16, -480i16, 1136i16, 296i16, -32i16, -228i16, 132i16,
    -48i16, -220i16, 868i16, -1016i16, -60i16, -1044i16, -464i16, 328i16, 916i16, 244i16, 12i16,
    -736i16, -296i16, 360i16, 468i16, -376i16, -108i16, -92i16, 788i16, 368i16, -56i16, 544i16,
    400i16, -672i16, -420i16, 728i16, 16i16, 320i16, 44i16, -284i16, -380i16, -796i16, 488i16,
    132i16, 204i16, -596i16, -372i16, 88i16, -152i16, -908i16, -636i16, -572i16, -624i16, -116i16,
    -692i16, -200i16, -56i16, 276i16, -88i16, 484i16, -324i16, 948i16, 864i16, 1000i16, -456i16,
    -184i16, -276i16, 292i16, -296i16, 156i16, 676i16, 320i16, 160i16, 908i16, -84i16, -1236i16,
    -288i16, -116i16, 260i16, -372i16, -644i16, 732i16, -756i16, -96i16, 84i16, 344i16, -520i16,
    348i16, -688i16, 240i16, -84i16, 216i16, -1044i16, -136i16, -676i16, -396i16, -1500i16, 960i16,
    -40i16, 176i16, 168i16, 1516i16, 420i16, -504i16, -344i16, -364i16, -360i16, 1216i16, -940i16,
    -380i16, -212i16, 252i16, -660i16, -708i16, 484i16, -444i16, -152i16, 928i16, -120i16, 1112i16,
    476i16, -260i16, 560i16, -148i16, -344i16, 108i16, -196i16, 228i16, -288i16, 504i16, 560i16,
    -328i16, -88i16, 288i16, -1008i16, 460i16, -228i16, 468i16, -836i16, -196i16, 76i16, 388i16,
    232i16, 412i16, -1168i16, -716i16, -644i16, 756i16, -172i16, -356i16, -504i16, 116i16, 432i16,
    528i16, 48i16, 476i16, -168i16, -608i16, 448i16, 160i16, -532i16, -272i16, 28i16, -676i16,
    -12i16, 828i16, 980i16, 456i16, 520i16, 104i16, -104i16, 256i16, -344i16, -4i16, -28i16,
    -368i16, -52i16, -524i16, -572i16, -556i16, -200i16, 768i16, 1124i16, -208i16, -512i16, 176i16,
    232i16, 248i16, -148i16, -888i16, 604i16, -600i16, -304i16, 804i16, -156i16, -212i16, 488i16,
    -192i16, -804i16, -256i16, 368i16, -360i16, -916i16, -328i16, 228i16, -240i16, -448i16,
    -472i16, 856i16, -556i16, -364i16, 572i16, -12i16, -156i16, -368i16, -340i16, 432i16, 252i16,
    -752i16, -152i16, 288i16, 268i16, -580i16, -848i16, -592i16, 108i16, -76i16, 244i16, 312i16,
    -716i16, 592i16, -80i16, 436i16, 360i16, 4i16, -248i16, 160i16, 516i16, 584i16, 732i16, 44i16,
    -468i16, -280i16, -292i16, -156i16, -588i16, 28i16, 308i16, 912i16, 24i16, 124i16, 156i16,
    180i16, -252i16, 944i16, -924i16, -772i16, -520i16, -428i16, -624i16, 300i16, -212i16,
    -1144i16, 32i16, -724i16, 800i16, -1128i16, -212i16, -1288i16, -848i16, 180i16, -416i16,
    440i16, 192i16, -576i16, -792i16, -76i16, -1080i16, 80i16, -532i16, -352i16, -132i16, 380i16,
    -820i16, 148i16, 1112i16, 128i16, 164i16, 456i16, 700i16, -924i16, 144i16, -668i16, -384i16,
    648i16, -832i16, 508i16, 552i16, -52i16, -100i16, -656i16, 208i16, -568i16, 748i16, -88i16,
    680i16, 232i16, 300i16, 192i16, -408i16, -1012i16, -152i16, -252i16, -268i16, 272i16, -876i16,
    -664i16, -648i16, -332i16, -136i16, 16i16, 12i16, 1152i16, -28i16, 332i16, -536i16, 320i16,
    -672i16, -460i16, -316i16, 532i16, -260i16, 228i16, -40i16, 1052i16, -816i16, 180i16, 88i16,
    -496i16, -556i16, -672i16, -368i16, 428i16, 92i16, 356i16, 404i16, -408i16, 252i16, 196i16,
    -176i16, -556i16, 792i16, 268i16, 32i16, 372i16, 40i16, 96i16, -332i16, 328i16, 120i16, 372i16,
    -900i16, -40i16, 472i16, -264i16, -592i16, 952i16, 128i16, 656i16, 112i16, 664i16, -232i16,
    420i16, 4i16, -344i16, -464i16, 556i16, 244i16, -416i16, -32i16, 252i16, 0i16, -412i16, 188i16,
    -696i16, 508i16, -476i16, 324i16, -1096i16, 656i16, -312i16, 560i16, 264i16, -136i16, 304i16,
    160i16, -64i16, -580i16, 248i16, 336i16, -720i16, 560i16, -348i16, -288i16, -276i16, -196i16,
    -500i16, 852i16, -544i16, -236i16, -1128i16, -992i16, -776i16, 116i16, 56i16, 52i16, 860i16,
    884i16, 212i16, -12i16, 168i16, 1020i16, 512i16, -552i16, 924i16, -148i16, 716i16, 188i16,
    164i16, -340i16, -520i16, -184i16, 880i16, -152i16, -680i16, -208i16, -1156i16, -300i16,
    -528i16, -472i16, 364i16, 100i16, -744i16, -1056i16, -32i16, 540i16, 280i16, 144i16, -676i16,
    -32i16, -232i16, -280i16, -224i16, 96i16, 568i16, -76i16, 172i16, 148i16, 148i16, 104i16,
    32i16, -296i16, -32i16, 788i16, -80i16, 32i16, -16i16, 280i16, 288i16, 944i16, 428i16, -484i16,
];
