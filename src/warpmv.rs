use crate::include::stdint::*;
use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn llabs(_: libc::c_longlong) -> libc::c_longlong;
}

use crate::include::dav1d::headers::Dav1dWarpedMotionType;




#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: Dav1dWarpedMotionParams_u,
}
use crate::include::dav1d::headers::Dav1dWarpedMotionParams_u;

#[derive(Copy, Clone)]
#[repr(C)]
pub union mv {
    pub c2rust_unnamed: C2RustUnnamed,
    pub n: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub y: int16_t,
    pub x: int16_t,
}
#[inline]
unsafe extern "C" fn apply_sign(v: libc::c_int, s: libc::c_int) -> libc::c_int {
    return if s < 0 as libc::c_int { -v } else { v };
}
#[inline]
unsafe extern "C" fn clz(mask: libc::c_uint) -> libc::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn clzll(mask: libc::c_ulonglong) -> libc::c_int {
    return mask.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn iclip(
    v: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> libc::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn apply_sign64(v: libc::c_int, s: int64_t) -> libc::c_int {
    return if s < 0 { -v } else { v };
}
#[inline]
unsafe extern "C" fn ulog2(v: libc::c_uint) -> libc::c_int {
    return 31 as libc::c_int - clz(v);
}
#[inline]
unsafe extern "C" fn u64log2(v: uint64_t) -> libc::c_int {
    return 63 as libc::c_int - clzll(v as libc::c_ulonglong);
}
static mut div_lut: [uint16_t; 257] = [
    16384 as libc::c_int as uint16_t,
    16320 as libc::c_int as uint16_t,
    16257 as libc::c_int as uint16_t,
    16194 as libc::c_int as uint16_t,
    16132 as libc::c_int as uint16_t,
    16070 as libc::c_int as uint16_t,
    16009 as libc::c_int as uint16_t,
    15948 as libc::c_int as uint16_t,
    15888 as libc::c_int as uint16_t,
    15828 as libc::c_int as uint16_t,
    15768 as libc::c_int as uint16_t,
    15709 as libc::c_int as uint16_t,
    15650 as libc::c_int as uint16_t,
    15592 as libc::c_int as uint16_t,
    15534 as libc::c_int as uint16_t,
    15477 as libc::c_int as uint16_t,
    15420 as libc::c_int as uint16_t,
    15364 as libc::c_int as uint16_t,
    15308 as libc::c_int as uint16_t,
    15252 as libc::c_int as uint16_t,
    15197 as libc::c_int as uint16_t,
    15142 as libc::c_int as uint16_t,
    15087 as libc::c_int as uint16_t,
    15033 as libc::c_int as uint16_t,
    14980 as libc::c_int as uint16_t,
    14926 as libc::c_int as uint16_t,
    14873 as libc::c_int as uint16_t,
    14821 as libc::c_int as uint16_t,
    14769 as libc::c_int as uint16_t,
    14717 as libc::c_int as uint16_t,
    14665 as libc::c_int as uint16_t,
    14614 as libc::c_int as uint16_t,
    14564 as libc::c_int as uint16_t,
    14513 as libc::c_int as uint16_t,
    14463 as libc::c_int as uint16_t,
    14413 as libc::c_int as uint16_t,
    14364 as libc::c_int as uint16_t,
    14315 as libc::c_int as uint16_t,
    14266 as libc::c_int as uint16_t,
    14218 as libc::c_int as uint16_t,
    14170 as libc::c_int as uint16_t,
    14122 as libc::c_int as uint16_t,
    14075 as libc::c_int as uint16_t,
    14028 as libc::c_int as uint16_t,
    13981 as libc::c_int as uint16_t,
    13935 as libc::c_int as uint16_t,
    13888 as libc::c_int as uint16_t,
    13843 as libc::c_int as uint16_t,
    13797 as libc::c_int as uint16_t,
    13752 as libc::c_int as uint16_t,
    13707 as libc::c_int as uint16_t,
    13662 as libc::c_int as uint16_t,
    13618 as libc::c_int as uint16_t,
    13574 as libc::c_int as uint16_t,
    13530 as libc::c_int as uint16_t,
    13487 as libc::c_int as uint16_t,
    13443 as libc::c_int as uint16_t,
    13400 as libc::c_int as uint16_t,
    13358 as libc::c_int as uint16_t,
    13315 as libc::c_int as uint16_t,
    13273 as libc::c_int as uint16_t,
    13231 as libc::c_int as uint16_t,
    13190 as libc::c_int as uint16_t,
    13148 as libc::c_int as uint16_t,
    13107 as libc::c_int as uint16_t,
    13066 as libc::c_int as uint16_t,
    13026 as libc::c_int as uint16_t,
    12985 as libc::c_int as uint16_t,
    12945 as libc::c_int as uint16_t,
    12906 as libc::c_int as uint16_t,
    12866 as libc::c_int as uint16_t,
    12827 as libc::c_int as uint16_t,
    12788 as libc::c_int as uint16_t,
    12749 as libc::c_int as uint16_t,
    12710 as libc::c_int as uint16_t,
    12672 as libc::c_int as uint16_t,
    12633 as libc::c_int as uint16_t,
    12596 as libc::c_int as uint16_t,
    12558 as libc::c_int as uint16_t,
    12520 as libc::c_int as uint16_t,
    12483 as libc::c_int as uint16_t,
    12446 as libc::c_int as uint16_t,
    12409 as libc::c_int as uint16_t,
    12373 as libc::c_int as uint16_t,
    12336 as libc::c_int as uint16_t,
    12300 as libc::c_int as uint16_t,
    12264 as libc::c_int as uint16_t,
    12228 as libc::c_int as uint16_t,
    12193 as libc::c_int as uint16_t,
    12157 as libc::c_int as uint16_t,
    12122 as libc::c_int as uint16_t,
    12087 as libc::c_int as uint16_t,
    12053 as libc::c_int as uint16_t,
    12018 as libc::c_int as uint16_t,
    11984 as libc::c_int as uint16_t,
    11950 as libc::c_int as uint16_t,
    11916 as libc::c_int as uint16_t,
    11882 as libc::c_int as uint16_t,
    11848 as libc::c_int as uint16_t,
    11815 as libc::c_int as uint16_t,
    11782 as libc::c_int as uint16_t,
    11749 as libc::c_int as uint16_t,
    11716 as libc::c_int as uint16_t,
    11683 as libc::c_int as uint16_t,
    11651 as libc::c_int as uint16_t,
    11619 as libc::c_int as uint16_t,
    11586 as libc::c_int as uint16_t,
    11555 as libc::c_int as uint16_t,
    11523 as libc::c_int as uint16_t,
    11491 as libc::c_int as uint16_t,
    11460 as libc::c_int as uint16_t,
    11429 as libc::c_int as uint16_t,
    11398 as libc::c_int as uint16_t,
    11367 as libc::c_int as uint16_t,
    11336 as libc::c_int as uint16_t,
    11305 as libc::c_int as uint16_t,
    11275 as libc::c_int as uint16_t,
    11245 as libc::c_int as uint16_t,
    11215 as libc::c_int as uint16_t,
    11185 as libc::c_int as uint16_t,
    11155 as libc::c_int as uint16_t,
    11125 as libc::c_int as uint16_t,
    11096 as libc::c_int as uint16_t,
    11067 as libc::c_int as uint16_t,
    11038 as libc::c_int as uint16_t,
    11009 as libc::c_int as uint16_t,
    10980 as libc::c_int as uint16_t,
    10951 as libc::c_int as uint16_t,
    10923 as libc::c_int as uint16_t,
    10894 as libc::c_int as uint16_t,
    10866 as libc::c_int as uint16_t,
    10838 as libc::c_int as uint16_t,
    10810 as libc::c_int as uint16_t,
    10782 as libc::c_int as uint16_t,
    10755 as libc::c_int as uint16_t,
    10727 as libc::c_int as uint16_t,
    10700 as libc::c_int as uint16_t,
    10673 as libc::c_int as uint16_t,
    10645 as libc::c_int as uint16_t,
    10618 as libc::c_int as uint16_t,
    10592 as libc::c_int as uint16_t,
    10565 as libc::c_int as uint16_t,
    10538 as libc::c_int as uint16_t,
    10512 as libc::c_int as uint16_t,
    10486 as libc::c_int as uint16_t,
    10460 as libc::c_int as uint16_t,
    10434 as libc::c_int as uint16_t,
    10408 as libc::c_int as uint16_t,
    10382 as libc::c_int as uint16_t,
    10356 as libc::c_int as uint16_t,
    10331 as libc::c_int as uint16_t,
    10305 as libc::c_int as uint16_t,
    10280 as libc::c_int as uint16_t,
    10255 as libc::c_int as uint16_t,
    10230 as libc::c_int as uint16_t,
    10205 as libc::c_int as uint16_t,
    10180 as libc::c_int as uint16_t,
    10156 as libc::c_int as uint16_t,
    10131 as libc::c_int as uint16_t,
    10107 as libc::c_int as uint16_t,
    10082 as libc::c_int as uint16_t,
    10058 as libc::c_int as uint16_t,
    10034 as libc::c_int as uint16_t,
    10010 as libc::c_int as uint16_t,
    9986 as libc::c_int as uint16_t,
    9963 as libc::c_int as uint16_t,
    9939 as libc::c_int as uint16_t,
    9916 as libc::c_int as uint16_t,
    9892 as libc::c_int as uint16_t,
    9869 as libc::c_int as uint16_t,
    9846 as libc::c_int as uint16_t,
    9823 as libc::c_int as uint16_t,
    9800 as libc::c_int as uint16_t,
    9777 as libc::c_int as uint16_t,
    9754 as libc::c_int as uint16_t,
    9732 as libc::c_int as uint16_t,
    9709 as libc::c_int as uint16_t,
    9687 as libc::c_int as uint16_t,
    9664 as libc::c_int as uint16_t,
    9642 as libc::c_int as uint16_t,
    9620 as libc::c_int as uint16_t,
    9598 as libc::c_int as uint16_t,
    9576 as libc::c_int as uint16_t,
    9554 as libc::c_int as uint16_t,
    9533 as libc::c_int as uint16_t,
    9511 as libc::c_int as uint16_t,
    9489 as libc::c_int as uint16_t,
    9468 as libc::c_int as uint16_t,
    9447 as libc::c_int as uint16_t,
    9425 as libc::c_int as uint16_t,
    9404 as libc::c_int as uint16_t,
    9383 as libc::c_int as uint16_t,
    9362 as libc::c_int as uint16_t,
    9341 as libc::c_int as uint16_t,
    9321 as libc::c_int as uint16_t,
    9300 as libc::c_int as uint16_t,
    9279 as libc::c_int as uint16_t,
    9259 as libc::c_int as uint16_t,
    9239 as libc::c_int as uint16_t,
    9218 as libc::c_int as uint16_t,
    9198 as libc::c_int as uint16_t,
    9178 as libc::c_int as uint16_t,
    9158 as libc::c_int as uint16_t,
    9138 as libc::c_int as uint16_t,
    9118 as libc::c_int as uint16_t,
    9098 as libc::c_int as uint16_t,
    9079 as libc::c_int as uint16_t,
    9059 as libc::c_int as uint16_t,
    9039 as libc::c_int as uint16_t,
    9020 as libc::c_int as uint16_t,
    9001 as libc::c_int as uint16_t,
    8981 as libc::c_int as uint16_t,
    8962 as libc::c_int as uint16_t,
    8943 as libc::c_int as uint16_t,
    8924 as libc::c_int as uint16_t,
    8905 as libc::c_int as uint16_t,
    8886 as libc::c_int as uint16_t,
    8867 as libc::c_int as uint16_t,
    8849 as libc::c_int as uint16_t,
    8830 as libc::c_int as uint16_t,
    8812 as libc::c_int as uint16_t,
    8793 as libc::c_int as uint16_t,
    8775 as libc::c_int as uint16_t,
    8756 as libc::c_int as uint16_t,
    8738 as libc::c_int as uint16_t,
    8720 as libc::c_int as uint16_t,
    8702 as libc::c_int as uint16_t,
    8684 as libc::c_int as uint16_t,
    8666 as libc::c_int as uint16_t,
    8648 as libc::c_int as uint16_t,
    8630 as libc::c_int as uint16_t,
    8613 as libc::c_int as uint16_t,
    8595 as libc::c_int as uint16_t,
    8577 as libc::c_int as uint16_t,
    8560 as libc::c_int as uint16_t,
    8542 as libc::c_int as uint16_t,
    8525 as libc::c_int as uint16_t,
    8508 as libc::c_int as uint16_t,
    8490 as libc::c_int as uint16_t,
    8473 as libc::c_int as uint16_t,
    8456 as libc::c_int as uint16_t,
    8439 as libc::c_int as uint16_t,
    8422 as libc::c_int as uint16_t,
    8405 as libc::c_int as uint16_t,
    8389 as libc::c_int as uint16_t,
    8372 as libc::c_int as uint16_t,
    8355 as libc::c_int as uint16_t,
    8339 as libc::c_int as uint16_t,
    8322 as libc::c_int as uint16_t,
    8306 as libc::c_int as uint16_t,
    8289 as libc::c_int as uint16_t,
    8273 as libc::c_int as uint16_t,
    8257 as libc::c_int as uint16_t,
    8240 as libc::c_int as uint16_t,
    8224 as libc::c_int as uint16_t,
    8208 as libc::c_int as uint16_t,
    8192 as libc::c_int as uint16_t,
];
#[inline]
unsafe extern "C" fn iclip_wmp(v: libc::c_int) -> libc::c_int {
    let cv: libc::c_int = iclip(
        v,
        -(32767 as libc::c_int) - 1 as libc::c_int,
        32767 as libc::c_int,
    );
    return apply_sign(abs(cv) + 32 as libc::c_int >> 6 as libc::c_int, cv)
        * ((1 as libc::c_int) << 6 as libc::c_int);
}
#[inline]
unsafe extern "C" fn resolve_divisor_32(
    d: libc::c_uint,
    shift: *mut libc::c_int,
) -> libc::c_int {
    *shift = ulog2(d);
    let e: libc::c_int = d.wrapping_sub(((1 as libc::c_int) << *shift) as libc::c_uint)
        as libc::c_int;
    let f: libc::c_int = if *shift > 8 as libc::c_int {
        e + ((1 as libc::c_int) << *shift - 9 as libc::c_int)
            >> *shift - 8 as libc::c_int
    } else {
        e << 8 as libc::c_int - *shift
    };
    if !(f <= 256 as libc::c_int) {
        unreachable!();
    }
    *shift += 14 as libc::c_int;
    return div_lut[f as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_shear_params(
    wm: *mut Dav1dWarpedMotionParams,
) -> libc::c_int {
    let mat: *const int32_t = ((*wm).matrix).as_mut_ptr();
    if *mat.offset(2 as libc::c_int as isize) <= 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    (*wm)
        .u
        .p
        .alpha = iclip_wmp(
        *mat.offset(2 as libc::c_int as isize) - 0x10000 as libc::c_int,
    ) as int16_t;
    (*wm).u.p.beta = iclip_wmp(*mat.offset(3 as libc::c_int as isize)) as int16_t;
    let mut shift: libc::c_int = 0;
    let y: libc::c_int = apply_sign(
        resolve_divisor_32(
            abs(*mat.offset(2 as libc::c_int as isize)) as libc::c_uint,
            &mut shift,
        ),
        *mat.offset(2 as libc::c_int as isize),
    );
    let v1: int64_t = *mat.offset(4 as libc::c_int as isize) as int64_t
        * 0x10000 * y as int64_t;
    let rnd: libc::c_int = (1 as libc::c_int) << shift >> 1 as libc::c_int;
    (*wm)
        .u
        .p
        .gamma = iclip_wmp(
        apply_sign64(
            (llabs(v1 as libc::c_longlong) + rnd as libc::c_longlong >> shift)
                as libc::c_int,
            v1,
        ),
    ) as int16_t;
    let v2: int64_t = *mat.offset(3 as libc::c_int as isize) as int64_t
        * *mat.offset(4 as libc::c_int as isize) as int64_t * y as int64_t;
    (*wm)
        .u
        .p
        .delta = iclip_wmp(
        *mat.offset(5 as libc::c_int as isize)
            - apply_sign64(
                (llabs(v2 as libc::c_longlong) + rnd as libc::c_longlong >> shift)
                    as libc::c_int,
                v2,
            ) - 0x10000 as libc::c_int,
    ) as int16_t;
    return (4 as libc::c_int * abs((*wm).u.p.alpha as libc::c_int)
        + 7 as libc::c_int * abs((*wm).u.p.beta as libc::c_int) >= 0x10000 as libc::c_int
        || 4 as libc::c_int * abs((*wm).u.p.gamma as libc::c_int)
            + 4 as libc::c_int * abs((*wm).u.p.delta as libc::c_int)
            >= 0x10000 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn resolve_divisor_64(
    d: uint64_t,
    shift: *mut libc::c_int,
) -> libc::c_int {
    *shift = u64log2(d);
    let e: int64_t = (d as libc::c_ulonglong)
        .wrapping_sub(((1 as libc::c_longlong) << *shift) as libc::c_ulonglong)
        as int64_t;
    let f: int64_t = (if *shift > 8 as libc::c_int {
        e as libc::c_longlong + ((1 as libc::c_longlong) << *shift - 9 as libc::c_int)
            >> *shift - 8 as libc::c_int
    } else {
        (e << 8 as libc::c_int - *shift) as libc::c_longlong
    }) as int64_t;
    if !(f <= 256) {
        unreachable!();
    }
    *shift += 14 as libc::c_int;
    return div_lut[f as usize] as libc::c_int;
}
unsafe extern "C" fn get_mult_shift_ndiag(
    px: int64_t,
    idet: libc::c_int,
    shift: libc::c_int,
) -> libc::c_int {
    let v1: int64_t = px * idet as int64_t;
    let v2: libc::c_int = apply_sign64(
        (llabs(v1 as libc::c_longlong)
            + ((1 as libc::c_longlong) << shift >> 1 as libc::c_int) >> shift)
            as libc::c_int,
        v1,
    );
    return iclip(v2, -(0x1fff as libc::c_int), 0x1fff as libc::c_int);
}
unsafe extern "C" fn get_mult_shift_diag(
    px: int64_t,
    idet: libc::c_int,
    shift: libc::c_int,
) -> libc::c_int {
    let v1: int64_t = px * idet as int64_t;
    let v2: libc::c_int = apply_sign64(
        (llabs(v1 as libc::c_longlong)
            + ((1 as libc::c_longlong) << shift >> 1 as libc::c_int) >> shift)
            as libc::c_int,
        v1,
    );
    return iclip(v2, 0xe001 as libc::c_int, 0x11fff as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_set_affine_mv2d(
    bw4: libc::c_int,
    bh4: libc::c_int,
    mv: mv,
    wm: *mut Dav1dWarpedMotionParams,
    bx4: libc::c_int,
    by4: libc::c_int,
) {
    let mat: *mut int32_t = ((*wm).matrix).as_mut_ptr();
    let rsuy: libc::c_int = 2 as libc::c_int * bh4 - 1 as libc::c_int;
    let rsux: libc::c_int = 2 as libc::c_int * bw4 - 1 as libc::c_int;
    let isuy: libc::c_int = by4 * 4 as libc::c_int + rsuy;
    let isux: libc::c_int = bx4 * 4 as libc::c_int + rsux;
    *mat
        .offset(
            0 as libc::c_int as isize,
        ) = iclip(
        mv.c2rust_unnamed.x as libc::c_int * 0x2000 as libc::c_int
            - (isux * (*mat.offset(2 as libc::c_int as isize) - 0x10000 as libc::c_int)
                + isuy * *mat.offset(3 as libc::c_int as isize)),
        -(0x800000 as libc::c_int),
        0x7fffff as libc::c_int,
    );
    *mat
        .offset(
            1 as libc::c_int as isize,
        ) = iclip(
        mv.c2rust_unnamed.y as libc::c_int * 0x2000 as libc::c_int
            - (isux * *mat.offset(4 as libc::c_int as isize)
                + isuy
                    * (*mat.offset(5 as libc::c_int as isize) - 0x10000 as libc::c_int)),
        -(0x800000 as libc::c_int),
        0x7fffff as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_find_affine_int(
    mut pts: *const [[libc::c_int; 2]; 2],
    np: libc::c_int,
    bw4: libc::c_int,
    bh4: libc::c_int,
    mv: mv,
    wm: *mut Dav1dWarpedMotionParams,
    bx4: libc::c_int,
    by4: libc::c_int,
) -> libc::c_int {
    let mat: *mut int32_t = ((*wm).matrix).as_mut_ptr();
    let mut a: [[libc::c_int; 2]; 2] = [
        [0 as libc::c_int, 0 as libc::c_int],
        [0 as libc::c_int, 0 as libc::c_int],
    ];
    let mut bx: [libc::c_int; 2] = [0 as libc::c_int, 0 as libc::c_int];
    let mut by: [libc::c_int; 2] = [0 as libc::c_int, 0 as libc::c_int];
    let rsuy: libc::c_int = 2 as libc::c_int * bh4 - 1 as libc::c_int;
    let rsux: libc::c_int = 2 as libc::c_int * bw4 - 1 as libc::c_int;
    let suy: libc::c_int = rsuy * 8 as libc::c_int;
    let sux: libc::c_int = rsux * 8 as libc::c_int;
    let duy: libc::c_int = suy + mv.c2rust_unnamed.y as libc::c_int;
    let dux: libc::c_int = sux + mv.c2rust_unnamed.x as libc::c_int;
    let isuy: libc::c_int = by4 * 4 as libc::c_int + rsuy;
    let isux: libc::c_int = bx4 * 4 as libc::c_int + rsux;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < np {
        let dx: libc::c_int = (*pts
            .offset(i as isize))[1 as libc::c_int as usize][0 as libc::c_int as usize]
            - dux;
        let dy: libc::c_int = (*pts
            .offset(i as isize))[1 as libc::c_int as usize][1 as libc::c_int as usize]
            - duy;
        let sx: libc::c_int = (*pts
            .offset(i as isize))[0 as libc::c_int as usize][0 as libc::c_int as usize]
            - sux;
        let sy: libc::c_int = (*pts
            .offset(i as isize))[0 as libc::c_int as usize][1 as libc::c_int as usize]
            - suy;
        if abs(sx - dx) < 256 as libc::c_int && abs(sy - dy) < 256 as libc::c_int {
            a[0 as libc::c_int as usize][0 as libc::c_int as usize]
                += (sx * sx >> 2 as libc::c_int) + sx * 2 as libc::c_int
                    + 8 as libc::c_int;
            a[0 as libc::c_int as usize][1 as libc::c_int as usize]
                += (sx * sy >> 2 as libc::c_int) + sx + sy + 4 as libc::c_int;
            a[1 as libc::c_int as usize][1 as libc::c_int as usize]
                += (sy * sy >> 2 as libc::c_int) + sy * 2 as libc::c_int
                    + 8 as libc::c_int;
            bx[0 as libc::c_int as usize]
                += (sx * dx >> 2 as libc::c_int) + sx + dx + 8 as libc::c_int;
            bx[1 as libc::c_int as usize]
                += (sy * dx >> 2 as libc::c_int) + sy + dx + 4 as libc::c_int;
            by[0 as libc::c_int as usize]
                += (sx * dy >> 2 as libc::c_int) + sx + dy + 4 as libc::c_int;
            by[1 as libc::c_int as usize]
                += (sy * dy >> 2 as libc::c_int) + sy + dy + 8 as libc::c_int;
        }
        i += 1;
    }
    let det: int64_t = a[0 as libc::c_int as usize][0 as libc::c_int as usize] as int64_t
        * a[1 as libc::c_int as usize][1 as libc::c_int as usize] as int64_t
        - a[0 as libc::c_int as usize][1 as libc::c_int as usize] as int64_t
            * a[0 as libc::c_int as usize][1 as libc::c_int as usize] as int64_t;
    if det == 0 {
        return 1 as libc::c_int;
    }
    let mut shift: libc::c_int = 0;
    let mut idet: libc::c_int = apply_sign64(
        resolve_divisor_64(llabs(det as libc::c_longlong) as uint64_t, &mut shift),
        det,
    );
    shift -= 16 as libc::c_int;
    if shift < 0 as libc::c_int {
        idet <<= -shift;
        shift = 0 as libc::c_int;
    }
    *mat
        .offset(
            2 as libc::c_int as isize,
        ) = get_mult_shift_diag(
        a[1 as libc::c_int as usize][1 as libc::c_int as usize] as int64_t
            * bx[0 as libc::c_int as usize] as int64_t
            - a[0 as libc::c_int as usize][1 as libc::c_int as usize] as int64_t
                * bx[1 as libc::c_int as usize] as int64_t,
        idet,
        shift,
    );
    *mat
        .offset(
            3 as libc::c_int as isize,
        ) = get_mult_shift_ndiag(
        a[0 as libc::c_int as usize][0 as libc::c_int as usize] as int64_t
            * bx[1 as libc::c_int as usize] as int64_t
            - a[0 as libc::c_int as usize][1 as libc::c_int as usize] as int64_t
                * bx[0 as libc::c_int as usize] as int64_t,
        idet,
        shift,
    );
    *mat
        .offset(
            4 as libc::c_int as isize,
        ) = get_mult_shift_ndiag(
        a[1 as libc::c_int as usize][1 as libc::c_int as usize] as int64_t
            * by[0 as libc::c_int as usize] as int64_t
            - a[0 as libc::c_int as usize][1 as libc::c_int as usize] as int64_t
                * by[1 as libc::c_int as usize] as int64_t,
        idet,
        shift,
    );
    *mat
        .offset(
            5 as libc::c_int as isize,
        ) = get_mult_shift_diag(
        a[0 as libc::c_int as usize][0 as libc::c_int as usize] as int64_t
            * by[1 as libc::c_int as usize] as int64_t
            - a[0 as libc::c_int as usize][1 as libc::c_int as usize] as int64_t
                * by[0 as libc::c_int as usize] as int64_t,
        idet,
        shift,
    );
    *mat
        .offset(
            0 as libc::c_int as isize,
        ) = iclip(
        mv.c2rust_unnamed.x as libc::c_int * 0x2000 as libc::c_int
            - (isux * (*mat.offset(2 as libc::c_int as isize) - 0x10000 as libc::c_int)
                + isuy * *mat.offset(3 as libc::c_int as isize)),
        -(0x800000 as libc::c_int),
        0x7fffff as libc::c_int,
    );
    *mat
        .offset(
            1 as libc::c_int as isize,
        ) = iclip(
        mv.c2rust_unnamed.y as libc::c_int * 0x2000 as libc::c_int
            - (isux * *mat.offset(4 as libc::c_int as isize)
                + isuy
                    * (*mat.offset(5 as libc::c_int as isize) - 0x10000 as libc::c_int)),
        -(0x800000 as libc::c_int),
        0x7fffff as libc::c_int,
    );
    return 0 as libc::c_int;
}
