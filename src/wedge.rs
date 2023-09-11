use std::cmp;

use crate::include::stdint::*;
use crate::src::align::Align16;
use crate::src::align::Align32;
use crate::src::align::Align64;
use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

use crate::src::levels::II_HOR_PRED;
use crate::src::levels::II_SMOOTH_PRED;
use crate::src::levels::II_VERT_PRED;

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
use crate::src::levels::N_BS_SIZES;
use crate::src::levels::N_INTER_INTRA_PRED_MODES;

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
    pub x_offset: uint8_t,
    pub y_offset: uint8_t,
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

pub const WEDGE_MASTER_LINE_ODD: WedgeMasterLineType = 0;
pub const WEDGE_MASTER_LINE_EVEN: WedgeMasterLineType = 1;
pub const WEDGE_MASTER_LINE_VERT: WedgeMasterLineType = 2;
pub type WedgeMasterLineType = libc::c_uint;
pub const N_WEDGE_MASTER_LINES: usize = 3;

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

static mut wedge_masks_444_32x32: Align64<[uint8_t; 32768]> = Align64([0; 32768]);
static mut wedge_masks_444_32x16: Align64<[uint8_t; 16384]> = Align64([0; 16384]);
static mut wedge_masks_444_32x8: Align64<[uint8_t; 8192]> = Align64([0; 8192]);
static mut wedge_masks_444_16x32: Align64<[uint8_t; 16384]> = Align64([0; 16384]);
static mut wedge_masks_444_16x16: Align64<[uint8_t; 8192]> = Align64([0; 8192]);
static mut wedge_masks_444_16x8: Align64<[uint8_t; 4096]> = Align64([0; 4096]);
static mut wedge_masks_444_8x32: Align64<[uint8_t; 8192]> = Align64([0; 8192]);
static mut wedge_masks_444_8x16: Align64<[uint8_t; 4096]> = Align64([0; 4096]);
static mut wedge_masks_444_8x8: Align64<[uint8_t; 2048]> = Align64([0; 2048]);
static mut wedge_masks_422_16x32: Align64<[uint8_t; 16384]> = Align64([0; 16384]);
static mut wedge_masks_422_16x16: Align64<[uint8_t; 8192]> = Align64([0; 8192]);
static mut wedge_masks_422_16x8: Align64<[uint8_t; 4096]> = Align64([0; 4096]);
static mut wedge_masks_422_8x32: Align64<[uint8_t; 8192]> = Align64([0; 8192]);
static mut wedge_masks_422_8x16: Align64<[uint8_t; 4096]> = Align64([0; 4096]);
static mut wedge_masks_422_8x8: Align64<[uint8_t; 2048]> = Align64([0; 2048]);
static mut wedge_masks_422_4x32: Align64<[uint8_t; 4096]> = Align64([0; 4096]);
static mut wedge_masks_422_4x16: Align64<[uint8_t; 2048]> = Align64([0; 2048]);
static mut wedge_masks_422_4x8: Align64<[uint8_t; 1024]> = Align64([0; 1024]);
static mut wedge_masks_420_16x16: Align64<[uint8_t; 8192]> = Align64([0; 8192]);
static mut wedge_masks_420_16x8: Align64<[uint8_t; 4096]> = Align64([0; 4096]);
static mut wedge_masks_420_16x4: Align64<[uint8_t; 2048]> = Align64([0; 2048]);
static mut wedge_masks_420_8x16: Align64<[uint8_t; 4096]> = Align64([0; 4096]);
static mut wedge_masks_420_8x8: Align64<[uint8_t; 2048]> = Align64([0; 2048]);
static mut wedge_masks_420_8x4: Align64<[uint8_t; 1024]> = Align64([0; 1024]);
static mut wedge_masks_420_4x16: Align64<[uint8_t; 2048]> = Align64([0; 2048]);
static mut wedge_masks_420_4x8: Align64<[uint8_t; 1024]> = Align64([0; 1024]);
static mut wedge_masks_420_4x4: Align64<[uint8_t; 512]> = Align64([0; 512]);

#[no_mangle]
pub static mut dav1d_wedge_masks: [[[[*const uint8_t; 16]; 2]; 3]; N_BS_SIZES] =
    [[[[0 as *const uint8_t; 16]; 2]; 3]; 22];
