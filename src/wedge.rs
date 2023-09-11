use std::cmp;

use crate::src::align::Align16;
use crate::src::align::Align32;
use crate::src::align::Align64;
use crate::src::const_fn::const_for;
use crate::src::levels::BS_16x16;
use crate::src::levels::BS_16x32;
use crate::src::levels::BS_16x8;
use crate::src::levels::BS_32x16;
use crate::src::levels::BS_32x32;
use crate::src::levels::BS_32x8;
use crate::src::levels::BS_8x16;
use crate::src::levels::BS_8x32;
use crate::src::levels::BS_8x8;
use crate::src::levels::BlockSize;
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
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
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

static mut wedge_masks_444_32x32: Align64<[u8; 2 * 16 * 32 * 32]> = Align64([0; 2 * 16 * 32 * 32]);
static mut wedge_masks_444_32x16: Align64<[u8; 2 * 16 * 32 * 16]> = Align64([0; 2 * 16 * 32 * 16]);
static mut wedge_masks_444_32x8: Align64<[u8; 2 * 16 * 32 * 8]> = Align64([0; 2 * 16 * 32 * 8]);
static mut wedge_masks_444_16x32: Align64<[u8; 2 * 16 * 16 * 32]> = Align64([0; 2 * 16 * 16 * 32]);
static mut wedge_masks_444_16x16: Align64<[u8; 2 * 16 * 16 * 16]> = Align64([0; 2 * 16 * 16 * 16]);
static mut wedge_masks_444_16x8: Align64<[u8; 2 * 16 * 16 * 8]> = Align64([0; 2 * 16 * 16 * 8]);
static mut wedge_masks_444_8x32: Align64<[u8; 2 * 16 * 8 * 32]> = Align64([0; 2 * 16 * 8 * 32]);
static mut wedge_masks_444_8x16: Align64<[u8; 2 * 16 * 8 * 16]> = Align64([0; 2 * 16 * 8 * 16]);
static mut wedge_masks_444_8x8: Align64<[u8; 2 * 16 * 8 * 8]> = Align64([0; 2 * 16 * 8 * 8]);

static mut wedge_masks_422_16x32: Align64<[u8; 2 * 16 * 16 * 32]> = Align64([0; 2 * 16 * 16 * 32]);
static mut wedge_masks_422_16x16: Align64<[u8; 2 * 16 * 16 * 16]> = Align64([0; 2 * 16 * 16 * 16]);
static mut wedge_masks_422_16x8: Align64<[u8; 2 * 16 * 16 * 8]> = Align64([0; 2 * 16 * 16 * 8]);
static mut wedge_masks_422_8x32: Align64<[u8; 2 * 16 * 8 * 32]> = Align64([0; 2 * 16 * 8 * 32]);
static mut wedge_masks_422_8x16: Align64<[u8; 2 * 16 * 8 * 16]> = Align64([0; 2 * 16 * 8 * 16]);
static mut wedge_masks_422_8x8: Align64<[u8; 2 * 16 * 8 * 8]> = Align64([0; 2 * 16 * 8 * 8]);
static mut wedge_masks_422_4x32: Align64<[u8; 2 * 16 * 4 * 32]> = Align64([0; 2 * 16 * 4 * 32]);
static mut wedge_masks_422_4x16: Align64<[u8; 2 * 16 * 4 * 16]> = Align64([0; 2 * 16 * 4 * 16]);
static mut wedge_masks_422_4x8: Align64<[u8; 2 * 16 * 4 * 8]> = Align64([0; 2 * 16 * 4 * 8]);

static mut wedge_masks_420_16x16: Align64<[u8; 2 * 16 * 16 * 16]> = Align64([0; 2 * 16 * 16 * 16]);
static mut wedge_masks_420_16x8: Align64<[u8; 2 * 16 * 16 * 8]> = Align64([0; 2 * 16 * 16 * 8]);
static mut wedge_masks_420_16x4: Align64<[u8; 2 * 16 * 16 * 4]> = Align64([0; 2 * 16 * 16 * 4]);
static mut wedge_masks_420_8x16: Align64<[u8; 2 * 16 * 8 * 16]> = Align64([0; 2 * 16 * 8 * 16]);
static mut wedge_masks_420_8x8: Align64<[u8; 2 * 16 * 8 * 8]> = Align64([0; 2 * 16 * 8 * 8]);
static mut wedge_masks_420_8x4: Align64<[u8; 2 * 16 * 8 * 4]> = Align64([0; 2 * 16 * 8 * 4]);
static mut wedge_masks_420_4x16: Align64<[u8; 2 * 16 * 4 * 16]> = Align64([0; 2 * 16 * 4 * 16]);
static mut wedge_masks_420_4x8: Align64<[u8; 2 * 16 * 4 * 8]> = Align64([0; 2 * 16 * 4 * 8]);
static mut wedge_masks_420_4x4: Align64<[u8; 2 * 16 * 4 * 4]> = Align64([0; 2 * 16 * 4 * 4]);

