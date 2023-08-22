use crate::include::common::bitdepth::AsPrimitive;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::intops::iclip;
use crate::include::common::intops::imin;
use crate::include::stddef::*;
use crate::include::stdint::*;

extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type itx_1d_fn =
    Option<unsafe extern "C" fn(*mut int32_t, ptrdiff_t, libc::c_int, libc::c_int) -> ()>;

pub unsafe extern "C" fn inv_txfm_add_rust<BD: BitDepth>(
    mut dst: *mut BD::Pixel,
    stride: ptrdiff_t,
    coeff: *mut BD::Coef,
    eob: libc::c_int,
    w: libc::c_int,
    h: libc::c_int,
    shift: libc::c_int,
    first_1d_fn: itx_1d_fn,
    second_1d_fn: itx_1d_fn,
    has_dconly: libc::c_int,
    bd: BD,
) {
    let bitdepth_max: libc::c_int = bd.bitdepth_max().as_();
    let stride = stride as usize;
    if !(w >= 4 && w <= 64) {
        unreachable!();
    }
    if !(h >= 4 && h <= 64) {
        unreachable!();
    }
    if !(eob >= 0) {
        unreachable!();
    }
    let is_rect2: libc::c_int = (w * 2 == h || h * 2 == w) as libc::c_int;
    let rnd = (1 as libc::c_int) << shift >> 1;
    if eob < has_dconly {
        let mut dc: libc::c_int = (*coeff.offset(0)).as_();
        *coeff.offset(0) = 0.as_();
        if is_rect2 != 0 {
            dc = dc * 181 + 128 >> 8;
        }
        dc = dc * 181 + 128 >> 8;
        dc = dc + rnd >> shift;
        dc = dc * 181 + 128 + 2048 >> 12;
        let mut y = 0;
        while y < h {
            let mut x = 0;
            while x < w {
                *dst.offset(x as isize) =
                    bd.iclip_pixel((*dst.offset(x as isize)).as_::<libc::c_int>() + dc);
                x += 1;
            }
            y += 1;
            dst = dst.offset(BD::pxstride(stride) as isize);
        }
        return;
    }
    let sh = imin(h, 32 as libc::c_int);
    let sw = imin(w, 32 as libc::c_int);
    let row_clip_min;
    let col_clip_min;
    if BD::BITDEPTH == 8 {
        row_clip_min = std::i16::MIN as i32;
        col_clip_min = std::i16::MIN as i32;
    } else {
        row_clip_min = ((!bitdepth_max) << 7) as libc::c_int;
        col_clip_min = ((!bitdepth_max) << 5) as libc::c_int;
    }
    let row_clip_max = !row_clip_min;
    let col_clip_max = !col_clip_min;
    let mut tmp: [int32_t; 4096] = [0; 4096];
    let mut c: *mut int32_t = tmp.as_mut_ptr();
    let mut y_0 = 0;
    while y_0 < sh {
        if is_rect2 != 0 {
            let mut x_0 = 0;
            while x_0 < sw {
                *c.offset(x_0 as isize) =
                    (*coeff.offset((y_0 + x_0 * sh) as isize)).as_::<libc::c_int>() * 181 + 128
                        >> 8;
                x_0 += 1;
            }
        } else {
            let mut x_1 = 0;
            while x_1 < sw {
                *c.offset(x_1 as isize) = (*coeff.offset((y_0 + x_1 * sh) as isize)).as_();
                x_1 += 1;
            }
        }
        first_1d_fn.expect("non-null function pointer")(
            c,
            1 as libc::c_int as ptrdiff_t,
            row_clip_min,
            row_clip_max,
        );
        y_0 += 1;
        c = c.offset(w as isize);
    }
    memset(
        coeff as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<BD::Coef>() as libc::c_ulong)
            .wrapping_mul(sw as libc::c_ulong)
            .wrapping_mul(sh as libc::c_ulong),
    );
    let mut i = 0;
    while i < w * sh {
        tmp[i as usize] = iclip(tmp[i as usize] + rnd >> shift, col_clip_min, col_clip_max);
        i += 1;
    }
    let mut x_2 = 0;
    while x_2 < w {
        second_1d_fn.expect("non-null function pointer")(
            &mut *tmp.as_mut_ptr().offset(x_2 as isize),
            w as ptrdiff_t,
            col_clip_min,
            col_clip_max,
        );
        x_2 += 1;
    }
    c = tmp.as_mut_ptr();
    let mut y_1 = 0;
    while y_1 < h {
        let mut x_3 = 0;
        while x_3 < w {
            let fresh0 = c;
            c = c.offset(1);
            *dst.offset(x_3 as isize) = bd
                .iclip_pixel((*dst.offset(x_3 as isize)).as_::<libc::c_int>() + (*fresh0 + 8 >> 4));
            x_3 += 1;
        }
        y_1 += 1;
        dst = dst.offset(BD::pxstride(stride) as isize);
    }
}
