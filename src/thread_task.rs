use crate::include::common::attributes::ctz;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::RAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::src::cdf::rav1d_cdf_thread_update;
use crate::src::decode::rav1d_decode_frame_exit;
use crate::src::decode::rav1d_decode_frame_init;
use crate::src::decode::rav1d_decode_frame_init_cdf;
use crate::src::decode::rav1d_decode_tile_sbrow;
use crate::src::error::Rav1dError::EGeneric;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dResult;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTask;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTileState;
use crate::src::internal::TaskThreadData;
use crate::src::internal::TaskType;
use crate::src::internal::RAV1D_TASK_TYPE_CDEF;
use crate::src::internal::RAV1D_TASK_TYPE_DEBLOCK_COLS;
use crate::src::internal::RAV1D_TASK_TYPE_DEBLOCK_ROWS;
use crate::src::internal::RAV1D_TASK_TYPE_ENTROPY_PROGRESS;
use crate::src::internal::RAV1D_TASK_TYPE_FG_APPLY;
use crate::src::internal::RAV1D_TASK_TYPE_FG_PREP;
use crate::src::internal::RAV1D_TASK_TYPE_INIT;
use crate::src::internal::RAV1D_TASK_TYPE_INIT_CDF;
use crate::src::internal::RAV1D_TASK_TYPE_LOOP_RESTORATION;
use crate::src::internal::RAV1D_TASK_TYPE_RECONSTRUCTION_PROGRESS;
use crate::src::internal::RAV1D_TASK_TYPE_SUPER_RESOLUTION;
use crate::src::internal::RAV1D_TASK_TYPE_TILE_ENTROPY;
use crate::src::internal::RAV1D_TASK_TYPE_TILE_RECONSTRUCTION;
use crate::src::picture::Rav1dThreadPicture;
use libc::memset;
use libc::pthread_cond_signal;
use libc::pthread_cond_wait;
use libc::pthread_mutex_lock;
use libc::pthread_mutex_unlock;
use libc::realloc;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_long;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::process::abort;

#[cfg(feature = "bitdepth_8")]
use crate::{
    src::fg_apply_tmpl_8::rav1d_apply_grain_row_8bpc, src::fg_apply_tmpl_8::rav1d_prep_grain_8bpc,
};

#[cfg(feature = "bitdepth_16")]
use crate::{
    src::fg_apply_tmpl_16::rav1d_apply_grain_row_16bpc,
    src::fg_apply_tmpl_16::rav1d_prep_grain_16bpc,
};

#[cfg(target_os = "linux")]
use libc::prctl;

#[cfg(target_os = "macos")]
use libc::pthread_setname_np;

pub const FRAME_ERROR: u32 = u32::MAX - 1;
pub const TILE_ERROR: i32 = i32::MAX - 1;

#[inline]
unsafe fn rav1d_set_thread_name(name: *const c_char) {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "linux")] {
            prctl(15 as c_int, name);
        } else if #[cfg(target_os = "macos")] {
            pthread_setname_np(name);
        } else {
            unimplemented!();
        }
    }
}

#[inline]
unsafe fn reset_task_cur(
    c: *const Rav1dContext,
    ttd: *mut TaskThreadData,
    mut frame_idx: c_uint,
) -> c_int {
    let min_frame_idx: c_uint;
    let cur_frame_idx: c_uint;
    let current_block: u64;
    let first: c_uint = ::core::intrinsics::atomic_load_seqcst(&mut (*ttd).first);
    let mut reset_frame_idx: c_uint =
        ::core::intrinsics::atomic_xchg_seqcst(&mut (*ttd).reset_task_cur, u32::MAX);
    if reset_frame_idx < first {
        if frame_idx == u32::MAX {
            return 0 as c_int;
        }
        reset_frame_idx = u32::MAX;
    }
    if (*ttd).cur == 0
        && ((*((*c).fc).offset(first as isize))
            .task_thread
            .task_cur_prev)
            .is_null()
    {
        return 0 as c_int;
    }
    if reset_frame_idx != u32::MAX {
        if frame_idx == u32::MAX {
            if reset_frame_idx > first.wrapping_add((*ttd).cur) {
                return 0 as c_int;
            }
            (*ttd).cur = reset_frame_idx.wrapping_sub(first);
            current_block = 12921688021154394536;
        } else {
            current_block = 5399440093318478209;
        }
    } else {
        if frame_idx == u32::MAX {
            return 0 as c_int;
        }
        current_block = 5399440093318478209;
    }
    match current_block {
        5399440093318478209 => {
            if frame_idx < first {
                frame_idx = frame_idx.wrapping_add((*c).n_fc);
            }
            min_frame_idx = cmp::min(reset_frame_idx, frame_idx);
            cur_frame_idx = first.wrapping_add((*ttd).cur);
            if (*ttd).cur < (*c).n_fc && cur_frame_idx < min_frame_idx {
                return 0 as c_int;
            }
            (*ttd).cur = min_frame_idx.wrapping_sub(first);
            while (*ttd).cur < (*c).n_fc {
                if !((*((*c).fc)
                    .offset(first.wrapping_add((*ttd).cur).wrapping_rem((*c).n_fc) as isize))
                .task_thread
                .task_head)
                    .is_null()
                {
                    break;
                }
                (*ttd).cur = ((*ttd).cur).wrapping_add(1);
            }
        }
        _ => {}
    }
    let mut i: c_uint = (*ttd).cur;
    while i < (*c).n_fc {
        let ref mut fresh0 = (*((*c).fc)
            .offset(first.wrapping_add(i).wrapping_rem((*c).n_fc) as isize))
        .task_thread
        .task_cur_prev;
        *fresh0 = 0 as *mut Rav1dTask;
        i = i.wrapping_add(1);
    }
    return 1 as c_int;
}

#[inline]
unsafe fn reset_task_cur_async(ttd: *mut TaskThreadData, mut frame_idx: c_uint, n_frames: c_uint) {
    let first: c_uint = ::core::intrinsics::atomic_load_seqcst(&mut (*ttd).first);
    if frame_idx < first {
        frame_idx = frame_idx.wrapping_add(n_frames);
    }
    let mut last_idx: c_uint = frame_idx;
    loop {
        frame_idx = last_idx;
        last_idx = ::core::intrinsics::atomic_xchg_seqcst(&mut (*ttd).reset_task_cur, frame_idx);
        if !(last_idx < frame_idx) {
            break;
        }
    }
    if frame_idx == first
        && ::core::intrinsics::atomic_load_seqcst(&mut (*ttd).first as *mut atomic_uint) != first
    {
        let mut expected: c_uint = frame_idx;
        let fresh1 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
            &mut (*ttd).reset_task_cur,
            *&mut expected,
            u32::MAX,
        );
        *&mut expected = fresh1.0;
        fresh1.1;
    }
}

unsafe fn insert_tasks_between(
    f: *mut Rav1dFrameContext,
    first: *mut Rav1dTask,
    last: *mut Rav1dTask,
    a: *mut Rav1dTask,
    b: *mut Rav1dTask,
    cond_signal: c_int,
) {
    let ttd: *mut TaskThreadData = (*f).task_thread.ttd;
    if ::core::intrinsics::atomic_load_seqcst((*(*f).c).flush) != 0 {
        return;
    }
    if !(a.is_null() || (*a).next == b) {
        unreachable!();
    }
    if a.is_null() {
        (*f).task_thread.task_head = first;
    } else {
        (*a).next = first;
    }
    if b.is_null() {
        (*f).task_thread.task_tail = last;
    }
    (*last).next = b;
    reset_task_cur((*f).c, ttd, (*first).frame_idx);
    if cond_signal != 0
        && ::core::intrinsics::atomic_or_seqcst(
            &mut (*ttd).cond_signaled as *mut atomic_int,
            1 as c_int,
        ) == 0
    {
        pthread_cond_signal(&mut (*ttd).cond);
    }
}

