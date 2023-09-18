use crate::src::align::Align16;
use crate::src::align::Align32;
use crate::src::align::Align64;
use crate::src::const_fn::const_for;
use crate::src::const_fn::const_min;
use crate::src::levels::BS_16x16;
use crate::src::levels::BS_16x32;
use crate::src::levels::BS_16x8;
use crate::src::levels::BS_32x16;
use crate::src::levels::BS_32x32;
use crate::src::levels::BS_32x8;
use crate::src::levels::BS_8x16;
use crate::src::levels::BS_8x32;
use crate::src::levels::BS_8x8;
use crate::src::levels::II_DC_PRED;
use crate::src::levels::II_HOR_PRED;
use crate::src::levels::II_SMOOTH_PRED;
use crate::src::levels::II_VERT_PRED;
use crate::src::levels::N_BS_SIZES;
use crate::src::levels::N_INTER_INTRA_PRED_MODES;
use crate::src::qm::transposed;

use paste::paste;

extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}

pub type WedgeDirectionType = u8;
pub const WEDGE_HORIZONTAL: WedgeDirectionType = 0;
pub const WEDGE_VERTICAL: WedgeDirectionType = 1;
pub const WEDGE_OBLIQUE27: WedgeDirectionType = 2;
pub const WEDGE_OBLIQUE63: WedgeDirectionType = 3;
pub const WEDGE_OBLIQUE117: WedgeDirectionType = 4;
pub const WEDGE_OBLIQUE153: WedgeDirectionType = 5;
pub const _N_WEDGE_DIRECTIONS: usize = 6;

#[repr(C)]
pub struct wedge_code_type {
    pub direction: WedgeDirectionType,
    pub x_offset: u8,
    pub y_offset: u8,
}

impl wedge_code_type {
    const fn new(x_offset: u8, y_offset: u8, direction: WedgeDirectionType) -> Self {
        Self {
            direction,
            x_offset,
            y_offset,
        }
    }
}

static wedge_codebook_16_hgtw: [wedge_code_type; 16] = [
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE27),
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE63),
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE117),
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE153),
    wedge_code_type::new(4, 2, WEDGE_HORIZONTAL),
    wedge_code_type::new(4, 4, WEDGE_HORIZONTAL),
    wedge_code_type::new(4, 6, WEDGE_HORIZONTAL),
    wedge_code_type::new(4, 4, WEDGE_VERTICAL),
    wedge_code_type::new(4, 2, WEDGE_OBLIQUE27),
    wedge_code_type::new(4, 6, WEDGE_OBLIQUE27),
    wedge_code_type::new(4, 2, WEDGE_OBLIQUE153),
    wedge_code_type::new(4, 6, WEDGE_OBLIQUE153),
    wedge_code_type::new(2, 4, WEDGE_OBLIQUE63),
    wedge_code_type::new(6, 4, WEDGE_OBLIQUE63),
    wedge_code_type::new(2, 4, WEDGE_OBLIQUE117),
    wedge_code_type::new(6, 4, WEDGE_OBLIQUE117),
];

static wedge_codebook_16_hltw: [wedge_code_type; 16] = [
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE27),
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE63),
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE117),
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE153),
    wedge_code_type::new(2, 4, WEDGE_VERTICAL),
    wedge_code_type::new(4, 4, WEDGE_VERTICAL),
    wedge_code_type::new(6, 4, WEDGE_VERTICAL),
    wedge_code_type::new(4, 4, WEDGE_HORIZONTAL),
    wedge_code_type::new(4, 2, WEDGE_OBLIQUE27),
    wedge_code_type::new(4, 6, WEDGE_OBLIQUE27),
    wedge_code_type::new(4, 2, WEDGE_OBLIQUE153),
    wedge_code_type::new(4, 6, WEDGE_OBLIQUE153),
    wedge_code_type::new(2, 4, WEDGE_OBLIQUE63),
    wedge_code_type::new(6, 4, WEDGE_OBLIQUE63),
    wedge_code_type::new(2, 4, WEDGE_OBLIQUE117),
    wedge_code_type::new(6, 4, WEDGE_OBLIQUE117),
];

