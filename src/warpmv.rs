use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn llabs(_: libc::c_longlong) -> libc::c_longlong;
}
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type Dav1dWarpedMotionType = libc::c_uint;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dWarpedMotionParams {
    pub type_0: Dav1dWarpedMotionType,
    pub matrix: [int32_t; 6],
    pub u: C2RustUnnamed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union C2RustUnnamed {
    pub p: C2RustUnnamed_0,
    pub abcd: [int16_t; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_0 {
    pub alpha: int16_t,
    pub beta: int16_t,
    pub gamma: int16_t,
    pub delta: int16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mv {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub n: uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C2RustUnnamed_1 {
    pub y: int16_t,
    pub x: int16_t,
}
#[inline]
unsafe extern "C" fn apply_sign(v: libc::c_int, s: libc::c_int) -> libc::c_int {
    return if s < 0i32 { -v } else { v };
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
unsafe extern "C" fn iclip(v: libc::c_int, min: libc::c_int, max: libc::c_int) -> libc::c_int {
    return if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    };
}
#[inline]
unsafe extern "C" fn apply_sign64(v: libc::c_int, s: int64_t) -> libc::c_int {
    return if s < 0i64 { -v } else { v };
}
#[inline]
unsafe extern "C" fn ulog2(v: libc::c_uint) -> libc::c_int {
    return 31i32 - clz(v);
}
#[inline]
unsafe extern "C" fn u64log2(v: uint64_t) -> libc::c_int {
    return 63i32 - clzll(v);
}
static mut div_lut: [uint16_t; 257] = [
    16384u16, 16320u16, 16257u16, 16194u16, 16132u16, 16070u16, 16009u16, 15948u16, 15888u16,
    15828u16, 15768u16, 15709u16, 15650u16, 15592u16, 15534u16, 15477u16, 15420u16, 15364u16,
    15308u16, 15252u16, 15197u16, 15142u16, 15087u16, 15033u16, 14980u16, 14926u16, 14873u16,
    14821u16, 14769u16, 14717u16, 14665u16, 14614u16, 14564u16, 14513u16, 14463u16, 14413u16,
    14364u16, 14315u16, 14266u16, 14218u16, 14170u16, 14122u16, 14075u16, 14028u16, 13981u16,
    13935u16, 13888u16, 13843u16, 13797u16, 13752u16, 13707u16, 13662u16, 13618u16, 13574u16,
    13530u16, 13487u16, 13443u16, 13400u16, 13358u16, 13315u16, 13273u16, 13231u16, 13190u16,
    13148u16, 13107u16, 13066u16, 13026u16, 12985u16, 12945u16, 12906u16, 12866u16, 12827u16,
    12788u16, 12749u16, 12710u16, 12672u16, 12633u16, 12596u16, 12558u16, 12520u16, 12483u16,
    12446u16, 12409u16, 12373u16, 12336u16, 12300u16, 12264u16, 12228u16, 12193u16, 12157u16,
    12122u16, 12087u16, 12053u16, 12018u16, 11984u16, 11950u16, 11916u16, 11882u16, 11848u16,
    11815u16, 11782u16, 11749u16, 11716u16, 11683u16, 11651u16, 11619u16, 11586u16, 11555u16,
    11523u16, 11491u16, 11460u16, 11429u16, 11398u16, 11367u16, 11336u16, 11305u16, 11275u16,
    11245u16, 11215u16, 11185u16, 11155u16, 11125u16, 11096u16, 11067u16, 11038u16, 11009u16,
    10980u16, 10951u16, 10923u16, 10894u16, 10866u16, 10838u16, 10810u16, 10782u16, 10755u16,
    10727u16, 10700u16, 10673u16, 10645u16, 10618u16, 10592u16, 10565u16, 10538u16, 10512u16,
    10486u16, 10460u16, 10434u16, 10408u16, 10382u16, 10356u16, 10331u16, 10305u16, 10280u16,
    10255u16, 10230u16, 10205u16, 10180u16, 10156u16, 10131u16, 10107u16, 10082u16, 10058u16,
    10034u16, 10010u16, 9986u16, 9963u16, 9939u16, 9916u16, 9892u16, 9869u16, 9846u16, 9823u16,
    9800u16, 9777u16, 9754u16, 9732u16, 9709u16, 9687u16, 9664u16, 9642u16, 9620u16, 9598u16,
    9576u16, 9554u16, 9533u16, 9511u16, 9489u16, 9468u16, 9447u16, 9425u16, 9404u16, 9383u16,
    9362u16, 9341u16, 9321u16, 9300u16, 9279u16, 9259u16, 9239u16, 9218u16, 9198u16, 9178u16,
    9158u16, 9138u16, 9118u16, 9098u16, 9079u16, 9059u16, 9039u16, 9020u16, 9001u16, 8981u16,
    8962u16, 8943u16, 8924u16, 8905u16, 8886u16, 8867u16, 8849u16, 8830u16, 8812u16, 8793u16,
    8775u16, 8756u16, 8738u16, 8720u16, 8702u16, 8684u16, 8666u16, 8648u16, 8630u16, 8613u16,
    8595u16, 8577u16, 8560u16, 8542u16, 8525u16, 8508u16, 8490u16, 8473u16, 8456u16, 8439u16,
    8422u16, 8405u16, 8389u16, 8372u16, 8355u16, 8339u16, 8322u16, 8306u16, 8289u16, 8273u16,
    8257u16, 8240u16, 8224u16, 8208u16, 8192u16,
];
#[inline]
unsafe extern "C" fn iclip_wmp(v: libc::c_int) -> libc::c_int {
    let cv: libc::c_int = iclip(v, -(32767i32) - 1i32, 32767i32);
    return apply_sign(abs(cv) + 32i32 >> 6i32, cv) * ((1i32) << 6i32);
}
#[inline]
unsafe extern "C" fn resolve_divisor_32(d: libc::c_uint, shift: *mut libc::c_int) -> libc::c_int {
    *shift = ulog2(d);
    let e: libc::c_int = d.wrapping_sub(((1i32) << *shift) as libc::c_uint) as libc::c_int;
    let f: libc::c_int = if *shift > 8i32 {
        e + ((1i32) << *shift - 9i32) >> *shift - 8i32
    } else {
        e << 8i32 - *shift
    };
    if !(f <= 256i32) {
        unreachable!();
    }
    *shift += 14i32;
    return div_lut[f as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_shear_params(wm: *mut Dav1dWarpedMotionParams) -> libc::c_int {
    let mat: *const int32_t = ((*wm).matrix).as_mut_ptr();
    if *mat.offset(2isize) <= 0i32 {
        return 1i32;
    }
    (*wm).u.p.alpha = iclip_wmp(*mat.offset(2isize) - 0x10000i32) as int16_t;
    (*wm).u.p.beta = iclip_wmp(*mat.offset(3isize)) as int16_t;
    let mut shift: libc::c_int = 0;
    let y: libc::c_int = apply_sign(
        resolve_divisor_32(abs(*mat.offset(2isize)) as libc::c_uint, &mut shift),
        *mat.offset(2isize),
    );
    let v1: int64_t = *mat.offset(4isize) as int64_t * 0x10000i64 * y as libc::c_long;
    let rnd: libc::c_int = (1i32) << shift >> 1i32;
    (*wm).u.p.gamma = iclip_wmp(apply_sign64(
        (llabs(v1) + rnd as libc::c_longlong >> shift) as libc::c_int,
        v1,
    )) as int16_t;
    let v2: int64_t =
        *mat.offset(3isize) as int64_t * *mat.offset(4isize) as libc::c_long * y as libc::c_long;
    (*wm).u.p.delta = iclip_wmp(
        *mat.offset(5isize)
            - apply_sign64(
                (llabs(v2) + rnd as libc::c_longlong >> shift) as libc::c_int,
                v2,
            )
            - 0x10000i32,
    ) as int16_t;
    return (4i32 * abs((*wm).u.p.alpha as libc::c_int) + 7i32 * abs((*wm).u.p.beta as libc::c_int)
        >= 0x10000i32
        || 4i32 * abs((*wm).u.p.gamma as libc::c_int) + 4i32 * abs((*wm).u.p.delta as libc::c_int)
            >= 0x10000i32) as libc::c_int;
}
unsafe extern "C" fn resolve_divisor_64(d: uint64_t, shift: *mut libc::c_int) -> libc::c_int {
    *shift = u64log2(d);
    let e: int64_t = (d).wrapping_sub(((1i64) << *shift) as libc::c_ulonglong) as int64_t;
    let f: int64_t = if *shift > 8i32 {
        e + ((1i64) << *shift - 9i32) >> *shift - 8i32
    } else {
        e << 8i32 - *shift
    };
    if !(f <= 256i64) {
        unreachable!();
    }
    *shift += 14i32;
    return div_lut[f as usize] as libc::c_int;
}
unsafe extern "C" fn get_mult_shift_ndiag(
    px: int64_t,
    idet: libc::c_int,
    shift: libc::c_int,
) -> libc::c_int {
    let v1: int64_t = px * idet as libc::c_long;
    let v2: libc::c_int = apply_sign64(
        (llabs(v1) + ((1i64) << shift >> 1i32) >> shift) as libc::c_int,
        v1,
    );
    return iclip(v2, -(0x1fffi32), 0x1fffi32);
}
unsafe extern "C" fn get_mult_shift_diag(
    px: int64_t,
    idet: libc::c_int,
    shift: libc::c_int,
) -> libc::c_int {
    let v1: int64_t = px * idet as libc::c_long;
    let v2: libc::c_int = apply_sign64(
        (llabs(v1) + ((1i64) << shift >> 1i32) >> shift) as libc::c_int,
        v1,
    );
    return iclip(v2, 0xe001i32, 0x11fffi32);
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
    let rsuy: libc::c_int = 2i32 * bh4 - 1i32;
    let rsux: libc::c_int = 2i32 * bw4 - 1i32;
    let isuy: libc::c_int = by4 * 4i32 + rsuy;
    let isux: libc::c_int = bx4 * 4i32 + rsux;
    *mat.offset(0isize) = iclip(
        mv.c2rust_unnamed.x as libc::c_int * 0x2000i32
            - (isux * (*mat.offset(2isize) - 0x10000i32) + isuy * *mat.offset(3isize)),
        -(0x800000i32),
        0x7fffffi32,
    );
    *mat.offset(1isize) = iclip(
        mv.c2rust_unnamed.y as libc::c_int * 0x2000i32
            - (isux * *mat.offset(4isize) + isuy * (*mat.offset(5isize) - 0x10000i32)),
        -(0x800000i32),
        0x7fffffi32,
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
    let mut a: [[libc::c_int; 2]; 2] = [[0i32, 0i32], [0i32, 0i32]];
    let mut bx: [libc::c_int; 2] = [0i32, 0i32];
    let mut by: [libc::c_int; 2] = [0i32, 0i32];
    let rsuy: libc::c_int = 2i32 * bh4 - 1i32;
    let rsux: libc::c_int = 2i32 * bw4 - 1i32;
    let suy: libc::c_int = rsuy * 8i32;
    let sux: libc::c_int = rsux * 8i32;
    let duy: libc::c_int = suy + mv.c2rust_unnamed.y as libc::c_int;
    let dux: libc::c_int = sux + mv.c2rust_unnamed.x as libc::c_int;
    let isuy: libc::c_int = by4 * 4i32 + rsuy;
    let isux: libc::c_int = bx4 * 4i32 + rsux;
    let mut i: libc::c_int = 0i32;
    while i < np {
        let dx: libc::c_int = (*pts.offset(i as isize))[1usize][0usize] - dux;
        let dy: libc::c_int = (*pts.offset(i as isize))[1usize][1usize] - duy;
        let sx: libc::c_int = (*pts.offset(i as isize))[0usize][0usize] - sux;
        let sy: libc::c_int = (*pts.offset(i as isize))[0usize][1usize] - suy;
        if abs(sx - dx) < 256i32 && abs(sy - dy) < 256i32 {
            a[0usize][0usize] += (sx * sx >> 2i32) + sx * 2i32 + 8i32;
            a[0usize][1usize] += (sx * sy >> 2i32) + sx + sy + 4i32;
            a[1usize][1usize] += (sy * sy >> 2i32) + sy * 2i32 + 8i32;
            bx[0usize] += (sx * dx >> 2i32) + sx + dx + 8i32;
            bx[1usize] += (sy * dx >> 2i32) + sy + dx + 4i32;
            by[0usize] += (sx * dy >> 2i32) + sx + dy + 4i32;
            by[1usize] += (sy * dy >> 2i32) + sy + dy + 8i32;
        }
        i += 1;
    }
    let det: int64_t = a[0usize][0usize] as int64_t * a[1usize][1usize] as libc::c_long
        - a[0usize][1usize] as int64_t * a[0usize][1usize] as libc::c_long;
    if det == 0i64 {
        return 1i32;
    }
    let mut shift: libc::c_int = 0;
    let mut idet: libc::c_int =
        apply_sign64(resolve_divisor_64(llabs(det) as uint64_t, &mut shift), det);
    shift -= 16i32;
    if shift < 0i32 {
        idet <<= -shift;
        shift = 0i32;
    }
    *mat.offset(2isize) = get_mult_shift_diag(
        a[1usize][1usize] as int64_t * bx[0usize] as libc::c_long
            - a[0usize][1usize] as int64_t * bx[1usize] as libc::c_long,
        idet,
        shift,
    );
    *mat.offset(3isize) = get_mult_shift_ndiag(
        a[0usize][0usize] as int64_t * bx[1usize] as libc::c_long
            - a[0usize][1usize] as int64_t * bx[0usize] as libc::c_long,
        idet,
        shift,
    );
    *mat.offset(4isize) = get_mult_shift_ndiag(
        a[1usize][1usize] as int64_t * by[0usize] as libc::c_long
            - a[0usize][1usize] as int64_t * by[1usize] as libc::c_long,
        idet,
        shift,
    );
    *mat.offset(5isize) = get_mult_shift_diag(
        a[0usize][0usize] as int64_t * by[1usize] as libc::c_long
            - a[0usize][1usize] as int64_t * by[0usize] as libc::c_long,
        idet,
        shift,
    );
    *mat.offset(0isize) = iclip(
        mv.c2rust_unnamed.x as libc::c_int * 0x2000i32
            - (isux * (*mat.offset(2isize) - 0x10000i32) + isuy * *mat.offset(3isize)),
        -(0x800000i32),
        0x7fffffi32,
    );
    *mat.offset(1isize) = iclip(
        mv.c2rust_unnamed.y as libc::c_int * 0x2000i32
            - (isux * *mat.offset(4isize) + isuy * (*mat.offset(5isize) - 0x10000i32)),
        -(0x800000i32),
        0x7fffffi32,
    );
    return 0i32;
}
