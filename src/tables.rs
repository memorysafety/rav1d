use crate::include::dav1d::headers::Rav1dFilterMode;
use crate::include::dav1d::headers::Rav1dWarpedMotionParams;
use crate::include::dav1d::headers::Rav1dWarpedMotionType;
use crate::src::align::Align16;
use crate::src::align::Align4;
use crate::src::align::Align64;
use crate::src::align::Align8;
use crate::src::enum_map::DefaultValue;
use crate::src::levels::BlockLevel;
use crate::src::levels::BlockPartition;
use crate::src::levels::BlockSize;
use crate::src::levels::Filter2d;
use crate::src::levels::InterPredMode;
use crate::src::levels::TxClass;
use crate::src::levels::TxfmSize;
use crate::src::levels::TxfmType;
use crate::src::levels::ADST_ADST;
use crate::src::levels::ADST_DCT;
use crate::src::levels::ADST_FLIPADST;
use crate::src::levels::DCT_ADST;
use crate::src::levels::DCT_DCT;
use crate::src::levels::DCT_FLIPADST;
use crate::src::levels::DC_PRED;
use crate::src::levels::DIAG_DOWN_LEFT_PRED;
use crate::src::levels::DIAG_DOWN_RIGHT_PRED;
use crate::src::levels::FLIPADST_ADST;
use crate::src::levels::FLIPADST_DCT;
use crate::src::levels::FLIPADST_FLIPADST;
use crate::src::levels::GLOBALMV;
use crate::src::levels::GLOBALMV_GLOBALMV;
use crate::src::levels::HOR_DOWN_PRED;
use crate::src::levels::HOR_PRED;
use crate::src::levels::HOR_UP_PRED;
use crate::src::levels::H_ADST;
use crate::src::levels::H_DCT;
use crate::src::levels::H_FLIPADST;
use crate::src::levels::IDTX;
use crate::src::levels::NEARESTMV;
use crate::src::levels::NEARESTMV_NEARESTMV;
use crate::src::levels::NEARESTMV_NEWMV;
use crate::src::levels::NEARMV;
use crate::src::levels::NEARMV_NEARMV;
use crate::src::levels::NEARMV_NEWMV;
use crate::src::levels::NEWMV;
use crate::src::levels::NEWMV_NEARESTMV;
use crate::src::levels::NEWMV_NEARMV;
use crate::src::levels::NEWMV_NEWMV;
use crate::src::levels::N_COMP_INTER_PRED_MODES;
use crate::src::levels::N_INTRA_PRED_MODES;
use crate::src::levels::N_TX_TYPES_PLUS_LL;
use crate::src::levels::N_UV_INTRA_PRED_MODES;
use crate::src::levels::PAETH_PRED;
use crate::src::levels::SMOOTH_H_PRED;
use crate::src::levels::SMOOTH_PRED;
use crate::src::levels::SMOOTH_V_PRED;
use crate::src::levels::VERT_LEFT_PRED;
use crate::src::levels::VERT_PRED;
use crate::src::levels::VERT_RIGHT_PRED;
use crate::src::levels::V_ADST;
use crate::src::levels::V_DCT;
use crate::src::levels::V_FLIPADST;
use std::ffi::c_uint;
use strum::EnumCount;

#[repr(C)]
pub struct TxfmInfo {
    pub w: u8,
    pub h: u8,
    pub lw: u8,
    pub lh: u8,
    pub min: u8,
    pub max: u8,
    pub sub: TxfmSize,
    pub ctx: u8,
}