unsafe fn insert_tasks(
    f: *mut Rav1dFrameContext,
    first: *mut Rav1dTask,
    last: *mut Rav1dTask,
    cond_signal: c_int,
) {
    let mut t_ptr: *mut Rav1dTask;
    let mut prev_t: *mut Rav1dTask = 0 as *mut Rav1dTask;
    let mut current_block_34: u64;
    t_ptr = (*f).task_thread.task_head;
    while !t_ptr.is_null() {
        if (*t_ptr).type_0 as c_uint == RAV1D_TASK_TYPE_TILE_ENTROPY as c_int as c_uint {
            if (*first).type_0 as c_uint > RAV1D_TASK_TYPE_TILE_ENTROPY as c_int as c_uint {
                current_block_34 = 11174649648027449784;
            } else if (*first).sby > (*t_ptr).sby {
                current_block_34 = 11174649648027449784;
            } else {
                if (*first).sby < (*t_ptr).sby {
                    insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
                current_block_34 = 15904375183555213903;
            }
        } else {
            if (*first).type_0 as c_uint == RAV1D_TASK_TYPE_TILE_ENTROPY as c_int as c_uint {
                insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                return;
            }
            if (*first).sby > (*t_ptr).sby {
                current_block_34 = 11174649648027449784;
            } else {
                if (*first).sby < (*t_ptr).sby {
                    insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
                if (*first).type_0 as c_uint > (*t_ptr).type_0 as c_uint {
                    current_block_34 = 11174649648027449784;
                } else {
                    if ((*first).type_0 as c_uint) < (*t_ptr).type_0 as c_uint {
                        insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                        return;
                    }
                    current_block_34 = 15904375183555213903;
                }
            }
        }
        match current_block_34 {
            15904375183555213903 => {
                if !((*first).type_0 as c_uint
                    == RAV1D_TASK_TYPE_TILE_RECONSTRUCTION as c_int as c_uint
                    || (*first).type_0 as c_uint == RAV1D_TASK_TYPE_TILE_ENTROPY as c_int as c_uint)
                {
                    unreachable!();
                }
                if !((*first).type_0 as c_uint == (*t_ptr).type_0 as c_uint) {
                    unreachable!();
                }
                if !((*t_ptr).sby == (*first).sby) {
                    unreachable!();
                }
                let p = ((*first).type_0 as c_uint
                    == RAV1D_TASK_TYPE_TILE_ENTROPY as c_int as c_uint)
                    as c_int;
                let t_tile_idx =
                    first.offset_from((*f).task_thread.tile_tasks[p as usize]) as c_long as c_int;
                let p_tile_idx =
                    t_ptr.offset_from((*f).task_thread.tile_tasks[p as usize]) as c_long as c_int;
                if !(t_tile_idx != p_tile_idx) {
                    unreachable!();
                }
                if !(t_tile_idx > p_tile_idx) {
                    insert_tasks_between(f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
            }
            _ => {}
        }
        prev_t = t_ptr;
        t_ptr = (*t_ptr).next;
    }
    insert_tasks_between(f, first, last, prev_t, 0 as *mut Rav1dTask, cond_signal);
}

#[inline]
unsafe fn insert_task(f: *mut Rav1dFrameContext, t: *mut Rav1dTask, cond_signal: c_int) {
    insert_tasks(f, t, t, cond_signal);
}

#[inline]
unsafe fn add_pending(f: *mut Rav1dFrameContext, t: *mut Rav1dTask) {
    pthread_mutex_lock(&mut (*f).task_thread.pending_tasks.lock);
    (*t).next = 0 as *mut Rav1dTask;
    if ((*f).task_thread.pending_tasks.head).is_null() {
        (*f).task_thread.pending_tasks.head = t;
    } else {
        (*(*f).task_thread.pending_tasks.tail).next = t;
    }
    (*f).task_thread.pending_tasks.tail = t;
    ::core::intrinsics::atomic_store_seqcst(&mut (*f).task_thread.pending_tasks.merge, 1 as c_int);
    pthread_mutex_unlock(&mut (*f).task_thread.pending_tasks.lock);
}

#[inline]
unsafe fn merge_pending_frame(f: *mut Rav1dFrameContext) -> c_int {
    let merge = ::core::intrinsics::atomic_load_seqcst(&mut (*f).task_thread.pending_tasks.merge);
    if merge != 0 {
        pthread_mutex_lock(&mut (*f).task_thread.pending_tasks.lock);
        let mut t: *mut Rav1dTask = (*f).task_thread.pending_tasks.head;
        (*f).task_thread.pending_tasks.head = 0 as *mut Rav1dTask;
        (*f).task_thread.pending_tasks.tail = 0 as *mut Rav1dTask;
        ::core::intrinsics::atomic_store_seqcst(
            &mut (*f).task_thread.pending_tasks.merge,
            0 as c_int,
        );
        pthread_mutex_unlock(&mut (*f).task_thread.pending_tasks.lock);
        while !t.is_null() {
            let tmp: *mut Rav1dTask = (*t).next;
            insert_task(f, t, 0 as c_int);
            t = tmp;
        }
    }
    return merge;
}

#[inline]
unsafe fn merge_pending(c: *const Rav1dContext) -> c_int {
    let mut res = 0;
    let mut i: c_uint = 0 as c_int as c_uint;
    while i < (*c).n_fc {
        res |= merge_pending_frame(&mut *((*c).fc).offset(i as isize));
        i = i.wrapping_add(1);
    }
    return res;
}

unsafe fn create_filter_sbrow(
    f: *mut Rav1dFrameContext,
    pass: c_int,
    res_t: *mut *mut Rav1dTask,
) -> c_int {
    let has_deblock = ((*(*f).frame_hdr).loopfilter.level_y[0] != 0
        || (*(*f).frame_hdr).loopfilter.level_y[1] != 0) as c_int;
    let has_cdef = (*(*f).seq_hdr).cdef;
    let has_resize = ((*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1]) as c_int;
    let has_lr = (*f).lf.restore_planes;
    let mut tasks: *mut Rav1dTask = (*f).task_thread.tasks;
    let uses_2pass = ((*(*f).c).n_fc > 1 as c_uint) as c_int;
    let num_tasks = (*f).sbh * (1 + uses_2pass);
    if num_tasks > (*f).task_thread.num_tasks {
        let size: usize = (::core::mem::size_of::<Rav1dTask>()).wrapping_mul(num_tasks as usize);
        tasks = realloc((*f).task_thread.tasks as *mut c_void, size) as *mut Rav1dTask;
        if tasks.is_null() {
            return -(1 as c_int);
        }
        memset(tasks as *mut c_void, 0 as c_int, size);
        (*f).task_thread.tasks = tasks;
        (*f).task_thread.num_tasks = num_tasks;
    }
    tasks = tasks.offset(((*f).sbh * (pass & 1)) as isize);
    if pass & 1 != 0 {
        (*f).frame_thread.entropy_progress = 0 as c_int;
    } else {
        let prog_sz = ((*f).sbh + 31 & !(31 as c_int)) >> 5;
        if prog_sz > (*f).frame_thread.prog_sz {
            let prog: *mut atomic_uint = realloc(
                (*f).frame_thread.frame_progress as *mut c_void,
                ((2 * prog_sz) as usize).wrapping_mul(::core::mem::size_of::<atomic_uint>()),
            ) as *mut atomic_uint;
            if prog.is_null() {
                return -(1 as c_int);
            }
            (*f).frame_thread.frame_progress = prog;
            (*f).frame_thread.copy_lpf_progress = prog.offset(prog_sz as isize);
        }
        (*f).frame_thread.prog_sz = prog_sz;
        memset(
            (*f).frame_thread.frame_progress as *mut c_void,
            0 as c_int,
            (prog_sz as usize).wrapping_mul(::core::mem::size_of::<atomic_uint>()),
        );
        memset(
            (*f).frame_thread.copy_lpf_progress as *mut c_void,
            0 as c_int,
            (prog_sz as usize).wrapping_mul(::core::mem::size_of::<atomic_uint>() as usize),
        );
        ::core::intrinsics::atomic_store_seqcst(
            &mut (*f).frame_thread.deblock_progress,
            0 as c_int,
        );
    }
    (*f).frame_thread.next_tile_row[(pass & 1) as usize] = 0 as c_int;
    let t: *mut Rav1dTask = &mut *tasks.offset(0) as *mut Rav1dTask;
    (*t).sby = 0 as c_int;
    (*t).recon_progress = 1 as c_int;
    (*t).deblock_progress = 0 as c_int;
    (*t).type_0 = (if pass == 1 {
        RAV1D_TASK_TYPE_ENTROPY_PROGRESS as c_int
    } else if has_deblock != 0 {
        RAV1D_TASK_TYPE_DEBLOCK_COLS as c_int
    } else if has_cdef != 0 || has_lr != 0 {
        RAV1D_TASK_TYPE_DEBLOCK_ROWS as c_int
    } else if has_resize != 0 {
        RAV1D_TASK_TYPE_SUPER_RESOLUTION as c_int
    } else {
        RAV1D_TASK_TYPE_RECONSTRUCTION_PROGRESS as c_int
    }) as TaskType;
    (*t).frame_idx = f.offset_from((*(*f).c).fc) as c_long as c_int as c_uint;
    *res_t = t;
    return 0 as c_int;
}

pub(crate) unsafe fn rav1d_task_create_tile_sbrow(
    f: *mut Rav1dFrameContext,
    pass: c_int,
    _cond_signal: c_int,
) -> Rav1dResult {
    let mut tasks: *mut Rav1dTask = (*f).task_thread.tile_tasks[0];
    let uses_2pass = ((*(*f).c).n_fc > 1 as c_uint) as c_int;
    let num_tasks = (*(*f).frame_hdr).tiling.cols * (*(*f).frame_hdr).tiling.rows;
    if pass < 2 {
        let alloc_num_tasks = num_tasks * (1 + uses_2pass);
        if alloc_num_tasks > (*f).task_thread.num_tile_tasks {
            let size: usize =
                (::core::mem::size_of::<Rav1dTask>()).wrapping_mul(alloc_num_tasks as usize);
            tasks = realloc((*f).task_thread.tile_tasks[0] as *mut c_void, size) as *mut Rav1dTask;
            if tasks.is_null() {
                return Err(EGeneric);
            }
            memset(tasks as *mut c_void, 0 as c_int, size);
            (*f).task_thread.tile_tasks[0] = tasks;
            (*f).task_thread.num_tile_tasks = alloc_num_tasks;
        }
        (*f).task_thread.tile_tasks[1] = tasks.offset(num_tasks as isize);
    }
    tasks = tasks.offset((num_tasks * (pass & 1)) as isize);
    let mut pf_t: *mut Rav1dTask = 0 as *mut Rav1dTask;
    if create_filter_sbrow(f, pass, &mut pf_t) != 0 {
        return Err(EGeneric);
    }
    let mut prev_t: *mut Rav1dTask = 0 as *mut Rav1dTask;
    let mut tile_idx = 0;
    while tile_idx < num_tasks {
        let ts: *mut Rav1dTileState =
            &mut *((*f).ts).offset(tile_idx as isize) as *mut Rav1dTileState;
        let t: *mut Rav1dTask = &mut *tasks.offset(tile_idx as isize) as *mut Rav1dTask;
        (*t).sby = (*ts).tiling.row_start >> (*f).sb_shift;
        if !pf_t.is_null() && (*t).sby != 0 {
            (*prev_t).next = pf_t;
            prev_t = pf_t;
            pf_t = 0 as *mut Rav1dTask;
        }
        (*t).recon_progress = 0 as c_int;
        (*t).deblock_progress = 0 as c_int;
        (*t).deps_skip = 0 as c_int;
        (*t).type_0 = (if pass != 1 as c_int {
            RAV1D_TASK_TYPE_TILE_RECONSTRUCTION as c_int
        } else {
            RAV1D_TASK_TYPE_TILE_ENTROPY as c_int
        }) as TaskType;
        (*t).frame_idx = f.offset_from((*(*f).c).fc) as c_long as c_int as c_uint;
        if !prev_t.is_null() {
            (*prev_t).next = t;
        }
        prev_t = t;
        tile_idx += 1;
    }
    if !pf_t.is_null() {
        (*prev_t).next = pf_t;
        prev_t = pf_t;
    }
    (*prev_t).next = 0 as *mut Rav1dTask;
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).task_thread.done)
            .as_mut_ptr()
            .offset((pass & 1) as isize) as *mut atomic_int,
        0 as c_int,
    );
    pthread_mutex_lock(&mut (*f).task_thread.pending_tasks.lock);
    if !(((*f).task_thread.pending_tasks.head).is_null() || pass == 2) {
        unreachable!();
    }
    if ((*f).task_thread.pending_tasks.head).is_null() {
        (*f).task_thread.pending_tasks.head = &mut *tasks.offset(0) as *mut Rav1dTask;
    } else {
        (*(*f).task_thread.pending_tasks.tail).next = &mut *tasks.offset(0) as *mut Rav1dTask;
    }
    (*f).task_thread.pending_tasks.tail = prev_t;
    ::core::intrinsics::atomic_store_seqcst(&mut (*f).task_thread.pending_tasks.merge, 1 as c_int);
    ::core::intrinsics::atomic_store_seqcst(&mut (*f).task_thread.init_done, 1 as c_int);
    pthread_mutex_unlock(&mut (*f).task_thread.pending_tasks.lock);
    Ok(())
}

pub(crate) unsafe fn rav1d_task_frame_init(f: *mut Rav1dFrameContext) {
    let c: *const Rav1dContext = (*f).c;
    ::core::intrinsics::atomic_store_seqcst(&mut (*f).task_thread.init_done, 0 as c_int);
    let t: *mut Rav1dTask = &mut (*f).task_thread.init_task;
    (*t).type_0 = RAV1D_TASK_TYPE_INIT;
    (*t).frame_idx = f.offset_from((*c).fc) as c_long as c_int as c_uint;
    (*t).sby = 0 as c_int;
    (*t).deblock_progress = 0 as c_int;
    (*t).recon_progress = (*t).deblock_progress;
    insert_task(f, t, 1 as c_int);
}

pub(crate) unsafe fn rav1d_task_delayed_fg(
    c: *mut Rav1dContext,
    out: *mut Rav1dPicture,
    in_0: *const Rav1dPicture,
) {
    let ttd: *mut TaskThreadData = &mut (*c).task_thread;
    (*ttd).delayed_fg.in_0 = in_0;
    (*ttd).delayed_fg.out = out;
    (*ttd).delayed_fg.type_0 = RAV1D_TASK_TYPE_FG_PREP;
    *(&mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(0) as *mut atomic_int) = 0 as c_int;
    *(&mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(1) as *mut atomic_int) = 0 as c_int;
    pthread_mutex_lock(&mut (*ttd).lock);
    (*ttd).delayed_fg.exec = 1 as c_int;
    pthread_cond_signal(&mut (*ttd).cond);
    pthread_cond_wait(&mut (*ttd).delayed_fg.cond, &mut (*ttd).lock);
    pthread_mutex_unlock(&mut (*ttd).lock);
}

#[inline]
unsafe fn ensure_progress(
    ttd: *mut TaskThreadData,
    f: *mut Rav1dFrameContext,
    t: *mut Rav1dTask,
    type_0: TaskType,
    state: *mut atomic_int,
    target: *mut c_int,
) -> c_int {
    let p1 = ::core::intrinsics::atomic_load_seqcst(state);
    if p1 < (*t).sby {
        (*t).type_0 = type_0;
        (*t).deblock_progress = 0 as c_int;
        (*t).recon_progress = (*t).deblock_progress;
        *target = (*t).sby;
        add_pending(f, t);
        pthread_mutex_lock(&mut (*ttd).lock);
        return 1 as c_int;
    }
    return 0 as c_int;
}

#[inline]
unsafe fn check_tile(t: *mut Rav1dTask, f: *mut Rav1dFrameContext, frame_mt: c_int) -> c_int {
    let tp = ((*t).type_0 as c_uint == RAV1D_TASK_TYPE_TILE_ENTROPY as c_int as c_uint) as c_int;
    let tile_idx = t.offset_from((*f).task_thread.tile_tasks[tp as usize]) as c_long as c_int;
    let ts: *mut Rav1dTileState = &mut *((*f).ts).offset(tile_idx as isize) as *mut Rav1dTileState;
    let p1 = ::core::intrinsics::atomic_load_seqcst(
        &mut *((*ts).progress).as_mut_ptr().offset(tp as isize) as *mut atomic_int,
    );
    if p1 < (*t).sby {
        return 1 as c_int;
    }
    let mut error = (p1 == TILE_ERROR) as c_int;
    error |= ::core::intrinsics::atomic_or_seqcst(&mut (*f).task_thread.error, error);
    if error == 0 && frame_mt != 0 && tp == 0 {
        let p2 = ::core::intrinsics::atomic_load_seqcst(
            &mut *((*ts).progress).as_mut_ptr().offset(1) as *mut atomic_int,
        );
        if p2 <= (*t).sby {
            return 1 as c_int;
        }
        error = (p2 == TILE_ERROR) as c_int;
        error |= ::core::intrinsics::atomic_or_seqcst(&mut (*f).task_thread.error, error);
    }
    if error == 0 && frame_mt != 0 && (*(*f).frame_hdr).frame_type as c_uint & 1 as c_uint != 0 {
        let p: *const Rav1dThreadPicture = &mut (*f).sr_cur;
        let ss_ver =
            ((*p).p.p.layout as c_uint == RAV1D_PIXEL_LAYOUT_I420 as c_int as c_uint) as c_int;
        let p_b: c_uint = (((*t).sby + 1) << (*f).sb_shift + 2) as c_uint;
        let tile_sby = (*t).sby - ((*ts).tiling.row_start >> (*f).sb_shift);
        let lowest_px: *const [c_int; 2] =
            (*((*ts).lowest_pixel).offset(tile_sby as isize)).as_mut_ptr() as *const [c_int; 2];
        let mut current_block_14: u64;
        let mut n = (*t).deps_skip;
        while n < 7 {
            let mut lowest: c_uint = 0;
            if tp != 0 {
                lowest = p_b;
                current_block_14 = 2370887241019905314;
            } else {
                let y = if (*lowest_px.offset(n as isize))[0] == i32::MIN {
                    i32::MIN
                } else {
                    (*lowest_px.offset(n as isize))[0] + 8
                };
                let uv = if (*lowest_px.offset(n as isize))[1] == i32::MIN {
                    i32::MIN
                } else {
                    (*lowest_px.offset(n as isize))[1] * ((1 as c_int) << ss_ver) + 8
                };
                let max = cmp::max(y, uv);
                if max == i32::MIN {
                    current_block_14 = 7651349459974463963;
                } else {
                    lowest = iclip(max, 1 as c_int, (*f).refp[n as usize].p.p.h) as c_uint;
                    current_block_14 = 2370887241019905314;
                }
            }
            match current_block_14 {
                2370887241019905314 => {
                    let p3: c_uint = ::core::intrinsics::atomic_load_seqcst(
                        &mut *((*((*f).refp).as_mut_ptr().offset(n as isize)).progress)
                            .offset((tp == 0) as c_int as isize)
                            as *mut atomic_uint,
                    );
                    if p3 < lowest {
                        return 1 as c_int;
                    }
                    ::core::intrinsics::atomic_or_seqcst(
                        &mut (*f).task_thread.error,
                        (p3 == FRAME_ERROR) as c_int,
                    );
                }
                _ => {}
            }
            n += 1;
            (*t).deps_skip += 1;
        }
    }
    return 0 as c_int;
}

#[inline]
unsafe fn get_frame_progress(c: *const Rav1dContext, f: *const Rav1dFrameContext) -> c_int {
    let frame_prog: c_uint = if (*c).n_fc > 1 as c_uint {
        ::core::intrinsics::atomic_load_seqcst(
            &mut *((*f).sr_cur.progress).offset(1) as *mut atomic_uint
        )
    } else {
        0 as c_int as c_uint
    };
    if frame_prog >= FRAME_ERROR {
        return (*f).sbh - 1;
    }
    let mut idx = (frame_prog >> (*f).sb_shift + 7) as c_int;
    let mut prog;
    loop {
        let state: *mut atomic_uint =
            &mut *((*f).frame_thread.frame_progress).offset(idx as isize) as *mut atomic_uint;
        let val: c_uint = !::core::intrinsics::atomic_load_seqcst(state);
        prog = if val != 0 { ctz(val) } else { 32 as c_int };
        if prog != 32 as c_int {
            break;
        }
        prog = 0 as c_int;
        idx += 1;
        if !(idx < (*f).frame_thread.prog_sz) {
            break;
        }
    }
    return (idx << 5 | prog) - 1;
}

#[inline]
unsafe fn abort_frame(f: *mut Rav1dFrameContext, error: Rav1dResult) {
    ::core::intrinsics::atomic_store_seqcst(
        &mut (*f).task_thread.error,
        if error == Err(EINVAL) {
            1 as c_int
        } else {
            -(1 as c_int)
        },
    );
    ::core::intrinsics::atomic_store_seqcst(&mut (*f).task_thread.task_counter, 0 as c_int);
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).task_thread.done).as_mut_ptr().offset(0) as *mut atomic_int,
        1 as c_int,
    );
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).task_thread.done).as_mut_ptr().offset(1) as *mut atomic_int,
        1 as c_int,
    );
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).sr_cur.progress).offset(0) as *mut atomic_uint,
        FRAME_ERROR,
    );
    ::core::intrinsics::atomic_store_seqcst(
        &mut *((*f).sr_cur.progress).offset(1) as *mut atomic_uint,
        FRAME_ERROR,
    );
    rav1d_decode_frame_exit(&mut *f, error);
    (*f).n_tile_data = 0 as c_int;
    pthread_cond_signal(&mut (*f).task_thread.cond);
}

