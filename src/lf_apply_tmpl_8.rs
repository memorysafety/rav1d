use crate::include::common::bitdepth::BitDepth8;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::src::env::BlockContext;
use crate::src::internal::Rav1dDSPContext;
use crate::src::internal::Rav1dFrameContext;

use crate::src::lf_apply::filter_plane_cols_y;
use crate::src::lf_apply::filter_plane_rows_y;
use crate::src::lf_mask::Av1Filter;

use libc::ptrdiff_t;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;

pub type pixel = u8;

#[inline]
unsafe fn filter_plane_cols_uv(
    f: *const Rav1dFrameContext,
    have_left: c_int,
    lvl: *const [u8; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[u16; 2]; 2],
    u: *mut pixel,
    v: *mut pixel,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
    ss_ver: c_int,
) {
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let mut x = 0;
    while x < w {
        if !(have_left == 0 && x == 0) {
            let mut hmask: [u32; 3] = [0; 3];
            if starty4 == 0 {
                hmask[0] = (*mask.offset(x as isize))[0][0] as u32;
                hmask[1] = (*mask.offset(x as isize))[1][0] as u32;
                if endy4 > 16 >> ss_ver {
                    hmask[0] |= ((*mask.offset(x as isize))[0][1] as c_uint) << (16 >> ss_ver);
                    hmask[1] |= ((*mask.offset(x as isize))[1][1] as c_uint) << (16 >> ss_ver);
                }
            } else {
                hmask[0] = (*mask.offset(x as isize))[0][1] as u32;
                hmask[1] = (*mask.offset(x as isize))[1][1] as u32;
            }
            hmask[2] = 0 as c_int as u32;
            (*dsp).lf.loop_filter_sb[1][0](
                u.offset((x * 4) as isize).cast(),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize)).as_ptr().offset(2) as *const u8 as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                endy4 - starty4,
                8,
            );
            (*dsp).lf.loop_filter_sb[1][0](
                v.offset((x * 4) as isize).cast(),
                ls,
                hmask.as_mut_ptr(),
                &*(*lvl.offset(x as isize)).as_ptr().offset(3) as *const u8 as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                endy4 - starty4,
                8,
            );
        }
        x += 1;
    }
}

#[inline]
unsafe fn filter_plane_rows_uv(
    f: *const Rav1dFrameContext,
    have_top: c_int,
    mut lvl: *const [u8; 4],
    b4_stride: ptrdiff_t,
    mask: *const [[u16; 2]; 2],
    u: *mut pixel,
    v: *mut pixel,
    ls: ptrdiff_t,
    w: c_int,
    starty4: c_int,
    endy4: c_int,
    ss_hor: c_int,
) {
    let dsp: *const Rav1dDSPContext = (*f).dsp;
    let mut off_l: ptrdiff_t = 0 as c_int as ptrdiff_t;
    let mut y = starty4;
    while y < endy4 {
        if !(have_top == 0 && y == 0) {
            let vmask: [u32; 3] = [
                (*mask.offset(y as isize))[0][0] as c_uint
                    | ((*mask.offset(y as isize))[0][1] as c_uint) << (16 >> ss_hor),
                (*mask.offset(y as isize))[1][0] as c_uint
                    | ((*mask.offset(y as isize))[1][1] as c_uint) << (16 >> ss_hor),
                0 as c_int as u32,
            ];
            (*dsp).lf.loop_filter_sb[1][1](
                u.offset(off_l as isize).cast(),
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0)).as_ptr().offset(2) as *const u8 as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                w,
                8,
            );
            (*dsp).lf.loop_filter_sb[1][1](
                v.offset(off_l as isize).cast(),
                ls,
                vmask.as_ptr(),
                &*(*lvl.offset(0)).as_ptr().offset(3) as *const u8 as *const [u8; 4],
                b4_stride,
                &(*f).lf.lim_lut.0,
                w,
                8,
            );
        }
        y += 1;
        off_l += 4 * ls;
        lvl = lvl.offset(b4_stride as isize);
    }
}

