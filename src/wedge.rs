use crate::include::stdint::*;
use crate::src::align::Align64;
use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}

use crate::src::levels::II_SMOOTH_PRED;
use crate::src::levels::II_HOR_PRED;
use crate::src::levels::II_VERT_PRED;

use crate::src::levels::BlockSize;

use crate::src::levels::BS_8x8;
use crate::src::levels::BS_8x16;
use crate::src::levels::BS_8x32;

use crate::src::levels::BS_16x8;
use crate::src::levels::BS_16x16;
use crate::src::levels::BS_16x32;

use crate::src::levels::BS_32x8;
use crate::src::levels::BS_32x16;
use crate::src::levels::BS_32x32;

#[derive(Copy, Clone)]
#[repr(C)]
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
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;
static mut wedge_codebook_16_hgtw: [wedge_code_type; 16] = [
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as libc::c_int as uint8_t,
            x_offset: 2 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as libc::c_int as uint8_t,
            x_offset: 6 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as libc::c_int as uint8_t,
            x_offset: 2 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as libc::c_int as uint8_t,
            x_offset: 6 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
];
static mut wedge_codebook_16_hltw: [wedge_code_type; 16] = [
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as libc::c_int as uint8_t,
            x_offset: 2 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as libc::c_int as uint8_t,
            x_offset: 6 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as libc::c_int as uint8_t,
            x_offset: 2 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as libc::c_int as uint8_t,
            x_offset: 6 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as libc::c_int as uint8_t,
            x_offset: 2 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as libc::c_int as uint8_t,
            x_offset: 6 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
];
static mut wedge_codebook_16_heqw: [wedge_code_type; 16] = [
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as libc::c_int as uint8_t,
            x_offset: 2 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as libc::c_int as uint8_t,
            x_offset: 6 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 2 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as libc::c_int as uint8_t,
            x_offset: 4 as libc::c_int as uint8_t,
            y_offset: 6 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as libc::c_int as uint8_t,
            x_offset: 2 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as libc::c_int as uint8_t,
            x_offset: 6 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as libc::c_int as uint8_t,
            x_offset: 2 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as libc::c_int as uint8_t,
            x_offset: 6 as libc::c_int as uint8_t,
            y_offset: 4 as libc::c_int as uint8_t,
        };
        init
    },
];
static mut wedge_masks_444_32x32: Align64<[uint8_t; 32768]> = Align64([0; 32768]);
static mut wedge_masks_444_32x16: Align64<[uint8_t; 16384]> = Align64([0; 16384]);
static mut wedge_masks_444_32x8:Align64<[uint8_t; 8192]> = Align64([0; 8192]);
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
pub static mut dav1d_wedge_masks: [[[[*const uint8_t; 16]; 2]; 3]; 22] = [[[[0
    as *const uint8_t; 16]; 2]; 3]; 22];