#[inline]
unsafe fn delayed_fg_task(c: *const Rav1dContext, ttd: *mut TaskThreadData) {
    let in_0 = (*ttd).delayed_fg.in_0;
    let out = (*ttd).delayed_fg.out;
    let mut off = 0;
    if (*out).p.bpc != 8 as c_int {
        off = ((*out).p.bpc >> 1) - 4;
    }
    let mut row;
    let mut progmax;
    let mut done;
    match (*ttd).delayed_fg.type_0 as c_uint {
        11 => {
            (*ttd).delayed_fg.exec = 0 as c_int;
            if ::core::intrinsics::atomic_load_seqcst(&mut (*ttd).cond_signaled as *mut atomic_int)
                != 0
            {
                pthread_cond_signal(&mut (*ttd).cond);
            }
            pthread_mutex_unlock(&mut (*ttd).lock);
            match (*out).p.bpc {
                #[cfg(feature = "bitdepth_8")]
                8 => {
                    rav1d_prep_grain_8bpc(
                        &(*((*c).dsp).as_ptr().offset(0)).fg,
                        out,
                        in_0,
                        ((*ttd)
                            .delayed_fg
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .scaling_8bpc
                            .0)
                            .as_mut_ptr(),
                        ((*ttd)
                            .delayed_fg
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .grain_lut_8bpc
                            .0)
                            .as_mut_ptr(),
                    );
                }
                #[cfg(feature = "bitdepth_16")]
                10 | 12 => {
                    rav1d_prep_grain_16bpc(
                        &(*((*c).dsp).as_ptr().offset(off as isize)).fg,
                        out,
                        in_0,
                        ((*ttd)
                            .delayed_fg
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .scaling_16bpc
                            .0)
                            .as_mut_ptr(),
                        ((*ttd)
                            .delayed_fg
                            .c2rust_unnamed
                            .c2rust_unnamed_0
                            .grain_lut_16bpc
                            .0)
                            .as_mut_ptr(),
                    );
                }
                _ => {
                    abort();
                }
            }
            (*ttd).delayed_fg.type_0 = RAV1D_TASK_TYPE_FG_APPLY;
            pthread_mutex_lock(&mut (*ttd).lock);
            (*ttd).delayed_fg.exec = 1 as c_int;
        }
        12 => {}
        _ => {
            abort();
        }
    }
    row = ::core::intrinsics::atomic_xadd_seqcst(
        &mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(0) as *mut atomic_int,
        1 as c_int,
    );
    pthread_mutex_unlock(&mut (*ttd).lock);
    progmax = (*out).p.h + 31 >> 5;
    loop {
        if (row + 1) < progmax {
            pthread_cond_signal(&mut (*ttd).cond);
        } else if row + 1 >= progmax {
            pthread_mutex_lock(&mut (*ttd).lock);
            (*ttd).delayed_fg.exec = 0 as c_int;
            if row >= progmax {
                break;
            }
            pthread_mutex_unlock(&mut (*ttd).lock);
        }
        match (*out).p.bpc {
            #[cfg(feature = "bitdepth_8")]
            8 => {
                rav1d_apply_grain_row_8bpc(
                    &(*((*c).dsp).as_ptr().offset(0)).fg,
                    out,
                    in_0,
                    ((*ttd)
                        .delayed_fg
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .scaling_8bpc
                        .0)
                        .as_mut_ptr(),
                    ((*ttd)
                        .delayed_fg
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .grain_lut_8bpc
                        .0)
                        .as_mut_ptr(),
                    row,
                );
            }
            #[cfg(feature = "bitdepth_16")]
            10 | 12 => {
                rav1d_apply_grain_row_16bpc(
                    &(*((*c).dsp).as_ptr().offset(off as isize)).fg,
                    out,
                    in_0,
                    ((*ttd)
                        .delayed_fg
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .scaling_16bpc
                        .0)
                        .as_mut_ptr(),
                    ((*ttd)
                        .delayed_fg
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .grain_lut_16bpc
                        .0)
                        .as_mut_ptr(),
                    row,
                );
            }
            _ => {
                abort();
            }
        }
        row = ::core::intrinsics::atomic_xadd_seqcst(
            &mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(0) as *mut atomic_int,
            1 as c_int,
        );
        #[allow(unused_assignments)]
        // TODO(kkysen) non-trivial due to the atomics, so leaving for later
        {
            done = ::core::intrinsics::atomic_xadd_seqcst(
                &mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(1) as *mut atomic_int,
                1 as c_int,
            ) + 1;
        }
        if row < progmax {
            continue;
        }
        pthread_mutex_lock(&mut (*ttd).lock);
        (*ttd).delayed_fg.exec = 0 as c_int;
        break;
    }
    done = ::core::intrinsics::atomic_xadd_seqcst(
        &mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(1) as *mut atomic_int,
        1 as c_int,
    ) + 1;
    progmax = ::core::intrinsics::atomic_load_seqcst(
        &mut *((*ttd).delayed_fg.progress).as_mut_ptr().offset(0) as *mut atomic_int,
    );
    if !(done < progmax) {
        pthread_cond_signal(&mut (*ttd).delayed_fg.cond);
    }
}