pub static dav1d_al_part_ctx: [[[u8; BlockPartition::COUNT]; BlockLevel::COUNT]; 2] = [
    [
        [0x00, 0x00, 0x10, 0xff, 0x00, 0x10, 0x10, 0x10, 0xff, 0xff],
        [0x10, 0x10, 0x18, 0xff, 0x10, 0x18, 0x18, 0x18, 0x10, 0x1c],
        [0x18, 0x18, 0x1c, 0xff, 0x18, 0x1c, 0x1c, 0x1c, 0x18, 0x1e],
        [0x1c, 0x1c, 0x1e, 0xff, 0x1c, 0x1e, 0x1e, 0x1e, 0x1c, 0x1f],
        [0x1e, 0x1e, 0x1f, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    ],
    [
        [0x00, 0x10, 0x00, 0xff, 0x10, 0x10, 0x00, 0x10, 0xff, 0xff],
        [0x10, 0x18, 0x10, 0xff, 0x18, 0x18, 0x10, 0x18, 0x1c, 0x10],
        [0x18, 0x1c, 0x18, 0xff, 0x1c, 0x1c, 0x18, 0x1c, 0x1e, 0x18],
        [0x1c, 0x1e, 0x1c, 0xff, 0x1e, 0x1e, 0x1c, 0x1e, 0x1f, 0x1c],
        [0x1e, 0x1f, 0x1e, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
    ],
];

pub static dav1d_block_sizes: [[[BlockSize; 2]; BlockPartition::COUNT]; BlockLevel::COUNT] = {
    use BlockSize::*;

    const DEFAULT: BlockSize = BlockSize::Bs128x128;

    [
        [
            [Bs128x128, DEFAULT],
            [Bs128x64, DEFAULT],
            [Bs64x128, DEFAULT],
            [DEFAULT; 2],
            [Bs64x64, Bs128x64],
            [Bs128x64, Bs64x64],
            [Bs64x64, Bs64x128],
            [Bs64x128, Bs64x64],
            [DEFAULT; 2],
            [DEFAULT; 2],
        ],
        [
            [Bs64x64, DEFAULT],
            [Bs64x32, DEFAULT],
            [Bs32x64, DEFAULT],
            [DEFAULT; 2],
            [Bs32x32, Bs64x32],
            [Bs64x32, Bs32x32],
            [Bs32x32, Bs32x64],
            [Bs32x64, Bs32x32],
            [Bs64x16, DEFAULT],
            [Bs16x64, DEFAULT],
        ],
        [
            [Bs32x32, DEFAULT],
            [Bs32x16, DEFAULT],
            [Bs16x32, DEFAULT],
            [DEFAULT; 2],
            [Bs16x16, Bs32x16],
            [Bs32x16, Bs16x16],
            [Bs16x16, Bs16x32],
            [Bs16x32, Bs16x16],
            [Bs32x8, DEFAULT],
            [Bs8x32, DEFAULT],
        ],
        [
            [Bs16x16, DEFAULT],
            [Bs16x8, DEFAULT],
            [Bs8x16, DEFAULT],
            [DEFAULT; 2],
            [Bs8x8, Bs16x8],
            [Bs16x8, Bs8x8],
            [Bs8x8, Bs8x16],
            [Bs8x16, Bs8x8],
            [Bs16x4, DEFAULT],
            [Bs4x16, DEFAULT],
        ],
        [
            [Bs8x8, DEFAULT],
            [Bs8x4, DEFAULT],
            [Bs4x8, DEFAULT],
            [Bs4x4, DEFAULT],
            [DEFAULT; 2],
            [DEFAULT; 2],
            [DEFAULT; 2],
            [DEFAULT; 2],
            [DEFAULT; 2],
            [DEFAULT; 2],
        ],
    ]
};

static dav1d_block_dimensions: [[u8; 4]; BlockSize::COUNT] = [
    [32, 32, 5, 5],
    [32, 16, 5, 4],
    [16, 32, 4, 5],
    [16, 16, 4, 4],
    [16, 8, 4, 3],
    [16, 4, 4, 2],
    [8, 16, 3, 4],
    [8, 8, 3, 3],
    [8, 4, 3, 2],
    [8, 2, 3, 1],
    [4, 16, 2, 4],
    [4, 8, 2, 3],
    [4, 4, 2, 2],
    [4, 2, 2, 1],
    [4, 1, 2, 0],
    [2, 8, 1, 3],
    [2, 4, 1, 2],
    [2, 2, 1, 1],
    [2, 1, 1, 0],
    [1, 4, 0, 2],
    [1, 2, 0, 1],
    [1, 1, 0, 0],
];

impl BlockSize {
    #[inline]
    pub fn dimensions(self) -> &'static [u8; 4] {
        &dav1d_block_dimensions[self as usize]
    }
}

pub const fn dav1d_txfm_size<const TX: usize>() -> TxfmSize {
    let Some(size) = TxfmSize::from_repr(TX) else {
        panic!("invalid `TxfmSize` discriminant");
    };
    size
}

pub const fn dav1d_txfm_dimension<const TX: usize>() -> TxfmInfo {
    use TxfmSize::*;
    match dav1d_txfm_size::<TX>() {
        S4x4 => TxfmInfo {
            w: 1,
            h: 1,
            lw: 0,
            lh: 0,
            min: 0,
            max: 0,
            sub: DefaultValue::DEFAULT,
            ctx: 0,
        },
        S8x8 => TxfmInfo {
            w: 2,
            h: 2,
            lw: 1,
            lh: 1,
            min: 1,
            max: 1,
            sub: S4x4,
            ctx: 1,
        },
        S16x16 => TxfmInfo {
            w: 4,
            h: 4,
            lw: 2,
            lh: 2,
            min: 2,
            max: 2,
            sub: S8x8,
            ctx: 2,
        },
        S32x32 => TxfmInfo {
            w: 8,
            h: 8,
            lw: 3,
            lh: 3,
            min: 3,
            max: 3,
            sub: S16x16,
            ctx: 3,
        },
        S64x64 => TxfmInfo {
            w: 16,
            h: 16,
            lw: 4,
            lh: 4,
            min: 4,
            max: 4,
            sub: S32x32,
            ctx: 4,
        },
        R4x8 => TxfmInfo {
            w: 1,
            h: 2,
            lw: 0,
            lh: 1,
            min: 0,
            max: 1,
            sub: S4x4,
            ctx: 1,
        },
        R8x4 => TxfmInfo {
            w: 2,
            h: 1,
            lw: 1,
            lh: 0,
            min: 0,
            max: 1,
            sub: S4x4,
            ctx: 1,
        },
        R8x16 => TxfmInfo {
            w: 2,
            h: 4,
            lw: 1,
            lh: 2,
            min: 1,
            max: 2,
            sub: S8x8,
            ctx: 2,
        },
        R16x8 => TxfmInfo {
            w: 4,
            h: 2,
            lw: 2,
            lh: 1,
            min: 1,
            max: 2,
            sub: S8x8,
            ctx: 2,
        },
        R16x32 => TxfmInfo {
            w: 4,
            h: 8,
            lw: 2,
            lh: 3,
            min: 2,
            max: 3,
            sub: S16x16,
            ctx: 3,
        },
        R32x16 => TxfmInfo {
            w: 8,
            h: 4,
            lw: 3,
            lh: 2,
            min: 2,
            max: 3,
            sub: S16x16,
            ctx: 3,
        },
        R32x64 => TxfmInfo {
            w: 8,
            h: 16,
            lw: 3,
            lh: 4,
            min: 3,
            max: 4,
            sub: S32x32,
            ctx: 4,
        },
        R64x32 => TxfmInfo {
            w: 16,
            h: 8,
            lw: 4,
            lh: 3,
            min: 3,
            max: 4,
            sub: S32x32,
            ctx: 4,
        },
        R4x16 => TxfmInfo {
            w: 1,
            h: 4,
            lw: 0,
            lh: 2,
            min: 0,
            max: 2,
            sub: R4x8,
            ctx: 1,
        },
        R16x4 => TxfmInfo {
            w: 4,
            h: 1,
            lw: 2,
            lh: 0,
            min: 0,
            max: 2,
            sub: R8x4,
            ctx: 1,
        },
        R8x32 => TxfmInfo {
            w: 2,
            h: 8,
            lw: 1,
            lh: 3,
            min: 1,
            max: 3,
            sub: R8x16,
            ctx: 2,
        },
        R32x8 => TxfmInfo {
            w: 8,
            h: 2,
            lw: 3,
            lh: 1,
            min: 1,
            max: 3,
            sub: R16x8,
            ctx: 2,
        },
        R16x64 => TxfmInfo {
            w: 4,
            h: 16,
            lw: 2,
            lh: 4,
            min: 2,
            max: 4,
            sub: R16x32,
            ctx: 3,
        },
        R64x16 => TxfmInfo {
            w: 16,
            h: 4,
            lw: 4,
            lh: 2,
            min: 2,
            max: 4,
            sub: R32x16,
            ctx: 3,
        },
    }
}

pub static dav1d_txfm_dimensions: [TxfmInfo; TxfmSize::COUNT] = {
    use TxfmSize::*;
    [
        dav1d_txfm_dimension::<{ S4x4 as _ }>(),
        dav1d_txfm_dimension::<{ S8x8 as _ }>(),
        dav1d_txfm_dimension::<{ S16x16 as _ }>(),
        dav1d_txfm_dimension::<{ S32x32 as _ }>(),
        dav1d_txfm_dimension::<{ S64x64 as _ }>(),
        dav1d_txfm_dimension::<{ R4x8 as _ }>(),
        dav1d_txfm_dimension::<{ R8x4 as _ }>(),
        dav1d_txfm_dimension::<{ R8x16 as _ }>(),
        dav1d_txfm_dimension::<{ R16x8 as _ }>(),
        dav1d_txfm_dimension::<{ R16x32 as _ }>(),
        dav1d_txfm_dimension::<{ R32x16 as _ }>(),
        dav1d_txfm_dimension::<{ R32x64 as _ }>(),
        dav1d_txfm_dimension::<{ R64x32 as _ }>(),
        dav1d_txfm_dimension::<{ R4x16 as _ }>(),
        dav1d_txfm_dimension::<{ R16x4 as _ }>(),
        dav1d_txfm_dimension::<{ R8x32 as _ }>(),
        dav1d_txfm_dimension::<{ R32x8 as _ }>(),
        dav1d_txfm_dimension::<{ R16x64 as _ }>(),
        dav1d_txfm_dimension::<{ R64x16 as _ }>(),
    ]
};

pub static dav1d_max_txfm_size_for_bs: [[TxfmSize; 4]; BlockSize::COUNT] = {
    use TxfmSize::*;
    const DEFAULT: TxfmSize = DefaultValue::DEFAULT;
    [
        [S64x64, S32x32, S32x32, S32x32],
        [S64x64, S32x32, S32x32, S32x32],
        [S64x64, S32x32, DEFAULT, S32x32],
        [S64x64, S32x32, S32x32, S32x32],
        [R64x32, R32x16, S32x32, S32x32],
        [R64x16, R32x8, R32x16, R32x16],
        [R32x64, R16x32, DEFAULT, S32x32],
        [S32x32, S16x16, R16x32, S32x32],
        [R32x16, R16x8, S16x16, R32x16],
        [R32x8, R16x4, R16x8, R32x8],
        [R16x64, R8x32, DEFAULT, R16x32],
        [R16x32, R8x16, DEFAULT, R16x32],
        [S16x16, S8x8, R8x16, S16x16],
        [R16x8, R8x4, S8x8, R16x8],
        [R16x4, R8x4, R8x4, R16x4],
        [R8x32, R4x16, DEFAULT, R8x32],
        [R8x16, R4x8, DEFAULT, R8x16],
        [S8x8, S4x4, R4x8, S8x8],
        [R8x4, S4x4, S4x4, R8x4],
        [R4x16, R4x8, DEFAULT, R4x16],
        [R4x8, S4x4, DEFAULT, R4x8],
        [S4x4, S4x4, S4x4, S4x4],
    ]
};

pub static dav1d_txtp_from_uvmode: [TxfmType; N_UV_INTRA_PRED_MODES] = {
    let mut tbl = [0; N_UV_INTRA_PRED_MODES];
    tbl[DC_PRED as usize] = DCT_DCT;
    tbl[VERT_PRED as usize] = ADST_DCT;
    tbl[HOR_PRED as usize] = DCT_ADST;
    tbl[DIAG_DOWN_LEFT_PRED as usize] = DCT_DCT;
    tbl[DIAG_DOWN_RIGHT_PRED as usize] = ADST_ADST;
    tbl[VERT_RIGHT_PRED as usize] = ADST_DCT;
    tbl[HOR_DOWN_PRED as usize] = DCT_ADST;
    tbl[HOR_UP_PRED as usize] = DCT_ADST;
    tbl[VERT_LEFT_PRED as usize] = ADST_DCT;
    tbl[SMOOTH_PRED as usize] = ADST_ADST;
    tbl[SMOOTH_V_PRED as usize] = ADST_DCT;
    tbl[SMOOTH_H_PRED as usize] = DCT_ADST;
    tbl[PAETH_PRED as usize] = ADST_ADST;
    tbl
};

pub static dav1d_comp_inter_pred_modes: [[InterPredMode; 2]; N_COMP_INTER_PRED_MODES] = {
    let mut tbl = [[0; 2]; 8];
    tbl[NEARESTMV_NEARESTMV as usize] = [NEARESTMV, NEARESTMV];
    tbl[NEARMV_NEARMV as usize] = [NEARMV, NEARMV];
    tbl[NEWMV_NEWMV as usize] = [NEWMV, NEWMV];
    tbl[GLOBALMV_GLOBALMV as usize] = [GLOBALMV, GLOBALMV];
    tbl[NEWMV_NEARESTMV as usize] = [NEWMV, NEARESTMV];
    tbl[NEWMV_NEARMV as usize] = [NEWMV, NEARMV];
    tbl[NEARESTMV_NEWMV as usize] = [NEARESTMV, NEWMV];
    tbl[NEARMV_NEWMV as usize] = [NEARMV, NEWMV];
    tbl
};

pub static dav1d_partition_type_count: [u8; BlockLevel::COUNT] = [
    BlockPartition::COUNT as u8 - 3,
    BlockPartition::COUNT as u8 - 1,
    BlockPartition::COUNT as u8 - 1,
    BlockPartition::COUNT as u8 - 1,
    BlockPartition::N_SUB8X8_PARTITIONS as u8 - 1,
];

pub static dav1d_tx_types_per_set: [u8; 40] = [
    IDTX as u8,
    DCT_DCT as u8,
    ADST_ADST as u8,
    ADST_DCT as u8,
    DCT_ADST as u8,
    IDTX as u8,
    DCT_DCT as u8,
    V_DCT as u8,
    H_DCT as u8,
    ADST_ADST as u8,
    ADST_DCT as u8,
    DCT_ADST as u8,
    IDTX as u8,
    V_DCT as u8,
    H_DCT as u8,
    DCT_DCT as u8,
    ADST_DCT as u8,
    DCT_ADST as u8,
    FLIPADST_DCT as u8,
    DCT_FLIPADST as u8,
    ADST_ADST as u8,
    FLIPADST_FLIPADST as u8,
    ADST_FLIPADST as u8,
    FLIPADST_ADST as u8,
    IDTX as u8,
    V_DCT as u8,
    H_DCT as u8,
    V_ADST as u8,
    H_ADST as u8,
    V_FLIPADST as u8,
    H_FLIPADST as u8,
    DCT_DCT as u8,
    ADST_DCT as u8,
    DCT_ADST as u8,
    FLIPADST_DCT as u8,
    DCT_FLIPADST as u8,
    ADST_ADST as u8,
    FLIPADST_FLIPADST as u8,
    ADST_FLIPADST as u8,
    FLIPADST_ADST as u8,
];

pub static dav1d_ymode_size_context: [u8; BlockSize::COUNT] = [
    3, 3, 3, 3, 3, 2, 3, 3, 2, 1, 2, 2, 2, 1, 0, 1, 1, 1, 0, 0, 0, 0,
];

pub static dav1d_lo_ctx_offsets: [[[u8; 5]; 5]; 3] = [
    [
        [0, 1, 6, 6, 21],
        [1, 6, 6, 21, 21],
        [6, 6, 21, 21, 21],
        [6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21],
    ],
    [
        [0, 16, 6, 6, 21],
        [16, 16, 6, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21],
        [16, 16, 21, 21, 21],
    ],
    [
        [0, 11, 11, 11, 11],
        [11, 11, 11, 11, 11],
        [6, 6, 21, 21, 21],
        [6, 21, 21, 21, 21],
        [21, 21, 21, 21, 21],
    ],
];

pub static dav1d_skip_ctx: [[u8; 5]; 5] = [
    [1, 2, 2, 2, 3],
    [2, 4, 4, 4, 5],
    [2, 4, 4, 4, 5],
    [2, 4, 4, 4, 5],
    [3, 5, 5, 5, 6],
];

pub static dav1d_tx_type_class: [TxClass; N_TX_TYPES_PLUS_LL] = [
    TxClass::TwoD,
    TxClass::TwoD,
    TxClass::TwoD,
    TxClass::TwoD,
    TxClass::TwoD,
    TxClass::TwoD,
    TxClass::TwoD,
    TxClass::TwoD,
    TxClass::TwoD,
    TxClass::TwoD,
    TxClass::V,
    TxClass::H,
    TxClass::V,
    TxClass::H,
    TxClass::V,
    TxClass::H,
    TxClass::TwoD,
];

pub const dav1d_filter_2d: [[Filter2d; Rav1dFilterMode::N_FILTERS]; Rav1dFilterMode::N_FILTERS] = {
    use Filter2d::*;

    const DEFAULT: Filter2d = Filter2d::Regular8Tap;

    [
        [Regular8Tap, RegularSmooth8Tap, RegularSharp8Tap, DEFAULT],
        [SmoothRegular8Tap, Smooth8Tap, SmoothSharp8Tap, DEFAULT],
        [SharpRegular8Tap, SharpSmooth8Tap, Sharp8Tap, DEFAULT],
        [DEFAULT, DEFAULT, DEFAULT, Bilinear],
    ]
};

pub const dav1d_filter_dir: [[Rav1dFilterMode; 2]; Filter2d::COUNT] = [
    [Rav1dFilterMode::Regular8Tap, Rav1dFilterMode::Regular8Tap],
    [Rav1dFilterMode::Smooth8Tap, Rav1dFilterMode::Regular8Tap],
    [Rav1dFilterMode::Sharp8Tap, Rav1dFilterMode::Regular8Tap],
    [Rav1dFilterMode::Regular8Tap, Rav1dFilterMode::Sharp8Tap],
    [Rav1dFilterMode::Smooth8Tap, Rav1dFilterMode::Sharp8Tap],
    [Rav1dFilterMode::Sharp8Tap, Rav1dFilterMode::Sharp8Tap],
    [Rav1dFilterMode::Regular8Tap, Rav1dFilterMode::Smooth8Tap],
    [Rav1dFilterMode::Smooth8Tap, Rav1dFilterMode::Smooth8Tap],
    [Rav1dFilterMode::Sharp8Tap, Rav1dFilterMode::Smooth8Tap],
    [Rav1dFilterMode::Bilinear, Rav1dFilterMode::Bilinear],
];

pub static dav1d_filter_mode_to_y_mode: [u8; 5] = [
    DC_PRED as u8,
    VERT_PRED as u8,
    HOR_PRED as u8,
    HOR_DOWN_PRED as u8,
    DC_PRED as u8,
];

pub static dav1d_intra_mode_context: [u8; N_INTRA_PRED_MODES] =
    [0, 1, 2, 3, 4, 4, 4, 4, 3, 0, 1, 2, 0];

pub static dav1d_wedge_ctx_lut: [u8; BlockSize::COUNT] = [
    0, 0, 0, 0, 0, 0, 0, 6, 5, 8, 0, 4, 3, 2, 0, 7, 1, 0, 0, 0, 0, 0,
];

pub const cfl_allowed_mask: c_uint = {
    use BlockSize::*;

    1 << Bs32x32 as u8
        | 1 << Bs32x16 as u8
        | 1 << Bs32x8 as u8
        | 1 << Bs16x32 as u8
        | 1 << Bs16x16 as u8
        | 1 << Bs16x8 as u8
        | 1 << Bs16x4 as u8
        | 1 << Bs8x32 as u8
        | 1 << Bs8x16 as u8
        | 1 << Bs8x8 as u8
        | 1 << Bs8x4 as u8
        | 1 << Bs4x16 as u8
        | 1 << Bs4x8 as u8
        | 1 << Bs4x4 as u8
};

pub const wedge_allowed_mask: c_uint = {
    use BlockSize::*;

    1 << Bs32x32 as u8
        | 1 << Bs32x16 as u8
        | 1 << Bs32x8 as u8
        | 1 << Bs16x32 as u8
        | 1 << Bs16x16 as u8
        | 1 << Bs16x8 as u8
        | 1 << Bs8x32 as u8
        | 1 << Bs8x16 as u8
        | 1 << Bs8x8 as u8
};

pub const interintra_allowed_mask: c_uint = {
    use BlockSize::*;

    1 << Bs32x32 as u8
        | 1 << Bs32x16 as u8
        | 1 << Bs16x32 as u8
        | 1 << Bs16x16 as u8
        | 1 << Bs16x8 as u8
        | 1 << Bs8x16 as u8
        | 1 << Bs8x8 as u8
};

impl Default for Rav1dWarpedMotionParams {
    fn default() -> Self {
        Self {
            r#type: Rav1dWarpedMotionType::Identity,
            matrix: [0, 0, 1 << 16, 0, 0, 1 << 16],
            abcd: Default::default(),
        }
    }
}

pub static dav1d_cdef_directions: [[i8; 2]; 12] = [
    [1 * 12 + 0, 2 * 12 + 0],
    [1 * 12 + 0, 2 * 12 - 1],
    [-1 * 12 + 1, -2 * 12 + 2],
    [0 * 12 + 1, -1 * 12 + 2],
    [0 * 12 + 1, 0 * 12 + 2],
    [0 * 12 + 1, 1 * 12 + 2],
    [1 * 12 + 1, 2 * 12 + 2],
    [1 * 12 + 0, 2 * 12 + 1],
    [1 * 12 + 0, 2 * 12 + 0],
    [1 * 12 + 0, 2 * 12 - 1],
    [-1 * 12 + 1, -2 * 12 + 2],
    [0 * 12 + 1, -1 * 12 + 2],
];

pub static dav1d_sgr_params: Align4<[[u16; 2]; 16]> = Align4([
    [140, 3236],
    [112, 2158],
    [93, 1618],
    [80, 1438],
    [70, 1295],
    [58, 1177],
    [47, 1079],
    [37, 996],
    [30, 925],
    [25, 863],
    [0, 2589],
    [0, 1618],
    [0, 1177],
    [0, 925],
    [56, 0],
    [22, 0],
]);

#[no_mangle]
pub static dav1d_sgr_x_by_x: Align64<[u8; 256]> = Align64([
    255, 128, 85, 64, 51, 43, 37, 32, 28, 26, 23, 21, 20, 18, 17, 16, 15, 14, 13, 13, 12, 12, 11,
    11, 10, 10, 9, 9, 9, 9, 8, 8, 8, 8, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
]);

#[no_mangle]
pub static dav1d_mc_subpel_filters: Align8<[[[i8; 8]; 15]; 6]> = Align8([
    [
        [0, 1, -3, 63, 4, -1, 0, 0],
        [0, 1, -5, 61, 9, -2, 0, 0],
        [0, 1, -6, 58, 14, -4, 1, 0],
        [0, 1, -7, 55, 19, -5, 1, 0],
        [0, 1, -7, 51, 24, -6, 1, 0],
        [0, 1, -8, 47, 29, -6, 1, 0],
        [0, 1, -7, 42, 33, -6, 1, 0],
        [0, 1, -7, 38, 38, -7, 1, 0],
        [0, 1, -6, 33, 42, -7, 1, 0],
        [0, 1, -6, 29, 47, -8, 1, 0],
        [0, 1, -6, 24, 51, -7, 1, 0],
        [0, 1, -5, 19, 55, -7, 1, 0],
        [0, 1, -4, 14, 58, -6, 1, 0],
        [0, 0, -2, 9, 61, -5, 1, 0],
        [0, 0, -1, 4, 63, -3, 1, 0],
    ],
    [
        [0, 1, 14, 31, 17, 1, 0, 0],
        [0, 0, 13, 31, 18, 2, 0, 0],
        [0, 0, 11, 31, 20, 2, 0, 0],
        [0, 0, 10, 30, 21, 3, 0, 0],
        [0, 0, 9, 29, 22, 4, 0, 0],
        [0, 0, 8, 28, 23, 5, 0, 0],
        [0, -1, 8, 27, 24, 6, 0, 0],
        [0, -1, 7, 26, 26, 7, -1, 0],
        [0, 0, 6, 24, 27, 8, -1, 0],
        [0, 0, 5, 23, 28, 8, 0, 0],
        [0, 0, 4, 22, 29, 9, 0, 0],
        [0, 0, 3, 21, 30, 10, 0, 0],
        [0, 0, 2, 20, 31, 11, 0, 0],
        [0, 0, 2, 18, 31, 13, 0, 0],
        [0, 0, 1, 17, 31, 14, 1, 0],
    ],
    [
        [-1, 1, -3, 63, 4, -1, 1, 0],
        [-1, 3, -6, 62, 8, -3, 2, -1],
        [-1, 4, -9, 60, 13, -5, 3, -1],
        [-2, 5, -11, 58, 19, -7, 3, -1],
        [-2, 5, -11, 54, 24, -9, 4, -1],
        [-2, 5, -12, 50, 30, -10, 4, -1],
        [-2, 5, -12, 45, 35, -11, 5, -1],
        [-2, 6, -12, 40, 40, -12, 6, -2],
        [-1, 5, -11, 35, 45, -12, 5, -2],
        [-1, 4, -10, 30, 50, -12, 5, -2],
        [-1, 4, -9, 24, 54, -11, 5, -2],
        [-1, 3, -7, 19, 58, -11, 5, -2],
        [-1, 3, -5, 13, 60, -9, 4, -1],
        [-1, 2, -3, 8, 62, -6, 3, -1],
        [0, 1, -1, 4, 63, -3, 1, -1],
    ],
    [
        [0, 0, -2, 63, 4, -1, 0, 0],
        [0, 0, -4, 61, 9, -2, 0, 0],
        [0, 0, -5, 58, 14, -3, 0, 0],
        [0, 0, -6, 55, 19, -4, 0, 0],
        [0, 0, -6, 51, 24, -5, 0, 0],
        [0, 0, -7, 47, 29, -5, 0, 0],
        [0, 0, -6, 42, 33, -5, 0, 0],
        [0, 0, -6, 38, 38, -6, 0, 0],
        [0, 0, -5, 33, 42, -6, 0, 0],
        [0, 0, -5, 29, 47, -7, 0, 0],
        [0, 0, -5, 24, 51, -6, 0, 0],
        [0, 0, -4, 19, 55, -6, 0, 0],
        [0, 0, -3, 14, 58, -5, 0, 0],
        [0, 0, -2, 9, 61, -4, 0, 0],
        [0, 0, -1, 4, 63, -2, 0, 0],
    ],
    [
        [0, 0, 15, 31, 17, 1, 0, 0],
        [0, 0, 13, 31, 18, 2, 0, 0],
        [0, 0, 11, 31, 20, 2, 0, 0],
        [0, 0, 10, 30, 21, 3, 0, 0],
        [0, 0, 9, 29, 22, 4, 0, 0],
        [0, 0, 8, 28, 23, 5, 0, 0],
        [0, 0, 7, 27, 24, 6, 0, 0],
        [0, 0, 6, 26, 26, 6, 0, 0],
        [0, 0, 6, 24, 27, 7, 0, 0],
        [0, 0, 5, 23, 28, 8, 0, 0],
        [0, 0, 4, 22, 29, 9, 0, 0],
        [0, 0, 3, 21, 30, 10, 0, 0],
        [0, 0, 2, 20, 31, 11, 0, 0],
        [0, 0, 2, 18, 31, 13, 0, 0],
        [0, 0, 1, 17, 31, 15, 0, 0],
    ],
    [
        [0, 0, 0, 60, 4, 0, 0, 0],
        [0, 0, 0, 56, 8, 0, 0, 0],
        [0, 0, 0, 52, 12, 0, 0, 0],
        [0, 0, 0, 48, 16, 0, 0, 0],
        [0, 0, 0, 44, 20, 0, 0, 0],
        [0, 0, 0, 40, 24, 0, 0, 0],
        [0, 0, 0, 36, 28, 0, 0, 0],
        [0, 0, 0, 32, 32, 0, 0, 0],
        [0, 0, 0, 28, 36, 0, 0, 0],
        [0, 0, 0, 24, 40, 0, 0, 0],
        [0, 0, 0, 20, 44, 0, 0, 0],
        [0, 0, 0, 16, 48, 0, 0, 0],
        [0, 0, 0, 12, 52, 0, 0, 0],
        [0, 0, 0, 8, 56, 0, 0, 0],
        [0, 0, 0, 4, 60, 0, 0, 0],
    ],
]);

#[no_mangle]
pub static dav1d_mc_warp_filter: Align8<[[i8; 8]; 193]> = Align8([
    [0, 0, 127, 1, 0, 0, 0, 0],
    [0, -1, 127, 2, 0, 0, 0, 0],
    [1, -3, 127, 4, -1, 0, 0, 0],
    [1, -4, 126, 6, -2, 1, 0, 0],
    [1, -5, 126, 8, -3, 1, 0, 0],
    [1, -6, 125, 11, -4, 1, 0, 0],
    [1, -7, 124, 13, -4, 1, 0, 0],
    [2, -8, 123, 15, -5, 1, 0, 0],
    [2, -9, 122, 18, -6, 1, 0, 0],
    [2, -10, 121, 20, -6, 1, 0, 0],
    [2, -11, 120, 22, -7, 2, 0, 0],
    [2, -12, 119, 25, -8, 2, 0, 0],
    [3, -13, 117, 27, -8, 2, 0, 0],
    [3, -13, 116, 29, -9, 2, 0, 0],
    [3, -14, 114, 32, -10, 3, 0, 0],
    [3, -15, 113, 35, -10, 2, 0, 0],
    [3, -15, 111, 37, -11, 3, 0, 0],
    [3, -16, 109, 40, -11, 3, 0, 0],
    [3, -16, 108, 42, -12, 3, 0, 0],
    [4, -17, 106, 45, -13, 3, 0, 0],
    [4, -17, 104, 47, -13, 3, 0, 0],
    [4, -17, 102, 50, -14, 3, 0, 0],
    [4, -17, 100, 52, -14, 3, 0, 0],
    [4, -18, 98, 55, -15, 4, 0, 0],
    [4, -18, 96, 58, -15, 3, 0, 0],
    [4, -18, 94, 60, -16, 4, 0, 0],
    [4, -18, 91, 63, -16, 4, 0, 0],
    [4, -18, 89, 65, -16, 4, 0, 0],
    [4, -18, 87, 68, -17, 4, 0, 0],
    [4, -18, 85, 70, -17, 4, 0, 0],
    [4, -18, 82, 73, -17, 4, 0, 0],
    [4, -18, 80, 75, -17, 4, 0, 0],
    [4, -18, 78, 78, -18, 4, 0, 0],
    [4, -17, 75, 80, -18, 4, 0, 0],
    [4, -17, 73, 82, -18, 4, 0, 0],
    [4, -17, 70, 85, -18, 4, 0, 0],
    [4, -17, 68, 87, -18, 4, 0, 0],
    [4, -16, 65, 89, -18, 4, 0, 0],
    [4, -16, 63, 91, -18, 4, 0, 0],
    [4, -16, 60, 94, -18, 4, 0, 0],
    [3, -15, 58, 96, -18, 4, 0, 0],
    [4, -15, 55, 98, -18, 4, 0, 0],
    [3, -14, 52, 100, -17, 4, 0, 0],
    [3, -14, 50, 102, -17, 4, 0, 0],
    [3, -13, 47, 104, -17, 4, 0, 0],
    [3, -13, 45, 106, -17, 4, 0, 0],
    [3, -12, 42, 108, -16, 3, 0, 0],
    [3, -11, 40, 109, -16, 3, 0, 0],
    [3, -11, 37, 111, -15, 3, 0, 0],
    [2, -10, 35, 113, -15, 3, 0, 0],
    [3, -10, 32, 114, -14, 3, 0, 0],
    [2, -9, 29, 116, -13, 3, 0, 0],
    [2, -8, 27, 117, -13, 3, 0, 0],
    [2, -8, 25, 119, -12, 2, 0, 0],
    [2, -7, 22, 120, -11, 2, 0, 0],
    [1, -6, 20, 121, -10, 2, 0, 0],
    [1, -6, 18, 122, -9, 2, 0, 0],
    [1, -5, 15, 123, -8, 2, 0, 0],
    [1, -4, 13, 124, -7, 1, 0, 0],
    [1, -4, 11, 125, -6, 1, 0, 0],
    [1, -3, 8, 126, -5, 1, 0, 0],
    [1, -2, 6, 126, -4, 1, 0, 0],
    [0, -1, 4, 127, -3, 1, 0, 0],
    [0, 0, 2, 127, -1, 0, 0, 0],
    [0, 0, 0, 127, 1, 0, 0, 0],
    [0, 0, -1, 127, 2, 0, 0, 0],
    [0, 1, -3, 127, 4, -2, 1, 0],
    [0, 1, -5, 127, 6, -2, 1, 0],
    [0, 2, -6, 126, 8, -3, 1, 0],
    [-1, 2, -7, 126, 11, -4, 2, -1],
    [-1, 3, -8, 125, 13, -5, 2, -1],
    [-1, 3, -10, 124, 16, -6, 3, -1],
    [-1, 4, -11, 123, 18, -7, 3, -1],
    [-1, 4, -12, 122, 20, -7, 3, -1],
    [-1, 4, -13, 121, 23, -8, 3, -1],
    [-2, 5, -14, 120, 25, -9, 4, -1],
    [-1, 5, -15, 119, 27, -10, 4, -1],
    [-1, 5, -16, 118, 30, -11, 4, -1],
    [-2, 6, -17, 116, 33, -12, 5, -1],
    [-2, 6, -17, 114, 35, -12, 5, -1],
    [-2, 6, -18, 113, 38, -13, 5, -1],
    [-2, 7, -19, 111, 41, -14, 6, -2],
    [-2, 7, -19, 110, 43, -15, 6, -2],
    [-2, 7, -20, 108, 46, -15, 6, -2],
    [-2, 7, -20, 106, 49, -16, 6, -2],
    [-2, 7, -21, 104, 51, -16, 7, -2],
    [-2, 7, -21, 102, 54, -17, 7, -2],
    [-2, 8, -21, 100, 56, -18, 7, -2],
    [-2, 8, -22, 98, 59, -18, 7, -2],
    [-2, 8, -22, 96, 62, -19, 7, -2],
    [-2, 8, -22, 94, 64, -19, 7, -2],
    [-2, 8, -22, 91, 67, -20, 8, -2],
    [-2, 8, -22, 89, 69, -20, 8, -2],
    [-2, 8, -22, 87, 72, -21, 8, -2],
    [-2, 8, -21, 84, 74, -21, 8, -2],
    [-2, 8, -22, 82, 77, -21, 8, -2],
    [-2, 8, -21, 79, 79, -21, 8, -2],
    [-2, 8, -21, 77, 82, -22, 8, -2],
    [-2, 8, -21, 74, 84, -21, 8, -2],
    [-2, 8, -21, 72, 87, -22, 8, -2],
    [-2, 8, -20, 69, 89, -22, 8, -2],
    [-2, 8, -20, 67, 91, -22, 8, -2],
    [-2, 7, -19, 64, 94, -22, 8, -2],
    [-2, 7, -19, 62, 96, -22, 8, -2],
    [-2, 7, -18, 59, 98, -22, 8, -2],
    [-2, 7, -18, 56, 100, -21, 8, -2],
    [-2, 7, -17, 54, 102, -21, 7, -2],
    [-2, 7, -16, 51, 104, -21, 7, -2],
    [-2, 6, -16, 49, 106, -20, 7, -2],
    [-2, 6, -15, 46, 108, -20, 7, -2],
    [-2, 6, -15, 43, 110, -19, 7, -2],
    [-2, 6, -14, 41, 111, -19, 7, -2],
    [-1, 5, -13, 38, 113, -18, 6, -2],
    [-1, 5, -12, 35, 114, -17, 6, -2],
    [-1, 5, -12, 33, 116, -17, 6, -2],
    [-1, 4, -11, 30, 118, -16, 5, -1],
    [-1, 4, -10, 27, 119, -15, 5, -1],
    [-1, 4, -9, 25, 120, -14, 5, -2],
    [-1, 3, -8, 23, 121, -13, 4, -1],
    [-1, 3, -7, 20, 122, -12, 4, -1],
    [-1, 3, -7, 18, 123, -11, 4, -1],
    [-1, 3, -6, 16, 124, -10, 3, -1],
    [-1, 2, -5, 13, 125, -8, 3, -1],
    [-1, 2, -4, 11, 126, -7, 2, -1],
    [0, 1, -3, 8, 126, -6, 2, 0],
    [0, 1, -2, 6, 127, -5, 1, 0],
    [0, 1, -2, 4, 127, -3, 1, 0],
    [0, 0, 0, 2, 127, -1, 0, 0],
    [0, 0, 0, 1, 127, 0, 0, 0],
    [0, 0, 0, -1, 127, 2, 0, 0],
    [0, 0, 1, -3, 127, 4, -1, 0],
    [0, 0, 1, -4, 126, 6, -2, 1],
    [0, 0, 1, -5, 126, 8, -3, 1],
    [0, 0, 1, -6, 125, 11, -4, 1],
    [0, 0, 1, -7, 124, 13, -4, 1],
    [0, 0, 2, -8, 123, 15, -5, 1],
    [0, 0, 2, -9, 122, 18, -6, 1],
    [0, 0, 2, -10, 121, 20, -6, 1],
    [0, 0, 2, -11, 120, 22, -7, 2],
    [0, 0, 2, -12, 119, 25, -8, 2],
    [0, 0, 3, -13, 117, 27, -8, 2],
    [0, 0, 3, -13, 116, 29, -9, 2],
    [0, 0, 3, -14, 114, 32, -10, 3],
    [0, 0, 3, -15, 113, 35, -10, 2],
    [0, 0, 3, -15, 111, 37, -11, 3],
    [0, 0, 3, -16, 109, 40, -11, 3],
    [0, 0, 3, -16, 108, 42, -12, 3],
    [0, 0, 4, -17, 106, 45, -13, 3],
    [0, 0, 4, -17, 104, 47, -13, 3],
    [0, 0, 4, -17, 102, 50, -14, 3],
    [0, 0, 4, -17, 100, 52, -14, 3],
    [0, 0, 4, -18, 98, 55, -15, 4],
    [0, 0, 4, -18, 96, 58, -15, 3],
    [0, 0, 4, -18, 94, 60, -16, 4],
    [0, 0, 4, -18, 91, 63, -16, 4],
    [0, 0, 4, -18, 89, 65, -16, 4],
    [0, 0, 4, -18, 87, 68, -17, 4],
    [0, 0, 4, -18, 85, 70, -17, 4],
    [0, 0, 4, -18, 82, 73, -17, 4],
    [0, 0, 4, -18, 80, 75, -17, 4],
    [0, 0, 4, -18, 78, 78, -18, 4],
    [0, 0, 4, -17, 75, 80, -18, 4],
    [0, 0, 4, -17, 73, 82, -18, 4],
    [0, 0, 4, -17, 70, 85, -18, 4],
    [0, 0, 4, -17, 68, 87, -18, 4],
    [0, 0, 4, -16, 65, 89, -18, 4],
    [0, 0, 4, -16, 63, 91, -18, 4],
    [0, 0, 4, -16, 60, 94, -18, 4],
    [0, 0, 3, -15, 58, 96, -18, 4],
    [0, 0, 4, -15, 55, 98, -18, 4],
    [0, 0, 3, -14, 52, 100, -17, 4],
    [0, 0, 3, -14, 50, 102, -17, 4],
    [0, 0, 3, -13, 47, 104, -17, 4],
    [0, 0, 3, -13, 45, 106, -17, 4],
    [0, 0, 3, -12, 42, 108, -16, 3],
    [0, 0, 3, -11, 40, 109, -16, 3],
    [0, 0, 3, -11, 37, 111, -15, 3],
    [0, 0, 2, -10, 35, 113, -15, 3],
    [0, 0, 3, -10, 32, 114, -14, 3],
    [0, 0, 2, -9, 29, 116, -13, 3],
    [0, 0, 2, -8, 27, 117, -13, 3],
    [0, 0, 2, -8, 25, 119, -12, 2],
    [0, 0, 2, -7, 22, 120, -11, 2],
    [0, 0, 1, -6, 20, 121, -10, 2],
    [0, 0, 1, -6, 18, 122, -9, 2],
    [0, 0, 1, -5, 15, 123, -8, 2],
    [0, 0, 1, -4, 13, 124, -7, 1],
    [0, 0, 1, -4, 11, 125, -6, 1],
    [0, 0, 1, -3, 8, 126, -5, 1],
    [0, 0, 1, -2, 6, 126, -4, 1],
    [0, 0, 0, -1, 4, 127, -3, 1],
    [0, 0, 0, 0, 2, 127, -1, 0],
    [0, 0, 0, 0, 2, 127, -1, 0],
]);

#[no_mangle]
pub static dav1d_resize_filter: Align8<[[i8; 8]; 64]> = Align8([
    [0, 0, 0, -128, 0, 0, 0, 0],
    [0, 0, 1, -128, -2, 1, 0, 0],
    [0, -1, 3, -127, -4, 2, -1, 0],
    [0, -1, 4, -127, -6, 3, -1, 0],
    [0, -2, 6, -126, -8, 3, -1, 0],
    [0, -2, 7, -125, -11, 4, -1, 0],
    [1, -2, 8, -125, -13, 5, -2, 0],
    [1, -3, 9, -124, -15, 6, -2, 0],
    [1, -3, 10, -123, -18, 6, -2, 1],
    [1, -3, 11, -122, -20, 7, -3, 1],
    [1, -4, 12, -121, -22, 8, -3, 1],
    [1, -4, 13, -120, -25, 9, -3, 1],
    [1, -4, 14, -118, -28, 9, -3, 1],
    [1, -4, 15, -117, -30, 10, -4, 1],
    [1, -5, 16, -116, -32, 11, -4, 1],
    [1, -5, 16, -114, -35, 12, -4, 1],
    [1, -5, 17, -112, -38, 12, -4, 1],
    [1, -5, 18, -111, -40, 13, -5, 1],
    [1, -5, 18, -109, -43, 14, -5, 1],
    [1, -6, 19, -107, -45, 14, -5, 1],
    [1, -6, 19, -105, -48, 15, -5, 1],
    [1, -6, 19, -103, -51, 16, -5, 1],
    [1, -6, 20, -101, -53, 16, -6, 1],
    [1, -6, 20, -99, -56, 17, -6, 1],
    [1, -6, 20, -97, -58, 17, -6, 1],
    [1, -6, 20, -95, -61, 18, -6, 1],
    [2, -7, 20, -93, -64, 18, -6, 2],
    [2, -7, 20, -91, -66, 19, -6, 1],
    [2, -7, 20, -88, -69, 19, -6, 1],
    [2, -7, 20, -86, -71, 19, -6, 1],
    [2, -7, 20, -84, -74, 20, -7, 2],
    [2, -7, 20, -81, -76, 20, -7, 1],
    [2, -7, 20, -79, -79, 20, -7, 2],
    [1, -7, 20, -76, -81, 20, -7, 2],
    [2, -7, 20, -74, -84, 20, -7, 2],
    [1, -6, 19, -71, -86, 20, -7, 2],
    [1, -6, 19, -69, -88, 20, -7, 2],
    [1, -6, 19, -66, -91, 20, -7, 2],
    [2, -6, 18, -64, -93, 20, -7, 2],
    [1, -6, 18, -61, -95, 20, -6, 1],
    [1, -6, 17, -58, -97, 20, -6, 1],
    [1, -6, 17, -56, -99, 20, -6, 1],
    [1, -6, 16, -53, -101, 20, -6, 1],
    [1, -5, 16, -51, -103, 19, -6, 1],
    [1, -5, 15, -48, -105, 19, -6, 1],
    [1, -5, 14, -45, -107, 19, -6, 1],
    [1, -5, 14, -43, -109, 18, -5, 1],
    [1, -5, 13, -40, -111, 18, -5, 1],
    [1, -4, 12, -38, -112, 17, -5, 1],
    [1, -4, 12, -35, -114, 16, -5, 1],
    [1, -4, 11, -32, -116, 16, -5, 1],
    [1, -4, 10, -30, -117, 15, -4, 1],
    [1, -3, 9, -28, -118, 14, -4, 1],
    [1, -3, 9, -25, -120, 13, -4, 1],
    [1, -3, 8, -22, -121, 12, -4, 1],
    [1, -3, 7, -20, -122, 11, -3, 1],
    [1, -2, 6, -18, -123, 10, -3, 1],
    [0, -2, 6, -15, -124, 9, -3, 1],
    [0, -2, 5, -13, -125, 8, -2, 1],
    [0, -1, 4, -11, -125, 7, -2, 0],
    [0, -1, 3, -8, -126, 6, -2, 0],
    [0, -1, 3, -6, -127, 4, -1, 0],
    [0, -1, 2, -4, -127, 3, -1, 0],
    [0, 0, 1, -2, -128, 1, 0, 0],
]);

#[no_mangle]
pub static dav1d_sm_weights: Align16<[u8; 128]> = Align16([
    0, 0, 255, 128, 255, 149, 85, 64, 255, 197, 146, 105, 73, 50, 37, 32, 255, 225, 196, 170, 145,
    123, 102, 84, 68, 54, 43, 33, 26, 20, 17, 16, 255, 240, 225, 210, 196, 182, 169, 157, 145, 133,
    122, 111, 101, 92, 83, 74, 66, 59, 52, 45, 39, 34, 29, 25, 21, 17, 14, 12, 10, 9, 8, 8, 255,
    248, 240, 233, 225, 218, 210, 203, 196, 189, 182, 176, 169, 163, 156, 150, 144, 138, 133, 127,
    121, 116, 111, 106, 101, 96, 91, 86, 82, 77, 73, 69, 65, 61, 57, 54, 50, 47, 44, 41, 38, 35,
    32, 29, 27, 25, 22, 20, 18, 16, 15, 13, 12, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 4,
]);

#[no_mangle]
pub static dav1d_dr_intra_derivative: [u16; 44] = [
    0, 1023, 0, 547, 372, 0, 0, 273, 215, 0, 178, 151, 0, 132, 116, 0, 102, 0, 90, 80, 0, 71, 64,
    0, 57, 51, 0, 45, 0, 40, 35, 0, 31, 27, 0, 23, 19, 0, 15, 0, 11, 0, 7, 3,
];

pub const FLT_INCR: usize = if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
    2
} else {
    1
};

