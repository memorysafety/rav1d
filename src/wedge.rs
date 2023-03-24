use ::libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type InterIntraPredMode = libc::c_uint;
pub const N_INTER_INTRA_PRED_MODES: InterIntraPredMode = 4;
pub const II_SMOOTH_PRED: InterIntraPredMode = 3;
pub const II_HOR_PRED: InterIntraPredMode = 2;
pub const II_VERT_PRED: InterIntraPredMode = 1;
pub const II_DC_PRED: InterIntraPredMode = 0;
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
pub struct wedge_code_type {
    pub direction: uint8_t,
    pub x_offset: uint8_t,
    pub y_offset: uint8_t,
}
pub const WEDGE_OBLIQUE117: WedgeDirectionType = 4;
pub const WEDGE_OBLIQUE63: WedgeDirectionType = 3;
pub const WEDGE_OBLIQUE153: WedgeDirectionType = 5;
pub const WEDGE_OBLIQUE27: WedgeDirectionType = 2;
pub const WEDGE_VERTICAL: WedgeDirectionType = 1;
pub const WEDGE_HORIZONTAL: WedgeDirectionType = 0;
pub const WEDGE_MASTER_LINE_ODD: WedgeMasterLineType = 0;
pub const WEDGE_MASTER_LINE_EVEN: WedgeMasterLineType = 1;
pub const WEDGE_MASTER_LINE_VERT: WedgeMasterLineType = 2;
pub type WedgeMasterLineType = libc::c_uint;
pub const N_WEDGE_MASTER_LINES: WedgeMasterLineType = 3;
pub type WedgeDirectionType = libc::c_uint;
pub const N_WEDGE_DIRECTIONS: WedgeDirectionType = 6;
#[inline]
unsafe extern "C" fn imax(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
static mut wedge_codebook_16_hgtw: [wedge_code_type; 16] = [
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as uint8_t,
            x_offset: 4u8,
            y_offset: 2u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as uint8_t,
            x_offset: 4u8,
            y_offset: 6u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as uint8_t,
            x_offset: 4u8,
            y_offset: 2u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as uint8_t,
            x_offset: 4u8,
            y_offset: 6u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as uint8_t,
            x_offset: 4u8,
            y_offset: 2u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as uint8_t,
            x_offset: 4u8,
            y_offset: 6u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as uint8_t,
            x_offset: 2u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as uint8_t,
            x_offset: 6u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as uint8_t,
            x_offset: 2u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as uint8_t,
            x_offset: 6u8,
            y_offset: 4u8,
        };
        init
    },
];
static mut wedge_codebook_16_hltw: [wedge_code_type; 16] = [
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as uint8_t,
            x_offset: 2u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as uint8_t,
            x_offset: 6u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as uint8_t,
            x_offset: 4u8,
            y_offset: 2u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as uint8_t,
            x_offset: 4u8,
            y_offset: 6u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as uint8_t,
            x_offset: 4u8,
            y_offset: 2u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as uint8_t,
            x_offset: 4u8,
            y_offset: 6u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as uint8_t,
            x_offset: 2u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as uint8_t,
            x_offset: 6u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as uint8_t,
            x_offset: 2u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as uint8_t,
            x_offset: 6u8,
            y_offset: 4u8,
        };
        init
    },
];
static mut wedge_codebook_16_heqw: [wedge_code_type; 16] = [
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as uint8_t,
            x_offset: 4u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as uint8_t,
            x_offset: 4u8,
            y_offset: 2u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as uint8_t,
            x_offset: 4u8,
            y_offset: 6u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as uint8_t,
            x_offset: 2u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as uint8_t,
            x_offset: 6u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as uint8_t,
            x_offset: 4u8,
            y_offset: 2u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as uint8_t,
            x_offset: 4u8,
            y_offset: 6u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as uint8_t,
            x_offset: 4u8,
            y_offset: 2u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as uint8_t,
            x_offset: 4u8,
            y_offset: 6u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as uint8_t,
            x_offset: 2u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as uint8_t,
            x_offset: 6u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as uint8_t,
            x_offset: 2u8,
            y_offset: 4u8,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as uint8_t,
            x_offset: 6u8,
            y_offset: 4u8,
        };
        init
    },
];
static mut wedge_masks_444_32x32: [uint8_t; 32768] = [0; 32768];
static mut wedge_masks_444_32x16: [uint8_t; 16384] = [0; 16384];
static mut wedge_masks_444_32x8: [uint8_t; 8192] = [0; 8192];
static mut wedge_masks_444_16x32: [uint8_t; 16384] = [0; 16384];
static mut wedge_masks_444_16x16: [uint8_t; 8192] = [0; 8192];
static mut wedge_masks_444_16x8: [uint8_t; 4096] = [0; 4096];
static mut wedge_masks_444_8x32: [uint8_t; 8192] = [0; 8192];
static mut wedge_masks_444_8x16: [uint8_t; 4096] = [0; 4096];
static mut wedge_masks_444_8x8: [uint8_t; 2048] = [0; 2048];
static mut wedge_masks_422_16x32: [uint8_t; 16384] = [0; 16384];
static mut wedge_masks_422_16x16: [uint8_t; 8192] = [0; 8192];
static mut wedge_masks_422_16x8: [uint8_t; 4096] = [0; 4096];
static mut wedge_masks_422_8x32: [uint8_t; 8192] = [0; 8192];
static mut wedge_masks_422_8x16: [uint8_t; 4096] = [0; 4096];
static mut wedge_masks_422_8x8: [uint8_t; 2048] = [0; 2048];
static mut wedge_masks_422_4x32: [uint8_t; 4096] = [0; 4096];
static mut wedge_masks_422_4x16: [uint8_t; 2048] = [0; 2048];
static mut wedge_masks_422_4x8: [uint8_t; 1024] = [0; 1024];
static mut wedge_masks_420_16x16: [uint8_t; 8192] = [0; 8192];
static mut wedge_masks_420_16x8: [uint8_t; 4096] = [0; 4096];
static mut wedge_masks_420_16x4: [uint8_t; 2048] = [0; 2048];
static mut wedge_masks_420_8x16: [uint8_t; 4096] = [0; 4096];
static mut wedge_masks_420_8x8: [uint8_t; 2048] = [0; 2048];
static mut wedge_masks_420_8x4: [uint8_t; 1024] = [0; 1024];
static mut wedge_masks_420_4x16: [uint8_t; 2048] = [0; 2048];
static mut wedge_masks_420_4x8: [uint8_t; 1024] = [0; 1024];
static mut wedge_masks_420_4x4: [uint8_t; 512] = [0; 512];
#[no_mangle]
pub static mut dav1d_wedge_masks: [[[[*const uint8_t; 16]; 2]; 3]; 22] =
    [[[[0 as *const uint8_t; 16]; 2]; 3]; 22];