unsafe extern "C" fn insert_border(
    dst: *mut uint8_t,
    src: *const uint8_t,
    ctr: libc::c_int,
) {
    if ctr > 4 {
        memset(
            dst as *mut libc::c_void,
            0 as libc::c_int,
            (ctr - 4) as libc::c_ulong,
        );
    }
    memcpy(
        dst
            .offset(imax(ctr, 4 as libc::c_int) as isize)
            .offset(-(4 as libc::c_int as isize)) as *mut libc::c_void,
        src.offset(imax(4 - ctr, 0 as libc::c_int) as isize)
            as *const libc::c_void,
        imin(64 - ctr, 8 as libc::c_int) as libc::c_ulong,
    );
    if ctr < 64 - 4 {
        memset(
            dst.offset(ctr as isize).offset(4)
                as *mut libc::c_void,
            64 as libc::c_int,
            (64 - 4 - ctr) as libc::c_ulong,
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
            x_off += 64 as libc::c_int;
        }
        y += 1;
        y_off += 64 as libc::c_int;
    }
}
unsafe extern "C" fn hflip(dst: *mut uint8_t, src: *const uint8_t) {
    let mut y = 0;
    let mut y_off = 0;
    while y < 64 {
        let mut x = 0;
        while x < 64 {
            *dst
                .offset(
                    (y_off + 64 - 1 - x) as isize,
                ) = *src.offset((y_off + x) as isize);
            x += 1;
        }
        y += 1;
        y_off += 64 as libc::c_int;
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
            *dst
                .offset(
                    (y_off + x) as isize,
                ) = (64 as libc::c_int
                - *src.offset((y_off + x) as isize) as libc::c_int) as uint8_t;
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
        memcpy(dst as *mut libc::c_void, src as *const libc::c_void, w as libc::c_ulong);
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
                sum
                    += *luma.offset((w + x) as isize) as libc::c_int
                        + *luma.offset((w + x + 1) as isize)
                            as libc::c_int + 1;
            }
            *chroma
                .offset(
                    (x >> 1) as isize,
                ) = (sum - sign >> 1 + ss_ver) as uint8_t;
            x += 2 as libc::c_int;
        }
        luma = luma.offset((w << ss_ver) as isize);
        chroma = chroma.offset((w >> 1) as isize);
        y += 1 + ss_ver;
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
    let mut n = 0;
    while n < 16 {
        copy2d(
            ptr,
            (*master.offset((*cb.offset(n as isize)).direction as isize)).as_ptr(),
            w,
            h,
            32 as libc::c_int
                - (w * (*cb.offset(n as isize)).x_offset as libc::c_int
                    >> 3),
            32 as libc::c_int
                - (h * (*cb.offset(n as isize)).y_offset as libc::c_int
                    >> 3),
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
        let sign = (signs >> n_1 & 1 as libc::c_uint)
            as libc::c_int;
        dav1d_wedge_masks[bs
            as usize][0 as libc::c_int
            as usize][0 as libc::c_int
            as usize][n_1
            as usize] = &mut *masks_444.offset((sign * sign_stride_444) as isize)
            as *mut uint8_t;
        dav1d_wedge_masks[bs
            as usize][0 as libc::c_int
            as usize][1 as libc::c_int
            as usize][n_1
            as usize] = &mut *masks_444.offset((sign * sign_stride_444) as isize)
            as *mut uint8_t;
        dav1d_wedge_masks[bs
            as usize][1 as libc::c_int
            as usize][0 as libc::c_int
            as usize][n_1
            as usize] = &mut *masks_422.offset((sign * sign_stride_422) as isize)
            as *mut uint8_t;
        dav1d_wedge_masks[bs
            as usize][1 as libc::c_int
            as usize][1 as libc::c_int
            as usize][n_1
            as usize] = &mut *masks_422
            .offset(((sign == 0) as libc::c_int * sign_stride_422) as isize)
            as *mut uint8_t;
        dav1d_wedge_masks[bs
            as usize][2 as libc::c_int
            as usize][0 as libc::c_int
            as usize][n_1
            as usize] = &mut *masks_420.offset((sign * sign_stride_420) as isize)
            as *mut uint8_t;
        dav1d_wedge_masks[bs
            as usize][2 as libc::c_int
            as usize][1 as libc::c_int
            as usize][n_1
            as usize] = &mut *masks_420
            .offset(((sign == 0) as libc::c_int * sign_stride_420) as isize)
            as *mut uint8_t;
        masks_444 = masks_444.offset(n_stride_444 as isize);
        masks_422 = masks_422.offset(n_stride_422 as isize);
        masks_420 = masks_420.offset(n_stride_420 as isize);
        init_chroma(
            dav1d_wedge_masks[bs
                as usize][1 as libc::c_int
                as usize][0][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs
                as usize][0 as libc::c_int
                as usize][0][n_1 as usize],
            0 as libc::c_int,
            w,
            h,
            0 as libc::c_int,
        );
        init_chroma(
            dav1d_wedge_masks[bs
                as usize][1 as libc::c_int
                as usize][1][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs
                as usize][0 as libc::c_int
                as usize][0][n_1 as usize],
            1 as libc::c_int,
            w,
            h,
            0 as libc::c_int,
        );
        init_chroma(
            dav1d_wedge_masks[bs
                as usize][2 as libc::c_int
                as usize][0][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs
                as usize][0 as libc::c_int
                as usize][0][n_1 as usize],
            0 as libc::c_int,
            w,
            h,
            1 as libc::c_int,
        );
        init_chroma(
            dav1d_wedge_masks[bs
                as usize][2 as libc::c_int
                as usize][1][n_1 as usize] as *mut uint8_t,
            dav1d_wedge_masks[bs
                as usize][0 as libc::c_int
                as usize][0][n_1 as usize],
            1 as libc::c_int,
            w,
            h,
            1 as libc::c_int,
        );
        n_1 += 1;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_wedge_masks() {
    static mut wedge_master_border: [[uint8_t; 8]; 3] = [
        [
            1 as libc::c_int as uint8_t,
            2 as libc::c_int as uint8_t,
            6 as libc::c_int as uint8_t,
            18 as libc::c_int as uint8_t,
            37 as libc::c_int as uint8_t,
            53 as libc::c_int as uint8_t,
            60 as libc::c_int as uint8_t,
            63 as libc::c_int as uint8_t,
        ],
        [
            1 as libc::c_int as uint8_t,
            4 as libc::c_int as uint8_t,
            11 as libc::c_int as uint8_t,
            27 as libc::c_int as uint8_t,
            46 as libc::c_int as uint8_t,
            58 as libc::c_int as uint8_t,
            62 as libc::c_int as uint8_t,
            63 as libc::c_int as uint8_t,
        ],
        [
            0 as libc::c_int as uint8_t,
            2 as libc::c_int as uint8_t,
            7 as libc::c_int as uint8_t,
            21 as libc::c_int as uint8_t,
            43 as libc::c_int as uint8_t,
            57 as libc::c_int as uint8_t,
            62 as libc::c_int as uint8_t,
            64 as libc::c_int as uint8_t,
        ],
    ];
    let mut master: [[uint8_t; 4096]; 6] = [[0; 4096]; 6];
    let mut y = 0;
    let mut off = 0;
    while y < 64 {
        insert_border(
            &mut *(*master.as_mut_ptr().offset(WEDGE_VERTICAL as libc::c_int as isize))
                .as_mut_ptr()
                .offset(off as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_VERT as libc::c_int as usize])
                .as_ptr(),
            32 as libc::c_int,
        );
        y += 1;
        off += 64 as libc::c_int;
    }
    let mut y_0 = 0;
    let mut off_0 = 0;
    let mut ctr = 48;
    while y_0 < 64 {
        insert_border(
            &mut *(*master.as_mut_ptr().offset(WEDGE_OBLIQUE63 as libc::c_int as isize))
                .as_mut_ptr()
                .offset(off_0 as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_EVEN as libc::c_int as usize])
                .as_ptr(),
            ctr,
        );
        insert_border(
            &mut *(*master.as_mut_ptr().offset(WEDGE_OBLIQUE63 as libc::c_int as isize))
                .as_mut_ptr()
                .offset((off_0 + 64) as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_ODD as libc::c_int as usize])
                .as_ptr(),
            ctr - 1,
        );
        y_0 += 2 as libc::c_int;
        off_0 += 128 as libc::c_int;
        ctr -= 1;
    }
    transpose(
        (master[WEDGE_OBLIQUE27 as libc::c_int as usize]).as_mut_ptr(),
        (master[WEDGE_OBLIQUE63 as libc::c_int as usize]).as_mut_ptr(),
    );
    transpose(
        (master[WEDGE_HORIZONTAL as libc::c_int as usize]).as_mut_ptr(),
        (master[WEDGE_VERTICAL as libc::c_int as usize]).as_mut_ptr(),
    );
    hflip(
        (master[WEDGE_OBLIQUE117 as libc::c_int as usize]).as_mut_ptr(),
        (master[WEDGE_OBLIQUE63 as libc::c_int as usize]).as_mut_ptr(),
    );
    hflip(
        (master[WEDGE_OBLIQUE153 as libc::c_int as usize]).as_mut_ptr(),
        (master[WEDGE_OBLIQUE27 as libc::c_int as usize]).as_mut_ptr(),
    );
    fill2d_16x2(
        wedge_masks_444_32x32.0.as_mut_ptr(),
        32 as libc::c_int,
        32 as libc::c_int,
        BS_32x32,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_32x32.0.as_mut_ptr(),
        wedge_masks_422_16x32.0.as_mut_ptr(),
        wedge_masks_420_16x16.0.as_mut_ptr(),
        0x7bfb as libc::c_int as libc::c_uint,
    );
    fill2d_16x2(
        wedge_masks_444_32x16.0.as_mut_ptr(),
        32 as libc::c_int,
        16 as libc::c_int,
        BS_32x16,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hltw.as_ptr(),
        wedge_masks_444_32x16.0.as_mut_ptr(),
        wedge_masks_422_16x16.0.as_mut_ptr(),
        wedge_masks_420_16x8.0.as_mut_ptr(),
        0x7beb as libc::c_int as libc::c_uint,
    );
    fill2d_16x2(
        wedge_masks_444_32x8.0.as_mut_ptr(),
        32 as libc::c_int,
        8 as libc::c_int,
        BS_32x8,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hltw.as_ptr(),
        wedge_masks_444_32x8.0.as_mut_ptr(),
        wedge_masks_422_16x8.0.as_mut_ptr(),
        wedge_masks_420_16x4.0.as_mut_ptr(),
        0x6beb as libc::c_int as libc::c_uint,
    );
    fill2d_16x2(
        wedge_masks_444_16x32.0.as_mut_ptr(),
        16 as libc::c_int,
        32 as libc::c_int,
        BS_16x32,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hgtw.as_ptr(),
        wedge_masks_444_16x32.0.as_mut_ptr(),
        wedge_masks_422_8x32.0.as_mut_ptr(),
        wedge_masks_420_8x16.0.as_mut_ptr(),
        0x7beb as libc::c_int as libc::c_uint,
    );
    fill2d_16x2(
        wedge_masks_444_16x16.0.as_mut_ptr(),
        16 as libc::c_int,
        16 as libc::c_int,
        BS_16x16,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_16x16.0.as_mut_ptr(),
        wedge_masks_422_8x16.0.as_mut_ptr(),
        wedge_masks_420_8x8.0.as_mut_ptr(),
        0x7bfb as libc::c_int as libc::c_uint,
    );
    fill2d_16x2(
        wedge_masks_444_16x8.0.as_mut_ptr(),
        16 as libc::c_int,
        8 as libc::c_int,
        BS_16x8,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hltw.as_ptr(),
        wedge_masks_444_16x8.0.as_mut_ptr(),
        wedge_masks_422_8x8.0.as_mut_ptr(),
        wedge_masks_420_8x4.0.as_mut_ptr(),
        0x7beb as libc::c_int as libc::c_uint,
    );
    fill2d_16x2(
        wedge_masks_444_8x32.0.as_mut_ptr(),
        8 as libc::c_int,
        32 as libc::c_int,
        BS_8x32,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hgtw.as_ptr(),
        wedge_masks_444_8x32.0.as_mut_ptr(),
        wedge_masks_422_4x32.0.as_mut_ptr(),
        wedge_masks_420_4x16.0.as_mut_ptr(),
        0x7aeb as libc::c_int as libc::c_uint,
    );
    fill2d_16x2(
        wedge_masks_444_8x16.0.as_mut_ptr(),
        8 as libc::c_int,
        16 as libc::c_int,
        BS_8x16,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_hgtw.as_ptr(),
        wedge_masks_444_8x16.0.as_mut_ptr(),
        wedge_masks_422_4x16.0.as_mut_ptr(),
        wedge_masks_420_4x8.0.as_mut_ptr(),
        0x7beb as libc::c_int as libc::c_uint,
    );
    fill2d_16x2(
        wedge_masks_444_8x8.0.as_mut_ptr(),
        8 as libc::c_int,
        8 as libc::c_int,
        BS_8x8,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        wedge_codebook_16_heqw.as_ptr(),
        wedge_masks_444_8x8.0.as_mut_ptr(),
        wedge_masks_422_4x8.0.as_mut_ptr(),
        wedge_masks_420_4x4.0.as_mut_ptr(),
        0x7bfb as libc::c_int as libc::c_uint,
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
pub static mut dav1d_ii_masks: [[[*const uint8_t; 4]; 3]; 22] = [[[0
    as *const uint8_t; 4]; 3]; 22];
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
        60 as libc::c_int as uint8_t,
        52 as libc::c_int as uint8_t,
        45 as libc::c_int as uint8_t,
        39 as libc::c_int as uint8_t,
        34 as libc::c_int as uint8_t,
        30 as libc::c_int as uint8_t,
        26 as libc::c_int as uint8_t,
        22 as libc::c_int as uint8_t,
        19 as libc::c_int as uint8_t,
        17 as libc::c_int as uint8_t,
        15 as libc::c_int as uint8_t,
        13 as libc::c_int as uint8_t,
        11 as libc::c_int as uint8_t,
        10 as libc::c_int as uint8_t,
        8 as libc::c_int as uint8_t,
        7 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        6 as libc::c_int as uint8_t,
        5 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        4 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        3 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        2 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
        1 as libc::c_int as uint8_t,
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
            *mask_sm
                .offset(
                    (off + x) as isize,
                ) = ii_weights_1d[(imin(x, y) * step) as usize];
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
        32 as libc::c_int,
        (32 * 32) as libc::c_ulong,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_32x32[(II_VERT_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_32x32[(II_HOR_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_32x32[(II_SMOOTH_PRED as libc::c_int - 1)
            as usize])
            .as_mut_ptr(),
        32 as libc::c_int,
        32 as libc::c_int,
        1 as libc::c_int,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_16x32[(II_VERT_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_16x32[(II_HOR_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_16x32[(II_SMOOTH_PRED as libc::c_int - 1)
            as usize])
            .as_mut_ptr(),
        16 as libc::c_int,
        32 as libc::c_int,
        1 as libc::c_int,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1)
            as usize])
            .as_mut_ptr(),
        16 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_8x32[(II_VERT_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_8x32[(II_HOR_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_8x32[(II_SMOOTH_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        8 as libc::c_int,
        32 as libc::c_int,
        1 as libc::c_int,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_8x16[(II_VERT_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_8x16[(II_HOR_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_8x16[(II_SMOOTH_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        8 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        8 as libc::c_int,
        8 as libc::c_int,
        4 as libc::c_int,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_4x16[(II_VERT_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_4x16[(II_HOR_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_4x16[(II_SMOOTH_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        4 as libc::c_int,
        16 as libc::c_int,
        2 as libc::c_int,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_4x8[(II_VERT_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_4x8[(II_HOR_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_4x8[(II_SMOOTH_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        4 as libc::c_int,
        8 as libc::c_int,
        4 as libc::c_int,
    );
    build_nondc_ii_masks(
        (ii_nondc_mask_4x4[(II_VERT_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_4x4[(II_HOR_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        (ii_nondc_mask_4x4[(II_SMOOTH_PRED as libc::c_int - 1) as usize])
            .as_mut_ptr(),
        4 as libc::c_int,
        4 as libc::c_int,
        8 as libc::c_int,
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
                (ii_nondc_mask_32x32[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_32x32[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
        ],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x32[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x32[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x32[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x32[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_16x16[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
        ],
        [[0 as *const uint8_t; 4]; 3],
        [[0 as *const uint8_t; 4]; 3],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x16[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x16[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x16[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x16[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
        ],
        [
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_8x8[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x8[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
            ],
            [
                ii_dc_mask.as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x4[(II_VERT_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x4[(II_HOR_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
                (ii_nondc_mask_4x4[(II_SMOOTH_PRED as libc::c_int - 1)
                    as usize])
                    .as_mut_ptr() as *const uint8_t,
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
