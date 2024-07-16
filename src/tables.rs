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

pub static dav1d_txfm_dimensions: [TxfmInfo; TxfmSize::COUNT] = {
    use TxfmSize::*;
    [
        TxfmInfo {
            w: 1,
            h: 1,
            lw: 0,
            lh: 0,
            min: 0,
            max: 0,
            sub: DefaultValue::DEFAULT,
            ctx: 0,
        },
        TxfmInfo {
            w: 2,
            h: 2,
            lw: 1,
            lh: 1,
            min: 1,
            max: 1,
            sub: S4x4,
            ctx: 1,
        },
        TxfmInfo {
            w: 4,
            h: 4,
            lw: 2,
            lh: 2,
            min: 2,
            max: 2,
            sub: S8x8,
            ctx: 2,
        },
        TxfmInfo {
            w: 8,
            h: 8,
            lw: 3,
            lh: 3,
            min: 3,
            max: 3,
            sub: S16x16,
            ctx: 3,
        },
        TxfmInfo {
            w: 16,
            h: 16,
            lw: 4,
            lh: 4,
            min: 4,
            max: 4,
            sub: S32x32,
            ctx: 4,
        },
        TxfmInfo {
            w: 1,
            h: 2,
            lw: 0,
            lh: 1,
            min: 0,
            max: 1,
            sub: S4x4,
            ctx: 1,
        },
        TxfmInfo {
            w: 2,
            h: 1,
            lw: 1,
            lh: 0,
            min: 0,
            max: 1,
            sub: S4x4,
            ctx: 1,
        },
        TxfmInfo {
            w: 2,
            h: 4,
            lw: 1,
            lh: 2,
            min: 1,
            max: 2,
            sub: S8x8,
            ctx: 2,
        },
        TxfmInfo {
            w: 4,
            h: 2,
            lw: 2,
            lh: 1,
            min: 1,
            max: 2,
            sub: S8x8,
            ctx: 2,
        },
        TxfmInfo {
            w: 4,
            h: 8,
            lw: 2,
            lh: 3,
            min: 2,
            max: 3,
            sub: S16x16,
            ctx: 3,
        },
        TxfmInfo {
            w: 8,
            h: 4,
            lw: 3,
            lh: 2,
            min: 2,
            max: 3,
            sub: S16x16,
            ctx: 3,
        },
        TxfmInfo {
            w: 8,
            h: 16,
            lw: 3,
            lh: 4,
            min: 3,
            max: 4,
            sub: S32x32,
            ctx: 4,
        },
        TxfmInfo {
            w: 16,
            h: 8,
            lw: 4,
            lh: 3,
            min: 3,
            max: 4,
            sub: S32x32,
            ctx: 4,
        },
        TxfmInfo {
            w: 1,
            h: 4,
            lw: 0,
            lh: 2,
            min: 0,
            max: 2,
            sub: R4x8,
            ctx: 1,
        },
        TxfmInfo {
            w: 4,
            h: 1,
            lw: 2,
            lh: 0,
            min: 0,
            max: 2,
            sub: R8x4,
            ctx: 1,
        },
        TxfmInfo {
            w: 2,
            h: 8,
            lw: 1,
            lh: 3,
            min: 1,
            max: 3,
            sub: R8x16,
            ctx: 2,
        },
        TxfmInfo {
            w: 8,
            h: 2,
            lw: 3,
            lh: 1,
            min: 1,
            max: 3,
            sub: R16x8,
            ctx: 2,
        },
        TxfmInfo {
            w: 4,
            h: 16,
            lw: 2,
            lh: 4,
            min: 2,
            max: 4,
            sub: R16x32,
            ctx: 3,
        },
        TxfmInfo {
            w: 16,
            h: 4,
            lw: 4,
            lh: 2,
            min: 2,
            max: 4,
            sub: R32x16,
            ctx: 3,
        },
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

/// Imports an `extern static` item from C and exposes it as a Rust fn.
///
/// Doing `extern_table! { pub static foo: T; }` imports an item named
/// `dav1d_foo` and generates a Rust function named `foo` that returns a
/// reference to the item. The declared type `T` must match the type and layout
/// of the corresponding C definition.
///
/// This is necessary for items declared as `extern
/// __attribute__((visibility("hidden")))` in C, as we have no way of
/// replicating that visibility in Rust. See
/// https://github.com/rust-lang/rust/issues/73958 for more information on why
/// `#[no_mangle]` doesn't work for us.
macro_rules! extern_table {
    (pub static $name:ident: $type:ty;) => {
        paste::paste! {
            #[inline]
            pub fn $name() -> &'static $type {
                extern "C" {
                    static [<dav1d_ $name>]: $type;
                }

                // SAFETY: Table is defined in `tables.c` and has the same type as the Rust
                // definition.
                unsafe {
                    &[<dav1d_ $name>]
                }
            }
        }
    };
}

extern_table! { pub static sgr_x_by_x: Align64<[u8; 256]>; }
extern_table! { pub static mc_subpel_filters: Align8<[[[i8; 8]; 15]; 6]>; }
extern_table! { pub static mc_warp_filter: Align8<[[i8; 8]; 193]>; }
extern_table! { pub static resize_filter: Align8<[[i8; 8]; 64]>; }
extern_table! { pub static sm_weights: Align16<[u8; 128]>; }
extern_table! { pub static dr_intra_derivative: [u16; 44]; }
extern_table! { pub static filter_intra_taps: [Align64<[i8; 64]>; 5]; }
extern_table! { pub static obmc_masks: Align16<[u8; 64]>; }
extern_table! { pub static gaussian_sequence: [i16; 2048]; }
