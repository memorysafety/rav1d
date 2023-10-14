use crate::include::common::bitdepth::BitDepth8;
use crate::include::dav1d::dav1d::RAV1D_INLOOPFILTER_RESTORATION;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I444;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTileState;
use crate::src::lr_apply_tmpl_8::rav1d_lr_sbrow_8bpc;
use crate::src::recon::rav1d_filter_sbrow_cdef;
use crate::src::recon::rav1d_filter_sbrow_deblock_cols;
use crate::src::recon::rav1d_filter_sbrow_deblock_rows;
use libc::memcpy;
use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_void;

pub type pixel = u8;

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_resize_8bpc(
    f: *mut Rav1dFrameContext,
    sby: c_int,
) {
    let sbsz = (*f).sb_step;
    let y = sby * sbsz * 4;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let p: [*const pixel; 3] = [
        ((*f).lf.p[0] as *mut pixel).offset((y as isize * (*f).cur.stride[0]) as isize)
            as *const pixel,
        ((*f).lf.p[1] as *mut pixel).offset((y as isize * (*f).cur.stride[1] >> ss_ver) as isize)
            as *const pixel,
        ((*f).lf.p[2] as *mut pixel).offset((y as isize * (*f).cur.stride[1] >> ss_ver) as isize)
            as *const pixel,
    ];
    let sr_p: [*mut pixel; 3] = [
        ((*f).lf.sr_p[0] as *mut pixel).offset((y as isize * (*f).sr_cur.p.stride[0]) as isize),
        ((*f).lf.sr_p[1] as *mut pixel)
            .offset((y as isize * (*f).sr_cur.p.stride[1] >> ss_ver) as isize),
        ((*f).lf.sr_p[2] as *mut pixel)
            .offset((y as isize * (*f).sr_cur.p.stride[1] >> ss_ver) as isize),
    ];
    let has_chroma =
        ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint) as c_int;
    let mut pl = 0;
    while pl < 1 + 2 * has_chroma {
        let ss_ver_0 = (pl != 0
            && (*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint)
            as c_int;
        let h_start = 8 * (sby != 0) as c_int >> ss_ver_0;
        let dst_stride: ptrdiff_t = (*f).sr_cur.p.stride[(pl != 0) as c_int as usize];
        let dst: *mut pixel =
            (sr_p[pl as usize]).offset(-((h_start as isize * dst_stride) as isize));
        let src_stride: ptrdiff_t = (*f).cur.stride[(pl != 0) as c_int as usize];
        let src: *const pixel =
            (p[pl as usize]).offset(-((h_start as isize * src_stride) as isize));
        let h_end = 4 * (sbsz - 2 * ((sby + 1) < (*f).sbh) as c_int) >> ss_ver_0;
        let ss_hor = (pl != 0
            && (*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint)
            as c_int;
        let dst_w = (*f).sr_cur.p.p.w + ss_hor >> ss_hor;
        let src_w = 4 * (*f).bw + ss_hor >> ss_hor;
        let img_h = (*f).cur.p.h - sbsz * 4 * sby + ss_ver_0 >> ss_ver_0;
        ((*(*f).dsp).mc.resize)(
            dst.cast(),
            dst_stride,
            src.cast(),
            src_stride,
            dst_w,
            cmp::min(img_h, h_end) + h_start,
            src_w,
            (*f).resize_step[(pl != 0) as c_int as usize],
            (*f).resize_start[(pl != 0) as c_int as usize],
            8,
        );
        pl += 1;
    }
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_lr_8bpc(f: *mut Rav1dFrameContext, sby: c_int) {
    if (*(*f).c).inloop_filters as c_uint & RAV1D_INLOOPFILTER_RESTORATION as c_int as c_uint == 0 {
        return;
    }
    let y = sby * (*f).sb_step * 4;
    let ss_ver =
        ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
    let sr_p: [*mut pixel; 3] = [
        ((*f).lf.sr_p[0] as *mut pixel).offset((y as isize * (*f).sr_cur.p.stride[0]) as isize),
        ((*f).lf.sr_p[1] as *mut pixel)
            .offset((y as isize * (*f).sr_cur.p.stride[1] >> ss_ver) as isize),
        ((*f).lf.sr_p[2] as *mut pixel)
            .offset((y as isize * (*f).sr_cur.p.stride[1] >> ss_ver) as isize),
    ];
    rav1d_lr_sbrow_8bpc(f, sr_p.as_ptr(), sby);
}

pub(crate) unsafe extern "C" fn rav1d_filter_sbrow_8bpc(f: *mut Rav1dFrameContext, sby: c_int) {
    rav1d_filter_sbrow_deblock_cols::<BitDepth8>(f, sby);
    rav1d_filter_sbrow_deblock_rows::<BitDepth8>(f, sby);
    if (*(*f).seq_hdr).cdef != 0 {
        rav1d_filter_sbrow_cdef::<BitDepth8>((*(*f).c).tc, sby);
    }
    if (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
        rav1d_filter_sbrow_resize_8bpc(f, sby);
    }
    if (*f).lf.restore_planes != 0 {
        rav1d_filter_sbrow_lr_8bpc(f, sby);
    }
}

pub(crate) unsafe extern "C" fn rav1d_backup_ipred_edge_8bpc(t: *mut Rav1dTaskContext) {
    let f: *const Rav1dFrameContext = (*t).f;
    let ts: *mut Rav1dTileState = (*t).ts;
    let sby = (*t).by >> (*f).sb_shift;
    let sby_off = (*f).sb128w * 128 * sby;
    let x_off = (*ts).tiling.col_start;
    let y: *const pixel = ((*f).cur.data[0] as *const pixel)
        .offset((x_off * 4) as isize)
        .offset(((((*t).by + (*f).sb_step) * 4 - 1) as isize * (*f).cur.stride[0]) as isize);
    memcpy(
        &mut *(*((*f).ipred_edge).as_ptr().offset(0) as *mut pixel)
            .offset((sby_off + x_off * 4) as isize) as *mut pixel as *mut c_void,
        y as *const c_void,
        (4 * ((*ts).tiling.col_end - x_off)) as usize,
    );
    if (*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I400 as c_int as c_uint {
        let ss_ver =
            ((*f).cur.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
        let ss_hor =
            ((*f).cur.p.layout as c_uint != RAV1D_PIXEL_LAYOUT_I444 as c_int as c_uint) as c_int;
        let uv_off: ptrdiff_t = (x_off * 4 >> ss_hor) as isize
            + ((((*t).by + (*f).sb_step) * 4 >> ss_ver) - 1) as isize * (*f).cur.stride[1];
        let mut pl = 1;
        while pl <= 2 {
            memcpy(
                &mut *(*((*f).ipred_edge).as_ptr().offset(pl as isize) as *mut pixel)
                    .offset((sby_off + (x_off * 4 >> ss_hor)) as isize)
                    as *mut pixel as *mut c_void,
                &*(*((*f).cur.data).as_ptr().offset(pl as isize) as *const pixel)
                    .offset(uv_off as isize) as *const pixel as *const c_void,
                (4 * ((*ts).tiling.col_end - x_off) >> ss_hor) as usize,
            );
            pl += 1;
        }
    }
}