pub static mut dav1d_wedge_masks: [[[[*const u8; 16]; 2]; 3]; N_BS_SIZES] =
    [[[[0 as *const u8; 16]; 2]; 3]; N_BS_SIZES];

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
        let len = if 64 - ctr > 8 { 8 } else { 64 - ctr };
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

unsafe fn invert(dst: *mut u8, src: *const u8, w: usize, h: usize) {
    let mut y = 0;
    let mut y_off = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            *dst.offset((y_off + x) as isize) = 64 - *src.offset((y_off + x) as isize);
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

#[cold]
unsafe fn init_chroma(
    mut chroma: *mut u8,
    mut luma: *const u8,
    sign: libc::c_int,
    w: usize,
    h: usize,
    ss_ver: usize,
) {
    let mut y = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            let mut sum = *luma.offset(x as isize) as libc::c_int
                + *luma.offset((x + 1) as isize) as libc::c_int
                + 1;
            if ss_ver != 0 {
                sum += *luma.offset((w + x) as isize) as libc::c_int
                    + *luma.offset((w + x + 1) as isize) as libc::c_int
                    + 1;
            }
            *chroma.offset((x >> 1) as isize) = (sum - sign >> 1 + ss_ver) as u8;
            x += 2;
        }
        luma = luma.offset((w << ss_ver) as isize);
        chroma = chroma.offset((w >> 1) as isize);
        y += 1 + ss_ver;
    }
}