pub(crate) unsafe fn rav1d_loopfilter_sbrow_cols_8bpc(
    f: *const Rav1dFrameContext,
    p: *const *mut pixel,
    lflvl: *mut Av1Filter,
    sby: c_int,
    start_of_tile_row: c_int,
) {
    let mut x;
    let mut have_left;
    let is_sb64 = ((*(*f).seq_hdr).sb128 == 0) as c_int;
    let starty4 = (sby & is_sb64) << 4;
    let sbsz = 32 >> is_sb64;
    let sbl2 = 5 - is_sb64;
    let halign = (*f).bh + 31 & !(31 as c_int);
    let ss_ver =
        ((*f).cur.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
    let ss_hor =
        ((*f).cur.p.layout as c_uint != Rav1dPixelLayout::I444 as c_int as c_uint) as c_int;
    let vmask = 16 >> ss_ver;
    let hmask = 16 >> ss_hor;
    let vmax: c_uint = (1 as c_uint) << vmask;
    let hmax: c_uint = (1 as c_uint) << hmask;
    let endy4: c_uint = (starty4 + cmp::min((*f).h4 - sby * sbsz, sbsz)) as c_uint;
    let uv_endy4: c_uint = endy4.wrapping_add(ss_ver as c_uint) >> ss_ver;
    let mut lpf_y: *const u8 = &mut *(*((*f).lf.tx_lpf_right_edge).as_ptr().offset(0))
        .offset((sby << sbl2) as isize) as *mut u8;
    let mut lpf_uv: *const u8 = &mut *(*((*f).lf.tx_lpf_right_edge).as_ptr().offset(1))
        .offset((sby << sbl2 - ss_ver) as isize) as *mut u8;
    let mut tile_col = 1;
    loop {
        x = (*(*f).frame_hdr).tiling.col_start_sb[tile_col as usize] as c_int;
        if x << sbl2 >= (*f).bw {
            break;
        }
        let bx4 = if x & is_sb64 != 0 {
            16 as c_int
        } else {
            0 as c_int
        };
        let cbx4 = bx4 >> ss_hor;
        x >>= is_sb64;
        let y_hmask: *mut [u16; 2] =
            ((*lflvl.offset(x as isize)).filter_y[0][bx4 as usize]).as_mut_ptr();
        let mut y: c_uint = starty4 as c_uint;
        let mut mask: c_uint = ((1 as c_int) << y) as c_uint;
        while y < endy4 {
            let sidx = (mask >= 0x10000 as c_uint) as c_int;
            let smask: c_uint = mask >> (sidx << 4);
            let idx = 2 as c_int
                * ((*y_hmask.offset(2))[sidx as usize] as c_uint & smask != 0) as c_int
                + ((*y_hmask.offset(1))[sidx as usize] as c_uint & smask != 0) as c_int;
            let ref mut fresh0 = (*y_hmask.offset(2))[sidx as usize];
            *fresh0 = (*fresh0 as c_uint & !smask) as u16;
            let ref mut fresh1 = (*y_hmask.offset(1))[sidx as usize];
            *fresh1 = (*fresh1 as c_uint & !smask) as u16;
            let ref mut fresh2 = (*y_hmask.offset(0))[sidx as usize];
            *fresh2 = (*fresh2 as c_uint & !smask) as u16;
            let ref mut fresh3 = (*y_hmask.offset(cmp::min(
                idx,
                *lpf_y.offset(y.wrapping_sub(starty4 as c_uint) as isize) as c_int,
            ) as isize))[sidx as usize];
            *fresh3 = (*fresh3 as c_uint | smask) as u16;
            y = y.wrapping_add(1);
            mask <<= 1;
        }
        if (*f).cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint {
            let uv_hmask: *mut [u16; 2] =
                ((*lflvl.offset(x as isize)).filter_uv[0][cbx4 as usize]).as_mut_ptr();
            let mut y_0: c_uint = (starty4 >> ss_ver) as c_uint;
            let mut uv_mask: c_uint = ((1 as c_int) << y_0) as c_uint;
            while y_0 < uv_endy4 {
                let sidx_0 = (uv_mask >= vmax) as c_int;
                let smask_0: c_uint = uv_mask >> (sidx_0 << 4 - ss_ver);
                let idx_0 =
                    ((*uv_hmask.offset(1))[sidx_0 as usize] as c_uint & smask_0 != 0) as c_int;
                let ref mut fresh4 = (*uv_hmask.offset(1))[sidx_0 as usize];
                *fresh4 = (*fresh4 as c_uint & !smask_0) as u16;
                let ref mut fresh5 = (*uv_hmask.offset(0))[sidx_0 as usize];
                *fresh5 = (*fresh5 as c_uint & !smask_0) as u16;
                let ref mut fresh6 = (*uv_hmask.offset(cmp::min(
                    idx_0,
                    *lpf_uv.offset(y_0.wrapping_sub((starty4 >> ss_ver) as c_uint) as isize)
                        as c_int,
                ) as isize))[sidx_0 as usize];
                *fresh6 = (*fresh6 as c_uint | smask_0) as u16;
                y_0 = y_0.wrapping_add(1);
                uv_mask <<= 1;
            }
        }
        lpf_y = lpf_y.offset(halign as isize);
        lpf_uv = lpf_uv.offset((halign >> ss_ver) as isize);
        tile_col += 1;
    }
    if start_of_tile_row != 0 {
        let mut a: *const BlockContext;
        x = 0 as c_int;
        a = &mut *((*f).a).offset(((*f).sb128w * (start_of_tile_row - 1)) as isize)
            as *mut BlockContext;
        while x < (*f).sb128w {
            let y_vmask: *mut [u16; 2] =
                ((*lflvl.offset(x as isize)).filter_y[1][starty4 as usize]).as_mut_ptr();
            let w: c_uint = cmp::min(32 as c_int, (*f).w4 - (x << 5)) as c_uint;
            let mut mask_0: c_uint = 1 as c_int as c_uint;
            let mut i: c_uint = 0 as c_int as c_uint;
            while i < w {
                let sidx_1 = (mask_0 >= 0x10000 as c_uint) as c_int;
                let smask_1: c_uint = mask_0 >> (sidx_1 << 4);
                let idx_1 = 2 as c_int
                    * ((*y_vmask.offset(2))[sidx_1 as usize] as c_uint & smask_1 != 0) as c_int
                    + ((*y_vmask.offset(1))[sidx_1 as usize] as c_uint & smask_1 != 0) as c_int;
                let ref mut fresh7 = (*y_vmask.offset(2))[sidx_1 as usize];
                *fresh7 = (*fresh7 as c_uint & !smask_1) as u16;
                let ref mut fresh8 = (*y_vmask.offset(1))[sidx_1 as usize];
                *fresh8 = (*fresh8 as c_uint & !smask_1) as u16;
                let ref mut fresh9 = (*y_vmask.offset(0))[sidx_1 as usize];
                *fresh9 = (*fresh9 as c_uint & !smask_1) as u16;
                let ref mut fresh10 = (*y_vmask
                    .offset(cmp::min(idx_1, (*a).tx_lpf_y[i as usize] as c_int) as isize))
                    [sidx_1 as usize];
                *fresh10 = (*fresh10 as c_uint | smask_1) as u16;
                mask_0 <<= 1;
                i = i.wrapping_add(1);
            }
            if (*f).cur.p.layout as c_uint != Rav1dPixelLayout::I400 as c_int as c_uint {
                let cw: c_uint = w.wrapping_add(ss_hor as c_uint) >> ss_hor;
                let uv_vmask: *mut [u16; 2] = ((*lflvl.offset(x as isize)).filter_uv[1]
                    [(starty4 >> ss_ver) as usize])
                    .as_mut_ptr();
                let mut uv_mask_0: c_uint = 1 as c_int as c_uint;
                let mut i_0: c_uint = 0 as c_int as c_uint;
                while i_0 < cw {
                    let sidx_2 = (uv_mask_0 >= hmax) as c_int;
                    let smask_2: c_uint = uv_mask_0 >> (sidx_2 << 4 - ss_hor);
                    let idx_2 =
                        ((*uv_vmask.offset(1))[sidx_2 as usize] as c_uint & smask_2 != 0) as c_int;
                    let ref mut fresh11 = (*uv_vmask.offset(1))[sidx_2 as usize];
                    *fresh11 = (*fresh11 as c_uint & !smask_2) as u16;
                    let ref mut fresh12 = (*uv_vmask.offset(0))[sidx_2 as usize];
                    *fresh12 = (*fresh12 as c_uint & !smask_2) as u16;
                    let ref mut fresh13 = (*uv_vmask
                        .offset(cmp::min(idx_2, (*a).tx_lpf_uv[i_0 as usize] as c_int) as isize))
                        [sidx_2 as usize];
                    *fresh13 = (*fresh13 as c_uint | smask_2) as u16;
                    uv_mask_0 <<= 1;
                    i_0 = i_0.wrapping_add(1);
                }
            }
            x += 1;
            a = a.offset(1);
        }
    }
    let mut ptr: *mut pixel;
    let mut level_ptr: *mut [u8; 4] =
        ((*f).lf.level).offset(((*f).b4_stride * sby as isize * sbsz as isize) as isize);
    ptr = *p.offset(0);
    have_left = 0 as c_int;
    x = 0 as c_int;
    while x < (*f).sb128w {
        filter_plane_cols_y::<BitDepth8>(
            f,
            have_left,
            level_ptr as *const [u8; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_y[0]).as_mut_ptr() as *const [[u16; 2]; 3],
            ptr,
            (*f).cur.stride[0],
            cmp::min(32 as c_int, (*f).w4 - x * 32),
            starty4,
            endy4 as c_int,
        );
        x += 1;
        have_left = 1 as c_int;
        ptr = ptr.offset(128);
        level_ptr = level_ptr.offset(32);
    }
    if (*(*f).frame_hdr).loopfilter.level_u == 0 && (*(*f).frame_hdr).loopfilter.level_v == 0 {
        return;
    }
    let mut uv_off: ptrdiff_t;
    level_ptr = ((*f).lf.level).offset(((*f).b4_stride * (sby * sbsz >> ss_ver) as isize) as isize);
    uv_off = 0 as c_int as ptrdiff_t;
    have_left = 0 as c_int;
    x = 0 as c_int;
    while x < (*f).sb128w {
        filter_plane_cols_uv(
            f,
            have_left,
            level_ptr as *const [u8; 4],
            (*f).b4_stride,
            ((*lflvl.offset(x as isize)).filter_uv[0]).as_mut_ptr() as *const [[u16; 2]; 2],
            &mut *(*p.offset(1)).offset(uv_off as isize),
            &mut *(*p.offset(2)).offset(uv_off as isize),
            (*f).cur.stride[1],
            cmp::min(32 as c_int, (*f).w4 - x * 32) + ss_hor >> ss_hor,
            starty4 >> ss_ver,
            uv_endy4 as c_int,
            ss_ver,
        );
        x += 1;
        have_left = 1 as c_int;
        uv_off += (128 >> ss_hor) as isize;
        level_ptr = level_ptr.offset((32 >> ss_hor) as isize);
    }
}

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
        filter_plane_rows_uv(
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