unsafe extern "C" fn insert_border(dst: *mut uint8_t, src: *const uint8_t, ctr: libc::c_int) {
    if ctr > 4 {
        memset(dst as *mut libc::c_void, 0, ctr as libc::c_ulong - 4);
    }
    memcpy(
        dst.offset(cmp::max(ctr, 4) as isize).offset(-4) as *mut libc::c_void,
        src.offset(cmp::max(4 - ctr, 0) as isize) as *const libc::c_void,
        cmp::min(64 - ctr as libc::c_ulong, 8),
    );
    if ctr < 64 - 4 {
        memset(
            dst.offset(ctr as isize).offset(4) as *mut libc::c_void,
            64,
            64 - 4 - ctr as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn transpose(dst: *mut uint8_t, src: *const uint8_t) {
    let mut y = 0;
    let mut y_off = 0;
    while y < 64 {
        let mut x = 0;
        let mut x_off = 0;
        while x < 64 {
            *dst.offset((x_off + y) as isize) = *src.offset((y_off + x) as isize);
            x += 1;
            x_off += 64;
        }
        y += 1;
        y_off += 64;
    }
}
unsafe extern "C" fn hflip(dst: *mut uint8_t, src: *const uint8_t) {
    let mut y = 0;
    let mut y_off = 0;
    while y < 64 {
        let mut x = 0;
        while x < 64 {
            *dst.offset((y_off + 64 - 1 - x) as isize) = *src.offset((y_off + x) as isize);
            x += 1;
        }
        y += 1;
        y_off += 64;
    }
}
unsafe extern "C" fn invert(
    dst: *mut uint8_t,
    src: *const uint8_t,
    w: libc::c_int,
    h: libc::c_int,
) {
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
unsafe extern "C" fn copy2d(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    w: libc::c_int,
    h: libc::c_int,
    x_off: libc::c_int,
    y_off: libc::c_int,
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
unsafe extern "C" fn init_chroma(
    mut chroma: *mut uint8_t,
    mut luma: *const uint8_t,
    sign: libc::c_int,
    w: libc::c_int,
    h: libc::c_int,
    ss_ver: libc::c_int,
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
            *chroma.offset((x >> 1) as isize) = (sum - sign >> 1 + ss_ver) as uint8_t;
            x += 2;
        }
        luma = luma.offset((w << ss_ver) as isize);
        chroma = chroma.offset((w >> 1) as isize);
        y += 1 + ss_ver;
    }
}
#[cold]
unsafe extern "C" fn fill2d_16x2(
    dst: *mut uint8_t,
    w: libc::c_int,
    h: libc::c_int,
    bs: BlockSize,
    master: *const [uint8_t; 4096],
    cb: *const wedge_code_type,
    mut masks_444: *mut uint8_t,
    mut masks_422: *mut uint8_t,
    mut masks_420: *mut uint8_t,
    signs: libc::c_uint,
) {
    let mut ptr: *mut uint8_t = dst;
    let mut n = 0;
    while n < 16 {
        copy2d(
            ptr,
            (*master.offset((*cb.offset(n as isize)).direction as isize)).as_ptr(),
            w,
            h,
            32 - (w * (*cb.offset(n as isize)).x_offset as libc::c_int >> 3),
            32 - (h * (*cb.offset(n as isize)).y_offset as libc::c_int >> 3),
        );
        ptr = ptr.offset((w * h) as isize);
        n += 1;
    }
    let mut n_0 = 0;
    let mut off = 0;
    while n_0 < 16 {
        invert(ptr.offset(off as isize), dst.offset(off as isize), w, h);
        n_0 += 1;
        off += w * h;
    }
    let n_stride_444 = w * h;
    let n_stride_422 = n_stride_444 >> 1;
    let n_stride_420 = n_stride_444 >> 2;
    let sign_stride_444 = 16 * n_stride_444;
    let sign_stride_422 = 16 * n_stride_422;
    let sign_stride_420 = 16 * n_stride_420;
    let mut n_1 = 0;
    while n_1 < 16 {
        let sign = (signs >> n_1 & 1) as libc::c_int;
        dav1d_wedge_masks[bs as usize][0][0][n_1 as usize] =
            &mut *masks_444.offset((sign * sign_stride_444) as isize) as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][0][1][n_1 as usize] =
            &mut *masks_444.offset((sign * sign_stride_444) as isize) as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][1][0][n_1 as usize] =
            &mut *masks_422.offset((sign * sign_stride_422) as isize) as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][1][1][n_1 as usize] = &mut *masks_422
            .offset(((sign == 0) as libc::c_int * sign_stride_422) as isize)
            as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][2][0][n_1 as usize] =
            &mut *masks_420.offset((sign * sign_stride_420) as isize) as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][2][1][n_1 as usize] = &mut *masks_420
            .offset(((sign == 0) as libc::c_int * sign_stride_420) as isize)
            as *mut uint8_t;
        masks_444 = masks_444.offset(n_stride_444 as isize);
        masks_422 = masks_422.offset(n_stride_422 as isize);
        masks_420 = masks_420.offset(n_stride_420 as isize);
        init_chroma(
            dav1d_wedge_masks[bs as usize][1][0][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs as usize][0][0][n_1 as usize],
            0,
            w,
            h,
            0,
        );
        init_chroma(
            dav1d_wedge_masks[bs as usize][1][1][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs as usize][0][0][n_1 as usize],
            1,
            w,
            h,
            0,
        );
        init_chroma(
            dav1d_wedge_masks[bs as usize][2][0][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs as usize][0][0][n_1 as usize],
            0,
            w,
            h,
            1,
        );
        init_chroma(
            dav1d_wedge_masks[bs as usize][2][1][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs as usize][0][0][n_1 as usize],
            1,
            w,
            h,
            1,
        );
        n_1 += 1;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_wedge_masks() {
    static mut wedge_master_border: [[uint8_t; 8]; N_WEDGE_MASTER_LINES] = [
        [1, 2, 6, 18, 37, 53, 60, 63],
        [1, 4, 11, 27, 46, 58, 62, 63],
        [0, 2, 7, 21, 43, 57, 62, 64],
    ];
    let mut master: [[uint8_t; 4096]; 6] = [[0; 4096]; 6];
    let mut y = 0;
    let mut off = 0;
    while y < 64 {
        insert_border(
            &mut *(*master.as_mut_ptr().offset(WEDGE_VERTICAL as isize))
                .as_mut_ptr()
                .offset(off as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_VERT as usize]).as_ptr(),
            32,
        );
        y += 1;
        off += 64;
    }
    let mut y_0 = 0;
    let mut off_0 = 0;
    let mut ctr = 48;
    while y_0 < 64 {
        insert_border(
            &mut *(*master.as_mut_ptr().offset(WEDGE_OBLIQUE63 as isize))
                .as_mut_ptr()
                .offset(off_0 as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_EVEN as usize]).as_ptr(),
            ctr,
        );
        insert_border(
            &mut *(*master.as_mut_ptr().offset(WEDGE_OBLIQUE63 as isize))
                .as_mut_ptr()
                .offset((off_0 + 64) as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_ODD as usize]).as_ptr(),
            ctr - 1,
        );
        y_0 += 2;
        off_0 += 128;
        ctr -= 1;
    }
    transpose(
        (master[WEDGE_OBLIQUE27 as usize]).as_mut_ptr(),
        (master[WEDGE_OBLIQUE63 as usize]).as_mut_ptr(),
    );
    transpose(
        (master[WEDGE_HORIZONTAL as usize]).as_mut_ptr(),
        (master[WEDGE_VERTICAL as usize]).as_mut_ptr(),
    );
    hflip(
        (master[WEDGE_OBLIQUE117 as usize]).as_mut_ptr(),
        (master[WEDGE_OBLIQUE63 as usize]).as_mut_ptr(),
    );
    hflip(
        (master[WEDGE_OBLIQUE153 as usize]).as_mut_ptr(),
        (master[WEDGE_OBLIQUE27 as usize]).as_mut_ptr(),
    );
    fill2d_16x2(
        wedge_masks_444_32x32.0.as_mut_ptr(),
        32,
        32,
        BS_32x32,
        master.as_mut_ptr() as *const [uint8_t; 4096],
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
        master.as_mut_ptr() as *const [uint8_t; 4096],
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
        master.as_mut_ptr() as *const [uint8_t; 4096],
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
        master.as_mut_ptr() as *const [uint8_t; 4096],
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
        master.as_mut_ptr() as *const [uint8_t; 4096],
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
        master.as_mut_ptr() as *const [uint8_t; 4096],
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
        master.as_mut_ptr() as *const [uint8_t; 4096],
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
        master.as_mut_ptr() as *const [uint8_t; 4096],
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
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_8x8.0.as_mut_ptr(),
        wedge_masks_422_4x8.0.as_mut_ptr(),
        wedge_masks_420_4x4.0.as_mut_ptr(),
        0x7bfb,
    );
}
static mut ii_dc_mask: Align64<[uint8_t; 1024]> = Align64([0; 1024]);
static mut ii_nondc_mask_32x32: Align64<[[uint8_t; 1024]; 3]> = Align64([[0; 1024]; 3]);
static mut ii_nondc_mask_16x32: Align64<[[uint8_t; 512]; 3]> = Align64([[0; 512]; 3]);
static mut ii_nondc_mask_16x16: Align64<[[uint8_t; 256]; 3]> = Align64([[0; 256]; 3]);
static mut ii_nondc_mask_8x32: Align64<[[uint8_t; 256]; 3]> = Align64([[0; 256]; 3]);
static mut ii_nondc_mask_8x16: Align64<[[uint8_t; 128]; 3]> = Align64([[0; 128]; 3]);
static mut ii_nondc_mask_8x8: Align64<[[uint8_t; 64]; 3]> = Align64([[0; 64]; 3]);
static mut ii_nondc_mask_4x16: Align64<[[uint8_t; 64]; 3]> = Align64([[0; 64]; 3]);
static mut ii_nondc_mask_4x8: Align32<[[uint8_t; 32]; 3]> = Align32([[0; 32]; 3]);
static mut ii_nondc_mask_4x4: Align16<[[uint8_t; 16]; 3]> = Align16([[0; 16]; 3]);
#[no_mangle]
pub static mut dav1d_ii_masks: [[[*const uint8_t; N_INTER_INTRA_PRED_MODES]; 3]; N_BS_SIZES] =
    [[[0 as *const uint8_t; 4]; 3]; 22];