#[cold]
unsafe fn fill2d_16x2(
    dst: *mut u8,
    w: usize,
    h: usize,
    bs: BlockSize,
    master: *const [u8; 64 * 64],
    cb: *const wedge_code_type,
    mut masks_444: *mut u8,
    mut masks_422: *mut u8,
    mut masks_420: *mut u8,
    signs: libc::c_uint,
) {
    let bs = bs as usize;

    let mut ptr: *mut u8 = dst;
    let mut n = 0;
    while n < 16 {
        copy2d(
            ptr,
            (*master.offset((*cb.offset(n as isize)).direction as isize)).as_ptr(),
            w,
            h,
            32 - (w * (*cb.offset(n as isize)).x_offset as usize >> 3),
            32 - (h * (*cb.offset(n as isize)).y_offset as usize >> 3),
        );
        ptr = ptr.offset((w * h) as isize);
        n += 1;
    }
    let mut n = 0;
    let mut off = 0;
    while n < 16 {
        invert(ptr.offset(off as isize), dst.offset(off as isize), w, h);
        n += 1;
        off += w * h;
    }

    let n_stride_444 = w * h;
    let n_stride_422 = n_stride_444 >> 1;
    let n_stride_420 = n_stride_444 >> 2;
    let sign_stride_444 = 16 * n_stride_444;
    let sign_stride_422 = 16 * n_stride_422;
    let sign_stride_420 = 16 * n_stride_420;
    // assign pointers in externally visible array
    let mut n = 0;
    while n < 16 {
        let sign = (signs >> n & 1) as usize;
        dav1d_wedge_masks[bs][0][0][n] = masks_444.offset((sign * sign_stride_444) as isize);
        // not using !sign is intentional here, since 444 does not require
        // any rounding since no chroma subsampling is applied.
        dav1d_wedge_masks[bs][0][1][n] = masks_444.offset((sign * sign_stride_444) as isize);
        dav1d_wedge_masks[bs][1][0][n] = masks_422.offset((sign * sign_stride_422) as isize);
        dav1d_wedge_masks[bs][1][1][n] =
            masks_422.offset(((sign == 0) as usize * sign_stride_422) as isize);
        dav1d_wedge_masks[bs][2][0][n] = masks_420.offset((sign * sign_stride_420) as isize);
        dav1d_wedge_masks[bs][2][1][n] =
            masks_420.offset(((sign == 0) as usize * sign_stride_420) as isize);
        masks_444 = masks_444.offset(n_stride_444 as isize);
        masks_422 = masks_422.offset(n_stride_422 as isize);
        masks_420 = masks_420.offset(n_stride_420 as isize);

        // since the pointers come from inside, we know that
        // violation of the const is OK here. Any other approach
        // means we would have to duplicate the sign correction
        // logic in two places, which isn't very nice, or mark
        // the table faced externally as non-const, which also sucks
        init_chroma(
            dav1d_wedge_masks[bs][1][0][n] as *mut u8,
            dav1d_wedge_masks[bs][0][0][n],
            0,
            w,
            h,
            0,
        );
        init_chroma(
            dav1d_wedge_masks[bs][1][1][n] as *mut u8,
            dav1d_wedge_masks[bs][0][0][n],
            1,
            w,
            h,
            0,
        );
        init_chroma(
            dav1d_wedge_masks[bs][2][0][n] as *mut u8,
            dav1d_wedge_masks[bs][0][0][n],
            0,
            w,
            h,
            1,
        );
        init_chroma(
            dav1d_wedge_masks[bs][2][1][n] as *mut u8,
            dav1d_wedge_masks[bs][0][0][n],
            1,
            w,
            h,
            1,
        );
        n += 1;
    }
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

    fill2d_16x2(
        wedge_masks_444_32x32.0.as_mut_ptr(),
        32,
        32,
        BS_32x32,
        master.as_ptr(),
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_32x32.0.as_mut_ptr(),
        wedge_masks_422_16x32.0.as_mut_ptr(),
        wedge_masks_420_16x16.0.as_mut_ptr(),
        0x7bfb,
    );
    fill2d_16x2(
        wedge_masks_444_32x16.0.as_mut_ptr(),
        32,
        16,
        BS_32x16,
        master.as_ptr(),
        wedge_codebook_16_hltw.as_ptr(),
        wedge_masks_444_32x16.0.as_mut_ptr(),
        wedge_masks_422_16x16.0.as_mut_ptr(),
        wedge_masks_420_16x8.0.as_mut_ptr(),
        0x7beb,
    );
    fill2d_16x2(
        wedge_masks_444_32x8.0.as_mut_ptr(),
        32,
        8,
        BS_32x8,
        master.as_ptr(),
        wedge_codebook_16_hltw.as_ptr(),
        wedge_masks_444_32x8.0.as_mut_ptr(),
        wedge_masks_422_16x8.0.as_mut_ptr(),
        wedge_masks_420_16x4.0.as_mut_ptr(),
        0x6beb,
    );
    fill2d_16x2(
        wedge_masks_444_16x32.0.as_mut_ptr(),
        16,
        32,
        BS_16x32,
        master.as_ptr(),
        wedge_codebook_16_hgtw.as_ptr(),
        wedge_masks_444_16x32.0.as_mut_ptr(),
        wedge_masks_422_8x32.0.as_mut_ptr(),
        wedge_masks_420_8x16.0.as_mut_ptr(),
        0x7beb,
    );
    fill2d_16x2(
        wedge_masks_444_16x16.0.as_mut_ptr(),
        16,
        16,
        BS_16x16,
        master.as_ptr(),
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_16x16.0.as_mut_ptr(),
        wedge_masks_422_8x16.0.as_mut_ptr(),
        wedge_masks_420_8x8.0.as_mut_ptr(),
        0x7bfb,
    );
    fill2d_16x2(
        wedge_masks_444_16x8.0.as_mut_ptr(),
        16,
        8,
        BS_16x8,
        master.as_ptr(),
        wedge_codebook_16_hltw.as_ptr(),
        wedge_masks_444_16x8.0.as_mut_ptr(),
        wedge_masks_422_8x8.0.as_mut_ptr(),
        wedge_masks_420_8x4.0.as_mut_ptr(),
        0x7beb,
    );
    fill2d_16x2(
        wedge_masks_444_8x32.0.as_mut_ptr(),
        8,
        32,
        BS_8x32,
        master.as_ptr(),
        wedge_codebook_16_hgtw.as_ptr(),
        wedge_masks_444_8x32.0.as_mut_ptr(),
        wedge_masks_422_4x32.0.as_mut_ptr(),
        wedge_masks_420_4x16.0.as_mut_ptr(),
        0x7aeb,
    );
    fill2d_16x2(
        wedge_masks_444_8x16.0.as_mut_ptr(),
        8,
        16,
        BS_8x16,
        master.as_ptr(),
        wedge_codebook_16_hgtw.as_ptr(),
        wedge_masks_444_8x16.0.as_mut_ptr(),
        wedge_masks_422_4x16.0.as_mut_ptr(),
        wedge_masks_420_4x8.0.as_mut_ptr(),
        0x7beb,
    );
    fill2d_16x2(
        wedge_masks_444_8x8.0.as_mut_ptr(),
        8,
        8,
        BS_8x8,
        master.as_ptr(),
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_8x8.0.as_mut_ptr(),
        wedge_masks_422_4x8.0.as_mut_ptr(),
        wedge_masks_420_4x4.0.as_mut_ptr(),
        0x7bfb,
    );
}