static wedge_codebook_16_heqw: [wedge_code_type; 16] = [
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE27),
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE63),
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE117),
    wedge_code_type::new(4, 4, WEDGE_OBLIQUE153),
    wedge_code_type::new(4, 2, WEDGE_HORIZONTAL),
    wedge_code_type::new(4, 6, WEDGE_HORIZONTAL),
    wedge_code_type::new(2, 4, WEDGE_VERTICAL),
    wedge_code_type::new(6, 4, WEDGE_VERTICAL),
    wedge_code_type::new(4, 2, WEDGE_OBLIQUE27),
    wedge_code_type::new(4, 6, WEDGE_OBLIQUE27),
    wedge_code_type::new(4, 2, WEDGE_OBLIQUE153),
    wedge_code_type::new(4, 6, WEDGE_OBLIQUE153),
    wedge_code_type::new(2, 4, WEDGE_OBLIQUE63),
    wedge_code_type::new(6, 4, WEDGE_OBLIQUE63),
    wedge_code_type::new(2, 4, WEDGE_OBLIQUE117),
    wedge_code_type::new(6, 4, WEDGE_OBLIQUE117),
];

static mut wedge_masks_444_32x32: Align64<[[[u8; 32 * 32]; 16]; 2]> =
    Align64([[[0; 32 * 32]; 16]; 2]);
static mut wedge_masks_444_32x16: Align64<[[[u8; 32 * 16]; 16]; 2]> =
    Align64([[[0; 32 * 16]; 16]; 2]);
static mut wedge_masks_444_32x8: Align64<[[[u8; 32 * 8]; 16]; 2]> = Align64([[[0; 32 * 8]; 16]; 2]);
static mut wedge_masks_444_16x32: Align64<[[[u8; 16 * 32]; 16]; 2]> =
    Align64([[[0; 16 * 32]; 16]; 2]);
static mut wedge_masks_444_16x16: Align64<[[[u8; 16 * 16]; 16]; 2]> =
    Align64([[[0; 16 * 16]; 16]; 2]);
static mut wedge_masks_444_16x8: Align64<[[[u8; 16 * 8]; 16]; 2]> = Align64([[[0; 16 * 8]; 16]; 2]);
static mut wedge_masks_444_8x32: Align64<[[[u8; 8 * 32]; 16]; 2]> = Align64([[[0; 8 * 32]; 16]; 2]);
static mut wedge_masks_444_8x16: Align64<[[[u8; 8 * 16]; 16]; 2]> = Align64([[[0; 8 * 16]; 16]; 2]);
static mut wedge_masks_444_8x8: Align64<[[[u8; 8 * 8]; 16]; 2]> = Align64([[[0; 8 * 8]; 16]; 2]);

static mut wedge_masks_422_16x32: Align64<[[[u8; 16 * 32]; 16]; 2]> =
    Align64([[[0; 16 * 32]; 16]; 2]);
static mut wedge_masks_422_16x16: Align64<[[[u8; 16 * 16]; 16]; 2]> =
    Align64([[[0; 16 * 16]; 16]; 2]);
static mut wedge_masks_422_16x8: Align64<[[[u8; 16 * 8]; 16]; 2]> = Align64([[[0; 16 * 8]; 16]; 2]);
static mut wedge_masks_422_8x32: Align64<[[[u8; 8 * 32]; 16]; 2]> = Align64([[[0; 8 * 32]; 16]; 2]);
static mut wedge_masks_422_8x16: Align64<[[[u8; 8 * 16]; 16]; 2]> = Align64([[[0; 8 * 16]; 16]; 2]);
static mut wedge_masks_422_8x8: Align64<[[[u8; 8 * 8]; 16]; 2]> = Align64([[[0; 8 * 8]; 16]; 2]);
static mut wedge_masks_422_4x32: Align64<[[[u8; 4 * 32]; 16]; 2]> = Align64([[[0; 4 * 32]; 16]; 2]);
static mut wedge_masks_422_4x16: Align64<[[[u8; 4 * 16]; 16]; 2]> = Align64([[[0; 4 * 16]; 16]; 2]);
static mut wedge_masks_422_4x8: Align64<[[[u8; 4 * 8]; 16]; 2]> = Align64([[[0; 4 * 8]; 16]; 2]);

static mut wedge_masks_420_16x16: Align64<[[[u8; 16 * 16]; 16]; 2]> =
    Align64([[[0; 16 * 16]; 16]; 2]);