pub unsafe extern "C" fn rav1d_worker_task(data: *mut c_void) -> *mut c_void {
    let mut flush;
    let mut error_0;
    let mut sby;
    let mut f: *mut Rav1dFrameContext;
    let mut t: *mut Rav1dTask;
    let mut prev_t: *mut Rav1dTask;
    let mut current_block: u64;
    let tc: *mut Rav1dTaskContext = data as *mut Rav1dTaskContext;
    let c: *const Rav1dContext = (*tc).c;
    let ttd: *mut TaskThreadData = (*tc).task_thread.ttd;
    rav1d_set_thread_name(b"dav1d-worker\0" as *const u8 as *const c_char);
    pthread_mutex_lock(&mut (*ttd).lock);
    's_18: while !(*tc).task_thread.die {
        if !(::core::intrinsics::atomic_load_seqcst((*c).flush) != 0) {
            merge_pending(c);
            if (*ttd).delayed_fg.exec != 0 {
                delayed_fg_task(c, ttd);
                continue;
            } else {
                f = 0 as *mut Rav1dFrameContext;
                t = 0 as *mut Rav1dTask;
                prev_t = 0 as *mut Rav1dTask;
                if (*c).n_fc > 1 as c_uint {
                    let mut i: c_uint = 0 as c_int as c_uint;
                    loop {
                        if !(i < (*c).n_fc) {
                            current_block = 5601891728916014340;
                            break;
                        }
                        let first: c_uint =
                            ::core::intrinsics::atomic_load_seqcst(&mut (*ttd).first);
                        f = &mut *((*c).fc)
                            .offset(first.wrapping_add(i).wrapping_rem((*c).n_fc) as isize)
                            as *mut Rav1dFrameContext;
                        if !(::core::intrinsics::atomic_load_seqcst(
                            &mut (*f).task_thread.init_done as *mut atomic_int,
                        ) != 0)
                        {
                            t = (*f).task_thread.task_head;
                            if !t.is_null() {
                                if (*t).type_0 as c_uint == RAV1D_TASK_TYPE_INIT as c_int as c_uint
                                {
                                    current_block = 7012560550443761033;
                                    break;
                                }
                                if (*t).type_0 as c_uint
                                    == RAV1D_TASK_TYPE_INIT_CDF as c_int as c_uint
                                {
                                    let p1 = (if !((*f).in_cdf.progress).is_null() {
                                        ::core::intrinsics::atomic_load_seqcst((*f).in_cdf.progress)
                                    } else {
                                        1 as c_int as c_uint
                                    }) as c_int;
                                    if p1 != 0 {
                                        ::core::intrinsics::atomic_or_seqcst(
                                            &mut (*f).task_thread.error,
                                            (p1 == TILE_ERROR) as c_int,
                                        );
                                        current_block = 7012560550443761033;
                                        break;
                                    }
                                }
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                } else {
                    current_block = 5601891728916014340;
                }
                's_107: loop {
                    match current_block {
                        5601891728916014340 => {
                            if (*ttd).cur < (*c).n_fc {
                                let first_0: c_uint =
                                    ::core::intrinsics::atomic_load_seqcst(&mut (*ttd).first);
                                f = &mut *((*c).fc).offset(
                                    first_0.wrapping_add((*ttd).cur).wrapping_rem((*c).n_fc)
                                        as isize,
                                ) as *mut Rav1dFrameContext;
                                merge_pending_frame(f);
                                prev_t = (*f).task_thread.task_cur_prev;
                                t = if !prev_t.is_null() {
                                    (*prev_t).next
                                } else {
                                    (*f).task_thread.task_head
                                };
                                while !t.is_null() {
                                    if !((*t).type_0 as c_uint
                                        == RAV1D_TASK_TYPE_INIT_CDF as c_int as c_uint)
                                    {
                                        if (*t).type_0 as c_uint
                                            == RAV1D_TASK_TYPE_TILE_ENTROPY as c_int as c_uint
                                            || (*t).type_0 as c_uint
                                                == RAV1D_TASK_TYPE_TILE_RECONSTRUCTION as c_int
                                                    as c_uint
                                        {
                                            if check_tile(t, f, ((*c).n_fc > 1 as c_uint) as c_int)
                                                == 0
                                            {
                                                current_block = 7012560550443761033;
                                                continue 's_107;
                                            }
                                        } else if (*t).recon_progress != 0 {
                                            let p = ((*t).type_0 as c_uint
                                                == RAV1D_TASK_TYPE_ENTROPY_PROGRESS as c_int
                                                    as c_uint)
                                                as c_int;
                                            let error = ::core::intrinsics::atomic_load_seqcst(
                                                &mut (*f).task_thread.error,
                                            );
                                            if !(::core::intrinsics::atomic_load_seqcst(
                                                &mut *((*f).task_thread.done)
                                                    .as_mut_ptr()
                                                    .offset(p as isize)
                                                    as *mut atomic_int,
                                            ) == 0
                                                || error != 0)
                                            {
                                                unreachable!();
                                            }
                                            let tile_row_base = (*(*f).frame_hdr).tiling.cols
                                                * (*f).frame_thread.next_tile_row[p as usize];
                                            if p != 0 {
                                                let prog: *mut atomic_int =
                                                    &mut (*f).frame_thread.entropy_progress;
                                                let p1_0 =
                                                    ::core::intrinsics::atomic_load_seqcst(prog);
                                                if p1_0 < (*t).sby {
                                                    current_block = 5395695591151878490;
                                                } else {
                                                    ::core::intrinsics::atomic_or_seqcst(
                                                        &mut (*f).task_thread.error,
                                                        (p1_0 == TILE_ERROR) as c_int,
                                                    );
                                                    current_block = 14832935472441733737;
                                                }
                                            } else {
                                                current_block = 14832935472441733737;
                                            }
                                            match current_block {
                                                5395695591151878490 => {}
                                                _ => {
                                                    let mut tc_0 = 0;
                                                    loop {
                                                        if !(tc_0 < (*(*f).frame_hdr).tiling.cols) {
                                                            current_block = 3222590281903869779;
                                                            break;
                                                        }
                                                        let ts: *mut Rav1dTileState = &mut *((*f)
                                                            .ts)
                                                            .offset((tile_row_base + tc_0) as isize)
                                                            as *mut Rav1dTileState;
                                                        let p2 =
                                                            ::core::intrinsics::atomic_load_seqcst(
                                                                &mut *((*ts).progress)
                                                                    .as_mut_ptr()
                                                                    .offset(p as isize)
                                                                    as *mut atomic_int,
                                                            );
                                                        if p2 < (*t).recon_progress {
                                                            current_block = 5395695591151878490;
                                                            break;
                                                        }
                                                        ::core::intrinsics::atomic_or_seqcst(
                                                            &mut (*f).task_thread.error,
                                                            (p2 == TILE_ERROR) as c_int,
                                                        );
                                                        tc_0 += 1;
                                                    }
                                                    match current_block {
                                                        5395695591151878490 => {}
                                                        _ => {
                                                            if ((*t).sby + 1) < (*f).sbh {
                                                                let next_t: *mut Rav1dTask = &mut *t
                                                                    .offset(1)
                                                                    as *mut Rav1dTask;
                                                                *next_t = (*t).clone();
                                                                (*next_t).sby += 1;
                                                                let ntr =
                                                                    (*f).frame_thread.next_tile_row
                                                                        [p as usize]
                                                                        + 1;
                                                                let start = (*(*f).frame_hdr)
                                                                    .tiling
                                                                    .row_start_sb
                                                                    [ntr as usize]
                                                                    as c_int;
                                                                if (*next_t).sby == start {
                                                                    (*f).frame_thread
                                                                        .next_tile_row
                                                                        [p as usize] = ntr;
                                                                }
                                                                (*next_t).recon_progress =
                                                                    (*next_t).sby + 1;
                                                                insert_task(f, next_t, 0 as c_int);
                                                            }
                                                            current_block = 7012560550443761033;
                                                            continue 's_107;
                                                        }
                                                    }
                                                }
                                            }
                                        } else if (*t).type_0 as c_uint
                                            == RAV1D_TASK_TYPE_CDEF as c_int as c_uint
                                        {
                                            let prog_0: *mut atomic_uint =
                                                (*f).frame_thread.copy_lpf_progress;
                                            let p1_1 = ::core::intrinsics::atomic_load_seqcst(
                                                &mut *prog_0.offset(((*t).sby - 1 >> 5) as isize)
                                                    as *mut atomic_uint,
                                            )
                                                as c_int;
                                            if p1_1 as c_uint & (1 as c_uint) << ((*t).sby - 1 & 31)
                                                != 0
                                            {
                                                current_block = 7012560550443761033;
                                                continue 's_107;
                                            }
                                        } else {
                                            if (*t).deblock_progress == 0 {
                                                unreachable!();
                                            }
                                            let p1_2 = ::core::intrinsics::atomic_load_seqcst(
                                                &mut (*f).frame_thread.deblock_progress,
                                            );
                                            if p1_2 >= (*t).deblock_progress {
                                                ::core::intrinsics::atomic_or_seqcst(
                                                    &mut (*f).task_thread.error,
                                                    (p1_2 == TILE_ERROR) as c_int,
                                                );
                                                current_block = 7012560550443761033;
                                                continue 's_107;
                                            }
                                        }
                                    }
                                    prev_t = t;
                                    t = (*t).next;
                                    (*f).task_thread.task_cur_prev = prev_t;
                                }
                                (*ttd).cur = ((*ttd).cur).wrapping_add(1);
                                current_block = 5601891728916014340;
                            } else {
                                if reset_task_cur(c, ttd, u32::MAX) != 0 {
                                    continue 's_18;
                                }
                                if merge_pending(c) != 0 {
                                    continue 's_18;
                                } else {
                                    current_block = 14728000373531839883;
                                    break;
                                }
                            }
                        }
                        _ => {
                            if !prev_t.is_null() {
                                (*prev_t).next = (*t).next;
                            } else {
                                (*f).task_thread.task_head = (*t).next;
                            }
                            if ((*t).next).is_null() {
                                (*f).task_thread.task_tail = prev_t;
                            }
                            if (*t).type_0 as c_uint > RAV1D_TASK_TYPE_INIT_CDF as c_int as c_uint
                                && ((*f).task_thread.task_head).is_null()
                            {
                                (*ttd).cur = ((*ttd).cur).wrapping_add(1);
                            }
                            (*t).next = 0 as *mut Rav1dTask;
                            ::core::intrinsics::atomic_store_seqcst(
                                &mut (*ttd).cond_signaled,
                                1 as c_int,
                            );
                            pthread_cond_signal(&mut (*ttd).cond);
                            pthread_mutex_unlock(&mut (*ttd).lock);
                            current_block = 8464383504555462953;
                            break;
                        }
                    }
                }
                match current_block {
                    14728000373531839883 => {}
                    _ => {
                        loop {
                            flush = ::core::intrinsics::atomic_load_seqcst((*c).flush);
                            error_0 = ::core::intrinsics::atomic_or_seqcst(
                                &mut (*f).task_thread.error,
                                flush,
                            ) | flush;
                            (*tc).f = f;
                            sby = (*t).sby;
                            match (*t).type_0 as c_uint {
                                RAV1D_TASK_TYPE_INIT => {
                                    if !((*c).n_fc > 1 as c_uint) {
                                        unreachable!();
                                    }
                                    let res = rav1d_decode_frame_init(&mut *f);
                                    let p1_3 = (if !((*f).in_cdf.progress).is_null() {
                                        ::core::intrinsics::atomic_load_seqcst((*f).in_cdf.progress)
                                    } else {
                                        1 as c_int as c_uint
                                    }) as c_int;
                                    if res.is_err() || p1_3 == TILE_ERROR {
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                        abort_frame(
                                            f,
                                            if res.is_err() { res } else { Err(EINVAL) },
                                        );
                                        reset_task_cur(c, ttd, (*t).frame_idx);
                                        continue 's_18;
                                    } else {
                                        (*t).type_0 = RAV1D_TASK_TYPE_INIT_CDF;
                                        if p1_3 != 0 {
                                            continue;
                                        }
                                        add_pending(f, t);
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                        continue 's_18;
                                    }
                                }
                                RAV1D_TASK_TYPE_INIT_CDF => {
                                    if !((*c).n_fc > 1 as c_uint) {
                                        unreachable!();
                                    }
                                    let mut res_0 = Err(EINVAL);
                                    if ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        res_0 = rav1d_decode_frame_init_cdf(&mut *f);
                                    }
                                    if (*(*f).frame_hdr).refresh_context != 0
                                        && !(*f).task_thread.update_set
                                    {
                                        ::core::intrinsics::atomic_store_seqcst(
                                            (*f).out_cdf.progress,
                                            (if res_0.is_err() {
                                                TILE_ERROR
                                            } else {
                                                1 as c_int
                                            })
                                                as c_uint,
                                        );
                                    }
                                    if res_0.is_ok() {
                                        if !((*c).n_fc > 1 as c_uint) {
                                            unreachable!();
                                        }
                                        let mut p_0 = 1;
                                        while p_0 <= 2 {
                                            let res_1 =
                                                rav1d_task_create_tile_sbrow(f, p_0, 0 as c_int);
                                            if res_1.is_err() {
                                                pthread_mutex_lock(&mut (*ttd).lock);
                                                ::core::intrinsics::atomic_store_seqcst(
                                                    &mut *((*f).task_thread.done)
                                                        .as_mut_ptr()
                                                        .offset((2 - p_0) as isize)
                                                        as *mut atomic_int,
                                                    1 as c_int,
                                                );
                                                ::core::intrinsics::atomic_store_seqcst(
                                                    &mut (*f).task_thread.error,
                                                    -(1 as c_int),
                                                );
                                                ::core::intrinsics::atomic_xsub_seqcst(
                                                    &mut (*f).task_thread.task_counter,
                                                    (*(*f).frame_hdr).tiling.cols
                                                        * (*(*f).frame_hdr).tiling.rows
                                                        + (*f).sbh,
                                                );
                                                ::core::intrinsics::atomic_store_seqcst(
                                                    &mut *((*f).sr_cur.progress)
                                                        .offset((p_0 - 1) as isize)
                                                        as *mut atomic_uint,
                                                    FRAME_ERROR,
                                                );
                                                if p_0 == 2
                                                    && ::core::intrinsics::atomic_load_seqcst(
                                                        &mut *((*f).task_thread.done)
                                                            .as_mut_ptr()
                                                            .offset(1)
                                                            as *mut atomic_int,
                                                    ) != 0
                                                {
                                                    if ::core::intrinsics::atomic_load_seqcst(
                                                        &mut (*f).task_thread.task_counter
                                                            as *mut atomic_int,
                                                    ) != 0
                                                    {
                                                        unreachable!();
                                                    }
                                                    rav1d_decode_frame_exit(&mut *f, Err(ENOMEM));
                                                    (*f).n_tile_data = 0 as c_int;
                                                    pthread_cond_signal(&mut (*f).task_thread.cond);
                                                } else {
                                                    pthread_mutex_unlock(&mut (*ttd).lock);
                                                }
                                            }
                                            p_0 += 1;
                                        }
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                    } else {
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                        abort_frame(f, res_0);
                                        reset_task_cur(c, ttd, (*t).frame_idx);
                                        ::core::intrinsics::atomic_store_seqcst(
                                            &mut (*f).task_thread.init_done,
                                            1 as c_int,
                                        );
                                    }
                                    continue 's_18;
                                }
                                RAV1D_TASK_TYPE_TILE_ENTROPY
                                | RAV1D_TASK_TYPE_TILE_RECONSTRUCTION => {
                                    let p_1 = ((*t).type_0 as c_uint
                                        == RAV1D_TASK_TYPE_TILE_ENTROPY as c_int as c_uint)
                                        as c_int;
                                    let tile_idx = t
                                        .offset_from((*f).task_thread.tile_tasks[p_1 as usize])
                                        as c_long
                                        as c_int;
                                    let ts_0: *mut Rav1dTileState = &mut *((*f).ts)
                                        .offset(tile_idx as isize)
                                        as *mut Rav1dTileState;
                                    (*tc).ts = ts_0;
                                    (*tc).by = sby << (*f).sb_shift;
                                    let uses_2pass = ((*c).n_fc > 1 as c_uint) as c_int;
                                    (*tc).frame_thread.pass = if uses_2pass == 0 {
                                        0 as c_int
                                    } else {
                                        1 as c_int
                                            + ((*t).type_0 as c_uint
                                                == RAV1D_TASK_TYPE_TILE_RECONSTRUCTION as c_int
                                                    as c_uint)
                                                as c_int
                                    };
                                    if error_0 == 0 {
                                        error_0 = match rav1d_decode_tile_sbrow(&mut *tc) {
                                            Ok(()) => 0,
                                            Err(()) => 1,
                                        };
                                    }
                                    let progress = if error_0 != 0 { TILE_ERROR } else { 1 + sby };
                                    ::core::intrinsics::atomic_or_seqcst(
                                        &mut (*f).task_thread.error,
                                        error_0,
                                    );
                                    if (sby + 1) << (*f).sb_shift < (*ts_0).tiling.row_end {
                                        (*t).sby += 1;
                                        (*t).deps_skip = 0 as c_int;
                                        if check_tile(t, f, uses_2pass) == 0 {
                                            ::core::intrinsics::atomic_store_seqcst(
                                                &mut *((*ts_0).progress)
                                                    .as_mut_ptr()
                                                    .offset(p_1 as isize)
                                                    as *mut atomic_int,
                                                progress,
                                            );
                                            reset_task_cur_async(ttd, (*t).frame_idx, (*c).n_fc);
                                            if ::core::intrinsics::atomic_or_seqcst(
                                                &mut (*ttd).cond_signaled as *mut atomic_int,
                                                1 as c_int,
                                            ) == 0
                                            {
                                                pthread_cond_signal(&mut (*ttd).cond);
                                            }
                                        } else {
                                            ::core::intrinsics::atomic_store_seqcst(
                                                &mut *((*ts_0).progress)
                                                    .as_mut_ptr()
                                                    .offset(p_1 as isize)
                                                    as *mut atomic_int,
                                                progress,
                                            );
                                            add_pending(f, t);
                                            pthread_mutex_lock(&mut (*ttd).lock);
                                            continue 's_18;
                                        }
                                    } else {
                                        pthread_mutex_lock(&mut (*ttd).lock);
                                        ::core::intrinsics::atomic_store_seqcst(
                                            &mut *((*ts_0).progress)
                                                .as_mut_ptr()
                                                .offset(p_1 as isize)
                                                as *mut atomic_int,
                                            progress,
                                        );
                                        reset_task_cur(c, ttd, (*t).frame_idx);
                                        error_0 = ::core::intrinsics::atomic_load_seqcst(
                                            &mut (*f).task_thread.error,
                                        );
                                        if (*(*f).frame_hdr).refresh_context != 0
                                            && (*tc).frame_thread.pass <= 1
                                            && (*f).task_thread.update_set
                                            && (*(*f).frame_hdr).tiling.update == tile_idx
                                        {
                                            if error_0 == 0 {
                                                rav1d_cdf_thread_update(
                                                    (*f).frame_hdr,
                                                    (*f).out_cdf.data.cdf,
                                                    &mut (*((*f).ts).offset(
                                                        (*(*f).frame_hdr).tiling.update as isize,
                                                    ))
                                                    .cdf,
                                                );
                                            }
                                            if (*c).n_fc > 1 as c_uint {
                                                ::core::intrinsics::atomic_store_seqcst(
                                                    (*f).out_cdf.progress,
                                                    (if error_0 != 0 {
                                                        TILE_ERROR
                                                    } else {
                                                        1 as c_int
                                                    })
                                                        as c_uint,
                                                );
                                            }
                                        }
                                        if ::core::intrinsics::atomic_xsub_seqcst(
                                            &mut (*f).task_thread.task_counter as *mut atomic_int,
                                            1 as c_int,
                                        ) - 1
                                            == 0
                                            && ::core::intrinsics::atomic_load_seqcst(
                                                &mut *((*f).task_thread.done).as_mut_ptr().offset(0)
                                                    as *mut atomic_int,
                                            ) != 0
                                            && (uses_2pass == 0
                                                || ::core::intrinsics::atomic_load_seqcst(
                                                    &mut *((*f).task_thread.done)
                                                        .as_mut_ptr()
                                                        .offset(1)
                                                        as *mut atomic_int,
                                                ) != 0)
                                        {
                                            rav1d_decode_frame_exit(
                                                &mut *f,
                                                if error_0 == 1 {
                                                    Err(EINVAL)
                                                } else if error_0 != 0 {
                                                    Err(ENOMEM)
                                                } else {
                                                    Ok(())
                                                },
                                            );
                                            (*f).n_tile_data = 0 as c_int;
                                            pthread_cond_signal(&mut (*f).task_thread.cond);
                                        }
                                        if !(::core::intrinsics::atomic_load_seqcst(
                                            &mut (*f).task_thread.task_counter as *mut atomic_int,
                                        ) >= 0)
                                        {
                                            unreachable!();
                                        }
                                        if ::core::intrinsics::atomic_or_seqcst(
                                            &mut (*ttd).cond_signaled as *mut atomic_int,
                                            1 as c_int,
                                        ) == 0
                                        {
                                            pthread_cond_signal(&mut (*ttd).cond);
                                        }
                                        continue 's_18;
                                    }
                                }
                                RAV1D_TASK_TYPE_DEBLOCK_COLS => {
                                    if ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        ((*f).bd_fn.filter_sbrow_deblock_cols)
                                            .expect("non-null function pointer")(
                                            f, sby
                                        );
                                    }
                                    if ensure_progress(
                                        ttd,
                                        f,
                                        t,
                                        RAV1D_TASK_TYPE_DEBLOCK_ROWS,
                                        &mut (*f).frame_thread.deblock_progress,
                                        &mut (*t).deblock_progress,
                                    ) != 0
                                    {
                                        continue 's_18;
                                    } else {
                                        current_block = 16164772378964453469;
                                        break;
                                    }
                                }
                                RAV1D_TASK_TYPE_DEBLOCK_ROWS => {
                                    current_block = 16164772378964453469;
                                    break;
                                }
                                RAV1D_TASK_TYPE_CDEF => {
                                    current_block = 5292528706010880565;
                                    break;
                                }
                                RAV1D_TASK_TYPE_SUPER_RESOLUTION => {
                                    current_block = 12196494833634779273;
                                    break;
                                }
                                RAV1D_TASK_TYPE_LOOP_RESTORATION => {
                                    current_block = 563177965161376451;
                                    break;
                                }
                                RAV1D_TASK_TYPE_RECONSTRUCTION_PROGRESS => {
                                    current_block = 18238912670629178022;
                                    break;
                                }
                                RAV1D_TASK_TYPE_ENTROPY_PROGRESS => {
                                    current_block = 7729400755948011248;
                                    break;
                                }
                                _ => {
                                    abort();
                                }
                            }
                        }
                        match current_block {
                            16164772378964453469 => {
                                if ::core::intrinsics::atomic_load_seqcst(
                                    &mut (*f).task_thread.error as *mut atomic_int,
                                ) == 0
                                {
                                    ((*f).bd_fn.filter_sbrow_deblock_rows)
                                        .expect("non-null function pointer")(
                                        f, sby
                                    );
                                }
                                if (*(*f).frame_hdr).loopfilter.level_y[0] != 0
                                    || (*(*f).frame_hdr).loopfilter.level_y[1] != 0
                                {
                                    error_0 = ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error,
                                    );
                                    ::core::intrinsics::atomic_store_seqcst(
                                        &mut (*f).frame_thread.deblock_progress,
                                        if error_0 != 0 { TILE_ERROR } else { sby + 1 },
                                    );
                                    reset_task_cur_async(ttd, (*t).frame_idx, (*c).n_fc);
                                    if ::core::intrinsics::atomic_or_seqcst(
                                        &mut (*ttd).cond_signaled as *mut atomic_int,
                                        1 as c_int,
                                    ) == 0
                                    {
                                        pthread_cond_signal(&mut (*ttd).cond);
                                    }
                                } else if (*(*f).seq_hdr).cdef != 0 || (*f).lf.restore_planes != 0 {
                                    ::core::intrinsics::atomic_or_seqcst(
                                        &mut *((*f).frame_thread.copy_lpf_progress)
                                            .offset((sby >> 5) as isize)
                                            as *mut atomic_uint,
                                        (1 as c_uint) << (sby & 31),
                                    );
                                    if sby != 0 {
                                        let prog_1 = ::core::intrinsics::atomic_load_seqcst(
                                            &mut *((*f).frame_thread.copy_lpf_progress)
                                                .offset((sby - 1 >> 5) as isize)
                                                as *mut atomic_uint,
                                        )
                                            as c_int;
                                        if !prog_1 as c_uint & (1 as c_uint) << (sby - 1 & 31) != 0
                                        {
                                            (*t).type_0 = RAV1D_TASK_TYPE_CDEF;
                                            (*t).deblock_progress = 0 as c_int;
                                            (*t).recon_progress = (*t).deblock_progress;
                                            add_pending(f, t);
                                            pthread_mutex_lock(&mut (*ttd).lock);
                                            continue;
                                        }
                                    }
                                }
                                current_block = 5292528706010880565;
                            }
                            _ => {}
                        }
                        match current_block {
                            5292528706010880565 => {
                                if (*(*f).seq_hdr).cdef != 0 {
                                    if ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        ((*f).bd_fn.filter_sbrow_cdef)
                                            .expect("non-null function pointer")(
                                            tc, sby
                                        );
                                    }
                                    reset_task_cur_async(ttd, (*t).frame_idx, (*c).n_fc);
                                    if ::core::intrinsics::atomic_or_seqcst(
                                        &mut (*ttd).cond_signaled as *mut atomic_int,
                                        1 as c_int,
                                    ) == 0
                                    {
                                        pthread_cond_signal(&mut (*ttd).cond);
                                    }
                                }
                                current_block = 12196494833634779273;
                            }
                            _ => {}
                        }
                        match current_block {
                            12196494833634779273 => {
                                if (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
                                    if ::core::intrinsics::atomic_load_seqcst(
                                        &mut (*f).task_thread.error as *mut atomic_int,
                                    ) == 0
                                    {
                                        ((*f).bd_fn.filter_sbrow_resize)
                                            .expect("non-null function pointer")(
                                            f, sby
                                        );
                                    }
                                }
                                current_block = 563177965161376451;
                            }
                            _ => {}
                        }
                        match current_block {
                            563177965161376451 => {
                                if ::core::intrinsics::atomic_load_seqcst(
                                    &mut (*f).task_thread.error as *mut atomic_int,
                                ) == 0
                                    && (*f).lf.restore_planes != 0
                                {
                                    ((*f).bd_fn.filter_sbrow_lr)
                                        .expect("non-null function pointer")(
                                        f, sby
                                    );
                                }
                                current_block = 18238912670629178022;
                            }
                            _ => {}
                        }
                        match current_block {
                            18238912670629178022 => {}
                            _ => {}
                        }
                        let uses_2pass_0 = ((*c).n_fc > 1 as c_uint) as c_int;
                        let sbh = (*f).sbh;
                        let sbsz = (*f).sb_step * 4;
                        if (*t).type_0 as c_uint
                            == RAV1D_TASK_TYPE_ENTROPY_PROGRESS as c_int as c_uint
                        {
                            error_0 =
                                ::core::intrinsics::atomic_load_seqcst(&mut (*f).task_thread.error);
                            let y: c_uint = if sby + 1 == sbh {
                                u32::MAX
                            } else {
                                ((sby + 1) as c_uint).wrapping_mul(sbsz as c_uint)
                            };
                            if !((*c).n_fc > 1 as c_uint) {
                                unreachable!();
                            }
                            if !((*f).sr_cur.p.data[0]).is_null() {
                                ::core::intrinsics::atomic_store_seqcst(
                                    &mut *((*f).sr_cur.progress).offset(0) as *mut atomic_uint,
                                    if error_0 != 0 { FRAME_ERROR } else { y },
                                );
                            }
                            ::core::intrinsics::atomic_store_seqcst(
                                &mut (*f).frame_thread.entropy_progress,
                                if error_0 != 0 { TILE_ERROR } else { sby + 1 },
                            );
                            if sby + 1 == sbh {
                                ::core::intrinsics::atomic_store_seqcst(
                                    &mut *((*f).task_thread.done).as_mut_ptr().offset(1)
                                        as *mut atomic_int,
                                    1 as c_int,
                                );
                            }
                            pthread_mutex_lock(&mut (*ttd).lock);
                            let num_tasks = ::core::intrinsics::atomic_xsub_seqcst(
                                &mut (*f).task_thread.task_counter,
                                1 as c_int,
                            ) - 1;
                            if (sby + 1) < sbh && num_tasks != 0 {
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            } else {
                                if num_tasks == 0
                                    && ::core::intrinsics::atomic_load_seqcst(
                                        &mut *((*f).task_thread.done).as_mut_ptr().offset(0)
                                            as *mut atomic_int,
                                    ) != 0
                                    && ::core::intrinsics::atomic_load_seqcst(
                                        &mut *((*f).task_thread.done).as_mut_ptr().offset(1)
                                            as *mut atomic_int,
                                    ) != 0
                                {
                                    rav1d_decode_frame_exit(
                                        &mut *f,
                                        if error_0 == 1 {
                                            Err(EINVAL)
                                        } else if error_0 != 0 {
                                            Err(ENOMEM)
                                        } else {
                                            Ok(())
                                        },
                                    );
                                    (*f).n_tile_data = 0 as c_int;
                                    pthread_cond_signal(&mut (*f).task_thread.cond);
                                }
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            }
                        } else {
                            ::core::intrinsics::atomic_or_seqcst(
                                &mut *((*f).frame_thread.frame_progress).offset((sby >> 5) as isize)
                                    as *mut atomic_uint,
                                (1 as c_uint) << (sby & 31),
                            );
                            pthread_mutex_lock(&mut (*f).task_thread.lock);
                            sby = get_frame_progress(c, f);
                            error_0 =
                                ::core::intrinsics::atomic_load_seqcst(&mut (*f).task_thread.error);
                            let y_0: c_uint = if sby + 1 == sbh {
                                u32::MAX
                            } else {
                                ((sby + 1) as c_uint).wrapping_mul(sbsz as c_uint)
                            };
                            if (*c).n_fc > 1 as c_uint && !((*f).sr_cur.p.data[0]).is_null() {
                                ::core::intrinsics::atomic_store_seqcst(
                                    &mut *((*f).sr_cur.progress).offset(1) as *mut atomic_uint,
                                    if error_0 != 0 { FRAME_ERROR } else { y_0 },
                                );
                            }
                            pthread_mutex_unlock(&mut (*f).task_thread.lock);
                            if sby + 1 == sbh {
                                ::core::intrinsics::atomic_store_seqcst(
                                    &mut *((*f).task_thread.done).as_mut_ptr().offset(0)
                                        as *mut atomic_int,
                                    1 as c_int,
                                );
                            }
                            pthread_mutex_lock(&mut (*ttd).lock);
                            let num_tasks_0 = ::core::intrinsics::atomic_xsub_seqcst(
                                &mut (*f).task_thread.task_counter,
                                1 as c_int,
                            ) - 1;
                            if (sby + 1) < sbh && num_tasks_0 != 0 {
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            } else {
                                if num_tasks_0 == 0
                                    && ::core::intrinsics::atomic_load_seqcst(
                                        &mut *((*f).task_thread.done).as_mut_ptr().offset(0)
                                            as *mut atomic_int,
                                    ) != 0
                                    && (uses_2pass_0 == 0
                                        || ::core::intrinsics::atomic_load_seqcst(
                                            &mut *((*f).task_thread.done).as_mut_ptr().offset(1)
                                                as *mut atomic_int,
                                        ) != 0)
                                {
                                    rav1d_decode_frame_exit(
                                        &mut *f,
                                        if error_0 == 1 {
                                            Err(EINVAL)
                                        } else if error_0 != 0 {
                                            Err(ENOMEM)
                                        } else {
                                            Ok(())
                                        },
                                    );
                                    (*f).n_tile_data = 0 as c_int;
                                    pthread_cond_signal(&mut (*f).task_thread.cond);
                                }
                                reset_task_cur(c, ttd, (*t).frame_idx);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        (*tc).task_thread.flushed = true;
        pthread_cond_signal(&mut (*tc).task_thread.td.cond);
        ::core::intrinsics::atomic_store_seqcst(&mut (*ttd).cond_signaled, 0 as c_int);
        pthread_cond_wait(&mut (*ttd).cond, &mut (*ttd).lock);
        (*tc).task_thread.flushed = false;
        reset_task_cur(c, ttd, u32::MAX);
    }
    pthread_mutex_unlock(&mut (*ttd).lock);
    return 0 as *mut c_void;
}
