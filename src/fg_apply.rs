use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BPC;
use libc::memset;
use std::ffi::c_int;
use std::ffi::c_void;

// TODO(kkysen) temporarily pub until mod is deduplicated
pub(crate) unsafe fn generate_scaling<BD: BitDepth>(
    bitdepth: c_int,
    points: *const [u8; 2],
    num: c_int,
    scaling: *mut u8,
) {
    let (shift_x, scaling_size) = match BD::BPC {
        BPC::BPC8 => (0, 256),
        BPC::BPC16 => {
            if !(bitdepth > 8) {
                unreachable!();
            }
            let shift_x = bitdepth - 8;
            let scaling_size = (1 as c_int) << bitdepth;
            (shift_x, scaling_size)
        }
    };
    if num == 0 {
        memset(scaling as *mut c_void, 0 as c_int, scaling_size as usize);
        return;
    }
    memset(
        scaling as *mut c_void,
        (*points.offset(0))[1] as c_int,
        (((*points.offset(0))[0] as c_int) << shift_x) as usize,
    );
    let mut i = 0;
    while i < num - 1 {
        let bx = (*points.offset(i as isize))[0] as c_int;
        let by = (*points.offset(i as isize))[1] as c_int;
        let ex = (*points.offset((i + 1) as isize))[0] as c_int;
        let ey = (*points.offset((i + 1) as isize))[1] as c_int;
        let dx = ex - bx;
        let dy = ey - by;
        if !(dx > 0) {
            unreachable!();
        }
        let delta = dy * ((0x10000 + (dx >> 1)) / dx);
        let mut x = 0;
        let mut d = 0x8000 as c_int;
        while x < dx {
            *scaling.offset((bx + x << shift_x) as isize) = (by + (d >> 16)) as u8;
            d += delta;
            x += 1;
        }
        i += 1;
    }
    let n = ((*points.offset((num - 1) as isize))[0] as c_int) << shift_x;
    memset(
        &mut *scaling.offset(n as isize) as *mut u8 as *mut c_void,
        (*points.offset((num - 1) as isize))[1] as c_int,
        (scaling_size - n) as usize,
    );

    if BD::BPC != BPC::BPC8 {
        let pad = (1 as c_int) << shift_x;
        let rnd = pad >> 1;
        let mut i_0 = 0;
        while i_0 < num - 1 {
            let bx_0 = ((*points.offset(i_0 as isize))[0] as c_int) << shift_x;
            let ex_0 = ((*points.offset((i_0 + 1) as isize))[0] as c_int) << shift_x;
            let dx_0 = ex_0 - bx_0;
            let mut x_0 = 0;
            while x_0 < dx_0 {
                let range = *scaling.offset((bx_0 + x_0 + pad) as isize) as c_int
                    - *scaling.offset((bx_0 + x_0) as isize) as c_int;
                let mut n_0 = 1;
                let mut r = rnd;
                while n_0 < pad {
                    r += range;
                    *scaling.offset((bx_0 + x_0 + n_0) as isize) =
                        (*scaling.offset((bx_0 + x_0) as isize) as c_int + (r >> shift_x)) as u8;
                    n_0 += 1;
                }
                x_0 += pad;
            }
            i_0 += 1;
        }
    }
}
