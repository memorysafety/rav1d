use crate::include::common::bitdepth::BitDepth8;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::env::BlockContext;

use crate::src::internal::Rav1dFrameContext;

use crate::src::lf_apply::filter_plane_cols_uv;
use crate::src::lf_apply::filter_plane_cols_y;
use crate::src::lf_apply::filter_plane_rows_uv;
use crate::src::lf_apply::filter_plane_rows_y;
use crate::src::lf_mask::Av1Filter;

use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type pixel = u8;

pub(crate) unsafe fn rav1d_loopfilter_sbrow_rows_8bpc(
    f: *const Rav1dFrameContext,
    p: *const *mut pixel,
    lflvl: *mut Av1Filter,
    sby: c_int,
) {
    let mut x;
    let have_top = (sby > 0) as c_int;
    let is_sb64 = ((*(*f).seq_hdr).sb128 == 0) as c_int;
    let starty4 = (sby & is_sb64) << 4;
    let sbsz = 32 >> is_sb64;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor =
        ((*f).cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let endy4: c_uint = (starty4 + cmp::min((*f).h4 - sby * sbsz, sbsz)) as c_uint;
    let uv_endy4: c_uint = endy4.wrapping_add(ss_ver as c_uint) >> ss_ver;
    let mut ptr: *mut pixel;
    let mut level_ptr: *mut [u8; 4] =
        ((*f).lf.level).offset(((*f).b4_stride * sby as isize * sbsz as isize) as isize);
    ptr = *p.offset(0);
    x = 0 as c_int;
    while x < (*f).sb128w {
        filter_plane_rows_y::<BitDepth8>(
            f,
            have_top,
            level_ptr as *const [u8; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_y[1]).as_mut_ptr() as *const [[u16; 2]; 3],
            ptr,
            (*f).cur.stride[0],
            cmp::min(32 as c_int, (*f).w4 - x * 32),
            starty4,
            endy4 as c_int,
        );
        x += 1;
        ptr = ptr.offset(128);
        level_ptr = level_ptr.offset(32);
    }
    if (*(*f).frame_hdr).loopfilter.level_u == 0 && (*(*f).frame_hdr).loopfilter.level_v == 0 {
        return;
    }
    let mut uv_off: ptrdiff_t;
    level_ptr = ((*f).lf.level).offset(((*f).b4_stride * (sby * sbsz >> ss_ver) as isize) as isize);
    uv_off = 0 as c_int as ptrdiff_t;
    x = 0 as c_int;
    while x < (*f).sb128w {
        filter_plane_rows_uv::<BitDepth8>(
            f,
            have_top,
            level_ptr as *const [u8; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_uv[1]).as_mut_ptr() as *const [[u16; 2]; 2],
            &mut *(*p.offset(1)).offset(uv_off as isize),
            &mut *(*p.offset(2)).offset(uv_off as isize),
            (*f).cur.stride[1],
            cmp::min(32 as c_int, (*f).w4 - x * 32) + ss_hor >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as c_int,
            ss_hor,
        );
        x += 1;
        uv_off += (128 >> ss_hor) as isize;
        level_ptr = level_ptr.offset((32 >> ss_hor) as isize);
    }
}