const FILTER_INDICES: [usize; 7] = if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
    [0, 1, 16, 17, 32, 33, 48]
} else {
    [0, 8, 16, 24, 32, 40, 48]
};

pub fn filter_fn(flt_ptr: &[i8], p: [i32; 7]) -> i32 {
    let flt_ptr = &flt_ptr[..48 + 1];
    let mut sum = 0;
    for i in 0..7 {
        sum += flt_ptr[FILTER_INDICES[i]] as i32 * p[i] as i32;
    }
    sum
}

const fn gen_filter(mut a: [i8; 64], idx: usize, f: [i8; 7]) -> [i8; 64] {
    let mut i = 0;
    while i < 7 {
        a[FLT_INCR * idx + FILTER_INDICES[i]] = f[i];
        i += 1;
    }
    a
}

const fn gen_filters(f: [[i8; 7]; 8]) -> Align64<[i8; 64]> {
    let mut a = [0; 64];

    let mut i = 0;
    while i < 8 {
        a = gen_filter(a, i, f[i]);
        i += 1;
    }

    Align64(a)
}

#[no_mangle]
pub static dav1d_filter_intra_taps: [Align64<[i8; 64]>; 5] = [
    gen_filters([
        [-6, 10, 0, 0, 0, 12, 0],
        [-5, 2, 10, 0, 0, 9, 0],
        [-3, 1, 1, 10, 0, 7, 0],
        [-3, 1, 1, 2, 10, 5, 0],
        [-4, 6, 0, 0, 0, 2, 12],
        [-3, 2, 6, 0, 0, 2, 9],
        [-3, 2, 2, 6, 0, 2, 7],
        [-3, 1, 2, 2, 6, 3, 5],
    ]),
    gen_filters([
        [-10, 16, 0, 0, 0, 10, 0],
        [-6, 0, 16, 0, 0, 6, 0],
        [-4, 0, 0, 16, 0, 4, 0],
        [-2, 0, 0, 0, 16, 2, 0],
        [-10, 16, 0, 0, 0, 0, 10],
        [-6, 0, 16, 0, 0, 0, 6],
        [-4, 0, 0, 16, 0, 0, 4],
        [-2, 0, 0, 0, 16, 0, 2],
    ]),
    gen_filters([
        [-8, 8, 0, 0, 0, 16, 0],
        [-8, 0, 8, 0, 0, 16, 0],
        [-8, 0, 0, 8, 0, 16, 0],
        [-8, 0, 0, 0, 8, 16, 0],
        [-4, 4, 0, 0, 0, 0, 16],
        [-4, 0, 4, 0, 0, 0, 16],
        [-4, 0, 0, 4, 0, 0, 16],
        [-4, 0, 0, 0, 4, 0, 16],
    ]),
    gen_filters([
        [-2, 8, 0, 0, 0, 10, 0],
        [-1, 3, 8, 0, 0, 6, 0],
        [-1, 2, 3, 8, 0, 4, 0],
        [0, 1, 2, 3, 8, 2, 0],
        [-1, 4, 0, 0, 0, 3, 10],
        [-1, 3, 4, 0, 0, 4, 6],
        [-1, 2, 3, 4, 0, 4, 4],
        [-1, 2, 2, 3, 4, 3, 3],
    ]),
    gen_filters([
        [-12, 14, 0, 0, 0, 14, 0],
        [-10, 0, 14, 0, 0, 12, 0],
        [-9, 0, 0, 14, 0, 11, 0],
        [-8, 0, 0, 0, 14, 10, 0],
        [-10, 12, 0, 0, 0, 0, 14],
        [-9, 1, 12, 0, 0, 0, 12],
        [-8, 0, 0, 12, 0, 1, 11],
        [-7, 0, 0, 1, 12, 1, 9],
    ]),
];