#[cold]
unsafe extern "C" fn build_nondc_ii_masks(
    mask_v: *mut uint8_t,
    mask_h: *mut uint8_t,
    mask_sm: *mut uint8_t,
    w: libc::c_int,
    h: libc::c_int,
    step: libc::c_int,
) {
    static mut ii_weights_1d: [uint8_t; 32] = [
        60, 52, 45, 39, 34, 30, 26, 22, 19, 17, 15, 13, 11, 10, 8, 7, 6, 6, 5, 4, 4, 3, 3, 2, 2, 2,
        2, 1, 1, 1, 1, 1,
    ];
    let mut y = 0;
    let mut off = 0;
    while y < h {
        memset(
            &mut *mask_v.offset(off as isize) as *mut uint8_t as *mut libc::c_void,
            ii_weights_1d[(y * step) as usize] as libc::c_int,
            w as libc::c_ulong,
        );
        let mut x = 0;
        while x < w {
            *mask_sm.offset((off + x) as isize) = ii_weights_1d[(cmp::min(x, y) * step) as usize];
            *mask_h.offset((off + x) as isize) = ii_weights_1d[(x * step) as usize];
            x += 1;
        }
        y += 1;
        off += w;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_interintra_masks() {
    memset(ii_dc_mask.0.as_mut_ptr() as *mut libc::c_void, 32, 32 * 32);
    build_nondc_ii_masks(
        (ii_nondc_mask_32x32[II_VERT_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_32x32[II_HOR_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_32x32[II_SMOOTH_PRED as usize - 1]).as_mut_ptr(),
        32,
        32,
        1,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_16x32[II_VERT_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_16x32[II_HOR_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_16x32[II_SMOOTH_PRED as usize - 1]).as_mut_ptr(),
        16,
        32,
        1,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_16x16[II_VERT_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_16x16[II_HOR_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_16x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr(),
        16,
        16,
        2,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_8x32[II_VERT_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_8x32[II_HOR_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_8x32[II_SMOOTH_PRED as usize - 1]).as_mut_ptr(),
        8,
        32,
        1,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_8x16[II_VERT_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_8x16[II_HOR_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_8x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr(),
        8,
        16,
        2,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_8x8[II_VERT_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_8x8[II_HOR_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_8x8[II_SMOOTH_PRED as usize - 1]).as_mut_ptr(),
        8,
        8,
        4,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_4x16[II_VERT_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_4x16[II_HOR_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_4x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr(),
        4,
        16,
        2,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_4x8[II_VERT_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_4x8[II_HOR_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_4x8[II_SMOOTH_PRED as usize - 1]).as_mut_ptr(),
        4,
        8,
        4,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_4x4[II_VERT_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_4x4[II_HOR_PRED as usize - 1]).as_mut_ptr(),
        (ii_nondc_mask_4x4[II_SMOOTH_PRED as usize - 1]).as_mut_ptr(),
        4,
        4,
        8,
    );
}
unsafe extern "C" fn run_static_initializers() {
    dav1d_ii_masks = [
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
        ],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x32[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x32[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x32[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
        ],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x16[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x16[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x16[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.0.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x4[II_VERT_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x4[II_HOR_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x4[II_SMOOTH_PRED as usize - 1]).as_mut_ptr() as *const uint8_t,
            ],
        ],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
