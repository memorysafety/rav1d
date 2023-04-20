use crate::include::stddef::*;
use crate::include::stdint::*;
use ::libc;
use crate::include::common::intops::iclip;
#[inline(never)]
unsafe extern "C" fn inv_dct4_1d_internal_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
    tx64: libc::c_int,
) {
    if !(stride > 0) {
        unreachable!();
    }
    let in0 = *c
        .offset((0 * stride) as isize);
    let in1 = *c
        .offset((1 * stride) as isize);
    let mut t0 = 0;
    let mut t1 = 0;
    let mut t2 = 0;
    let mut t3 = 0;
    if tx64 != 0 {
        t1 = in0 * 181 + 128 >> 8;
        t0 = t1;
        t2 = in1 * 1567 + 2048 >> 12;
        t3 = in1 * 3784 + 2048 >> 12;
    } else {
        let in2 = *c
            .offset((2 * stride) as isize);
        let in3 = *c
            .offset((3 * stride) as isize);
        t0 = (in0 + in2) * 181 + 128 >> 8;
        t1 = (in0 - in2) * 181 + 128 >> 8;
        t2 = (in1 * 1567
            - in3 * (3784 - 4096) + 2048
            >> 12) - in3;
        t3 = (in1 * (3784 - 4096)
            + in3 * 1567 + 2048 >> 12)
            + in1;
    }
    *c
        .offset(
            (0 * stride) as isize,
        ) = iclip(t0 + t3, min, max);
    *c
        .offset(
            (1 * stride) as isize,
        ) = iclip(t1 + t2, min, max);
    *c
        .offset(
            (2 * stride) as isize,
        ) = iclip(t1 - t2, min, max);
    *c
        .offset(
            (3 * stride) as isize,
        ) = iclip(t0 - t3, min, max);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_dct4_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_dct4_1d_internal_c(c, stride, min, max, 0 as libc::c_int);
}
#[inline(never)]
unsafe extern "C" fn inv_dct8_1d_internal_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
    tx64: libc::c_int,
) {
    if !(stride > 0) {
        unreachable!();
    }
    inv_dct4_1d_internal_c(c, stride << 1, min, max, tx64);
    let in1 = *c
        .offset((1 * stride) as isize);
    let in3 = *c
        .offset((3 * stride) as isize);
    let mut t4a = 0;
    let mut t5a = 0;
    let mut t6a = 0;
    let mut t7a = 0;
    if tx64 != 0 {
        t4a = in1 * 799 + 2048 >> 12;
        t5a = in3 * -(2276 as libc::c_int) + 2048 >> 12;
        t6a = in3 * 3406 + 2048 >> 12;
        t7a = in1 * 4017 + 2048 >> 12;
    } else {
        let in5 = *c
            .offset((5 * stride) as isize);
        let in7 = *c
            .offset((7 * stride) as isize);
        t4a = (in1 * 799
            - in7 * (4017 - 4096) + 2048
            >> 12) - in7;
        t5a = in5 * 1703 - in3 * 1138 + 1024
            >> 11;
        t6a = in5 * 1138 + in3 * 1703 + 1024
            >> 11;
        t7a = (in1 * (4017 - 4096)
            + in7 * 799 + 2048 >> 12) + in1;
    }
    let t4 = iclip(t4a + t5a, min, max);
    t5a = iclip(t4a - t5a, min, max);
    let t7 = iclip(t7a + t6a, min, max);
    t6a = iclip(t7a - t6a, min, max);
    let t5 = (t6a - t5a) * 181 + 128
        >> 8;
    let t6 = (t6a + t5a) * 181 + 128
        >> 8;
    let t0 = *c
        .offset((0 * stride) as isize);
    let t1 = *c
        .offset((2 * stride) as isize);
    let t2 = *c
        .offset((4 * stride) as isize);
    let t3 = *c
        .offset((6 * stride) as isize);
    *c
        .offset(
            (0 * stride) as isize,
        ) = iclip(t0 + t7, min, max);
    *c
        .offset(
            (1 * stride) as isize,
        ) = iclip(t1 + t6, min, max);
    *c
        .offset(
            (2 * stride) as isize,
        ) = iclip(t2 + t5, min, max);
    *c
        .offset(
            (3 * stride) as isize,
        ) = iclip(t3 + t4, min, max);
    *c
        .offset(
            (4 * stride) as isize,
        ) = iclip(t3 - t4, min, max);
    *c
        .offset(
            (5 * stride) as isize,
        ) = iclip(t2 - t5, min, max);
    *c
        .offset(
            (6 * stride) as isize,
        ) = iclip(t1 - t6, min, max);
    *c
        .offset(
            (7 * stride) as isize,
        ) = iclip(t0 - t7, min, max);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_dct8_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_dct8_1d_internal_c(c, stride, min, max, 0 as libc::c_int);
}
#[inline(never)]
unsafe extern "C" fn inv_dct16_1d_internal_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
    mut tx64: libc::c_int,
) {
    if !(stride > 0) {
        unreachable!();
    }
    inv_dct8_1d_internal_c(c, stride << 1, min, max, tx64);
    let in1 = *c
        .offset((1 * stride) as isize);
    let in3 = *c
        .offset((3 * stride) as isize);
    let in5 = *c
        .offset((5 * stride) as isize);
    let in7 = *c
        .offset((7 * stride) as isize);
    let mut t8a = 0;
    let mut t9a = 0;
    let mut t10a = 0;
    let mut t11a = 0;
    let mut t12a = 0;
    let mut t13a = 0;
    let mut t14a = 0;
    let mut t15a = 0;
    if tx64 != 0 {
        t8a = in1 * 401 + 2048 >> 12;
        t9a = in7 * -(2598 as libc::c_int) + 2048 >> 12;
        t10a = in5 * 1931 + 2048 >> 12;
        t11a = in3 * -(1189 as libc::c_int) + 2048 >> 12;
        t12a = in3 * 3920 + 2048 >> 12;
        t13a = in5 * 3612 + 2048 >> 12;
        t14a = in7 * 3166 + 2048 >> 12;
        t15a = in1 * 4076 + 2048 >> 12;
    } else {
        let in9 = *c
            .offset((9 * stride) as isize);
        let in11 = *c
            .offset((11 * stride) as isize);
        let in13 = *c
            .offset((13 * stride) as isize);
        let in15 = *c
            .offset((15 * stride) as isize);
        t8a = (in1 * 401
            - in15 * (4076 - 4096) + 2048
            >> 12) - in15;
        t9a = in9 * 1583 - in7 * 1299 + 1024
            >> 11;
        t10a = (in5 * 1931
            - in11 * (3612 - 4096) + 2048
            >> 12) - in11;
        t11a = (in13 * (3920 - 4096)
            - in3 * 1189 + 2048 >> 12)
            + in13;
        t12a = (in13 * 1189
            + in3 * (3920 - 4096) + 2048
            >> 12) + in3;
        t13a = (in5 * (3612 - 4096)
            + in11 * 1931 + 2048 >> 12)
            + in5;
        t14a = in9 * 1299 + in7 * 1583
            + 1024 >> 11;
        t15a = (in1 * (4076 - 4096)
            + in15 * 401 + 2048 >> 12)
            + in1;
    }
    let mut t8 = iclip(t8a + t9a, min, max);
    let mut t9 = iclip(t8a - t9a, min, max);
    let mut t10 = iclip(t11a - t10a, min, max);
    let mut t11 = iclip(t11a + t10a, min, max);
    let mut t12 = iclip(t12a + t13a, min, max);
    let mut t13 = iclip(t12a - t13a, min, max);
    let mut t14 = iclip(t15a - t14a, min, max);
    let mut t15 = iclip(t15a + t14a, min, max);
    t9a = (t14 * 1567 - t9 * (3784 - 4096)
        + 2048 >> 12) - t9;
    t14a = (t14 * (3784 - 4096) + t9 * 1567
        + 2048 >> 12) + t14;
    t10a = (-(t13 * (3784 - 4096)
        + t10 * 1567) + 2048 >> 12) - t13;
    t13a = (t13 * 1567 - t10 * (3784 - 4096)
        + 2048 >> 12) - t10;
    t8a = iclip(t8 + t11, min, max);
    t9 = iclip(t9a + t10a, min, max);
    t10 = iclip(t9a - t10a, min, max);
    t11a = iclip(t8 - t11, min, max);
    t12a = iclip(t15 - t12, min, max);
    t13 = iclip(t14a - t13a, min, max);
    t14 = iclip(t14a + t13a, min, max);
    t15a = iclip(t15 + t12, min, max);
    t10a = (t13 - t10) * 181 + 128 >> 8;
    t13a = (t13 + t10) * 181 + 128 >> 8;
    t11 = (t12a - t11a) * 181 + 128 >> 8;
    t12 = (t12a + t11a) * 181 + 128 >> 8;
    let t0 = *c
        .offset((0 * stride) as isize);
    let t1 = *c
        .offset((2 * stride) as isize);
    let t2 = *c
        .offset((4 * stride) as isize);
    let t3 = *c
        .offset((6 * stride) as isize);
    let t4 = *c
        .offset((8 * stride) as isize);
    let t5 = *c
        .offset((10 * stride) as isize);
    let t6 = *c
        .offset((12 * stride) as isize);
    let t7 = *c
        .offset((14 * stride) as isize);
    *c
        .offset(
            (0 * stride) as isize,
        ) = iclip(t0 + t15a, min, max);
    *c
        .offset(
            (1 * stride) as isize,
        ) = iclip(t1 + t14, min, max);
    *c
        .offset(
            (2 * stride) as isize,
        ) = iclip(t2 + t13a, min, max);
    *c
        .offset(
            (3 * stride) as isize,
        ) = iclip(t3 + t12, min, max);
    *c
        .offset(
            (4 * stride) as isize,
        ) = iclip(t4 + t11, min, max);
    *c
        .offset(
            (5 * stride) as isize,
        ) = iclip(t5 + t10a, min, max);
    *c
        .offset(
            (6 * stride) as isize,
        ) = iclip(t6 + t9, min, max);
    *c
        .offset(
            (7 * stride) as isize,
        ) = iclip(t7 + t8a, min, max);
    *c
        .offset(
            (8 * stride) as isize,
        ) = iclip(t7 - t8a, min, max);
    *c
        .offset(
            (9 * stride) as isize,
        ) = iclip(t6 - t9, min, max);
    *c
        .offset(
            (10 * stride) as isize,
        ) = iclip(t5 - t10a, min, max);
    *c
        .offset(
            (11 * stride) as isize,
        ) = iclip(t4 - t11, min, max);
    *c
        .offset(
            (12 * stride) as isize,
        ) = iclip(t3 - t12, min, max);
    *c
        .offset(
            (13 * stride) as isize,
        ) = iclip(t2 - t13a, min, max);
    *c
        .offset(
            (14 * stride) as isize,
        ) = iclip(t1 - t14, min, max);
    *c
        .offset(
            (15 * stride) as isize,
        ) = iclip(t0 - t15a, min, max);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_dct16_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_dct16_1d_internal_c(c, stride, min, max, 0 as libc::c_int);
}
#[inline(never)]
unsafe extern "C" fn inv_dct32_1d_internal_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
    tx64: libc::c_int,
) {
    if !(stride > 0) {
        unreachable!();
    }
    inv_dct16_1d_internal_c(c, stride << 1, min, max, tx64);
    let in1 = *c
        .offset((1 * stride) as isize);
    let in3 = *c
        .offset((3 * stride) as isize);
    let in5 = *c
        .offset((5 * stride) as isize);
    let in7 = *c
        .offset((7 * stride) as isize);
    let in9 = *c
        .offset((9 * stride) as isize);
    let in11 = *c
        .offset((11 * stride) as isize);
    let in13 = *c
        .offset((13 * stride) as isize);
    let in15 = *c
        .offset((15 * stride) as isize);
    let mut t16a = 0;
    let mut t17a = 0;
    let mut t18a = 0;
    let mut t19a = 0;
    let mut t20a = 0;
    let mut t21a = 0;
    let mut t22a = 0;
    let mut t23a = 0;
    let mut t24a = 0;
    let mut t25a = 0;
    let mut t26a = 0;
    let mut t27a = 0;
    let mut t28a = 0;
    let mut t29a = 0;
    let mut t30a = 0;
    let mut t31a = 0;
    if tx64 != 0 {
        t16a = in1 * 201 + 2048 >> 12;
        t17a = in15 * -(2751 as libc::c_int) + 2048 >> 12;
        t18a = in9 * 1751 + 2048 >> 12;
        t19a = in7 * -(1380 as libc::c_int) + 2048 >> 12;
        t20a = in5 * 995 + 2048 >> 12;
        t21a = in11 * -(2106 as libc::c_int) + 2048 >> 12;
        t22a = in13 * 2440 + 2048 >> 12;
        t23a = in3 * -(601 as libc::c_int) + 2048 >> 12;
        t24a = in3 * 4052 + 2048 >> 12;
        t25a = in13 * 3290 + 2048 >> 12;
        t26a = in11 * 3513 + 2048 >> 12;
        t27a = in5 * 3973 + 2048 >> 12;
        t28a = in7 * 3857 + 2048 >> 12;
        t29a = in9 * 3703 + 2048 >> 12;
        t30a = in15 * 3035 + 2048 >> 12;
        t31a = in1 * 4091 + 2048 >> 12;
    } else {
        let in17 = *c
            .offset((17 * stride) as isize);
        let in19 = *c
            .offset((19 * stride) as isize);
        let in21 = *c
            .offset((21 * stride) as isize);
        let in23 = *c
            .offset((23 * stride) as isize);
        let in25 = *c
            .offset((25 * stride) as isize);
        let in27 = *c
            .offset((27 * stride) as isize);
        let in29 = *c
            .offset((29 * stride) as isize);
        let in31 = *c
            .offset((31 * stride) as isize);
        t16a = (in1 * 201
            - in31 * (4091 - 4096) + 2048
            >> 12) - in31;
        t17a = (in17 * (3035 - 4096)
            - in15 * 2751 + 2048 >> 12)
            + in17;
        t18a = (in9 * 1751
            - in23 * (3703 - 4096) + 2048
            >> 12) - in23;
        t19a = (in25 * (3857 - 4096)
            - in7 * 1380 + 2048 >> 12)
            + in25;
        t20a = (in5 * 995
            - in27 * (3973 - 4096) + 2048
            >> 12) - in27;
        t21a = (in21 * (3513 - 4096)
            - in11 * 2106 + 2048 >> 12)
            + in21;
        t22a = in13 * 1220 - in19 * 1645
            + 1024 >> 11;
        t23a = (in29 * (4052 - 4096)
            - in3 * 601 + 2048 >> 12)
            + in29;
        t24a = (in29 * 601
            + in3 * (4052 - 4096) + 2048
            >> 12) + in3;
        t25a = in13 * 1645 + in19 * 1220
            + 1024 >> 11;
        t26a = (in21 * 2106
            + in11 * (3513 - 4096) + 2048
            >> 12) + in11;
        t27a = (in5 * (3973 - 4096)
            + in27 * 995 + 2048 >> 12)
            + in5;
        t28a = (in25 * 1380
            + in7 * (3857 - 4096) + 2048
            >> 12) + in7;
        t29a = (in9 * (3703 - 4096)
            + in23 * 1751 + 2048 >> 12)
            + in9;
        t30a = (in17 * 2751
            + in15 * (3035 - 4096) + 2048
            >> 12) + in15;
        t31a = (in1 * (4091 - 4096)
            + in31 * 201 + 2048 >> 12)
            + in1;
    }
    let mut t16 = iclip(t16a + t17a, min, max);
    let mut t17 = iclip(t16a - t17a, min, max);
    let mut t18 = iclip(t19a - t18a, min, max);
    let mut t19 = iclip(t19a + t18a, min, max);
    let mut t20 = iclip(t20a + t21a, min, max);
    let mut t21 = iclip(t20a - t21a, min, max);
    let mut t22 = iclip(t23a - t22a, min, max);
    let mut t23 = iclip(t23a + t22a, min, max);
    let mut t24 = iclip(t24a + t25a, min, max);
    let mut t25 = iclip(t24a - t25a, min, max);
    let mut t26 = iclip(t27a - t26a, min, max);
    let mut t27 = iclip(t27a + t26a, min, max);
    let mut t28 = iclip(t28a + t29a, min, max);
    let mut t29 = iclip(t28a - t29a, min, max);
    let mut t30 = iclip(t31a - t30a, min, max);
    let mut t31 = iclip(t31a + t30a, min, max);
    t17a = (t30 * 799 - t17 * (4017 - 4096)
        + 2048 >> 12) - t17;
    t30a = (t30 * (4017 - 4096) + t17 * 799
        + 2048 >> 12) + t30;
    t18a = (-(t29 * (4017 - 4096)
        + t18 * 799) + 2048 >> 12) - t29;
    t29a = (t29 * 799 - t18 * (4017 - 4096)
        + 2048 >> 12) - t18;
    t21a = t26 * 1703 - t21 * 1138 + 1024
        >> 11;
    t26a = t26 * 1138 + t21 * 1703 + 1024
        >> 11;
    t22a = -(t25 * 1138 + t22 * 1703) + 1024
        >> 11;
    t25a = t25 * 1703 - t22 * 1138 + 1024
        >> 11;
    t16a = iclip(t16 + t19, min, max);
    t17 = iclip(t17a + t18a, min, max);
    t18 = iclip(t17a - t18a, min, max);
    t19a = iclip(t16 - t19, min, max);
    t20a = iclip(t23 - t20, min, max);
    t21 = iclip(t22a - t21a, min, max);
    t22 = iclip(t22a + t21a, min, max);
    t23a = iclip(t23 + t20, min, max);
    t24a = iclip(t24 + t27, min, max);
    t25 = iclip(t25a + t26a, min, max);
    t26 = iclip(t25a - t26a, min, max);
    t27a = iclip(t24 - t27, min, max);
    t28a = iclip(t31 - t28, min, max);
    t29 = iclip(t30a - t29a, min, max);
    t30 = iclip(t30a + t29a, min, max);
    t31a = iclip(t31 + t28, min, max);
    t18a = (t29 * 1567 - t18 * (3784 - 4096)
        + 2048 >> 12) - t18;
    t29a = (t29 * (3784 - 4096) + t18 * 1567
        + 2048 >> 12) + t29;
    t19 = (t28a * 1567
        - t19a * (3784 - 4096) + 2048
        >> 12) - t19a;
    t28 = (t28a * (3784 - 4096)
        + t19a * 1567 + 2048 >> 12) + t28a;
    t20 = (-(t27a * (3784 - 4096)
        + t20a * 1567) + 2048 >> 12) - t27a;
    t27 = (t27a * 1567
        - t20a * (3784 - 4096) + 2048
        >> 12) - t20a;
    t21a = (-(t26 * (3784 - 4096)
        + t21 * 1567) + 2048 >> 12) - t26;
    t26a = (t26 * 1567 - t21 * (3784 - 4096)
        + 2048 >> 12) - t21;
    t16 = iclip(t16a + t23a, min, max);
    t17a = iclip(t17 + t22, min, max);
    t18 = iclip(t18a + t21a, min, max);
    t19a = iclip(t19 + t20, min, max);
    t20a = iclip(t19 - t20, min, max);
    t21 = iclip(t18a - t21a, min, max);
    t22a = iclip(t17 - t22, min, max);
    t23 = iclip(t16a - t23a, min, max);
    t24 = iclip(t31a - t24a, min, max);
    t25a = iclip(t30 - t25, min, max);
    t26 = iclip(t29a - t26a, min, max);
    t27a = iclip(t28 - t27, min, max);
    t28a = iclip(t28 + t27, min, max);
    t29 = iclip(t29a + t26a, min, max);
    t30a = iclip(t30 + t25, min, max);
    t31 = iclip(t31a + t24a, min, max);
    t20 = (t27a - t20a) * 181 + 128 >> 8;
    t27 = (t27a + t20a) * 181 + 128 >> 8;
    t21a = (t26 - t21) * 181 + 128 >> 8;
    t26a = (t26 + t21) * 181 + 128 >> 8;
    t22 = (t25a - t22a) * 181 + 128 >> 8;
    t25 = (t25a + t22a) * 181 + 128 >> 8;
    t23a = (t24 - t23) * 181 + 128 >> 8;
    t24a = (t24 + t23) * 181 + 128 >> 8;
    let t0 = *c
        .offset((0 * stride) as isize);
    let t1 = *c
        .offset((2 * stride) as isize);
    let t2 = *c
        .offset((4 * stride) as isize);
    let t3 = *c
        .offset((6 * stride) as isize);
    let t4 = *c
        .offset((8 * stride) as isize);
    let t5 = *c
        .offset((10 * stride) as isize);
    let t6 = *c
        .offset((12 * stride) as isize);
    let t7 = *c
        .offset((14 * stride) as isize);
    let t8 = *c
        .offset((16 * stride) as isize);
    let t9 = *c
        .offset((18 * stride) as isize);
    let t10 = *c
        .offset((20 * stride) as isize);
    let t11 = *c
        .offset((22 * stride) as isize);
    let t12 = *c
        .offset((24 * stride) as isize);
    let t13 = *c
        .offset((26 * stride) as isize);
    let t14 = *c
        .offset((28 * stride) as isize);
    let t15 = *c
        .offset((30 * stride) as isize);
    *c
        .offset(
            (0 * stride) as isize,
        ) = iclip(t0 + t31, min, max);
    *c
        .offset(
            (1 * stride) as isize,
        ) = iclip(t1 + t30a, min, max);
    *c
        .offset(
            (2 * stride) as isize,
        ) = iclip(t2 + t29, min, max);
    *c
        .offset(
            (3 * stride) as isize,
        ) = iclip(t3 + t28a, min, max);
    *c
        .offset(
            (4 * stride) as isize,
        ) = iclip(t4 + t27, min, max);
    *c
        .offset(
            (5 * stride) as isize,
        ) = iclip(t5 + t26a, min, max);
    *c
        .offset(
            (6 * stride) as isize,
        ) = iclip(t6 + t25, min, max);
    *c
        .offset(
            (7 * stride) as isize,
        ) = iclip(t7 + t24a, min, max);
    *c
        .offset(
            (8 * stride) as isize,
        ) = iclip(t8 + t23a, min, max);
    *c
        .offset(
            (9 * stride) as isize,
        ) = iclip(t9 + t22, min, max);
    *c
        .offset(
            (10 * stride) as isize,
        ) = iclip(t10 + t21a, min, max);
    *c
        .offset(
            (11 * stride) as isize,
        ) = iclip(t11 + t20, min, max);
    *c
        .offset(
            (12 * stride) as isize,
        ) = iclip(t12 + t19a, min, max);
    *c
        .offset(
            (13 * stride) as isize,
        ) = iclip(t13 + t18, min, max);
    *c
        .offset(
            (14 * stride) as isize,
        ) = iclip(t14 + t17a, min, max);
    *c
        .offset(
            (15 * stride) as isize,
        ) = iclip(t15 + t16, min, max);
    *c
        .offset(
            (16 * stride) as isize,
        ) = iclip(t15 - t16, min, max);
    *c
        .offset(
            (17 * stride) as isize,
        ) = iclip(t14 - t17a, min, max);
    *c
        .offset(
            (18 * stride) as isize,
        ) = iclip(t13 - t18, min, max);
    *c
        .offset(
            (19 * stride) as isize,
        ) = iclip(t12 - t19a, min, max);
    *c
        .offset(
            (20 * stride) as isize,
        ) = iclip(t11 - t20, min, max);
    *c
        .offset(
            (21 * stride) as isize,
        ) = iclip(t10 - t21a, min, max);
    *c
        .offset(
            (22 * stride) as isize,
        ) = iclip(t9 - t22, min, max);
    *c
        .offset(
            (23 * stride) as isize,
        ) = iclip(t8 - t23a, min, max);
    *c
        .offset(
            (24 * stride) as isize,
        ) = iclip(t7 - t24a, min, max);
    *c
        .offset(
            (25 * stride) as isize,
        ) = iclip(t6 - t25, min, max);
    *c
        .offset(
            (26 * stride) as isize,
        ) = iclip(t5 - t26a, min, max);
    *c
        .offset(
            (27 * stride) as isize,
        ) = iclip(t4 - t27, min, max);
    *c
        .offset(
            (28 * stride) as isize,
        ) = iclip(t3 - t28a, min, max);
    *c
        .offset(
            (29 * stride) as isize,
        ) = iclip(t2 - t29, min, max);
    *c
        .offset(
            (30 * stride) as isize,
        ) = iclip(t1 - t30a, min, max);
    *c
        .offset(
            (31 * stride) as isize,
        ) = iclip(t0 - t31, min, max);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_dct32_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_dct32_1d_internal_c(c, stride, min, max, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_dct64_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    if !(stride > 0) {
        unreachable!();
    }
    inv_dct32_1d_internal_c(c, stride << 1, min, max, 1 as libc::c_int);
    let in1 = *c
        .offset((1 * stride) as isize);
    let in3 = *c
        .offset((3 * stride) as isize);
    let in5 = *c
        .offset((5 * stride) as isize);
    let in7 = *c
        .offset((7 * stride) as isize);
    let in9 = *c
        .offset((9 * stride) as isize);
    let in11 = *c
        .offset((11 * stride) as isize);
    let in13 = *c
        .offset((13 * stride) as isize);
    let in15 = *c
        .offset((15 * stride) as isize);
    let in17 = *c
        .offset((17 * stride) as isize);
    let in19 = *c
        .offset((19 * stride) as isize);
    let in21 = *c
        .offset((21 * stride) as isize);
    let in23 = *c
        .offset((23 * stride) as isize);
    let in25 = *c
        .offset((25 * stride) as isize);
    let in27 = *c
        .offset((27 * stride) as isize);
    let in29 = *c
        .offset((29 * stride) as isize);
    let in31 = *c
        .offset((31 * stride) as isize);
    let mut t32a = in1 * 101 + 2048
        >> 12;
    let mut t33a = in31 * -(2824 as libc::c_int) + 2048
        >> 12;
    let mut t34a = in17 * 1660 + 2048
        >> 12;
    let mut t35a = in15 * -(1474 as libc::c_int) + 2048
        >> 12;
    let mut t36a = in9 * 897 + 2048
        >> 12;
    let mut t37a = in23 * -(2191 as libc::c_int) + 2048
        >> 12;
    let mut t38a = in25 * 2359 + 2048
        >> 12;
    let mut t39a = in7 * -(700 as libc::c_int) + 2048
        >> 12;
    let mut t40a = in5 * 501 + 2048
        >> 12;
    let mut t41a = in27 * -(2520 as libc::c_int) + 2048
        >> 12;
    let mut t42a = in21 * 2019 + 2048
        >> 12;
    let mut t43a = in11 * -(1092 as libc::c_int) + 2048
        >> 12;
    let mut t44a = in13 * 1285 + 2048
        >> 12;
    let mut t45a = in19 * -(1842 as libc::c_int) + 2048
        >> 12;
    let mut t46a = in29 * 2675 + 2048
        >> 12;
    let mut t47a = in3 * -(301 as libc::c_int) + 2048
        >> 12;
    let mut t48a = in3 * 4085 + 2048
        >> 12;
    let mut t49a = in29 * 3102 + 2048
        >> 12;
    let mut t50a = in19 * 3659 + 2048
        >> 12;
    let mut t51a = in13 * 3889 + 2048
        >> 12;
    let mut t52a = in11 * 3948 + 2048
        >> 12;
    let mut t53a = in21 * 3564 + 2048
        >> 12;
    let mut t54a = in27 * 3229 + 2048
        >> 12;
    let mut t55a = in5 * 4065 + 2048
        >> 12;
    let mut t56a = in7 * 4036 + 2048
        >> 12;
    let mut t57a = in25 * 3349 + 2048
        >> 12;
    let mut t58a = in23 * 3461 + 2048
        >> 12;
    let mut t59a = in9 * 3996 + 2048
        >> 12;
    let mut t60a = in15 * 3822 + 2048
        >> 12;
    let mut t61a = in17 * 3745 + 2048
        >> 12;
    let mut t62a = in31 * 2967 + 2048
        >> 12;
    let mut t63a = in1 * 4095 + 2048
        >> 12;
    let mut t32 = iclip(t32a + t33a, min, max);
    let mut t33 = iclip(t32a - t33a, min, max);
    let mut t34 = iclip(t35a - t34a, min, max);
    let mut t35 = iclip(t35a + t34a, min, max);
    let mut t36 = iclip(t36a + t37a, min, max);
    let mut t37 = iclip(t36a - t37a, min, max);
    let mut t38 = iclip(t39a - t38a, min, max);
    let mut t39 = iclip(t39a + t38a, min, max);
    let mut t40 = iclip(t40a + t41a, min, max);
    let mut t41 = iclip(t40a - t41a, min, max);
    let mut t42 = iclip(t43a - t42a, min, max);
    let mut t43 = iclip(t43a + t42a, min, max);
    let mut t44 = iclip(t44a + t45a, min, max);
    let mut t45 = iclip(t44a - t45a, min, max);
    let mut t46 = iclip(t47a - t46a, min, max);
    let mut t47 = iclip(t47a + t46a, min, max);
    let mut t48 = iclip(t48a + t49a, min, max);
    let mut t49 = iclip(t48a - t49a, min, max);
    let mut t50 = iclip(t51a - t50a, min, max);
    let mut t51 = iclip(t51a + t50a, min, max);
    let mut t52 = iclip(t52a + t53a, min, max);
    let mut t53 = iclip(t52a - t53a, min, max);
    let mut t54 = iclip(t55a - t54a, min, max);
    let mut t55 = iclip(t55a + t54a, min, max);
    let mut t56 = iclip(t56a + t57a, min, max);
    let mut t57 = iclip(t56a - t57a, min, max);
    let mut t58 = iclip(t59a - t58a, min, max);
    let mut t59 = iclip(t59a + t58a, min, max);
    let mut t60 = iclip(t60a + t61a, min, max);
    let mut t61 = iclip(t60a - t61a, min, max);
    let mut t62 = iclip(t63a - t62a, min, max);
    let mut t63 = iclip(t63a + t62a, min, max);
    t33a = (t33 * (4096 - 4076) + t62 * 401
        + 2048 >> 12) - t33;
    t34a = (t34 * -(401 as libc::c_int)
        + t61 * (4096 - 4076) + 2048
        >> 12) - t61;
    t37a = t37 * -(1299 as libc::c_int) + t58 * 1583 + 1024
        >> 11;
    t38a = t38 * -(1583 as libc::c_int) + t57 * -(1299 as libc::c_int)
        + 1024 >> 11;
    t41a = (t41 * (4096 - 3612) + t54 * 1931
        + 2048 >> 12) - t41;
    t42a = (t42 * -(1931 as libc::c_int)
        + t53 * (4096 - 3612) + 2048
        >> 12) - t53;
    t45a = (t45 * -(1189 as libc::c_int)
        + t50 * (3920 - 4096) + 2048
        >> 12) + t50;
    t46a = (t46 * (4096 - 3920)
        + t49 * -(1189 as libc::c_int) + 2048 >> 12) - t46;
    t49a = (t46 * -(1189 as libc::c_int)
        + t49 * (3920 - 4096) + 2048
        >> 12) + t49;
    t50a = (t45 * (3920 - 4096) + t50 * 1189
        + 2048 >> 12) + t45;
    t53a = (t42 * (4096 - 3612) + t53 * 1931
        + 2048 >> 12) - t42;
    t54a = (t41 * 1931 + t54 * (3612 - 4096)
        + 2048 >> 12) + t54;
    t57a = t38 * -(1299 as libc::c_int) + t57 * 1583 + 1024
        >> 11;
    t58a = t37 * 1583 + t58 * 1299 + 1024
        >> 11;
    t61a = (t34 * (4096 - 4076) + t61 * 401
        + 2048 >> 12) - t34;
    t62a = (t33 * 401 + t62 * (4076 - 4096)
        + 2048 >> 12) + t62;
    t32a = iclip(t32 + t35, min, max);
    t33 = iclip(t33a + t34a, min, max);
    t34 = iclip(t33a - t34a, min, max);
    t35a = iclip(t32 - t35, min, max);
    t36a = iclip(t39 - t36, min, max);
    t37 = iclip(t38a - t37a, min, max);
    t38 = iclip(t38a + t37a, min, max);
    t39a = iclip(t39 + t36, min, max);
    t40a = iclip(t40 + t43, min, max);
    t41 = iclip(t41a + t42a, min, max);
    t42 = iclip(t41a - t42a, min, max);
    t43a = iclip(t40 - t43, min, max);
    t44a = iclip(t47 - t44, min, max);
    t45 = iclip(t46a - t45a, min, max);
    t46 = iclip(t46a + t45a, min, max);
    t47a = iclip(t47 + t44, min, max);
    t48a = iclip(t48 + t51, min, max);
    t49 = iclip(t49a + t50a, min, max);
    t50 = iclip(t49a - t50a, min, max);
    t51a = iclip(t48 - t51, min, max);
    t52a = iclip(t55 - t52, min, max);
    t53 = iclip(t54a - t53a, min, max);
    t54 = iclip(t54a + t53a, min, max);
    t55a = iclip(t55 + t52, min, max);
    t56a = iclip(t56 + t59, min, max);
    t57 = iclip(t57a + t58a, min, max);
    t58 = iclip(t57a - t58a, min, max);
    t59a = iclip(t56 - t59, min, max);
    t60a = iclip(t63 - t60, min, max);
    t61 = iclip(t62a - t61a, min, max);
    t62 = iclip(t62a + t61a, min, max);
    t63a = iclip(t63 + t60, min, max);
    t34a = (t34 * (4096 - 4017) + t61 * 799
        + 2048 >> 12) - t34;
    t35 = (t35a * (4096 - 4017) + t60a * 799
        + 2048 >> 12) - t35a;
    t36 = (t36a * -(799 as libc::c_int)
        + t59a * (4096 - 4017) + 2048
        >> 12) - t59a;
    t37a = (t37 * -(799 as libc::c_int)
        + t58 * (4096 - 4017) + 2048
        >> 12) - t58;
    t42a = t42 * -(1138 as libc::c_int) + t53 * 1703 + 1024
        >> 11;
    t43 = t43a * -(1138 as libc::c_int) + t52a * 1703
        + 1024 >> 11;
    t44 = t44a * -(1703 as libc::c_int) + t51a * -(1138 as libc::c_int)
        + 1024 >> 11;
    t45a = t45 * -(1703 as libc::c_int) + t50 * -(1138 as libc::c_int)
        + 1024 >> 11;
    t50a = t45 * -(1138 as libc::c_int) + t50 * 1703 + 1024
        >> 11;
    t51 = t44a * -(1138 as libc::c_int) + t51a * 1703
        + 1024 >> 11;
    t52 = t43a * 1703 + t52a * 1138 + 1024
        >> 11;
    t53a = t42 * 1703 + t53 * 1138 + 1024
        >> 11;
    t58a = (t37 * (4096 - 4017) + t58 * 799
        + 2048 >> 12) - t37;
    t59 = (t36a * (4096 - 4017) + t59a * 799
        + 2048 >> 12) - t36a;
    t60 = (t35a * 799 + t60a * (4017 - 4096)
        + 2048 >> 12) + t60a;
    t61a = (t34 * 799 + t61 * (4017 - 4096)
        + 2048 >> 12) + t61;
    t32 = iclip(t32a + t39a, min, max);
    t33a = iclip(t33 + t38, min, max);
    t34 = iclip(t34a + t37a, min, max);
    t35a = iclip(t35 + t36, min, max);
    t36a = iclip(t35 - t36, min, max);
    t37 = iclip(t34a - t37a, min, max);
    t38a = iclip(t33 - t38, min, max);
    t39 = iclip(t32a - t39a, min, max);
    t40 = iclip(t47a - t40a, min, max);
    t41a = iclip(t46 - t41, min, max);
    t42 = iclip(t45a - t42a, min, max);
    t43a = iclip(t44 - t43, min, max);
    t44a = iclip(t44 + t43, min, max);
    t45 = iclip(t45a + t42a, min, max);
    t46a = iclip(t46 + t41, min, max);
    t47 = iclip(t47a + t40a, min, max);
    t48 = iclip(t48a + t55a, min, max);
    t49a = iclip(t49 + t54, min, max);
    t50 = iclip(t50a + t53a, min, max);
    t51a = iclip(t51 + t52, min, max);
    t52a = iclip(t51 - t52, min, max);
    t53 = iclip(t50a - t53a, min, max);
    t54a = iclip(t49 - t54, min, max);
    t55 = iclip(t48a - t55a, min, max);
    t56 = iclip(t63a - t56a, min, max);
    t57a = iclip(t62 - t57, min, max);
    t58 = iclip(t61a - t58a, min, max);
    t59a = iclip(t60 - t59, min, max);
    t60a = iclip(t60 + t59, min, max);
    t61 = iclip(t61a + t58a, min, max);
    t62a = iclip(t62 + t57, min, max);
    t63 = iclip(t63a + t56a, min, max);
    t36 = (t36a * (4096 - 3784)
        + t59a * 1567 + 2048 >> 12) - t36a;
    t37a = (t37 * (4096 - 3784) + t58 * 1567
        + 2048 >> 12) - t37;
    t38 = (t38a * (4096 - 3784)
        + t57a * 1567 + 2048 >> 12) - t38a;
    t39a = (t39 * (4096 - 3784) + t56 * 1567
        + 2048 >> 12) - t39;
    t40a = (t40 * -(1567 as libc::c_int)
        + t55 * (4096 - 3784) + 2048
        >> 12) - t55;
    t41 = (t41a * -(1567 as libc::c_int)
        + t54a * (4096 - 3784) + 2048
        >> 12) - t54a;
    t42a = (t42 * -(1567 as libc::c_int)
        + t53 * (4096 - 3784) + 2048
        >> 12) - t53;
    t43 = (t43a * -(1567 as libc::c_int)
        + t52a * (4096 - 3784) + 2048
        >> 12) - t52a;
    t52 = (t43a * (4096 - 3784)
        + t52a * 1567 + 2048 >> 12) - t43a;
    t53a = (t42 * (4096 - 3784) + t53 * 1567
        + 2048 >> 12) - t42;
    t54 = (t41a * (4096 - 3784)
        + t54a * 1567 + 2048 >> 12) - t41a;
    t55a = (t40 * (4096 - 3784) + t55 * 1567
        + 2048 >> 12) - t40;
    t56a = (t39 * 1567 + t56 * (3784 - 4096)
        + 2048 >> 12) + t56;
    t57 = (t38a * 1567
        + t57a * (3784 - 4096) + 2048
        >> 12) + t57a;
    t58a = (t37 * 1567 + t58 * (3784 - 4096)
        + 2048 >> 12) + t58;
    t59 = (t36a * 1567
        + t59a * (3784 - 4096) + 2048
        >> 12) + t59a;
    t32a = iclip(t32 + t47, min, max);
    t33 = iclip(t33a + t46a, min, max);
    t34a = iclip(t34 + t45, min, max);
    t35 = iclip(t35a + t44a, min, max);
    t36a = iclip(t36 + t43, min, max);
    t37 = iclip(t37a + t42a, min, max);
    t38a = iclip(t38 + t41, min, max);
    t39 = iclip(t39a + t40a, min, max);
    t40 = iclip(t39a - t40a, min, max);
    t41a = iclip(t38 - t41, min, max);
    t42 = iclip(t37a - t42a, min, max);
    t43a = iclip(t36 - t43, min, max);
    t44 = iclip(t35a - t44a, min, max);
    t45a = iclip(t34 - t45, min, max);
    t46 = iclip(t33a - t46a, min, max);
    t47a = iclip(t32 - t47, min, max);
    t48a = iclip(t63 - t48, min, max);
    t49 = iclip(t62a - t49a, min, max);
    t50a = iclip(t61 - t50, min, max);
    t51 = iclip(t60a - t51a, min, max);
    t52a = iclip(t59 - t52, min, max);
    t53 = iclip(t58a - t53a, min, max);
    t54a = iclip(t57 - t54, min, max);
    t55 = iclip(t56a - t55a, min, max);
    t56 = iclip(t56a + t55a, min, max);
    t57a = iclip(t57 + t54, min, max);
    t58 = iclip(t58a + t53a, min, max);
    t59a = iclip(t59 + t52, min, max);
    t60 = iclip(t60a + t51a, min, max);
    t61a = iclip(t61 + t50, min, max);
    t62 = iclip(t62a + t49a, min, max);
    t63a = iclip(t63 + t48, min, max);
    t40a = (t55 - t40) * 181 + 128 >> 8;
    t41 = (t54a - t41a) * 181 + 128 >> 8;
    t42a = (t53 - t42) * 181 + 128 >> 8;
    t43 = (t52a - t43a) * 181 + 128 >> 8;
    t44a = (t51 - t44) * 181 + 128 >> 8;
    t45 = (t50a - t45a) * 181 + 128 >> 8;
    t46a = (t49 - t46) * 181 + 128 >> 8;
    t47 = (t48a - t47a) * 181 + 128 >> 8;
    t48 = (t47a + t48a) * 181 + 128 >> 8;
    t49a = (t46 + t49) * 181 + 128 >> 8;
    t50 = (t45a + t50a) * 181 + 128 >> 8;
    t51a = (t44 + t51) * 181 + 128 >> 8;
    t52 = (t43a + t52a) * 181 + 128 >> 8;
    t53a = (t42 + t53) * 181 + 128 >> 8;
    t54 = (t41a + t54a) * 181 + 128 >> 8;
    t55a = (t40 + t55) * 181 + 128 >> 8;
    let t0 = *c
        .offset((0 * stride) as isize);
    let t1 = *c
        .offset((2 * stride) as isize);
    let t2 = *c
        .offset((4 * stride) as isize);
    let t3 = *c
        .offset((6 * stride) as isize);
    let t4 = *c
        .offset((8 * stride) as isize);
    let t5 = *c
        .offset((10 * stride) as isize);
    let t6 = *c
        .offset((12 * stride) as isize);
    let t7 = *c
        .offset((14 * stride) as isize);
    let t8 = *c
        .offset((16 * stride) as isize);
    let t9 = *c
        .offset((18 * stride) as isize);
    let t10 = *c
        .offset((20 * stride) as isize);
    let t11 = *c
        .offset((22 * stride) as isize);
    let t12 = *c
        .offset((24 * stride) as isize);
    let t13 = *c
        .offset((26 * stride) as isize);
    let t14 = *c
        .offset((28 * stride) as isize);
    let t15 = *c
        .offset((30 * stride) as isize);
    let t16 = *c
        .offset((32 * stride) as isize);
    let t17 = *c
        .offset((34 * stride) as isize);
    let t18 = *c
        .offset((36 * stride) as isize);
    let t19 = *c
        .offset((38 * stride) as isize);
    let t20 = *c
        .offset((40 * stride) as isize);
    let t21 = *c
        .offset((42 * stride) as isize);
    let t22 = *c
        .offset((44 * stride) as isize);
    let t23 = *c
        .offset((46 * stride) as isize);
    let t24 = *c
        .offset((48 * stride) as isize);
    let t25 = *c
        .offset((50 * stride) as isize);
    let t26 = *c
        .offset((52 * stride) as isize);
    let t27 = *c
        .offset((54 * stride) as isize);
    let t28 = *c
        .offset((56 * stride) as isize);
    let t29 = *c
        .offset((58 * stride) as isize);
    let t30 = *c
        .offset((60 * stride) as isize);
    let t31 = *c
        .offset((62 * stride) as isize);
    *c
        .offset(
            (0 * stride) as isize,
        ) = iclip(t0 + t63a, min, max);
    *c
        .offset(
            (1 * stride) as isize,
        ) = iclip(t1 + t62, min, max);
    *c
        .offset(
            (2 * stride) as isize,
        ) = iclip(t2 + t61a, min, max);
    *c
        .offset(
            (3 * stride) as isize,
        ) = iclip(t3 + t60, min, max);
    *c
        .offset(
            (4 * stride) as isize,
        ) = iclip(t4 + t59a, min, max);
    *c
        .offset(
            (5 * stride) as isize,
        ) = iclip(t5 + t58, min, max);
    *c
        .offset(
            (6 * stride) as isize,
        ) = iclip(t6 + t57a, min, max);
    *c
        .offset(
            (7 * stride) as isize,
        ) = iclip(t7 + t56, min, max);
    *c
        .offset(
            (8 * stride) as isize,
        ) = iclip(t8 + t55a, min, max);
    *c
        .offset(
            (9 * stride) as isize,
        ) = iclip(t9 + t54, min, max);
    *c
        .offset(
            (10 * stride) as isize,
        ) = iclip(t10 + t53a, min, max);
    *c
        .offset(
            (11 * stride) as isize,
        ) = iclip(t11 + t52, min, max);
    *c
        .offset(
            (12 * stride) as isize,
        ) = iclip(t12 + t51a, min, max);
    *c
        .offset(
            (13 * stride) as isize,
        ) = iclip(t13 + t50, min, max);
    *c
        .offset(
            (14 * stride) as isize,
        ) = iclip(t14 + t49a, min, max);
    *c
        .offset(
            (15 * stride) as isize,
        ) = iclip(t15 + t48, min, max);
    *c
        .offset(
            (16 * stride) as isize,
        ) = iclip(t16 + t47, min, max);
    *c
        .offset(
            (17 * stride) as isize,
        ) = iclip(t17 + t46a, min, max);
    *c
        .offset(
            (18 * stride) as isize,
        ) = iclip(t18 + t45, min, max);
    *c
        .offset(
            (19 * stride) as isize,
        ) = iclip(t19 + t44a, min, max);
    *c
        .offset(
            (20 * stride) as isize,
        ) = iclip(t20 + t43, min, max);
    *c
        .offset(
            (21 * stride) as isize,
        ) = iclip(t21 + t42a, min, max);
    *c
        .offset(
            (22 * stride) as isize,
        ) = iclip(t22 + t41, min, max);
    *c
        .offset(
            (23 * stride) as isize,
        ) = iclip(t23 + t40a, min, max);
    *c
        .offset(
            (24 * stride) as isize,
        ) = iclip(t24 + t39, min, max);
    *c
        .offset(
            (25 * stride) as isize,
        ) = iclip(t25 + t38a, min, max);
    *c
        .offset(
            (26 * stride) as isize,
        ) = iclip(t26 + t37, min, max);
    *c
        .offset(
            (27 * stride) as isize,
        ) = iclip(t27 + t36a, min, max);
    *c
        .offset(
            (28 * stride) as isize,
        ) = iclip(t28 + t35, min, max);
    *c
        .offset(
            (29 * stride) as isize,
        ) = iclip(t29 + t34a, min, max);
    *c
        .offset(
            (30 * stride) as isize,
        ) = iclip(t30 + t33, min, max);
    *c
        .offset(
            (31 * stride) as isize,
        ) = iclip(t31 + t32a, min, max);
    *c
        .offset(
            (32 * stride) as isize,
        ) = iclip(t31 - t32a, min, max);
    *c
        .offset(
            (33 * stride) as isize,
        ) = iclip(t30 - t33, min, max);
    *c
        .offset(
            (34 * stride) as isize,
        ) = iclip(t29 - t34a, min, max);
    *c
        .offset(
            (35 * stride) as isize,
        ) = iclip(t28 - t35, min, max);
    *c
        .offset(
            (36 * stride) as isize,
        ) = iclip(t27 - t36a, min, max);
    *c
        .offset(
            (37 * stride) as isize,
        ) = iclip(t26 - t37, min, max);
    *c
        .offset(
            (38 * stride) as isize,
        ) = iclip(t25 - t38a, min, max);
    *c
        .offset(
            (39 * stride) as isize,
        ) = iclip(t24 - t39, min, max);
    *c
        .offset(
            (40 * stride) as isize,
        ) = iclip(t23 - t40a, min, max);
    *c
        .offset(
            (41 * stride) as isize,
        ) = iclip(t22 - t41, min, max);
    *c
        .offset(
            (42 * stride) as isize,
        ) = iclip(t21 - t42a, min, max);
    *c
        .offset(
            (43 * stride) as isize,
        ) = iclip(t20 - t43, min, max);
    *c
        .offset(
            (44 * stride) as isize,
        ) = iclip(t19 - t44a, min, max);
    *c
        .offset(
            (45 * stride) as isize,
        ) = iclip(t18 - t45, min, max);
    *c
        .offset(
            (46 * stride) as isize,
        ) = iclip(t17 - t46a, min, max);
    *c
        .offset(
            (47 * stride) as isize,
        ) = iclip(t16 - t47, min, max);
    *c
        .offset(
            (48 * stride) as isize,
        ) = iclip(t15 - t48, min, max);
    *c
        .offset(
            (49 * stride) as isize,
        ) = iclip(t14 - t49a, min, max);
    *c
        .offset(
            (50 * stride) as isize,
        ) = iclip(t13 - t50, min, max);
    *c
        .offset(
            (51 * stride) as isize,
        ) = iclip(t12 - t51a, min, max);
    *c
        .offset(
            (52 * stride) as isize,
        ) = iclip(t11 - t52, min, max);
    *c
        .offset(
            (53 * stride) as isize,
        ) = iclip(t10 - t53a, min, max);
    *c
        .offset(
            (54 * stride) as isize,
        ) = iclip(t9 - t54, min, max);
    *c
        .offset(
            (55 * stride) as isize,
        ) = iclip(t8 - t55a, min, max);
    *c
        .offset(
            (56 * stride) as isize,
        ) = iclip(t7 - t56, min, max);
    *c
        .offset(
            (57 * stride) as isize,
        ) = iclip(t6 - t57a, min, max);
    *c
        .offset(
            (58 * stride) as isize,
        ) = iclip(t5 - t58, min, max);
    *c
        .offset(
            (59 * stride) as isize,
        ) = iclip(t4 - t59a, min, max);
    *c
        .offset(
            (60 * stride) as isize,
        ) = iclip(t3 - t60, min, max);
    *c
        .offset(
            (61 * stride) as isize,
        ) = iclip(t2 - t61a, min, max);
    *c
        .offset(
            (62 * stride) as isize,
        ) = iclip(t1 - t62, min, max);
    *c
        .offset(
            (63 * stride) as isize,
        ) = iclip(t0 - t63a, min, max);
}
#[inline(never)]
unsafe extern "C" fn inv_adst4_1d_internal_c(
    in_0: *const int32_t,
    in_s: ptrdiff_t,
    _min: libc::c_int,
    _max: libc::c_int,
    out: *mut int32_t,
    out_s: ptrdiff_t,
) {
    if !(in_s > 0
        && out_s != 0)
    {
        unreachable!();
    }
    let in0 = *in_0
        .offset((0 * in_s) as isize);
    let in1 = *in_0
        .offset((1 * in_s) as isize);
    let in2 = *in_0
        .offset((2 * in_s) as isize);
    let in3 = *in_0
        .offset((3 * in_s) as isize);
    *out
        .offset(
            (0 * out_s) as isize,
        ) = (1321 * in0
        + (3803 - 4096) * in2
        + (2482 - 4096) * in3
        + (3344 - 4096) * in1 + 2048
        >> 12) + in2 + in3 + in1;
    *out
        .offset(
            (1 * out_s) as isize,
        ) = ((2482 - 4096) * in0
        - 1321 * in2 - (3803 - 4096) * in3
        + (3344 - 4096) * in1 + 2048
        >> 12) + in0 - in3 + in1;
    *out
        .offset(
            (2 * out_s) as isize,
        ) = 209 * (in0 - in2 + in3) + 128
        >> 8;
    *out
        .offset(
            (3 * out_s) as isize,
        ) = ((3803 - 4096) * in0
        + (2482 - 4096) * in2 - 1321 * in3
        - (3344 - 4096) * in1 + 2048
        >> 12) + in0 + in2 - in1;
}
#[inline(never)]
unsafe extern "C" fn inv_adst8_1d_internal_c(
    in_0: *const int32_t,
    in_s: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
    out: *mut int32_t,
    out_s: ptrdiff_t,
) {
    if !(in_s > 0
        && out_s != 0)
    {
        unreachable!();
    }
    let in0 = *in_0
        .offset((0 * in_s) as isize);
    let in1 = *in_0
        .offset((1 * in_s) as isize);
    let in2 = *in_0
        .offset((2 * in_s) as isize);
    let in3 = *in_0
        .offset((3 * in_s) as isize);
    let in4 = *in_0
        .offset((4 * in_s) as isize);
    let in5 = *in_0
        .offset((5 * in_s) as isize);
    let in6 = *in_0
        .offset((6 * in_s) as isize);
    let in7 = *in_0
        .offset((7 * in_s) as isize);
    let t0a = ((4076 - 4096) * in7
        + 401 * in0 + 2048 >> 12) + in7;
    let t1a = (401 * in7
        - (4076 - 4096) * in0 + 2048
        >> 12) - in0;
    let t2a = ((3612 - 4096) * in5
        + 1931 * in2 + 2048 >> 12) + in5;
    let t3a = (1931 * in5
        - (3612 - 4096) * in2 + 2048
        >> 12) - in2;
    let mut t4a = 1299 * in3 + 1583 * in4
        + 1024 >> 11;
    let mut t5a = 1583 * in3 - 1299 * in4
        + 1024 >> 11;
    let mut t6a = (1189 * in1
        + (3920 - 4096) * in6 + 2048
        >> 12) + in6;
    let mut t7a = ((3920 - 4096) * in1
        - 1189 * in6 + 2048 >> 12) + in1;
    let t0 = iclip(t0a + t4a, min, max);
    let t1 = iclip(t1a + t5a, min, max);
    let mut t2 = iclip(t2a + t6a, min, max);
    let mut t3 = iclip(t3a + t7a, min, max);
    let t4 = iclip(t0a - t4a, min, max);
    let t5 = iclip(t1a - t5a, min, max);
    let mut t6 = iclip(t2a - t6a, min, max);
    let mut t7 = iclip(t3a - t7a, min, max);
    t4a = ((3784 - 4096) * t4 + 1567 * t5
        + 2048 >> 12) + t4;
    t5a = (1567 * t4 - (3784 - 4096) * t5
        + 2048 >> 12) - t5;
    t6a = ((3784 - 4096) * t7 - 1567 * t6
        + 2048 >> 12) + t7;
    t7a = (1567 * t7 + (3784 - 4096) * t6
        + 2048 >> 12) + t6;
    *out
        .offset(
            (0 * out_s) as isize,
        ) = iclip(t0 + t2, min, max);
    *out
        .offset(
            (7 * out_s) as isize,
        ) = -iclip(t1 + t3, min, max);
    t2 = iclip(t0 - t2, min, max);
    t3 = iclip(t1 - t3, min, max);
    *out
        .offset(
            (1 * out_s) as isize,
        ) = -iclip(t4a + t6a, min, max);
    *out
        .offset(
            (6 * out_s) as isize,
        ) = iclip(t5a + t7a, min, max);
    t6 = iclip(t4a - t6a, min, max);
    t7 = iclip(t5a - t7a, min, max);
    *out
        .offset(
            (3 * out_s) as isize,
        ) = -((t2 + t3) * 181 + 128 >> 8);
    *out
        .offset(
            (4 * out_s) as isize,
        ) = (t2 - t3) * 181 + 128 >> 8;
    *out
        .offset(
            (2 * out_s) as isize,
        ) = (t6 + t7) * 181 + 128 >> 8;
    *out
        .offset(
            (5 * out_s) as isize,
        ) = -((t6 - t7) * 181 + 128 >> 8);
}
#[inline(never)]
unsafe extern "C" fn inv_adst16_1d_internal_c(
    in_0: *const int32_t,
    in_s: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
    out: *mut int32_t,
    out_s: ptrdiff_t,
) {
    if !(in_s > 0
        && out_s != 0)
    {
        unreachable!();
    }
    let in0 = *in_0
        .offset((0 * in_s) as isize);
    let in1 = *in_0
        .offset((1 * in_s) as isize);
    let in2 = *in_0
        .offset((2 * in_s) as isize);
    let in3 = *in_0
        .offset((3 * in_s) as isize);
    let in4 = *in_0
        .offset((4 * in_s) as isize);
    let in5 = *in_0
        .offset((5 * in_s) as isize);
    let in6 = *in_0
        .offset((6 * in_s) as isize);
    let in7 = *in_0
        .offset((7 * in_s) as isize);
    let in8 = *in_0
        .offset((8 * in_s) as isize);
    let in9 = *in_0
        .offset((9 * in_s) as isize);
    let in10 = *in_0
        .offset((10 * in_s) as isize);
    let in11 = *in_0
        .offset((11 * in_s) as isize);
    let in12 = *in_0
        .offset((12 * in_s) as isize);
    let in13 = *in_0
        .offset((13 * in_s) as isize);
    let in14 = *in_0
        .offset((14 * in_s) as isize);
    let in15 = *in_0
        .offset((15 * in_s) as isize);
    let mut t0 = (in15 * (4091 - 4096)
        + in0 * 201 + 2048 >> 12) + in15;
    let mut t1 = (in15 * 201
        - in0 * (4091 - 4096) + 2048
        >> 12) - in0;
    let mut t2 = (in13 * (3973 - 4096)
        + in2 * 995 + 2048 >> 12) + in13;
    let mut t3 = (in13 * 995
        - in2 * (3973 - 4096) + 2048
        >> 12) - in2;
    let mut t4 = (in11 * (3703 - 4096)
        + in4 * 1751 + 2048 >> 12) + in11;
    let mut t5 = (in11 * 1751
        - in4 * (3703 - 4096) + 2048
        >> 12) - in4;
    let mut t6 = in9 * 1645 + in6 * 1220
        + 1024 >> 11;
    let mut t7 = in9 * 1220 - in6 * 1645
        + 1024 >> 11;
    let mut t8 = (in7 * 2751
        + in8 * (3035 - 4096) + 2048
        >> 12) + in8;
    let mut t9 = (in7 * (3035 - 4096)
        - in8 * 2751 + 2048 >> 12) + in7;
    let mut t10 = (in5 * 2106
        + in10 * (3513 - 4096) + 2048
        >> 12) + in10;
    let mut t11 = (in5 * (3513 - 4096)
        - in10 * 2106 + 2048 >> 12) + in5;
    let mut t12 = (in3 * 1380
        + in12 * (3857 - 4096) + 2048
        >> 12) + in12;
    let mut t13 = (in3 * (3857 - 4096)
        - in12 * 1380 + 2048 >> 12) + in3;
    let mut t14 = (in1 * 601
        + in14 * (4052 - 4096) + 2048
        >> 12) + in14;
    let mut t15 = (in1 * (4052 - 4096)
        - in14 * 601 + 2048 >> 12) + in1;
    let mut t0a = iclip(t0 + t8, min, max);
    let mut t1a = iclip(t1 + t9, min, max);
    let mut t2a = iclip(t2 + t10, min, max);
    let mut t3a = iclip(t3 + t11, min, max);
    let mut t4a = iclip(t4 + t12, min, max);
    let mut t5a = iclip(t5 + t13, min, max);
    let mut t6a = iclip(t6 + t14, min, max);
    let mut t7a = iclip(t7 + t15, min, max);
    let mut t8a = iclip(t0 - t8, min, max);
    let mut t9a = iclip(t1 - t9, min, max);
    let mut t10a = iclip(t2 - t10, min, max);
    let mut t11a = iclip(t3 - t11, min, max);
    let mut t12a = iclip(t4 - t12, min, max);
    let mut t13a = iclip(t5 - t13, min, max);
    let mut t14a = iclip(t6 - t14, min, max);
    let mut t15a = iclip(t7 - t15, min, max);
    t8 = (t8a * (4017 - 4096) + t9a * 799
        + 2048 >> 12) + t8a;
    t9 = (t8a * 799 - t9a * (4017 - 4096)
        + 2048 >> 12) - t9a;
    t10 = (t10a * 2276
        + t11a * (3406 - 4096) + 2048
        >> 12) + t11a;
    t11 = (t10a * (3406 - 4096)
        - t11a * 2276 + 2048 >> 12) + t10a;
    t12 = (t13a * (4017 - 4096) - t12a * 799
        + 2048 >> 12) + t13a;
    t13 = (t13a * 799 + t12a * (4017 - 4096)
        + 2048 >> 12) + t12a;
    t14 = (t15a * 2276
        - t14a * (3406 - 4096) + 2048
        >> 12) - t14a;
    t15 = (t15a * (3406 - 4096)
        + t14a * 2276 + 2048 >> 12) + t15a;
    t0 = iclip(t0a + t4a, min, max);
    t1 = iclip(t1a + t5a, min, max);
    t2 = iclip(t2a + t6a, min, max);
    t3 = iclip(t3a + t7a, min, max);
    t4 = iclip(t0a - t4a, min, max);
    t5 = iclip(t1a - t5a, min, max);
    t6 = iclip(t2a - t6a, min, max);
    t7 = iclip(t3a - t7a, min, max);
    t8a = iclip(t8 + t12, min, max);
    t9a = iclip(t9 + t13, min, max);
    t10a = iclip(t10 + t14, min, max);
    t11a = iclip(t11 + t15, min, max);
    t12a = iclip(t8 - t12, min, max);
    t13a = iclip(t9 - t13, min, max);
    t14a = iclip(t10 - t14, min, max);
    t15a = iclip(t11 - t15, min, max);
    t4a = (t4 * (3784 - 4096) + t5 * 1567
        + 2048 >> 12) + t4;
    t5a = (t4 * 1567 - t5 * (3784 - 4096)
        + 2048 >> 12) - t5;
    t6a = (t7 * (3784 - 4096) - t6 * 1567
        + 2048 >> 12) + t7;
    t7a = (t7 * 1567 + t6 * (3784 - 4096)
        + 2048 >> 12) + t6;
    t12 = (t12a * (3784 - 4096)
        + t13a * 1567 + 2048 >> 12) + t12a;
    t13 = (t12a * 1567
        - t13a * (3784 - 4096) + 2048
        >> 12) - t13a;
    t14 = (t15a * (3784 - 4096)
        - t14a * 1567 + 2048 >> 12) + t15a;
    t15 = (t15a * 1567
        + t14a * (3784 - 4096) + 2048
        >> 12) + t14a;
    *out
        .offset(
            (0 * out_s) as isize,
        ) = iclip(t0 + t2, min, max);
    *out
        .offset(
            (15 * out_s) as isize,
        ) = -iclip(t1 + t3, min, max);
    t2a = iclip(t0 - t2, min, max);
    t3a = iclip(t1 - t3, min, max);
    *out
        .offset(
            (3 * out_s) as isize,
        ) = -iclip(t4a + t6a, min, max);
    *out
        .offset(
            (12 * out_s) as isize,
        ) = iclip(t5a + t7a, min, max);
    t6 = iclip(t4a - t6a, min, max);
    t7 = iclip(t5a - t7a, min, max);
    *out
        .offset(
            (1 * out_s) as isize,
        ) = -iclip(t8a + t10a, min, max);
    *out
        .offset(
            (14 * out_s) as isize,
        ) = iclip(t9a + t11a, min, max);
    t10 = iclip(t8a - t10a, min, max);
    t11 = iclip(t9a - t11a, min, max);
    *out
        .offset(
            (2 * out_s) as isize,
        ) = iclip(t12 + t14, min, max);
    *out
        .offset(
            (13 * out_s) as isize,
        ) = -iclip(t13 + t15, min, max);
    t14a = iclip(t12 - t14, min, max);
    t15a = iclip(t13 - t15, min, max);
    *out
        .offset(
            (7 * out_s) as isize,
        ) = -((t2a + t3a) * 181 + 128 >> 8);
    *out
        .offset(
            (8 * out_s) as isize,
        ) = (t2a - t3a) * 181 + 128 >> 8;
    *out
        .offset(
            (4 * out_s) as isize,
        ) = (t6 + t7) * 181 + 128 >> 8;
    *out
        .offset(
            (11 * out_s) as isize,
        ) = -((t6 - t7) * 181 + 128 >> 8);
    *out
        .offset(
            (6 * out_s) as isize,
        ) = (t10 + t11) * 181 + 128 >> 8;
    *out
        .offset(
            (9 * out_s) as isize,
        ) = -((t10 - t11) * 181 + 128 >> 8);
    *out
        .offset(
            (5 * out_s) as isize,
        ) = -((t14a + t15a) * 181 + 128
        >> 8);
    *out
        .offset(
            (10 * out_s) as isize,
        ) = (t14a - t15a) * 181 + 128 >> 8;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_flipadst4_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_adst4_1d_internal_c(
        c,
        stride,
        min,
        max,
        &mut *c
            .offset(
                ((4 - 1) as isize * stride) as isize,
            ),
        -stride,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_adst4_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_adst4_1d_internal_c(c, stride, min, max, c, stride);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_adst8_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_adst8_1d_internal_c(c, stride, min, max, c, stride);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_flipadst8_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_adst8_1d_internal_c(
        c,
        stride,
        min,
        max,
        &mut *c
            .offset(
                ((8 - 1) as isize * stride) as isize,
            ),
        -stride,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_flipadst16_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_adst16_1d_internal_c(
        c,
        stride,
        min,
        max,
        &mut *c
            .offset(
                (16 - 1) as isize * stride,
            ),
        -stride,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_adst16_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    min: libc::c_int,
    max: libc::c_int,
) {
    inv_adst16_1d_internal_c(c, stride, min, max, c, stride);
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_identity4_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    _min: libc::c_int,
    _max: libc::c_int,
) {
    if !(stride > 0) {
        unreachable!();
    }
    let mut i = 0;
    while i < 4 {
        let in_0 = *c.offset(stride * i as isize);
        *c
            .offset(
                stride * i as isize,
            ) = in_0
            + (in_0 * 1697 + 2048 >> 12);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_identity8_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    _min: libc::c_int,
    _max: libc::c_int,
) {
    if !(stride > 0) {
        unreachable!();
    }
    let mut i = 0;
    while i < 8 {
        let ref mut fresh0 = *c.offset((stride * i as isize) as isize);
        *fresh0 *= 2 as libc::c_int;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_identity16_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    _min: libc::c_int,
    _max: libc::c_int,
) {
    if !(stride > 0) {
        unreachable!();
    }
    let mut i = 0;
    while i < 16 {
        let in_0 = *c.offset((stride * i as isize) as isize);
        *c
            .offset(
                (stride * i as isize) as isize,
            ) = 2 * in_0
            + (in_0 * 1697 + 1024 >> 11);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_identity32_1d_c(
    c: *mut int32_t,
    stride: ptrdiff_t,
    _min: libc::c_int,
    _max: libc::c_int,
) {
    if !(stride > 0) {
        unreachable!();
    }
    let mut i = 0;
    while i < 32 {
        let ref mut fresh1 = *c.offset((stride * i as isize) as isize);
        *fresh1 *= 4 as libc::c_int;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_inv_wht4_1d_c(c: *mut int32_t, stride: ptrdiff_t) {
    if !(stride > 0) {
        unreachable!();
    }
    let in0 = *c
        .offset((0 * stride) as isize);
    let in1 = *c
        .offset((1 * stride) as isize);
    let in2 = *c
        .offset((2 * stride) as isize);
    let in3 = *c
        .offset((3 * stride) as isize);
    let t0 = in0 + in1;
    let t2 = in2 - in3;
    let t4 = t0 - t2 >> 1;
    let t3 = t4 - in3;
    let t1 = t4 - in1;
    *c.offset((0 * stride) as isize) = t0 - t3;
    *c.offset((1 * stride) as isize) = t3;
    *c.offset((2 * stride) as isize) = t1;
    *c.offset((3 * stride) as isize) = t2 + t1;
}