static mut ii_dc_mask: Align64<[u8; 32 * 32]> = Align64([0; 32 * 32]);

const N_II_PRED_MODES: usize = N_INTER_INTRA_PRED_MODES - 1;

static mut ii_nondc_mask_32x32: Align64<[[u8; 32 * 32]; N_II_PRED_MODES]> =
    Align64([[0; 32 * 32]; N_II_PRED_MODES]);
static mut ii_nondc_mask_16x32: Align64<[[u8; 16 * 32]; N_II_PRED_MODES]> =
    Align64([[0; 16 * 32]; N_II_PRED_MODES]);
static mut ii_nondc_mask_16x16: Align64<[[u8; 16 * 16]; N_II_PRED_MODES]> =
    Align64([[0; 16 * 16]; N_II_PRED_MODES]);
static mut ii_nondc_mask_8x32: Align64<[[u8; 8 * 32]; N_II_PRED_MODES]> =
    Align64([[0; 8 * 32]; N_II_PRED_MODES]);
static mut ii_nondc_mask_8x16: Align64<[[u8; 8 * 16]; N_II_PRED_MODES]> =
    Align64([[0; 8 * 16]; N_II_PRED_MODES]);
static mut ii_nondc_mask_8x8: Align64<[[u8; 8 * 8]; N_II_PRED_MODES]> =
    Align64([[0; 8 * 8]; N_II_PRED_MODES]);
static mut ii_nondc_mask_4x16: Align64<[[u8; 4 * 16]; N_II_PRED_MODES]> =
    Align64([[0; 4 * 16]; N_II_PRED_MODES]);
static mut ii_nondc_mask_4x8: Align32<[[u8; 4 * 8]; N_II_PRED_MODES]> =
    Align32([[0; 4 * 8]; N_II_PRED_MODES]);
static mut ii_nondc_mask_4x4: Align16<[[u8; 4 * 4]; N_II_PRED_MODES]> =
    Align16([[0; 4 * 4]; N_II_PRED_MODES]);

