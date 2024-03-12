use crate::include::common::attributes::ctz;
use crate::include::common::bitdepth::BitDepth;
use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::cdf::rav1d_cdf_thread_update;
use crate::src::decode::rav1d_decode_frame_exit;
use crate::src::decode::rav1d_decode_frame_init;
use crate::src::decode::rav1d_decode_frame_init_cdf;
use crate::src::decode::rav1d_decode_tile_sbrow;
use crate::src::error::Rav1dError::EGeneric;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dResult;
use crate::src::fg_apply::rav1d_apply_grain_row;
use crate::src::fg_apply::rav1d_prep_grain;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameData;
use crate::src::internal::Rav1dTask;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTaskContext_task_thread;
use crate::src::internal::Rav1dTileState;
use crate::src::internal::TaskThreadData;
use crate::src::internal::TaskThreadData_delayed_fg;
use crate::src::internal::TaskType;
use crate::src::picture::Rav1dThreadPicture;
use libc::memset;
use libc::realloc;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_long;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::mem;
use std::process::abort;
use std::ptr;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::MutexGuard;

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

/// This function resets the cur pointer to the first frame theoretically
/// executable after a task completed (ie. each time we update some progress or
/// insert some tasks in the queue).
/// When frame_idx is set, it can be either from a completed task, or from tasks
/// inserted in the queue, in which case we have to make sure the cur pointer
/// isn't past this insert.
/// The special case where frame_idx is UINT_MAX is to handle the reset after
/// completing a task and locklessly signaling progress. In this case we don't
/// enter a critical section, which is needed for this function, so we set an
/// atomic for a delayed handling, happening here. Meaning we can call this
/// function without any actual update other than what's in the atomic, hence
/// this special case.
#[inline]
unsafe fn reset_task_cur(c: &Rav1dContext, ttd: &TaskThreadData, mut frame_idx: c_uint) -> c_int {
    let min_frame_idx: c_uint;
    let cur_frame_idx: c_uint;
    let current_block: u64;
    let first = ttd.first.load(Ordering::SeqCst);
    let mut reset_frame_idx: c_uint = ttd.reset_task_cur.swap(u32::MAX, Ordering::SeqCst);
    if reset_frame_idx < first {
        if frame_idx == u32::MAX {
            return 0 as c_int;
        }
        reset_frame_idx = u32::MAX;
    }
    if ttd.cur.load(Ordering::Relaxed) == 0
        && ((*(c.fc).offset(first as isize)).task_thread.task_cur_prev).is_null()
    {
        return 0 as c_int;
    }
    if reset_frame_idx != u32::MAX {
        if frame_idx == u32::MAX {
            if reset_frame_idx > first.wrapping_add(ttd.cur.load(Ordering::Relaxed)) {
                return 0 as c_int;
            }
            ttd.cur
                .store(reset_frame_idx.wrapping_sub(first), Ordering::Relaxed);
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
                frame_idx = frame_idx.wrapping_add(c.n_fc);
            }
            min_frame_idx = cmp::min(reset_frame_idx, frame_idx);
            cur_frame_idx = first.wrapping_add(ttd.cur.load(Ordering::Relaxed));
            if ttd.cur.load(Ordering::Relaxed) < c.n_fc && cur_frame_idx < min_frame_idx {
                return 0 as c_int;
            }
            ttd.cur
                .store(min_frame_idx.wrapping_sub(first), Ordering::Relaxed);
            while ttd.cur.load(Ordering::Relaxed) < c.n_fc {
                if !((*(c.fc).offset(
                    first
                        .wrapping_add(ttd.cur.load(Ordering::Relaxed))
                        .wrapping_rem(c.n_fc) as isize,
                ))
                .task_thread
                .task_head)
                    .is_null()
                {
                    break;
                }
                ttd.cur.fetch_add(1, Ordering::Relaxed);
            }
        }
        _ => {}
    }
    let mut i: c_uint = ttd.cur.load(Ordering::Relaxed);
    while i < c.n_fc {
        let ref mut fresh0 = (*(c.fc).offset(first.wrapping_add(i).wrapping_rem(c.n_fc) as isize))
            .task_thread
            .task_cur_prev;
        *fresh0 = 0 as *mut Rav1dTask;
        i = i.wrapping_add(1);
    }
    return 1 as c_int;
}

#[inline]
unsafe fn reset_task_cur_async(ttd: &TaskThreadData, mut frame_idx: c_uint, n_frames: c_uint) {
    let first = ttd.first.load(Ordering::SeqCst);
    if frame_idx < first {
        frame_idx = frame_idx.wrapping_add(n_frames);
    }
    let mut last_idx: c_uint = frame_idx;
    loop {
        frame_idx = last_idx;
        last_idx = ttd.reset_task_cur.swap(frame_idx, Ordering::SeqCst);
        if !(last_idx < frame_idx) {
            break;
        }
    }
    if frame_idx == first && ttd.first.load(Ordering::SeqCst) != first {
        let _ = ttd.reset_task_cur.compare_exchange(
            frame_idx,
            u32::MAX,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
    }
}

unsafe fn insert_tasks_between(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    first: *mut Rav1dTask,
    last: *mut Rav1dTask,
    a: *mut Rav1dTask,
    b: *mut Rav1dTask,
    cond_signal: c_int,
) {
    let ttd: &TaskThreadData = &*f.task_thread.ttd;
    if c.flush.load(Ordering::SeqCst) != 0 {
        return;
    }
    if !(a.is_null() || (*a).next == b) {
        unreachable!();
    }
    if a.is_null() {
        f.task_thread.task_head = first;
    } else {
        (*a).next = first;
    }
    if b.is_null() {
        f.task_thread.task_tail = last;
    }
    (*last).next = b;
    reset_task_cur(c, ttd, (*first).frame_idx);
    if cond_signal != 0 && ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
        ttd.cond.notify_one();
    }
}