static mut wedge_masks_420_16x8: Align64<[[[u8; 16 * 8]; 16]; 2]> = Align64([[[0; 16 * 8]; 16]; 2]);
static mut wedge_masks_420_16x4: Align64<[[[u8; 16 * 4]; 16]; 2]> = Align64([[[0; 16 * 4]; 16]; 2]);
static mut wedge_masks_420_8x16: Align64<[[[u8; 8 * 16]; 16]; 2]> = Align64([[[0; 8 * 16]; 16]; 2]);
static mut wedge_masks_420_8x8: Align64<[[[u8; 8 * 8]; 16]; 2]> = Align64([[[0; 8 * 8]; 16]; 2]);
static mut wedge_masks_420_8x4: Align64<[[[u8; 8 * 4]; 16]; 2]> = Align64([[[0; 8 * 4]; 16]; 2]);
static mut wedge_masks_420_4x16: Align64<[[[u8; 4 * 16]; 16]; 2]> = Align64([[[0; 4 * 16]; 16]; 2]);
static mut wedge_masks_420_4x8: Align64<[[[u8; 4 * 8]; 16]; 2]> = Align64([[[0; 4 * 8]; 16]; 2]);
static mut wedge_masks_420_4x4: Align64<[[[u8; 4 * 4]; 16]; 2]> = Align64([[[0; 4 * 4]; 16]; 2]);