pub static dav1d_ii_masks: [[[Option<&'static [u8]>; N_INTER_INTRA_PRED_MODES]; 3]; N_BS_SIZES] = {
    let mut masks = [[[None; N_INTER_INTRA_PRED_MODES]; 3]; N_BS_SIZES];

    macro_rules! set1 {
        ($sz:ident) => {{
            let mut a: [Option<&'static [u8]>; N_INTER_INTRA_PRED_MODES] = [None; 4];
            paste! {
                // Safety: [`dav1d_init_interintra_masks`] is only called once at the beginning.
                unsafe {
                    a[II_DC_PRED as usize] = Some(&ii_dc_mask.0);
                    a[II_VERT_PRED as usize] = Some(&[<ii_nondc_mask $sz>].0[II_VERT_PRED as usize - 1]);
                    a[II_HOR_PRED as usize] = Some(&[<ii_nondc_mask $sz>].0[II_HOR_PRED as usize - 1]);
                    a[II_SMOOTH_PRED as usize] = Some(&[<ii_nondc_mask $sz>].0[II_SMOOTH_PRED as usize - 1]);
                }
            }
            a
        }};
    }

    macro_rules! set {
        ($sz_444:ident, $sz_422:ident, $sz_420:ident) => {
            [set1!($sz_444), set1!($sz_422), set1!($sz_420)]
        };
    }

    masks[BS_8x8 as usize] = set!(_8x8, _4x8, _4x4);
    masks[BS_8x16 as usize] = set!(_8x16, _4x16, _4x8);
    masks[BS_16x8 as usize] = set!(_16x16, _8x8, _8x8);
    masks[BS_16x16 as usize] = set!(_16x16, _8x16, _8x8);
    masks[BS_16x32 as usize] = set!(_16x32, _8x32, _8x16);
    masks[BS_32x16 as usize] = set!(_32x32, _16x16, _16x16);
    masks[BS_32x32 as usize] = set!(_32x32, _16x32, _16x16);

    masks
};

#[cold]
unsafe fn build_nondc_ii_masks(
    mask_v: &mut [u8],
    mask_h: &mut [u8],
    mask_sm: &mut [u8],
    w: usize,
    h: usize,
    step: usize,
) {
    static ii_weights_1d: [u8; 32] = [
        60, 52, 45, 39, 34, 30, 26, 22, 19, 17, 15, 13, 11, 10, 8, 7, 6, 6, 5, 4, 4, 3, 3, 2, 2, 2,
        2, 1, 1, 1, 1, 1,
    ];

    let mut y = 0;
    let mut off = 0;
    while y < h {
        mask_v[off..][..w].fill(ii_weights_1d[y * step]);
        let mut x = 0;
        while x < w {
            mask_sm[off + x] = ii_weights_1d[cmp::min(x, y) * step];
            mask_h[off + x] = ii_weights_1d[x * step];
            x += 1;
        }
        y += 1;
        off += w;
    }
}

#[cold]
pub unsafe fn dav1d_init_interintra_masks() {
    // This function is guaranteed to be called only once

    memset(ii_dc_mask.0.as_mut_ptr() as *mut libc::c_void, 32, 32 * 32);
    build_nondc_ii_masks(
        &mut ii_nondc_mask_32x32.0[II_VERT_PRED as usize - 1],
        &mut ii_nondc_mask_32x32.0[II_HOR_PRED as usize - 1],
        &mut ii_nondc_mask_32x32.0[II_SMOOTH_PRED as usize - 1],
        32,
        32,
        1,
    );
    build_nondc_ii_masks(
        &mut ii_nondc_mask_16x32.0[II_VERT_PRED as usize - 1],
        &mut ii_nondc_mask_16x32.0[II_HOR_PRED as usize - 1],
        &mut ii_nondc_mask_16x32.0[II_SMOOTH_PRED as usize - 1],
        16,
        32,
        1,
    );
    build_nondc_ii_masks(
        &mut ii_nondc_mask_16x16.0[II_VERT_PRED as usize - 1],
        &mut ii_nondc_mask_16x16.0[II_HOR_PRED as usize - 1],
        &mut ii_nondc_mask_16x16.0[II_SMOOTH_PRED as usize - 1],
        16,
        16,
        2,
    );
    build_nondc_ii_masks(
        &mut ii_nondc_mask_8x32.0[II_VERT_PRED as usize - 1],
        &mut ii_nondc_mask_8x32.0[II_HOR_PRED as usize - 1],
        &mut ii_nondc_mask_8x32.0[II_SMOOTH_PRED as usize - 1],
        8,
        32,
        1,
    );
    build_nondc_ii_masks(
        &mut ii_nondc_mask_8x16.0[II_VERT_PRED as usize - 1],
        &mut ii_nondc_mask_8x16.0[II_HOR_PRED as usize - 1],
        &mut ii_nondc_mask_8x16.0[II_SMOOTH_PRED as usize - 1],
        8,
        16,
        2,
    );
    build_nondc_ii_masks(
        &mut ii_nondc_mask_8x8.0[II_VERT_PRED as usize - 1],
        &mut ii_nondc_mask_8x8.0[II_HOR_PRED as usize - 1],
        &mut ii_nondc_mask_8x8.0[II_SMOOTH_PRED as usize - 1],
        8,
        8,
        4,
    );
    build_nondc_ii_masks(
        &mut ii_nondc_mask_4x16.0[II_VERT_PRED as usize - 1],
        &mut ii_nondc_mask_4x16.0[II_HOR_PRED as usize - 1],
        &mut ii_nondc_mask_4x16.0[II_SMOOTH_PRED as usize - 1],
        4,
        16,
        2,
    );
    build_nondc_ii_masks(
        &mut ii_nondc_mask_4x8.0[II_VERT_PRED as usize - 1],
        &mut ii_nondc_mask_4x8.0[II_HOR_PRED as usize - 1],
        &mut ii_nondc_mask_4x8.0[II_SMOOTH_PRED as usize - 1],
        4,
        8,
        4,
    );
    build_nondc_ii_masks(
        &mut ii_nondc_mask_4x4.0[II_VERT_PRED as usize - 1],
        &mut ii_nondc_mask_4x4.0[II_HOR_PRED as usize - 1],
        &mut ii_nondc_mask_4x4.0[II_SMOOTH_PRED as usize - 1],
        4,
        4,
        8,
    );
}