unsafe extern "C" fn insert_border(dst: *mut uint8_t, src: *const uint8_t, ctr: libc::c_int) {
    if ctr > 4i32 {
        memset(
            dst as *mut libc::c_void,
            0i32,
            (ctr - 4i32) as libc::c_ulong,
        );
    }
    memcpy(
        dst.offset(imax(ctr, 4i32) as isize).offset(-(4isize)) as *mut libc::c_void,
        src.offset(imax(4i32 - ctr, 0i32) as isize) as *const libc::c_void,
        imin(64i32 - ctr, 8i32) as libc::c_ulong,
    );
    if ctr < 64i32 - 4i32 {
        memset(
            dst.offset(ctr as isize).offset(4isize) as *mut libc::c_void,
            64i32,
            (64i32 - 4i32 - ctr) as libc::c_ulong,
        );
    }
}
unsafe extern "C" fn transpose(dst: *mut uint8_t, src: *const uint8_t) {
    let mut y: libc::c_int = 0i32;
    let mut y_off: libc::c_int = 0i32;
    while y < 64i32 {
        let mut x: libc::c_int = 0i32;
        let mut x_off: libc::c_int = 0i32;
        while x < 64i32 {
            *dst.offset((x_off + y) as isize) = *src.offset((y_off + x) as isize);
            x += 1;
            x_off += 64i32;
        }
        y += 1;
        y_off += 64i32;
    }
}
unsafe extern "C" fn hflip(dst: *mut uint8_t, src: *const uint8_t) {
    let mut y: libc::c_int = 0i32;
    let mut y_off: libc::c_int = 0i32;
    while y < 64i32 {
        let mut x: libc::c_int = 0i32;
        while x < 64i32 {
            *dst.offset((y_off + 64i32 - 1i32 - x) as isize) = *src.offset((y_off + x) as isize);
            x += 1;
        }
        y += 1;
        y_off += 64i32;
    }
}
unsafe extern "C" fn invert(
    dst: *mut uint8_t,
    src: *const uint8_t,
    w: libc::c_int,
    h: libc::c_int,
) {
    let mut y: libc::c_int = 0i32;
    let mut y_off: libc::c_int = 0i32;
    while y < h {
        let mut x: libc::c_int = 0i32;
        while x < w {
            *dst.offset((y_off + x) as isize) =
                (64i32 - *src.offset((y_off + x) as isize) as libc::c_int) as uint8_t;
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
    src = src.offset((y_off * 64i32 + x_off) as isize);
    let mut y: libc::c_int = 0i32;
    while y < h {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            w as libc::c_ulong,
        );
        src = src.offset(64isize);
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
    let mut y: libc::c_int = 0i32;
    while y < h {
        let mut x: libc::c_int = 0i32;
        while x < w {
            let mut sum: libc::c_int = *luma.offset(x as isize) as libc::c_int
                + *luma.offset((x + 1i32) as isize) as libc::c_int
                + 1i32;
            if ss_ver != 0 {
                sum += *luma.offset((w + x) as isize) as libc::c_int
                    + *luma.offset((w + x + 1i32) as isize) as libc::c_int
                    + 1i32;
            }
            *chroma.offset((x >> 1i32) as isize) = (sum - sign >> 1i32 + ss_ver) as uint8_t;
            x += 2i32;
        }
        luma = luma.offset((w << ss_ver) as isize);
        chroma = chroma.offset((w >> 1i32) as isize);
        y += 1i32 + ss_ver;
    }
}
#[cold]
unsafe extern "C" fn fill2d_16x2(
    mut dst: *mut uint8_t,
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
    let mut n: libc::c_int = 0i32;
    while n < 16i32 {
        copy2d(
            ptr,
            (*master.offset((*cb.offset(n as isize)).direction as isize)).as_ptr(),
            w,
            h,
            32i32 - (w * (*cb.offset(n as isize)).x_offset as libc::c_int >> 3i32),
            32i32 - (h * (*cb.offset(n as isize)).y_offset as libc::c_int >> 3i32),
        );
        ptr = ptr.offset((w * h) as isize);
        n += 1;
    }
    let mut n_0: libc::c_int = 0i32;
    let mut off: libc::c_int = 0i32;
    while n_0 < 16i32 {
        invert(ptr.offset(off as isize), dst.offset(off as isize), w, h);
        n_0 += 1;
        off += w * h;
    }
    let n_stride_444: libc::c_int = w * h;
    let n_stride_422: libc::c_int = n_stride_444 >> 1i32;
    let n_stride_420: libc::c_int = n_stride_444 >> 2i32;
    let sign_stride_444: libc::c_int = 16i32 * n_stride_444;
    let sign_stride_422: libc::c_int = 16i32 * n_stride_422;
    let sign_stride_420: libc::c_int = 16i32 * n_stride_420;
    let mut n_1: libc::c_int = 0i32;
    while n_1 < 16i32 {
        let sign: libc::c_int = (signs >> n_1 & 1u32) as libc::c_int;
        dav1d_wedge_masks[bs as usize][0usize][0usize][n_1 as usize] =
            &mut *masks_444.offset((sign * sign_stride_444) as isize) as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][0usize][1usize][n_1 as usize] =
            &mut *masks_444.offset((sign * sign_stride_444) as isize) as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][1usize][0usize][n_1 as usize] =
            &mut *masks_422.offset((sign * sign_stride_422) as isize) as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][1usize][1usize][n_1 as usize] = &mut *masks_422
            .offset(((sign == 0) as libc::c_int * sign_stride_422) as isize)
            as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][2usize][0usize][n_1 as usize] =
            &mut *masks_420.offset((sign * sign_stride_420) as isize) as *mut uint8_t;
        dav1d_wedge_masks[bs as usize][2usize][1usize][n_1 as usize] = &mut *masks_420
            .offset(((sign == 0) as libc::c_int * sign_stride_420) as isize)
            as *mut uint8_t;
        masks_444 = masks_444.offset(n_stride_444 as isize);
        masks_422 = masks_422.offset(n_stride_422 as isize);
        masks_420 = masks_420.offset(n_stride_420 as isize);
        init_chroma(
            dav1d_wedge_masks[bs as usize][1usize][0usize][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs as usize][0usize][0usize][n_1 as usize],
            0i32,
            w,
            h,
            0i32,
        );
        init_chroma(
            dav1d_wedge_masks[bs as usize][1usize][1usize][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs as usize][0usize][0usize][n_1 as usize],
            1i32,
            w,
            h,
            0i32,
        );
        init_chroma(
            dav1d_wedge_masks[bs as usize][2usize][0usize][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs as usize][0usize][0usize][n_1 as usize],
            0i32,
            w,
            h,
            1i32,
        );
        init_chroma(
            dav1d_wedge_masks[bs as usize][2usize][1usize][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs as usize][0usize][0usize][n_1 as usize],
            1i32,
            w,
            h,
            1i32,
        );
        n_1 += 1;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_wedge_masks() {
    static mut wedge_master_border: [[uint8_t; 8]; 3] = [
        [1u8, 2u8, 6u8, 18u8, 37u8, 53u8, 60u8, 63u8],
        [1u8, 4u8, 11u8, 27u8, 46u8, 58u8, 62u8, 63u8],
        [0u8, 2u8, 7u8, 21u8, 43u8, 57u8, 62u8, 64u8],
    ];
    let mut master: [[uint8_t; 4096]; 6] = [[0; 4096]; 6];
    let mut y: libc::c_int = 0i32;
    let mut off: libc::c_int = 0i32;
    while y < 64i32 {
        insert_border(
            &mut *(*master
                .as_mut_ptr()
                .offset(WEDGE_VERTICAL as libc::c_int as isize))
            .as_mut_ptr()
            .offset(off as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_VERT as usize]).as_ptr(),
            32i32,
        );
        y += 1;
        off += 64i32;
    }
    let mut y_0: libc::c_int = 0i32;
    let mut off_0: libc::c_int = 0i32;
    let mut ctr: libc::c_int = 48i32;
    while y_0 < 64i32 {
        insert_border(
            &mut *(*master
                .as_mut_ptr()
                .offset(WEDGE_OBLIQUE63 as libc::c_int as isize))
            .as_mut_ptr()
            .offset(off_0 as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_EVEN as usize]).as_ptr(),
            ctr,
        );
        insert_border(
            &mut *(*master
                .as_mut_ptr()
                .offset(WEDGE_OBLIQUE63 as libc::c_int as isize))
            .as_mut_ptr()
            .offset((off_0 + 64i32) as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_ODD as usize]).as_ptr(),
            ctr - 1i32,
        );
        y_0 += 2i32;
        off_0 += 128i32;
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
        wedge_masks_444_32x32.as_mut_ptr(),
        32i32,
        32i32,
        BS_32x32,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_32x32.as_mut_ptr(),
        wedge_masks_422_16x32.as_mut_ptr(),
        wedge_masks_420_16x16.as_mut_ptr(),
        0x7bfbu32,
    );
    fill2d_16x2(
        wedge_masks_444_32x16.as_mut_ptr(),
        32i32,
        16i32,
        BS_32x16,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hltw.as_ptr(),
        wedge_masks_444_32x16.as_mut_ptr(),
        wedge_masks_422_16x16.as_mut_ptr(),
        wedge_masks_420_16x8.as_mut_ptr(),
        0x7bebu32,
    );
    fill2d_16x2(
        wedge_masks_444_32x8.as_mut_ptr(),
        32i32,
        8i32,
        BS_32x8,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hltw.as_ptr(),
        wedge_masks_444_32x8.as_mut_ptr(),
        wedge_masks_422_16x8.as_mut_ptr(),
        wedge_masks_420_16x4.as_mut_ptr(),
        0x6bebu32,
    );
    fill2d_16x2(
        wedge_masks_444_16x32.as_mut_ptr(),
        16i32,
        32i32,
        BS_16x32,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hgtw.as_ptr(),
        wedge_masks_444_16x32.as_mut_ptr(),
        wedge_masks_422_8x32.as_mut_ptr(),
        wedge_masks_420_8x16.as_mut_ptr(),
        0x7bebu32,
    );
    fill2d_16x2(
        wedge_masks_444_16x16.as_mut_ptr(),
        16i32,
        16i32,
        BS_16x16,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_16x16.as_mut_ptr(),
        wedge_masks_422_8x16.as_mut_ptr(),
        wedge_masks_420_8x8.as_mut_ptr(),
        0x7bfbu32,
    );
    fill2d_16x2(
        wedge_masks_444_16x8.as_mut_ptr(),
        16i32,
        8i32,
        BS_16x8,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hltw.as_ptr(),
        wedge_masks_444_16x8.as_mut_ptr(),
        wedge_masks_422_8x8.as_mut_ptr(),
        wedge_masks_420_8x4.as_mut_ptr(),
        0x7bebu32,
    );
    fill2d_16x2(
        wedge_masks_444_8x32.as_mut_ptr(),
        8i32,
        32i32,
        BS_8x32,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hgtw.as_ptr(),
        wedge_masks_444_8x32.as_mut_ptr(),
        wedge_masks_422_4x32.as_mut_ptr(),
        wedge_masks_420_4x16.as_mut_ptr(),
        0x7aebu32,
    );
    fill2d_16x2(
        wedge_masks_444_8x16.as_mut_ptr(),
        8i32,
        16i32,
        BS_8x16,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hgtw.as_ptr(),
        wedge_masks_444_8x16.as_mut_ptr(),
        wedge_masks_422_4x16.as_mut_ptr(),
        wedge_masks_420_4x8.as_mut_ptr(),
        0x7bebu32,
    );
    fill2d_16x2(
        wedge_masks_444_8x8.as_mut_ptr(),
        8i32,
        8i32,
        BS_8x8,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_8x8.as_mut_ptr(),
        wedge_masks_422_4x8.as_mut_ptr(),
        wedge_masks_420_4x4.as_mut_ptr(),
        0x7bfbu32,
    );
}
static mut ii_dc_mask: [uint8_t; 1024] = [0; 1024];
static mut ii_nondc_mask_32x32: [[uint8_t; 1024]; 3] = [[0; 1024]; 3];
static mut ii_nondc_mask_16x32: [[uint8_t; 512]; 3] = [[0; 512]; 3];
static mut ii_nondc_mask_16x16: [[uint8_t; 256]; 3] = [[0; 256]; 3];
static mut ii_nondc_mask_8x32: [[uint8_t; 256]; 3] = [[0; 256]; 3];
static mut ii_nondc_mask_8x16: [[uint8_t; 128]; 3] = [[0; 128]; 3];
static mut ii_nondc_mask_8x8: [[uint8_t; 64]; 3] = [[0; 64]; 3];
static mut ii_nondc_mask_4x16: [[uint8_t; 64]; 3] = [[0; 64]; 3];
static mut ii_nondc_mask_4x8: [[uint8_t; 32]; 3] = [[0; 32]; 3];
static mut ii_nondc_mask_4x4: [[uint8_t; 16]; 3] = [[0; 16]; 3];
#[no_mangle]
pub static mut dav1d_ii_masks: [[[*const uint8_t; 4]; 3]; 22] = [[[0 as *const uint8_t; 4]; 3]; 22];
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
        60u8, 52u8, 45u8, 39u8, 34u8, 30u8, 26u8, 22u8, 19u8, 17u8, 15u8, 13u8, 11u8, 10u8, 8u8,
        7u8, 6u8, 6u8, 5u8, 4u8, 4u8, 3u8, 3u8, 2u8, 2u8, 2u8, 2u8, 1u8, 1u8, 1u8, 1u8, 1u8,
    ];
    let mut y: libc::c_int = 0i32;
    let mut off: libc::c_int = 0i32;
    while y < h {
        memset(
            &mut *mask_v.offset(off as isize) as *mut uint8_t as *mut libc::c_void,
            ii_weights_1d[(y * step) as usize] as libc::c_int,
            w as libc::c_ulong,
        );
        let mut x: libc::c_int = 0i32;
        while x < w {
            *mask_sm.offset((off + x) as isize) = ii_weights_1d[(imin(x, y) * step) as usize];
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
    memset(
        ii_dc_mask.as_mut_ptr() as *mut libc::c_void,
        32i32,
        (32i32 * 32i32) as libc::c_ulong,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_32x32[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_32x32[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_32x32[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        32i32,
        32i32,
        1i32,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_16x32[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_16x32[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_16x32[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        16i32,
        32i32,
        1i32,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        16i32,
        16i32,
        2i32,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_8x32[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_8x32[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_8x32[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        8i32,
        32i32,
        1i32,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_8x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_8x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_8x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        8i32,
        16i32,
        2i32,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        8i32,
        8i32,
        4i32,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_4x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_4x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_4x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        4i32,
        16i32,
        2i32,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_4x8[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_4x8[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_4x8[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        4i32,
        8i32,
        4i32,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_4x4[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_4x4[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        (ii_nondc_mask_4x4[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr(),
        4i32,
        4i32,
        8i32,
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
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_32x32[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_32x32[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x32[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x32[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_32x32[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_32x32[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
        ],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x32[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x32[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x32[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x32[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x32[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
        ],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x16[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_4x16[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_4x16[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_4x8[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_4x8[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_4x8[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_4x8[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x4[(II_VERT_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_4x4[(II_HOR_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
                (ii_nondc_mask_4x4[(II_SMOOTH_PRED as libc::c_int - 1i32) as usize]).as_mut_ptr()
                    as *const uint8_t,
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