pub static mut dav1d_wedge_masks: [[[[&'static [u8]; 16]; 2]; 3]; N_BS_SIZES] =
    [[[[&[]; 16]; 2]; 3]; N_BS_SIZES];

const fn insert_border(
    mut dst: [u8; 64 * 64],
    y: usize,
    src: &[u8; 8],
    ctr: usize,
) -> [u8; 64 * 64] {
    let dst_off = y * 64;

    {
        if ctr > 4 {
            const_for!(i in 0..ctr - 4 => {
                dst[dst_off + i] = 0;
            });
        }
    }
    {
        let dst_off = dst_off + ctr.saturating_sub(4);
        let src_off = 4usize.saturating_sub(ctr);
        let len = const_min!(64 - ctr, 8);
        const_for!(i in 0..len => {
            dst[dst_off + i] = src[src_off + i];
        });
    }
    {
        let ctr = ctr + 4;
        let dst_off = dst_off + ctr;
        if ctr < 64 {
            const_for!(i in 0..64 - ctr => {
                dst[dst_off + i] = 64;
            });
        }
    }

    dst
}

const fn hflip(src: &[u8; 64 * 64]) -> [u8; 64 * 64] {
    let mut dst = [0; 64 * 64];
    const_for!(y in 0..64 => {
        const_for!(x in 0..64 => {
            dst[(y * 64) + 64 - 1 - x] = src[(y * 64) + x];
        });
    });
    dst
}

unsafe fn invert(dst: &mut [u8], src: &[u8], w: usize, h: usize) {
    let mut y = 0;
    let mut y_off = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            dst[y_off + x] = 64 - src[y_off + x];
            x += 1;
        }
        y += 1;
        y_off += w;
    }
}

unsafe fn copy2d(
    mut dst: *mut u8,
    mut src: *const u8,
    w: usize,
    h: usize,
    x_off: usize,
    y_off: usize,
) {
    src = src.offset((y_off * 64 + x_off) as isize);
    let mut y = 0;
    while y < h {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            w as libc::c_ulong,
        );
        src = src.offset(64);
        dst = dst.offset(w as isize);
        y += 1;
    }
}

const fn init_chroma<const LEN_LUMA: usize, const LEN_CHROMA: usize>(
    luma: &[u8; LEN_LUMA],
    sign: bool,
    w: usize,
    h: usize,
    ss_ver: bool,
) -> [u8; LEN_CHROMA] {
    let sign = sign as u16;
    let ss_ver = ss_ver as usize;

    let mut chroma = [0; LEN_CHROMA];

    let mut luma_off = 0;
    let mut chroma_off = 0;
    const_for!(_y in 0..h, step_by 1 + ss_ver => {
        const_for!(x in 0..w, step_by 2 => {
            let mut sum = luma[luma_off + x] as u16 + luma[luma_off + x + 1] as u16 + 1;
            if ss_ver != 0 {
                sum += luma[luma_off + w + x] as u16 + luma[luma_off + w + x + 1] as u16 + 1;
            }
            chroma[chroma_off + (x >> 1)] = (sum - sign >> 1 + ss_ver) as u8;
        });
        luma_off += w << ss_ver;
        chroma_off += w >> 1;
    });

    chroma
}

#[cold]
unsafe fn fill2d_16x2<const LEN_444: usize, const LEN_422: usize, const LEN_420: usize>(
    dst: &mut [[[u8; LEN_444]; 16]; 2],
    w: usize,
    h: usize,
    master: &[[u8; 64 * 64]; 6],
    cb: &[wedge_code_type; 16],
    masks_444: &'static mut [[[u8; LEN_444]; 16]; 2],
    masks_422: &'static mut [[[u8; LEN_422]; 16]; 2],
    masks_420: &'static mut [[[u8; LEN_420]; 16]; 2],
    signs: libc::c_uint,
) -> [[[&'static [u8]; 16]; 2]; 3] {
    assert!(LEN_444 == (w * h) >> 0);
    assert!(LEN_422 == (w * h) >> 1);
    assert!(LEN_420 == (w * h) >> 2);

    for n in 0..16 {
        copy2d(
            dst[0][n].as_mut_ptr(),
            master[cb[n].direction as usize].as_ptr(),
            w,
            h,
            32 - (w * cb[n].x_offset as usize >> 3),
            32 - (h * cb[n].y_offset as usize >> 3),
        );
    }
    for n in 0..16 {
        let [dst_0, dst_1] = dst;
        invert(&mut dst_1[n], &dst_0[n], w, h);
    }

    for n in 0..16 {
        let sign = (signs >> n & 1) != 0;
        let luma = &masks_444[sign as usize][n];

        masks_422[sign as usize][n] = init_chroma(luma, false, w, h, false);
        masks_422[!sign as usize][n] = init_chroma(luma, true, w, h, false);
        masks_420[sign as usize][n] = init_chroma(luma, false, w, h, true);
        masks_420[!sign as usize][n] = init_chroma(luma, true, w, h, true);
    }

    let mut masks = [[[&[] as &'static [u8]; 16]; 2]; 3];

    // assign pointers in externally visible array
    for n in 0..16 {
        let sign = (signs >> n & 1) != 0;

        masks[0][0][n] = &masks_444[sign as usize][n];
        // not using !sign is intentional here, since 444 does not require
        // any rounding since no chroma subsampling is applied.
        masks[0][1][n] = &masks_444[sign as usize][n];
        masks[1][0][n] = &masks_422[sign as usize][n];
        masks[1][1][n] = &masks_422[!sign as usize][n];
        masks[2][0][n] = &masks_420[sign as usize][n];
        masks[2][1][n] = &masks_420[!sign as usize][n];
    }

    masks
}

#[cold]
pub unsafe fn dav1d_init_wedge_masks() {
    // This function is guaranteed to be called only once

    pub const WEDGE_MASTER_LINE_ODD: WedgeMasterLineType = 0;
    pub const WEDGE_MASTER_LINE_EVEN: WedgeMasterLineType = 1;
    pub const WEDGE_MASTER_LINE_VERT: WedgeMasterLineType = 2;
    pub type WedgeMasterLineType = libc::c_uint;
    pub const N_WEDGE_MASTER_LINES: usize = 3;

    static wedge_master_border: [[u8; 8]; N_WEDGE_MASTER_LINES] = [
        [1, 2, 6, 18, 37, 53, 60, 63],
        [1, 4, 11, 27, 46, 58, 62, 63],
        [0, 2, 7, 21, 43, 57, 62, 64],
    ];
    let mut master: [[u8; 64 * 64]; 6] = [[0; 64 * 64]; 6];

    // create master templates
    let mut y = 0;
    while y < 64 {
        master[WEDGE_VERTICAL as usize] = insert_border(
            master[WEDGE_VERTICAL as usize],
            y,
            &wedge_master_border[WEDGE_MASTER_LINE_VERT as usize],
            32,
        );
        y += 1;
    }
    let mut y = 0;
    let mut ctr = 48;
    while y < 64 {
        master[WEDGE_OBLIQUE63 as usize] = insert_border(
            master[WEDGE_OBLIQUE63 as usize],
            y,
            &wedge_master_border[WEDGE_MASTER_LINE_EVEN as usize],
            ctr,
        );
        master[WEDGE_OBLIQUE63 as usize] = insert_border(
            master[WEDGE_OBLIQUE63 as usize],
            y + 1,
            &wedge_master_border[WEDGE_MASTER_LINE_ODD as usize],
            ctr - 1,
        );
        y += 2;
        ctr -= 1;
    }

    master[WEDGE_OBLIQUE27 as usize] = transposed(&master[WEDGE_OBLIQUE63 as usize], 64, 64);
    master[WEDGE_HORIZONTAL as usize] = transposed(&master[WEDGE_VERTICAL as usize], 64, 64);
    master[WEDGE_OBLIQUE117 as usize] = hflip(&master[WEDGE_OBLIQUE63 as usize]);
    master[WEDGE_OBLIQUE153 as usize] = hflip(&master[WEDGE_OBLIQUE27 as usize]);

    dav1d_wedge_masks[BS_32x32 as usize] = fill2d_16x2(
        &mut wedge_masks_444_32x32.0,
        32,
        32,
        &master,
        &wedge_codebook_16_heqw,
        &mut wedge_masks_444_32x32.0,
        &mut wedge_masks_422_16x32.0,
        &mut wedge_masks_420_16x16.0,
        0x7bfb,
    );
    dav1d_wedge_masks[BS_32x16 as usize] = fill2d_16x2(
        &mut wedge_masks_444_32x16.0,
        32,
        16,
        &master,
        &wedge_codebook_16_hltw,
        &mut wedge_masks_444_32x16.0,
        &mut wedge_masks_422_16x16.0,
        &mut wedge_masks_420_16x8.0,
        0x7beb,
    );
    dav1d_wedge_masks[BS_32x8 as usize] = fill2d_16x2(
        &mut wedge_masks_444_32x8.0,
        32,
        8,
        &master,
        &wedge_codebook_16_hltw,
        &mut wedge_masks_444_32x8.0,
        &mut wedge_masks_422_16x8.0,
        &mut wedge_masks_420_16x4.0,
        0x6beb,
    );
    dav1d_wedge_masks[BS_16x32 as usize] = fill2d_16x2(
        &mut wedge_masks_444_16x32.0,
        16,
        32,
        &master,
        &wedge_codebook_16_hgtw,
        &mut wedge_masks_444_16x32.0,
        &mut wedge_masks_422_8x32.0,
        &mut wedge_masks_420_8x16.0,
        0x7beb,
    );
    dav1d_wedge_masks[BS_16x16 as usize] = fill2d_16x2(
        &mut wedge_masks_444_16x16.0,
        16,
        16,
        &master,
        &wedge_codebook_16_heqw,
        &mut wedge_masks_444_16x16.0,
        &mut wedge_masks_422_8x16.0,
        &mut wedge_masks_420_8x8.0,
        0x7bfb,
    );
    dav1d_wedge_masks[BS_16x8 as usize] = fill2d_16x2(
        &mut wedge_masks_444_16x8.0,
        16,
        8,
        &master,
        &wedge_codebook_16_hltw,
        &mut wedge_masks_444_16x8.0,
        &mut wedge_masks_422_8x8.0,
        &mut wedge_masks_420_8x4.0,
        0x7beb,
    );
    dav1d_wedge_masks[BS_8x32 as usize] = fill2d_16x2(
        &mut wedge_masks_444_8x32.0,
        8,
        32,
        &master,
        &wedge_codebook_16_hgtw,
        &mut wedge_masks_444_8x32.0,
        &mut wedge_masks_422_4x32.0,
        &mut wedge_masks_420_4x16.0,
        0x7aeb,
    );
    dav1d_wedge_masks[BS_8x16 as usize] = fill2d_16x2(
        &mut wedge_masks_444_8x16.0,
        8,
        16,
        &master,
        &wedge_codebook_16_hgtw,
        &mut wedge_masks_444_8x16.0,
        &mut wedge_masks_422_4x16.0,
        &mut wedge_masks_420_4x8.0,
        0x7beb,
    );
    dav1d_wedge_masks[BS_8x8 as usize] = fill2d_16x2(
        &mut wedge_masks_444_8x8.0,
        8,
        8,
        &master,
        &wedge_codebook_16_heqw,
        &mut wedge_masks_444_8x8.0,
        &mut wedge_masks_422_4x8.0,
        &mut wedge_masks_420_4x4.0,
        0x7bfb,
    );
}

static ii_dc_mask: Align64<[u8; 32 * 32]> = Align64([32; 32 * 32]);

const N_II_PRED_MODES: usize = N_INTER_INTRA_PRED_MODES - 1;

const fn build_nondc_ii_masks<const N: usize>(
    w: usize,
    h: usize,
    step: usize,
) -> [[u8; N]; N_II_PRED_MODES] {
    const ii_weights_1d: [u8; 32] = [
        60, 52, 45, 39, 34, 30, 26, 22, 19, 17, 15, 13, 11, 10, 8, 7, 6, 6, 5, 4, 4, 3, 3, 2, 2, 2,
        2, 1, 1, 1, 1, 1,
    ];

    let mut masks = [[0; N]; N_II_PRED_MODES];

    const_for!(y in 0..h => {
        let off = y * w;
        const_for!(i in 0..w => {
            masks[II_VERT_PRED as usize - 1][off + i] = ii_weights_1d[y * step];
        });
        const_for!(x in 0..w => {
            masks[II_SMOOTH_PRED as usize - 1][off + x] = ii_weights_1d[const_min!(x, y) * step];
            masks[II_HOR_PRED as usize - 1][off + x] = ii_weights_1d[x * step];
        });
    });

    masks
}

static ii_nondc_mask_32x32: Align64<[[u8; 32 * 32]; N_II_PRED_MODES]> =
    Align64(build_nondc_ii_masks(32, 32, 1));
static ii_nondc_mask_16x32: Align64<[[u8; 16 * 32]; N_II_PRED_MODES]> =
    Align64(build_nondc_ii_masks(16, 32, 1));
static ii_nondc_mask_16x16: Align64<[[u8; 16 * 16]; N_II_PRED_MODES]> =
    Align64(build_nondc_ii_masks(16, 16, 2));
static ii_nondc_mask_8x32: Align64<[[u8; 8 * 32]; N_II_PRED_MODES]> =
    Align64(build_nondc_ii_masks(8, 32, 1));
static ii_nondc_mask_8x16: Align64<[[u8; 8 * 16]; N_II_PRED_MODES]> =
    Align64(build_nondc_ii_masks(8, 16, 2));
static ii_nondc_mask_8x8: Align64<[[u8; 8 * 8]; N_II_PRED_MODES]> =
    Align64(build_nondc_ii_masks(8, 8, 4));
static ii_nondc_mask_4x16: Align64<[[u8; 4 * 16]; N_II_PRED_MODES]> =
    Align64(build_nondc_ii_masks(4, 16, 2));
static ii_nondc_mask_4x8: Align32<[[u8; 4 * 8]; N_II_PRED_MODES]> =
    Align32(build_nondc_ii_masks(4, 8, 4));
static ii_nondc_mask_4x4: Align16<[[u8; 4 * 4]; N_II_PRED_MODES]> =
    Align16(build_nondc_ii_masks(4, 4, 8));

pub static dav1d_ii_masks: [[[&'static [u8]; N_INTER_INTRA_PRED_MODES]; 3]; N_BS_SIZES] = {
    let mut masks = [[[&[] as &'static [u8]; N_INTER_INTRA_PRED_MODES]; 3]; N_BS_SIZES];

    macro_rules! set {
        ($h:literal x $w:literal) => {{
            let mut a = [&[] as &'static [u8]; N_INTER_INTRA_PRED_MODES];
            paste! {
                a[II_DC_PRED as usize] = &ii_dc_mask.0;
                a[II_VERT_PRED as usize] = &[<ii_nondc_mask _ $h x $w>].0[II_VERT_PRED as usize - 1];
                a[II_HOR_PRED as usize] = &[<ii_nondc_mask _ $h x $w>].0[II_HOR_PRED as usize - 1];
                a[II_SMOOTH_PRED as usize] = &[<ii_nondc_mask _ $h x $w>].0[II_SMOOTH_PRED as usize - 1];
            }
            a
        }};
        ([$($h:literal x $w:literal),*]) => {
            [$(set!($h x $w),)*]
        }
    }

    masks[BS_8x8 as usize] = set!([8 x 8, 4 x 8, 4 x 4]);
    masks[BS_8x16 as usize] = set!([8 x 16, 4 x 16, 4 x 8]);
    masks[BS_16x8 as usize] = set!([16 x 16, 8 x 8, 8 x 8]);
    masks[BS_16x16 as usize] = set!([16 x 16, 8 x 16, 8 x 8]);
    masks[BS_16x32 as usize] = set!([16 x 32, 8 x 32, 8 x 16]);
    masks[BS_32x16 as usize] = set!([32 x 32, 16 x 16, 16 x 16]);
    masks[BS_32x32 as usize] = set!([32 x 32, 16 x 32, 16 x 16]);

    masks
};
