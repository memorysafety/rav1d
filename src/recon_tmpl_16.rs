use crate::include::common::bitdepth::BitDepth16;
use crate::include::dav1d::dav1d::RAV1D_INLOOPFILTER_RESTORATION;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTileState;
use crate::src::lr_apply_tmpl_16::rav1d_lr_sbrow_16bpc;
use crate::src::recon::rav1d_filter_sbrow_cdef;
use crate::src::recon::rav1d_filter_sbrow_deblock_cols;
use crate::src::recon::rav1d_filter_sbrow_deblock_rows;
use crate::src::recon::rav1d_filter_sbrow_resize;
use libc::memcpy;
use libc::ptrdiff_t;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

pub type pixel = u16;

#[inline]
unsafe fn PXSTRIDE(x: ptrdiff_t) -> ptrdiff_t {
    if x & 1 != 0 {
        unreachable!();
    }
    return x >> 1;
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_lr_16bpc(f: *mut Rav1dFrameContext, sby: c_int) {
    if (*(*f).c).inloop_filters as c_uint & RAV1D_INLOOPFILTER_RESTORATION as c_int as c_uint == 0 {
        return;
    }
    let y = sby * (*f).sb_step * 4;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let sr_p: [*mut pixel; 3] = [
        ((*f).lf.sr_p[0] as *mut pixel).offset(y as isize * PXSTRIDE((*f).sr_cur.p.stride[0])),
        ((*f).lf.sr_p[1] as *mut pixel)
            .offset(y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver),
        ((*f).lf.sr_p[2] as *mut pixel)
            .offset(y as isize * PXSTRIDE((*f).sr_cur.p.stride[1]) >> ss_ver),
    ];
    rav1d_lr_sbrow_16bpc(f, sr_p.as_ptr(), sby);
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_16bpc(f: *mut Rav1dFrameContext, sby: c_int) {
    rav1d_filter_sbrow_deblock_cols::<BitDepth16>(f, sby);
    rav1d_filter_sbrow_deblock_rows::<BitDepth16>(f, sby);
    if (*(*f).seq_hdr).cdef != 0 {
        rav1d_filter_sbrow_cdef::<BitDepth16>((*(*f).c).tc, sby);
    }
    if (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
        rav1d_filter_sbrow_resize::<BitDepth16>(f, sby);
    }
    if (*f).lf.restore_planes != 0 {
        rav1d_filter_sbrow_lr_16bpc(f, sby);
    }
}

pub(crate) unsafe extern "C" fn rav1d_backup_ipred_edge_16bpc(t: *mut Rav1dTaskContext) {
    let f: *const Rav1dFrameContext = (*t).f;
    let ts: *mut Rav1dTileState = (*t).ts;
    let sby = (*t).by >> (*f).sb_shift;
    let sby_off = (*f).sb128w * 128 * sby;
    let x_off = (*ts).tiling.col_start;
    let y: *const pixel = ((*f).cur.data[0] as *const pixel)
        .offset((x_off * 4) as isize)
        .offset(
            ((((*t).by + (*f).sb_step) * 4 - 1) as isize * PXSTRIDE((*f).cur.stride[0])) as isize,
        );
    memcpy(
        &mut *(*((*f).ipred_edge).as_ptr().offset(0) as *mut pixel)
            .offset((sby_off + x_off * 4) as isize) as *mut pixel as *mut c_void,
        y as *const c_void,
        (4 * ((*ts).tiling.col_end - x_off) << 1) as usize,
    );
    if (*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint {
        let ss_ver =
            ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
        let ss_hor =
            ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
        let uv_off: ptrdiff_t = (x_off * 4 >> ss_hor) as isize
            + ((((*t).by + (*f).sb_step) * 4 >> ss_ver) - 1) as isize
                * PXSTRIDE((*f).cur.stride[1]);
        let mut pl = 1;
        while pl <= 2 {
            memcpy(
                &mut *(*((*f).ipred_edge).as_ptr().offset(pl as isize) as *mut pixel)
                    .offset((sby_off + (x_off * 4 >> ss_hor)) as isize)
                    as *mut pixel as *mut c_void,
                &*(*((*f).cur.data).as_ptr().offset(pl as isize) as *const pixel)
                    .offset(uv_off as isize) as *const pixel as *const c_void,
                (4 * ((*ts).tiling.col_end - x_off) >> ss_hor << 1) as usize,
            );
            pl += 1;
        }
    }
}