unsafe fn insert_tasks(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    first: *mut Rav1dTask,
    last: *mut Rav1dTask,
    cond_signal: c_int,
) {
    let mut t_ptr: *mut Rav1dTask;
    let mut prev_t: *mut Rav1dTask = 0 as *mut Rav1dTask;
    let mut current_block_34: u64;
    t_ptr = f.task_thread.task_head;
    while !t_ptr.is_null() {
        if (*t_ptr).type_0 == TaskType::TileEntropy {
            if (*first).type_0 > TaskType::TileEntropy {
                current_block_34 = 11174649648027449784;
            } else if (*first).sby > (*t_ptr).sby {
                current_block_34 = 11174649648027449784;
            } else {
                if (*first).sby < (*t_ptr).sby {
                    insert_tasks_between(c, f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
                current_block_34 = 15904375183555213903;
            }
        } else {
            if (*first).type_0 == TaskType::TileEntropy {
                insert_tasks_between(c, f, first, last, prev_t, t_ptr, cond_signal);
                return;
            }
            if (*first).sby > (*t_ptr).sby {
                current_block_34 = 11174649648027449784;
            } else {
                if (*first).sby < (*t_ptr).sby {
                    insert_tasks_between(c, f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
                if (*first).type_0 as c_uint > (*t_ptr).type_0 as c_uint {
                    current_block_34 = 11174649648027449784;
                } else {
                    if ((*first).type_0 as c_uint) < (*t_ptr).type_0 as c_uint {
                        insert_tasks_between(c, f, first, last, prev_t, t_ptr, cond_signal);
                        return;
                    }
                    current_block_34 = 15904375183555213903;
                }
            }
        }
        match current_block_34 {
            15904375183555213903 => {
                if !((*first).type_0 == TaskType::TileReconstruction
                    || (*first).type_0 == TaskType::TileEntropy)
                {
                    unreachable!();
                }
                if !((*first).type_0 as c_uint == (*t_ptr).type_0 as c_uint) {
                    unreachable!();
                }
                if !((*t_ptr).sby == (*first).sby) {
                    unreachable!();
                }
                let p = ((*first).type_0 == TaskType::TileEntropy) as c_int;
                let t_tile_idx =
                    first.offset_from(f.task_thread.tile_tasks[p as usize]) as c_long as c_int;
                let p_tile_idx =
                    t_ptr.offset_from(f.task_thread.tile_tasks[p as usize]) as c_long as c_int;
                if !(t_tile_idx != p_tile_idx) {
                    unreachable!();
                }
                if !(t_tile_idx > p_tile_idx) {
                    insert_tasks_between(c, f, first, last, prev_t, t_ptr, cond_signal);
                    return;
                }
            }
            _ => {}
        }
        prev_t = t_ptr;
        t_ptr = (*t_ptr).next;
    }
    insert_tasks_between(c, f, first, last, prev_t, 0 as *mut Rav1dTask, cond_signal);
}

#[inline]
unsafe fn insert_task(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    t: *mut Rav1dTask,
    cond_signal: c_int,
) {
    insert_tasks(c, f, t, t, cond_signal);
}

#[inline]
unsafe fn add_pending(f: &Rav1dFrameData, t: *mut Rav1dTask) {
    let mut pending_tasks = f.task_thread.pending_tasks.lock().unwrap();
    (*t).next = 0 as *mut Rav1dTask;
    if pending_tasks.head.is_null() {
        pending_tasks.head = t;
    } else {
        (*pending_tasks.tail).next = t;
    }
    pending_tasks.tail = t;
    f.task_thread.pending_tasks_merge.store(1, Ordering::SeqCst);
}

#[inline]
unsafe fn merge_pending_frame(c: &Rav1dContext, f: &mut Rav1dFrameData) -> c_int {
    let merge = f.task_thread.pending_tasks_merge.load(Ordering::SeqCst);
    if merge != 0 {
        let mut t = {
            let mut pending_tasks = f.task_thread.pending_tasks.lock().unwrap();
            let old_head = mem::take(&mut *pending_tasks).head;
            f.task_thread.pending_tasks_merge.store(0, Ordering::SeqCst);
            old_head
        };
        while !t.is_null() {
            let tmp: *mut Rav1dTask = (*t).next;
            insert_task(c, f, t, 0 as c_int);
            t = tmp;
        }
    }
    return merge;
}

#[inline]
unsafe fn merge_pending(c: &Rav1dContext) -> c_int {
    let mut res = 0;
    let mut i: c_uint = 0 as c_int as c_uint;
    while i < c.n_fc {
        res |= merge_pending_frame(c, &mut *(c.fc).offset(i as isize));
        i = i.wrapping_add(1);
    }
    return res;
}

unsafe fn create_filter_sbrow(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    pass: c_int,
    res_t: *mut *mut Rav1dTask,
) -> c_int {
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let has_deblock =
        (frame_hdr.loopfilter.level_y[0] != 0 || frame_hdr.loopfilter.level_y[1] != 0) as c_int;
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let has_cdef = seq_hdr.cdef;
    let has_resize = (frame_hdr.size.width[0] != frame_hdr.size.width[1]) as c_int;
    let has_lr = f.lf.restore_planes;
    let mut tasks: *mut Rav1dTask = f.task_thread.tasks;
    let uses_2pass = (c.n_fc > 1 as c_uint) as c_int;
    let num_tasks = f.sbh * (1 + uses_2pass);
    if num_tasks > f.task_thread.num_tasks {
        let size: usize = (::core::mem::size_of::<Rav1dTask>()).wrapping_mul(num_tasks as usize);
        tasks = realloc(f.task_thread.tasks as *mut c_void, size) as *mut Rav1dTask;
        if tasks.is_null() {
            return -(1 as c_int);
        }
        memset(tasks as *mut c_void, 0 as c_int, size);
        f.task_thread.tasks = tasks;
        f.task_thread.num_tasks = num_tasks;
    }
    tasks = tasks.offset((f.sbh * (pass & 1)) as isize);
    if pass & 1 != 0 {
        f.frame_thread_progress.entropy = AtomicI32::new(0);
    } else {
        let prog_sz = ((f.sbh + 31 & !(31 as c_int)) >> 5) as usize;
        f.frame_thread_progress.frame.clear();
        f.frame_thread_progress
            .frame
            .resize_with(prog_sz, || AtomicU32::new(0));
        f.frame_thread_progress.copy_lpf.clear();
        f.frame_thread_progress
            .copy_lpf
            .resize_with(prog_sz, || AtomicU32::new(0));
        f.frame_thread_progress.deblock.store(0, Ordering::SeqCst);
    }
    f.frame_thread.next_tile_row[(pass & 1) as usize] = 0 as c_int;
    let t: *mut Rav1dTask = &mut *tasks.offset(0) as *mut Rav1dTask;
    (*t).sby = 0 as c_int;
    (*t).recon_progress = 1 as c_int;
    (*t).deblock_progress = 0 as c_int;
    (*t).type_0 = if pass == 1 {
        TaskType::EntropyProgress
    } else if has_deblock != 0 {
        TaskType::DeblockCols
    } else if has_cdef != 0 || has_lr != 0 {
        TaskType::DeblockRows
    } else if has_resize != 0 {
        TaskType::SuperResolution
    } else {
        TaskType::ReconstructionProgress
    };
    (*t).frame_idx = (f as *mut Rav1dFrameData).offset_from(c.fc) as c_long as c_int as c_uint;
    *res_t = t;
    return 0 as c_int;
}

pub(crate) unsafe fn rav1d_task_create_tile_sbrow(
    c: &Rav1dContext,
    f: &mut Rav1dFrameData,
    pass: c_int,
    _cond_signal: c_int,
) -> Rav1dResult {
    let mut tasks: *mut Rav1dTask = f.task_thread.tile_tasks[0];
    let uses_2pass = (c.n_fc > 1 as c_uint) as c_int;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let num_tasks = frame_hdr.tiling.cols * frame_hdr.tiling.rows;
    if pass < 2 {
        let alloc_num_tasks = num_tasks * (1 + uses_2pass);
        if alloc_num_tasks > f.task_thread.num_tile_tasks {
            let size: usize =
                (::core::mem::size_of::<Rav1dTask>()).wrapping_mul(alloc_num_tasks as usize);
            tasks = realloc(f.task_thread.tile_tasks[0] as *mut c_void, size) as *mut Rav1dTask;
            if tasks.is_null() {
                return Err(EGeneric);
            }
            memset(tasks as *mut c_void, 0 as c_int, size);
            f.task_thread.tile_tasks[0] = tasks;
            f.task_thread.num_tile_tasks = alloc_num_tasks;
        }
        f.task_thread.tile_tasks[1] = tasks.offset(num_tasks as isize);
    }
    tasks = tasks.offset((num_tasks * (pass & 1)) as isize);
    let mut pf_t: *mut Rav1dTask = 0 as *mut Rav1dTask;
    if create_filter_sbrow(c, f, pass, &mut pf_t) != 0 {
        return Err(EGeneric);
    }
    let mut prev_t: *mut Rav1dTask = 0 as *mut Rav1dTask;
    let mut tile_idx = 0;
    while tile_idx < num_tasks {
        let ts: *mut Rav1dTileState = &mut *(f.ts).offset(tile_idx as isize) as *mut Rav1dTileState;
        let t: *mut Rav1dTask = &mut *tasks.offset(tile_idx as isize) as *mut Rav1dTask;
        (*t).sby = (*ts).tiling.row_start >> f.sb_shift;
        if !pf_t.is_null() && (*t).sby != 0 {
            (*prev_t).next = pf_t;
            prev_t = pf_t;
            pf_t = 0 as *mut Rav1dTask;
        }
        (*t).recon_progress = 0 as c_int;
        (*t).deblock_progress = 0 as c_int;
        (*t).deps_skip = 0 as c_int;
        (*t).type_0 = if pass != 1 {
            TaskType::TileReconstruction
        } else {
            TaskType::TileEntropy
        };
        (*t).frame_idx = (f as *mut Rav1dFrameData).offset_from(c.fc) as c_long as c_int as c_uint;
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
    f.task_thread.done[(pass & 1) as usize].store(0, Ordering::SeqCst);
    let mut pending_tasks = f.task_thread.pending_tasks.lock().unwrap();
    if !(pending_tasks.head.is_null() || pass == 2) {
        unreachable!();
    }
    if pending_tasks.head.is_null() {
        pending_tasks.head = tasks.offset(0);
    } else {
        (*pending_tasks.tail).next = tasks.offset(0);
    }
    pending_tasks.tail = prev_t;
    f.task_thread.pending_tasks_merge.store(1, Ordering::SeqCst);
    f.task_thread.init_done.store(1, Ordering::SeqCst);
    Ok(())
}

pub(crate) unsafe fn rav1d_task_frame_init(c: &Rav1dContext, f: &mut Rav1dFrameData) {
    f.task_thread.init_done.store(0, Ordering::SeqCst);
    let t: *mut Rav1dTask = &mut f.task_thread.init_task;
    (*t).type_0 = TaskType::Init;
    // TODO(SJC): add a frame context index so we don't have to rely on linear offsets
    (*t).frame_idx = (f as *mut Rav1dFrameData).offset_from(c.fc) as c_long as c_int as c_uint;
    (*t).sby = 0 as c_int;
    (*t).deblock_progress = 0 as c_int;
    (*t).recon_progress = (*t).deblock_progress;
    insert_task(c, f, t, 1 as c_int);
}

pub(crate) unsafe fn rav1d_task_delayed_fg(
    c: &mut Rav1dContext,
    out: &mut Rav1dPicture,
    in_0: &Rav1dPicture,
) {
    let ttd: &TaskThreadData = &c.task_thread;
    let mut delayed_fg = ttd.delayed_fg.lock().unwrap();
    delayed_fg.in_0 = in_0;
    delayed_fg.out = out;
    delayed_fg.type_0 = TaskType::FgPrep;
    ttd.delayed_fg_progress[0].store(0, Ordering::SeqCst);
    ttd.delayed_fg_progress[1].store(0, Ordering::SeqCst);
    delayed_fg.exec = 1 as c_int;
    ttd.cond.notify_one();
    drop(ttd.delayed_fg_cond.wait(delayed_fg).unwrap());
}

#[inline]
unsafe fn ensure_progress<'l, 'ttd: 'l>(
    ttd: &'ttd TaskThreadData,
    f: &Rav1dFrameData,
    t: *mut Rav1dTask,
    type_0: TaskType,
    state: &AtomicI32,
    target: *mut c_int,
    task_thread_lock: &'l mut Option<MutexGuard<'ttd, TaskThreadData_delayed_fg>>,
) -> c_int {
    let p1 = state.load(Ordering::SeqCst);
    if p1 < (*t).sby {
        (*t).type_0 = type_0;
        (*t).deblock_progress = 0 as c_int;
        (*t).recon_progress = (*t).deblock_progress;
        *target = (*t).sby;
        add_pending(f, t);
        *task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
        return 1 as c_int;
    }
    return 0 as c_int;
}

#[inline]
unsafe fn check_tile(t: *mut Rav1dTask, f: &mut Rav1dFrameData, frame_mt: c_int) -> c_int {
    let tp = ((*t).type_0 == TaskType::TileEntropy) as c_int; // TODO make bool
    let tile_idx = t.offset_from(f.task_thread.tile_tasks[tp as usize]) as c_long as c_int;
    let ts: *mut Rav1dTileState = &mut *(f.ts).offset(tile_idx as isize) as *mut Rav1dTileState;
    let p1 = (*ts).progress[tp as usize].load(Ordering::SeqCst);
    if p1 < (*t).sby {
        return 1 as c_int;
    }
    let mut error = (p1 == TILE_ERROR) as c_int;
    error |= f.task_thread.error.fetch_or(error, Ordering::SeqCst);
    if error == 0 && frame_mt != 0 && tp == 0 {
        let p2 = (*ts).progress[1].load(Ordering::SeqCst);
        if p2 <= (*t).sby {
            return 1 as c_int;
        }
        error = (p2 == TILE_ERROR) as c_int;
        error |= f.task_thread.error.fetch_or(error, Ordering::SeqCst);
    }
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if error == 0 && frame_mt != 0 && !frame_hdr.frame_type.is_key_or_intra() {
        let p: *const Rav1dThreadPicture = &mut f.sr_cur;
        let ss_ver =
            ((*p).p.p.layout as c_uint == Rav1dPixelLayout::I420 as c_int as c_uint) as c_int;
        let p_b: c_uint = (((*t).sby + 1) << f.sb_shift + 2) as c_uint;
        let tile_sby = (*t).sby - ((*ts).tiling.row_start >> f.sb_shift);
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
                    lowest = iclip(max, 1 as c_int, f.refp[n as usize].p.p.h) as c_uint;
                    current_block_14 = 2370887241019905314;
                }
            }
            match current_block_14 {
                2370887241019905314 => {
                    let p3 = f.refp[n as usize].progress.as_ref().unwrap()[(tp == 0) as usize]
                        .load(Ordering::SeqCst);
                    if p3 < lowest {
                        return 1 as c_int;
                    }
                    f.task_thread
                        .error
                        .fetch_or((p3 == FRAME_ERROR) as c_int, Ordering::SeqCst);
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
unsafe fn get_frame_progress(f: &Rav1dFrameData) -> c_int {
    // Note that `progress.is_some() == c.n_fc > 1`.
    let frame_prog = f
        .sr_cur
        .progress
        .as_ref()
        .map(|progress| progress[1].load(Ordering::SeqCst))
        .unwrap_or(0);
    if frame_prog >= FRAME_ERROR {
        return f.sbh - 1;
    }
    let mut idx = (frame_prog >> f.sb_shift + 7) as c_int;
    let mut prog;
    loop {
        let val: c_uint = !(f.frame_thread_progress.frame)[idx as usize].load(Ordering::SeqCst);
        prog = if val != 0 { ctz(val) } else { 32 as c_int };
        if prog != 32 as c_int {
            break;
        }
        prog = 0 as c_int;
        idx += 1;
        if !((idx as usize) < f.frame_thread_progress.frame.len()) {
            break;
        }
    }
    return (idx << 5 | prog) - 1;
}

#[inline]
unsafe fn abort_frame(c: &Rav1dContext, f: &mut Rav1dFrameData, error: Rav1dResult) {
    f.task_thread.error.store(
        if error == Err(EINVAL) {
            1 as c_int
        } else {
            -(1 as c_int)
        },
        Ordering::SeqCst,
    );
    f.task_thread.task_counter.store(0, Ordering::SeqCst);
    f.task_thread.done[0].store(1, Ordering::SeqCst);
    f.task_thread.done[1].store(1, Ordering::SeqCst);
    let progress = &**f.sr_cur.progress.as_ref().unwrap();
    progress[0].store(FRAME_ERROR, Ordering::SeqCst);
    progress[1].store(FRAME_ERROR, Ordering::SeqCst);
    rav1d_decode_frame_exit(c, f, error);
    f.task_thread.cond.notify_one();
}

#[inline]
unsafe fn delayed_fg_task<'l, 'ttd: 'l>(
    c: &Rav1dContext,
    ttd: &'ttd TaskThreadData,
    task_thread_lock: &'l mut Option<MutexGuard<'ttd, TaskThreadData_delayed_fg>>,
) {
    let delayed_fg = &mut task_thread_lock.as_mut().unwrap();
    let in_0 = delayed_fg.in_0;
    let out = delayed_fg.out;
    let mut off = 0;
    if (*out).p.bpc != 8 as c_int {
        off = ((*out).p.bpc >> 1) - 4;
    }
    let mut row;
    let mut progmax;
    let mut done;
    match delayed_fg.type_0 {
        TaskType::FgPrep => {
            delayed_fg.exec = 0 as c_int;
            if ttd.cond_signaled.load(Ordering::SeqCst) != 0 {
                ttd.cond.notify_one();
            }
            // TODO(SJC): the thread lock was dropped here, but we need the grain out of it...
            match (*out).p.bpc {
                #[cfg(feature = "bitdepth_8")]
                8 => {
                    rav1d_prep_grain::<BitDepth8>(
                        &(*(c.dsp).as_ptr().offset(0)).fg,
                        &mut *out,
                        &*in_0,
                        BitDepth8::select_mut(&mut delayed_fg.grain),
                    );
                }
                #[cfg(feature = "bitdepth_16")]
                10 | 12 => {
                    rav1d_prep_grain::<BitDepth16>(
                        &(*(c.dsp).as_ptr().offset(off as isize)).fg,
                        &mut *out,
                        &*in_0,
                        BitDepth16::select_mut(&mut delayed_fg.grain),
                    );
                }
                _ => {
                    abort();
                }
            }
            delayed_fg.type_0 = TaskType::FgApply;
            delayed_fg.exec = 1 as c_int;
        }
        TaskType::FgApply => {}
        _ => {
            abort();
        }
    }
    row = ttd.delayed_fg_progress[0].fetch_add(1, Ordering::SeqCst);
    let _ = task_thread_lock.take();
    progmax = (*out).p.h + 31 >> 5;
    loop {
        if (row + 1) < progmax {
            ttd.cond.notify_one();
        } else if row + 1 >= progmax {
            let mut delayed_fg = ttd.delayed_fg.lock().unwrap();
            delayed_fg.exec = 0 as c_int;
            *task_thread_lock = Some(delayed_fg);
            if row >= progmax {
                break;
            }
            let _ = task_thread_lock.take();
        }
        {
            let delayed_fg = ttd.delayed_fg.lock().unwrap();
            match (*out).p.bpc {
                #[cfg(feature = "bitdepth_8")]
                8 => {
                    rav1d_apply_grain_row::<BitDepth8>(
                        &(*((*c).dsp).as_ptr().offset(0)).fg,
                        &mut *out,
                        &*in_0,
                        BitDepth8::select(&delayed_fg.grain),
                        row as usize,
                    );
                }
                #[cfg(feature = "bitdepth_16")]
                10 | 12 => {
                    rav1d_apply_grain_row::<BitDepth16>(
                        &(*((*c).dsp).as_ptr().offset(off as isize)).fg,
                        &mut *out,
                        &*in_0,
                        BitDepth16::select(&delayed_fg.grain),
                        row as usize,
                    );
                }
                _ => {
                    abort();
                }
            }
        }
        row = ttd.delayed_fg_progress[0].fetch_add(1, Ordering::SeqCst);
        #[allow(unused_assignments)]
        // TODO(kkysen) non-trivial due to the atomics, so leaving for later
        {
            done = ttd.delayed_fg_progress[1].fetch_add(1, Ordering::SeqCst) + 1;
        }
        if row < progmax {
            continue;
        }
        let mut delayed_fg = ttd.delayed_fg.lock().unwrap();
        delayed_fg.exec = 0 as c_int;
        *task_thread_lock = Some(delayed_fg);
        break;
    }
    done = ttd.delayed_fg_progress[1].fetch_add(1, Ordering::SeqCst) + 1;
    progmax = ttd.delayed_fg_progress[0].load(Ordering::SeqCst);
    if !(done < progmax) {
        ttd.delayed_fg_cond.notify_one();
    }
}

pub unsafe fn rav1d_worker_task(c: &Rav1dContext, task_thread: Arc<Rav1dTaskContext_task_thread>) {
    let mut tc = Rav1dTaskContext::new(task_thread);

    // We clone the Arc here for the lifetime of this function to avoid an
    // immutable borrow of tc across the call to park
    let ttd_clone = Arc::clone(&tc.task_thread.ttd);
    let ttd = &*ttd_clone;

    rav1d_set_thread_name(b"dav1d-worker\0" as *const u8 as *const c_char);

    unsafe fn park<'ttd>(
        c: &Rav1dContext,
        tc: &mut Rav1dTaskContext,
        ttd: &TaskThreadData,
        task_thread_lock: MutexGuard<'ttd, TaskThreadData_delayed_fg>,
    ) -> MutexGuard<'ttd, TaskThreadData_delayed_fg> {
        tc.task_thread.flushed.store(true, Ordering::Relaxed);
        tc.task_thread.cond.notify_one();
        // we want to be woken up next time progress is signaled
        ttd.cond_signaled.store(0, Ordering::SeqCst);
        let task_thread_lock = ttd.cond.wait(task_thread_lock).unwrap();
        tc.task_thread.flushed.store(false, Ordering::Relaxed);
        reset_task_cur(c, ttd, u32::MAX);
        task_thread_lock
    }

    let mut task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
    'outer: while !tc.task_thread.die.load(Ordering::Relaxed) {
        if c.flush.load(Ordering::SeqCst) != 0 {
            task_thread_lock = Some(park(c, &mut tc, ttd, task_thread_lock.take().unwrap()));
            continue 'outer;
        }

        merge_pending(c);
        if task_thread_lock.as_ref().unwrap().exec != 0 {
            // run delayed film grain first
            delayed_fg_task(c, ttd, &mut task_thread_lock);
            continue 'outer;
        }

        let (f, t, mut prev_t) = 'found: {
            if c.n_fc > 1 as c_uint {
                // run init tasks second
                'init_tasks: for i in 0..c.n_fc {
                    let first = ttd.first.load(Ordering::SeqCst);
                    let f =
                        &mut *(c.fc).offset(first.wrapping_add(i).wrapping_rem(c.n_fc) as isize);
                    if f.task_thread.init_done.load(Ordering::SeqCst) != 0 {
                        continue 'init_tasks;
                    }
                    let Some(t) = f.task_thread.task_head.as_mut() else {
                        continue 'init_tasks;
                    };
                    if t.type_0 == TaskType::Init {
                        break 'found (f, t, None);
                    }
                    if t.type_0 == TaskType::InitCdf {
                        // XXX This can be a simple else, if adding tasks of both
                        // passes at once (in dav1d_task_create_tile_sbrow).
                        // Adding the tasks to the pending Q can result in a
                        // thread merging them before setting init_done.
                        // We will need to set init_done before adding to the
                        // pending Q, so maybe return the tasks, set init_done,
                        // and add to pending Q only then.
                        let p1 = (if !(f.in_cdf.progress).is_null() {
                            (*f.in_cdf.progress).load(Ordering::SeqCst)
                        } else {
                            1 as c_int as c_uint
                        }) as c_int;
                        if p1 != 0 {
                            f.task_thread
                                .error
                                .fetch_or((p1 == TILE_ERROR) as c_int, Ordering::SeqCst);
                            break 'found (f, t, None);
                        }
                    }
                }
            }
            // run decoding tasks last
            while ttd.cur.load(Ordering::Relaxed) < c.n_fc {
                let first_0 = ttd.first.load(Ordering::SeqCst);
                let f = &mut *(c.fc).offset(
                    first_0
                        .wrapping_add(ttd.cur.load(Ordering::Relaxed))
                        .wrapping_rem(c.n_fc) as isize,
                );
                merge_pending_frame(c, f);
                let mut prev_t = f.task_thread.task_cur_prev.as_mut();
                let mut next_t = if let Some(prev_t) = prev_t.as_deref_mut() {
                    prev_t.next.as_mut()
                } else {
                    f.task_thread.task_head.as_mut()
                };
                while let Some(t) = next_t {
                    'next: {
                        if t.type_0 == TaskType::InitCdf {
                            break 'next;
                        }
                        if t.type_0 == TaskType::TileEntropy
                            || t.type_0 == TaskType::TileReconstruction
                        {
                            // if not bottom sbrow of tile, this task will be re-added
                            // after it's finished
                            if check_tile(t, f, (c.n_fc > 1 as c_uint) as c_int) == 0 {
                                break 'found (f, t, prev_t);
                            }
                        } else if t.recon_progress != 0 {
                            let p = (t.type_0 == TaskType::EntropyProgress) as c_int; // TODO make bool
                            let error = f.task_thread.error.load(Ordering::SeqCst);
                            if !(f.task_thread.done[p as usize].load(Ordering::SeqCst) == 0
                                || error != 0)
                            {
                                unreachable!();
                            }
                            let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                            let tile_row_base =
                                frame_hdr.tiling.cols * f.frame_thread.next_tile_row[p as usize];
                            if p != 0 {
                                let p1_0 = f.frame_thread_progress.entropy.load(Ordering::SeqCst);
                                if p1_0 < t.sby {
                                    break 'next;
                                }
                                f.task_thread
                                    .error
                                    .fetch_or((p1_0 == TILE_ERROR) as c_int, Ordering::SeqCst);
                            }
                            for tc_0 in 0..frame_hdr.tiling.cols {
                                let ts: *mut Rav1dTileState = &mut *(f.ts)
                                    .offset((tile_row_base + tc_0) as isize)
                                    as *mut Rav1dTileState;
                                let p2 = (*ts).progress[p as usize].load(Ordering::SeqCst);
                                if p2 < t.recon_progress {
                                    break 'next;
                                }
                                f.task_thread
                                    .error
                                    .fetch_or((p2 == TILE_ERROR) as c_int, Ordering::SeqCst);
                            }
                            if (t.sby + 1) < f.sbh {
                                // add sby+1 to list to replace this one
                                // TODO(sjc): t is a reference to an array
                                // element, we need to replace this pointer
                                // arithmetic with proper indexing
                                let next_t: *mut Rav1dTask =
                                    &mut *(t as *mut Rav1dTask).offset(1) as *mut Rav1dTask;
                                *next_t = t.clone();
                                (*next_t).sby += 1;
                                let ntr = f.frame_thread.next_tile_row[p as usize] + 1;
                                let start = frame_hdr.tiling.row_start_sb[ntr as usize] as c_int;
                                if (*next_t).sby == start {
                                    f.frame_thread.next_tile_row[p as usize] = ntr;
                                }
                                (*next_t).recon_progress = (*next_t).sby + 1;
                                insert_task(c, f, next_t, 0 as c_int);
                            }
                            break 'found (f, t, prev_t);
                        } else if t.type_0 == TaskType::Cdef {
                            let p1_1 = f.frame_thread_progress.copy_lpf[(t.sby - 1 >> 5) as usize]
                                .load(Ordering::SeqCst);
                            if p1_1 as c_uint & (1 as c_uint) << (t.sby - 1 & 31) != 0 {
                                break 'found (f, t, prev_t);
                            }
                        } else {
                            if t.deblock_progress == 0 {
                                unreachable!();
                            }
                            let p1_2 = f.frame_thread_progress.deblock.load(Ordering::SeqCst);
                            if p1_2 >= t.deblock_progress {
                                f.task_thread
                                    .error
                                    .fetch_or((p1_2 == TILE_ERROR) as c_int, Ordering::SeqCst);
                                break 'found (f, t, prev_t);
                            }
                        }
                    }
                    // next:
                    next_t = t.next.as_mut();
                    prev_t = Some(t);
                    f.task_thread.task_cur_prev = prev_t.as_deref_mut().unwrap() as *mut Rav1dTask;
                }
                ttd.cur.fetch_add(1, Ordering::Relaxed);
            }
            if reset_task_cur(c, ttd, u32::MAX) != 0 {
                continue 'outer;
            }
            if merge_pending(c) != 0 {
                continue 'outer;
            }
            task_thread_lock = Some(park(c, &mut tc, ttd, task_thread_lock.take().unwrap()));
            continue 'outer;
        };
        // found:
        // remove t from list
        if let Some(prev_t) = prev_t.as_deref_mut() {
            prev_t.next = t.next;
        } else {
            f.task_thread.task_head = t.next;
        }
        if t.next.is_null() {
            f.task_thread.task_tail = prev_t
                .as_deref_mut()
                .map_or_else(ptr::null_mut, |p| p as *mut Rav1dTask);
        }
        if t.type_0 > TaskType::InitCdf && (f.task_thread.task_head).is_null() {
            ttd.cur.fetch_add(1, Ordering::Relaxed);
        }
        t.next = 0 as *mut Rav1dTask;
        // we don't need to check cond_signaled here, since we found a task
        // after the last signal so we want to re-signal the next waiting thread
        // and again won't need to signal after that
        ttd.cond_signaled.store(1, Ordering::SeqCst);
        ttd.cond.notify_one();
        drop(task_thread_lock.take().expect("thread lock was not held"));

        'found_unlocked: loop {
            let flush = c.flush.load(Ordering::SeqCst);
            let mut error_0 = f.task_thread.error.fetch_or(flush, Ordering::SeqCst) | flush;

            // run it
            let mut sby = t.sby;
            let mut task_type = t.type_0;
            'fallthrough: loop {
                match task_type {
                    TaskType::Init => {
                        if !(c.n_fc > 1 as c_uint) {
                            unreachable!();
                        }
                        let res = rav1d_decode_frame_init(c, f);
                        let p1_3 = (if !(f.in_cdf.progress).is_null() {
                            (*f.in_cdf.progress).load(Ordering::SeqCst)
                        } else {
                            1 as c_int as c_uint
                        }) as c_int;
                        if res.is_err() || p1_3 == TILE_ERROR {
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
                            abort_frame(c, f, if res.is_err() { res } else { Err(EINVAL) });
                            reset_task_cur(c, ttd, t.frame_idx);
                        } else {
                            t.type_0 = TaskType::InitCdf;
                            if p1_3 != 0 {
                                continue 'found_unlocked;
                            }
                            add_pending(f, t);
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
                        }
                        continue 'outer;
                    }
                    TaskType::InitCdf => {
                        if !(c.n_fc > 1 as c_uint) {
                            unreachable!();
                        }
                        let mut res_0 = Err(EINVAL);
                        if f.task_thread.error.load(Ordering::SeqCst) == 0 {
                            res_0 = rav1d_decode_frame_init_cdf(c, f);
                        }
                        let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                        if frame_hdr.refresh_context != 0 && !f.task_thread.update_set {
                            (*f.out_cdf.progress).store(
                                (if res_0.is_err() {
                                    TILE_ERROR
                                } else {
                                    1 as c_int
                                }) as c_uint,
                                Ordering::SeqCst,
                            );
                        }
                        if res_0.is_ok() {
                            if !(c.n_fc > 1 as c_uint) {
                                unreachable!();
                            }
                            let mut p_0 = 1;
                            while p_0 <= 2 {
                                let res_1 = rav1d_task_create_tile_sbrow(c, f, p_0, 0 as c_int);
                                if res_1.is_err() {
                                    assert!(
                                        task_thread_lock.is_none(),
                                        "thread lock should not be held"
                                    );
                                    task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
                                    // memory allocation failed
                                    f.task_thread.done[(2 - p_0) as usize]
                                        .store(1 as c_int, Ordering::SeqCst);
                                    f.task_thread.error.store(-(1 as c_int), Ordering::SeqCst);
                                    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                                    f.task_thread.task_counter.fetch_sub(
                                        frame_hdr.tiling.cols * frame_hdr.tiling.rows + f.sbh,
                                        Ordering::SeqCst,
                                    );

                                    // Note that `progress.is_some() == c.n_fc > 1`.
                                    let progress = &**f.sr_cur.progress.as_ref().unwrap();
                                    progress[(p_0 - 1) as usize]
                                        .store(FRAME_ERROR, Ordering::SeqCst);
                                    if p_0 == 2 && f.task_thread.done[1].load(Ordering::SeqCst) != 0
                                    {
                                        if f.task_thread.task_counter.load(Ordering::SeqCst) != 0 {
                                            unreachable!();
                                        }
                                        rav1d_decode_frame_exit(c, f, Err(ENOMEM));
                                        f.task_thread.cond.notify_one();
                                    } else {
                                        drop(
                                            task_thread_lock
                                                .take()
                                                .expect("thread lock should have been held"),
                                        );
                                    }
                                }
                                p_0 += 1;
                            }
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
                        } else {
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
                            abort_frame(c, f, res_0);
                            reset_task_cur(c, ttd, t.frame_idx);
                            f.task_thread.init_done.store(1, Ordering::SeqCst);
                        }
                        continue 'outer;
                    }
                    TaskType::TileEntropy | TaskType::TileReconstruction => {
                        let p_1 = (t.type_0 == TaskType::TileEntropy) as c_int; // TODO make bool

                        // TODO(sjc): t is a reference to an array element, we
                        // need to replace this pointer arithmetic with proper
                        // indexing
                        let tile_idx = (t as *const Rav1dTask)
                            .offset_from(f.task_thread.tile_tasks[p_1 as usize])
                            as c_long as c_int;
                        let ts_0: *mut Rav1dTileState =
                            &mut *(f.ts).offset(tile_idx as isize) as *mut Rav1dTileState;
                        tc.ts = ts_0;
                        tc.by = sby << f.sb_shift;
                        let uses_2pass = (c.n_fc > 1 as c_uint) as c_int;
                        tc.frame_thread.pass = if uses_2pass == 0 {
                            0 as c_int
                        } else {
                            1 as c_int + (t.type_0 == TaskType::TileReconstruction) as c_int
                        };
                        if error_0 == 0 {
                            error_0 = match rav1d_decode_tile_sbrow(c, &mut tc, f) {
                                Ok(()) => 0,
                                Err(()) => 1,
                            };
                        }
                        let progress = if error_0 != 0 { TILE_ERROR } else { 1 + sby };

                        // signal progress
                        f.task_thread.error.fetch_or(error_0, Ordering::SeqCst);
                        if (sby + 1) << f.sb_shift < (*ts_0).tiling.row_end {
                            t.sby += 1;
                            t.deps_skip = 0 as c_int;
                            if check_tile(t, f, uses_2pass) == 0 {
                                (*ts_0).progress[p_1 as usize].store(progress, Ordering::SeqCst);
                                reset_task_cur_async(ttd, t.frame_idx, c.n_fc);
                                if ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
                                    ttd.cond.notify_one();
                                }
                                continue 'found_unlocked;
                            }
                            (*ts_0).progress[p_1 as usize].store(progress, Ordering::SeqCst);
                            add_pending(f, t);
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
                        } else {
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
                            (*ts_0).progress[p_1 as usize].store(progress, Ordering::SeqCst);
                            reset_task_cur(c, ttd, t.frame_idx);
                            error_0 = f.task_thread.error.load(Ordering::SeqCst);
                            let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                            if frame_hdr.refresh_context != 0
                                && tc.frame_thread.pass <= 1
                                && f.task_thread.update_set
                                && frame_hdr.tiling.update == tile_idx
                            {
                                if error_0 == 0 {
                                    rav1d_cdf_thread_update(
                                        frame_hdr,
                                        &mut *f.out_cdf.data.cdf,
                                        &(*(f.ts).offset(frame_hdr.tiling.update as isize)).cdf,
                                    );
                                }
                                if c.n_fc > 1 as c_uint {
                                    (*f.out_cdf.progress).store(
                                        (if error_0 != 0 { TILE_ERROR } else { 1 as c_int })
                                            as c_uint,
                                        Ordering::SeqCst,
                                    );
                                }
                            }
                            if f.task_thread.task_counter.fetch_sub(1, Ordering::SeqCst) - 1 == 0
                                && f.task_thread.done[0].load(Ordering::SeqCst) != 0
                                && (uses_2pass == 0
                                    || f.task_thread.done[1].load(Ordering::SeqCst) != 0)
                            {
                                error_0 = f.task_thread.error.load(Ordering::SeqCst);
                                rav1d_decode_frame_exit(
                                    c,
                                    f,
                                    if error_0 == 1 {
                                        Err(EINVAL)
                                    } else if error_0 != 0 {
                                        Err(ENOMEM)
                                    } else {
                                        Ok(())
                                    },
                                );
                                f.task_thread.cond.notify_one();
                            }
                            if !(f.task_thread.task_counter.load(Ordering::SeqCst) >= 0) {
                                unreachable!();
                            }
                            if ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
                                ttd.cond.notify_one();
                            }
                        }
                        continue 'outer;
                    }
                    TaskType::DeblockCols => {
                        if f.task_thread.error.load(Ordering::SeqCst) == 0 {
                            (f.bd_fn().filter_sbrow_deblock_cols)(c, f, &mut tc, sby);
                        }
                        if ensure_progress(
                            ttd,
                            f,
                            t,
                            TaskType::DeblockRows,
                            &f.frame_thread_progress.deblock,
                            &mut t.deblock_progress,
                            &mut task_thread_lock,
                        ) != 0
                        {
                            continue 'outer;
                        }
                        task_type = TaskType::DeblockRows;
                        continue 'fallthrough;
                    }
                    TaskType::DeblockRows => {
                        if f.task_thread.error.load(Ordering::SeqCst) == 0 {
                            (f.bd_fn().filter_sbrow_deblock_rows)(c, f, &mut tc, sby);
                        }
                        // signal deblock progress
                        let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
                        let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                        if frame_hdr.loopfilter.level_y[0] != 0
                            || frame_hdr.loopfilter.level_y[1] != 0
                        {
                            error_0 = f.task_thread.error.load(Ordering::SeqCst);
                            f.frame_thread_progress.deblock.store(
                                if error_0 != 0 { TILE_ERROR } else { sby + 1 },
                                Ordering::SeqCst,
                            );
                            reset_task_cur_async(ttd, t.frame_idx, c.n_fc);
                            if ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
                                ttd.cond.notify_one();
                            }
                        } else if seq_hdr.cdef != 0 || f.lf.restore_planes != 0 {
                            f.frame_thread_progress.copy_lpf[(sby >> 5) as usize]
                                .fetch_or((1 as c_uint) << (sby & 31), Ordering::SeqCst);
                            // CDEF needs the top buffer to be saved by lr_copy_lpf of the
                            // previous sbrow
                            if sby != 0 {
                                let prog_1 = f.frame_thread_progress.copy_lpf
                                    [(sby - 1 >> 5) as usize]
                                    .load(Ordering::SeqCst);
                                if !prog_1 as c_uint & (1 as c_uint) << (sby - 1 & 31) != 0 {
                                    t.type_0 = TaskType::Cdef;
                                    t.deblock_progress = 0 as c_int;
                                    t.recon_progress = t.deblock_progress;
                                    add_pending(f, t);
                                    assert!(
                                        task_thread_lock.is_none(),
                                        "thread lock should not be held"
                                    );
                                    task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
                                    continue 'outer;
                                }
                            }
                        }
                        task_type = TaskType::Cdef;
                        continue 'fallthrough;
                    }
                    TaskType::Cdef => {
                        let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
                        if seq_hdr.cdef != 0 {
                            if f.task_thread.error.load(Ordering::SeqCst) == 0 {
                                (f.bd_fn().filter_sbrow_cdef)(c, f, &mut tc, sby);
                            }
                            reset_task_cur_async(ttd, t.frame_idx, c.n_fc);
                            if ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
                                ttd.cond.notify_one();
                            }
                        }
                        task_type = TaskType::SuperResolution;
                        continue 'fallthrough;
                    }
                    TaskType::SuperResolution => {
                        let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                        if frame_hdr.size.width[0] != frame_hdr.size.width[1] {
                            if f.task_thread.error.load(Ordering::SeqCst) == 0 {
                                (f.bd_fn().filter_sbrow_resize)(c, f, &mut tc, sby);
                            }
                        }
                        task_type = TaskType::LoopRestoration;
                        continue 'fallthrough;
                    }
                    TaskType::LoopRestoration => {
                        if f.task_thread.error.load(Ordering::SeqCst) == 0
                            && f.lf.restore_planes != 0
                        {
                            (f.bd_fn().filter_sbrow_lr)(c, f, &mut tc, sby);
                        }
                        task_type = TaskType::ReconstructionProgress;
                        continue 'fallthrough;
                    }
                    TaskType::ReconstructionProgress => {
                        // dummy to cover for no post-filters
                    }
                    TaskType::EntropyProgress => {
                        // dummy to convert tile progress to frame
                    }
                    _ => {
                        abort();
                    }
                }
                break 'fallthrough;
            }
            // if task completed [typically LR], signal picture progress as per below
            let uses_2pass_0 = (c.n_fc > 1 as c_uint) as c_int;
            let sbh = f.sbh;
            let sbsz = f.sb_step * 4;
            if t.type_0 == TaskType::EntropyProgress {
                error_0 = f.task_thread.error.load(Ordering::SeqCst);
                let y: c_uint = if sby + 1 == sbh {
                    u32::MAX
                } else {
                    ((sby + 1) as c_uint).wrapping_mul(sbsz as c_uint)
                };
                // Note that `progress.is_some() == c.n_fc > 1`.
                let progress = &**f.sr_cur.progress.as_ref().unwrap();
                if !(f.sr_cur.p.data.data[0]).is_null() {
                    progress[0].store(if error_0 != 0 { FRAME_ERROR } else { y }, Ordering::SeqCst);
                }
                f.frame_thread_progress.entropy.store(
                    if error_0 != 0 { TILE_ERROR } else { sby + 1 },
                    Ordering::SeqCst,
                );
                if sby + 1 == sbh {
                    f.task_thread.done[1].store(1, Ordering::SeqCst);
                }
                assert!(task_thread_lock.is_none(), "thread lock should not be held");
                task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
                let num_tasks = f.task_thread.task_counter.fetch_sub(1, Ordering::SeqCst) - 1;
                if (sby + 1) < sbh && num_tasks != 0 {
                    reset_task_cur(c, ttd, t.frame_idx);
                    continue 'outer;
                }
                if num_tasks == 0
                    && f.task_thread.done[0].load(Ordering::SeqCst) != 0
                    && f.task_thread.done[1].load(Ordering::SeqCst) != 0
                {
                    error_0 = f.task_thread.error.load(Ordering::SeqCst);
                    rav1d_decode_frame_exit(
                        c,
                        f,
                        if error_0 == 1 {
                            Err(EINVAL)
                        } else if error_0 != 0 {
                            Err(ENOMEM)
                        } else {
                            Ok(())
                        },
                    );
                    f.task_thread.cond.notify_one();
                }
                reset_task_cur(c, ttd, t.frame_idx);
                continue 'outer;
            }
            // t->type != DAV1D_TASK_TYPE_ENTROPY_PROGRESS
            f.frame_thread_progress.frame[(sby >> 5) as usize]
                .fetch_or((1 as c_uint) << (sby & 31), Ordering::SeqCst);
            {
                let _task_thread_lock = f.task_thread.lock.lock().unwrap();
                sby = get_frame_progress(f);
                error_0 = f.task_thread.error.load(Ordering::SeqCst);
                let y_0: c_uint = if sby + 1 == sbh {
                    u32::MAX
                } else {
                    ((sby + 1) as c_uint).wrapping_mul(sbsz as c_uint)
                };
                // Note that `progress.is_some() == c.n_fc > 1`.
                if let Some(progress) = &f.sr_cur.progress {
                    // upon flush, this can be free'ed already
                    if !(f.sr_cur.p.data.data[0]).is_null() {
                        progress[1].store(
                            if error_0 != 0 { FRAME_ERROR } else { y_0 },
                            Ordering::SeqCst,
                        );
                    }
                }
            }
            if sby + 1 == sbh {
                f.task_thread.done[0].store(1, Ordering::SeqCst);
            }
            assert!(task_thread_lock.is_none(), "thread lock should not be held");
            task_thread_lock = Some(ttd.delayed_fg.lock().unwrap());
            let num_tasks_0 = f.task_thread.task_counter.fetch_sub(1, Ordering::SeqCst) - 1;
            if (sby + 1) < sbh && num_tasks_0 != 0 {
                reset_task_cur(c, ttd, t.frame_idx);
                continue 'outer;
            }
            if num_tasks_0 == 0
                && f.task_thread.done[0].load(Ordering::SeqCst) != 0
                && (uses_2pass_0 == 0 || f.task_thread.done[1].load(Ordering::SeqCst) != 0)
            {
                error_0 = f.task_thread.error.load(Ordering::SeqCst);
                rav1d_decode_frame_exit(
                    c,
                    f,
                    if error_0 == 1 {
                        Err(EINVAL)
                    } else if error_0 != 0 {
                        Err(ENOMEM)
                    } else {
                        Ok(())
                    },
                );
                f.task_thread.cond.notify_one();
            }
            reset_task_cur(c, ttd, t.frame_idx);

            break 'found_unlocked;
        }
    }
    drop(task_thread_lock.take().expect("thread lock was not held"));
}