#[no_mangle]
pub static dav1d_obmc_masks: Align16<[u8; 64]> = Align16([
    0, 0, 19, 0, 25, 14, 5, 0, 28, 22, 16, 11, 7, 3, 0, 0, 30, 27, 24, 21, 18, 15, 12, 10, 8, 6, 4,
    3, 0, 0, 0, 0, 31, 29, 28, 26, 24, 23, 21, 20, 19, 17, 16, 14, 13, 12, 11, 9, 8, 7, 6, 5, 4, 4,
    3, 2, 0, 0, 0, 0, 0, 0, 0, 0,
]);

#[no_mangle]
pub static dav1d_gaussian_sequence: [i16; 2048] = [
    56, 568, -180, 172, 124, -84, 172, -64, -900, 24, 820, 224, 1248, 996, 272, -8, -916, -388,
    -732, -104, -188, 800, 112, -652, -320, -376, 140, -252, 492, -168, 44, -788, 588, -584, 500,
    -228, 12, 680, 272, -476, 972, -100, 652, 368, 432, -196, -720, -192, 1000, -332, 652, -136,
    -552, -604, -4, 192, -220, -136, 1000, -52, 372, -96, -624, 124, -24, 396, 540, -12, -104, 640,
    464, 244, -208, -84, 368, -528, -740, 248, -968, -848, 608, 376, -60, -292, -40, -156, 252,
    -292, 248, 224, -280, 400, -244, 244, -60, 76, -80, 212, 532, 340, 128, -36, 824, -352, -60,
    -264, -96, -612, 416, -704, 220, -204, 640, -160, 1220, -408, 900, 336, 20, -336, -96, -792,
    304, 48, -28, -1232, -1172, -448, 104, -292, -520, 244, 60, -948, 0, -708, 268, 108, 356, -548,
    488, -344, -136, 488, -196, -224, 656, -236, -1128, 60, 4, 140, 276, -676, -376, 168, -108,
    464, 8, 564, 64, 240, 308, -300, -400, -456, -136, 56, 120, -408, -116, 436, 504, -232, 328,
    844, -164, -84, 784, -168, 232, -224, 348, -376, 128, 568, 96, -1244, -288, 276, 848, 832,
    -360, 656, 464, -384, -332, -356, 728, -388, 160, -192, 468, 296, 224, 140, -776, -100, 280, 4,
    196, 44, -36, -648, 932, 16, 1428, 28, 528, 808, 772, 20, 268, 88, -332, -284, 124, -384, -448,
    208, -228, -1044, -328, 660, 380, -148, -300, 588, 240, 540, 28, 136, -88, -436, 256, 296,
    -1000, 1400, 0, -48, 1056, -136, 264, -528, -1108, 632, -484, -592, -344, 796, 124, -668, -768,
    388, 1296, -232, -188, -200, -288, -4, 308, 100, -168, 256, -500, 204, -508, 648, -136, 372,
    -272, -120, -1004, -552, -548, -384, 548, -296, 428, -108, -8, -912, -324, -224, -88, -112,
    -220, -100, 996, -796, 548, 360, -216, 180, 428, -200, -212, 148, 96, 148, 284, 216, -412,
    -320, 120, -300, -384, -604, -572, -332, -8, -180, -176, 696, 116, -88, 628, 76, 44, -516, 240,
    -208, -40, 100, -592, 344, -308, -452, -228, 20, 916, -1752, -136, -340, -804, 140, 40, 512,
    340, 248, 184, -492, 896, -156, 932, -628, 328, -688, -448, -616, -752, -100, 560, -1020, 180,
    -800, -64, 76, 576, 1068, 396, 660, 552, -108, -28, 320, -628, 312, -92, -92, -472, 268, 16,
    560, 516, -672, -52, 492, -100, 260, 384, 284, 292, 304, -148, 88, -152, 1012, 1064, -228, 164,
    -376, -684, 592, -392, 156, 196, -524, -64, -884, 160, -176, 636, 648, 404, -396, -436, 864,
    424, -728, 988, -604, 904, -592, 296, -224, 536, -176, -920, 436, -48, 1176, -884, 416, -776,
    -824, -884, 524, -548, -564, -68, -164, -96, 692, 364, -692, -1012, -68, 260, -480, 876, -1116,
    452, -332, -352, 892, -1088, 1220, -676, 12, -292, 244, 496, 372, -32, 280, 200, 112, -440,
    -96, 24, -644, -184, 56, -432, 224, -980, 272, -260, 144, -436, 420, 356, 364, -528, 76, 172,
    -744, -368, 404, -752, -416, 684, -688, 72, 540, 416, 92, 444, 480, -72, -1416, 164, -1172,
    -68, 24, 424, 264, 1040, 128, -912, -524, -356, 64, 876, -12, 4, -88, 532, 272, -524, 320, 276,
    -508, 940, 24, -400, -120, 756, 60, 236, -412, 100, 376, -484, 400, -100, -740, -108, -260,
    328, -268, 224, -200, -416, 184, -604, -564, -20, 296, 60, 892, -888, 60, 164, 68, -760, 216,
    -296, 904, -336, -28, 404, -356, -568, -208, -1480, -512, 296, 328, -360, -164, -1560, -776,
    1156, -428, 164, -504, -112, 120, -216, -148, -264, 308, 32, 64, -72, 72, 116, 176, -64, -272,
    460, -536, -784, -280, 348, 108, -752, -132, 524, -540, -776, 116, -296, -1196, -288, -560,
    1040, -472, 116, -848, -1116, 116, 636, 696, 284, -176, 1016, 204, -864, -648, -248, 356, 972,
    -584, -204, 264, 880, 528, -24, -184, 116, 448, -144, 828, 524, 212, -212, 52, 12, 200, 268,
    -488, -404, -880, 824, -672, -40, 908, -248, 500, 716, -576, 492, -576, 16, 720, -108, 384,
    124, 344, 280, 576, -500, 252, 104, -308, 196, -188, -8, 1268, 296, 1032, -1196, 436, 316, 372,
    -432, -200, -660, 704, -224, 596, -132, 268, 32, -452, 884, 104, -1008, 424, -1348, -280, 4,
    -1168, 368, 476, 696, 300, -8, 24, 180, -592, -196, 388, 304, 500, 724, -160, 244, -84, 272,
    -256, -420, 320, 208, -144, -156, 156, 364, 452, 28, 540, 316, 220, -644, -248, 464, 72, 360,
    32, -388, 496, -680, -48, 208, -116, -408, 60, -604, -392, 548, -840, 784, -460, 656, -544,
    -388, -264, 908, -800, -628, -612, -568, 572, -220, 164, 288, -16, -308, 308, -112, -636, -760,
    280, -668, 432, 364, 240, -196, 604, 340, 384, 196, 592, -44, -500, 432, -580, -132, 636, -76,
    392, 4, -412, 540, 508, 328, -356, -36, 16, -220, -64, -248, -60, 24, -192, 368, 1040, 92, -24,
    -1044, -32, 40, 104, 148, 192, -136, -520, 56, -816, -224, 732, 392, 356, 212, -80, -424,
    -1008, -324, 588, -1496, 576, 460, -816, -848, 56, -580, -92, -1372, -112, -496, 200, 364, 52,
    -140, 48, -48, -60, 84, 72, 40, 132, -356, -268, -104, -284, -404, 732, -520, 164, -304, -540,
    120, 328, -76, -460, 756, 388, 588, 236, -436, -72, -176, -404, -316, -148, 716, -604, 404,
    -72, -88, -888, -68, 944, 88, -220, -344, 960, 472, 460, -232, 704, 120, 832, -228, 692, -508,
    132, -476, 844, -748, -364, -44, 1116, -1104, -1056, 76, 428, 552, -692, 60, 356, 96, -384,
    -188, -612, -576, 736, 508, 892, 352, -1132, 504, -24, -352, 324, 332, -600, -312, 292, 508,
    -144, -8, 484, 48, 284, -260, -240, 256, -100, -292, -204, -44, 472, -204, 908, -188, -1000,
    -256, 92, 1164, -392, 564, 356, 652, -28, -884, 256, 484, -192, 760, -176, 376, -524, -452,
    -436, 860, -736, 212, 124, 504, -476, 468, 76, -472, 552, -692, -944, -620, 740, -240, 400,
    132, 20, 192, -196, 264, -668, -1012, -60, 296, -316, -828, 76, -156, 284, -768, -448, -832,
    148, 248, 652, 616, 1236, 288, -328, -400, -124, 588, 220, 520, -696, 1032, 768, -740, -92,
    -272, 296, 448, -464, 412, -200, 392, 440, -200, 264, -152, -260, 320, 1032, 216, 320, -8, -64,
    156, -1016, 1084, 1172, 536, 484, -432, 132, 372, -52, -256, 84, 116, -352, 48, 116, 304, -384,
    412, 924, -300, 528, 628, 180, 648, 44, -980, -220, 1320, 48, 332, 748, 524, -268, -720, 540,
    -276, 564, -344, -208, -196, 436, 896, 88, -392, 132, 80, -964, -288, 568, 56, -48, -456, 888,
    8, 552, -156, -292, 948, 288, 128, -716, -292, 1192, -152, 876, 352, -600, -260, -812, -468,
    -28, -120, -32, -44, 1284, 496, 192, 464, 312, -76, -516, -380, -456, -1012, -48, 308, -156,
    36, 492, -156, -808, 188, 1652, 68, -120, -116, 316, 160, -140, 352, 808, -416, 592, 316, -480,
    56, 528, -204, -568, 372, -232, 752, -344, 744, -4, 324, -416, -600, 768, 268, -248, -88, -132,
    -420, -432, 80, -288, 404, -316, -1216, -588, 520, -108, 92, -320, 368, -480, -216, -92, 1688,
    -300, 180, 1020, -176, 820, -68, -228, -260, 436, -904, 20, 40, -508, 440, -736, 312, 332, 204,
    760, -372, 728, 96, -20, -632, -520, -560, 336, 1076, -64, -532, 776, 584, 192, 396, -728,
    -520, 276, -188, 80, -52, -612, -252, -48, 648, 212, -688, 228, -52, -260, 428, -412, -272,
    -404, 180, 816, -796, 48, 152, 484, -88, -216, 988, 696, 188, -528, 648, -116, -180, 316, 476,
    12, -564, 96, 476, -252, -364, -376, -392, 556, -256, -576, 260, -352, 120, -16, -136, -260,
    -492, 72, 556, 660, 580, 616, 772, 436, 424, -32, -324, -1268, 416, -324, -80, 920, 160, 228,
    724, 32, -516, 64, 384, 68, -128, 136, 240, 248, -204, -68, 252, -932, -120, -480, -628, -84,
    192, 852, -404, -288, -132, 204, 100, 168, -68, -196, -868, 460, 1080, 380, -80, 244, 0, 484,
    -888, 64, 184, 352, 600, 460, 164, 604, -196, 320, -64, 588, -184, 228, 12, 372, 48, -848,
    -344, 224, 208, -200, 484, 128, -20, 272, -468, -840, 384, 256, -720, -520, -464, -580, 112,
    -120, 644, -356, -208, -608, -528, 704, 560, -424, 392, 828, 40, 84, 200, -152, 0, -144, 584,
    280, -120, 80, -556, -972, -196, -472, 724, 80, 168, -32, 88, 160, -688, 0, 160, 356, 372,
    -776, 740, -128, 676, -248, -480, 4, -364, 96, 544, 232, -1032, 956, 236, 356, 20, -40, 300,
    24, -676, -596, 132, 1120, -104, 532, -1096, 568, 648, 444, 508, 380, 188, -376, -604, 1488,
    424, 24, 756, -220, -192, 716, 120, 920, 688, 168, 44, -460, 568, 284, 1144, 1160, 600, 424,
    888, 656, -356, -320, 220, 316, -176, -724, -188, -816, -628, -348, -228, -380, 1012, -452,
    -660, 736, 928, 404, -696, -72, -268, -892, 128, 184, -344, -780, 360, 336, 400, 344, 428, 548,
    -112, 136, -228, -216, -820, -516, 340, 92, -136, 116, -300, 376, -244, 100, -316, -520, -284,
    -12, 824, 164, -548, -180, -128, 116, -924, -828, 268, -368, -580, 620, 192, 160, 0, -1676,
    1068, 424, -56, -360, 468, -156, 720, 288, -528, 556, -364, 548, -148, 504, 316, 152, -648,
    -620, -684, -24, -376, -384, -108, -920, -1032, 768, 180, -264, -508, -1268, -260, -60, 300,
    -240, 988, 724, -376, -576, -212, -736, 556, 192, 1092, -620, -880, 376, -56, -4, -216, -32,
    836, 268, 396, 1332, 864, -600, 100, 56, -412, -92, 356, 180, 884, -468, -436, 292, -388, -804,
    -704, -840, 368, -348, 140, -724, 1536, 940, 372, 112, -372, 436, -480, 1136, 296, -32, -228,
    132, -48, -220, 868, -1016, -60, -1044, -464, 328, 916, 244, 12, -736, -296, 360, 468, -376,
    -108, -92, 788, 368, -56, 544, 400, -672, -420, 728, 16, 320, 44, -284, -380, -796, 488, 132,
    204, -596, -372, 88, -152, -908, -636, -572, -624, -116, -692, -200, -56, 276, -88, 484, -324,
    948, 864, 1000, -456, -184, -276, 292, -296, 156, 676, 320, 160, 908, -84, -1236, -288, -116,
    260, -372, -644, 732, -756, -96, 84, 344, -520, 348, -688, 240, -84, 216, -1044, -136, -676,
    -396, -1500, 960, -40, 176, 168, 1516, 420, -504, -344, -364, -360, 1216, -940, -380, -212,
    252, -660, -708, 484, -444, -152, 928, -120, 1112, 476, -260, 560, -148, -344, 108, -196, 228,
    -288, 504, 560, -328, -88, 288, -1008, 460, -228, 468, -836, -196, 76, 388, 232, 412, -1168,
    -716, -644, 756, -172, -356, -504, 116, 432, 528, 48, 476, -168, -608, 448, 160, -532, -272,
    28, -676, -12, 828, 980, 456, 520, 104, -104, 256, -344, -4, -28, -368, -52, -524, -572, -556,
    -200, 768, 1124, -208, -512, 176, 232, 248, -148, -888, 604, -600, -304, 804, -156, -212, 488,
    -192, -804, -256, 368, -360, -916, -328, 228, -240, -448, -472, 856, -556, -364, 572, -12,
    -156, -368, -340, 432, 252, -752, -152, 288, 268, -580, -848, -592, 108, -76, 244, 312, -716,
    592, -80, 436, 360, 4, -248, 160, 516, 584, 732, 44, -468, -280, -292, -156, -588, 28, 308,
    912, 24, 124, 156, 180, -252, 944, -924, -772, -520, -428, -624, 300, -212, -1144, 32, -724,
    800, -1128, -212, -1288, -848, 180, -416, 440, 192, -576, -792, -76, -1080, 80, -532, -352,
    -132, 380, -820, 148, 1112, 128, 164, 456, 700, -924, 144, -668, -384, 648, -832, 508, 552,
    -52, -100, -656, 208, -568, 748, -88, 680, 232, 300, 192, -408, -1012, -152, -252, -268, 272,
    -876, -664, -648, -332, -136, 16, 12, 1152, -28, 332, -536, 320, -672, -460, -316, 532, -260,
    228, -40, 1052, -816, 180, 88, -496, -556, -672, -368, 428, 92, 356, 404, -408, 252, 196, -176,
    -556, 792, 268, 32, 372, 40, 96, -332, 328, 120, 372, -900, -40, 472, -264, -592, 952, 128,
    656, 112, 664, -232, 420, 4, -344, -464, 556, 244, -416, -32, 252, 0, -412, 188, -696, 508,
    -476, 324, -1096, 656, -312, 560, 264, -136, 304, 160, -64, -580, 248, 336, -720, 560, -348,
    -288, -276, -196, -500, 852, -544, -236, -1128, -992, -776, 116, 56, 52, 860, 884, 212, -12,
    168, 1020, 512, -552, 924, -148, 716, 188, 164, -340, -520, -184, 880, -152, -680, -208, -1156,
    -300, -528, -472, 364, 100, -744, -1056, -32, 540, 280, 144, -676, -32, -232, -280, -224, 96,
    568, -76, 172, 148, 148, 104, 32, -296, -32, 788, -80, 32, -16, 280, 288, 944, 428, -484,
];
