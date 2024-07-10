#[cfg(feature = "bitdepth_16")]
use crate::include::common::bitdepth::BitDepth16;
#[cfg(feature = "bitdepth_8")]
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::intops::iclip;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::cdf::rav1d_cdf_thread_update;
use crate::src::decode::rav1d_decode_frame_exit;
use crate::src::decode::rav1d_decode_frame_init;
use crate::src::decode::rav1d_decode_frame_init_cdf;
use crate::src::decode::rav1d_decode_tile_sbrow;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dResult;
use crate::src::fg_apply::rav1d_apply_grain_row;
use crate::src::fg_apply::rav1d_prep_grain;
use crate::src::filmgrain::FG_BLOCK_SIZE;
use crate::src::internal::Grain;
use crate::src::internal::Rav1dBitDepthDSPContext;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dFrameContextTaskThread;
use crate::src::internal::Rav1dFrameData;
use crate::src::internal::Rav1dTask;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTaskContextTaskThread;
use crate::src::internal::TaskThreadData;
use crate::src::internal::TaskType;
use crate::src::iter::wrapping_iter;
use crate::src::relaxed_atomic::RelaxedAtomic;
use atomig::Atom;
use atomig::Atomic;
use parking_lot::Mutex;
use parking_lot::MutexGuard;
use parking_lot::RwLock;
use parking_lot::RwLockReadGuard;
use std::cmp;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::mem;
use std::num::NonZeroU32;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;
use std::process::abort;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

pub const FRAME_ERROR: u32 = u32::MAX - 1;
pub const TILE_ERROR: i32 = i32::MAX - 1;

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
fn reset_task_cur(c: &Rav1dContext, ttd: &TaskThreadData, mut frame_idx: c_uint) -> c_int {
    fn curr_found(c: &Rav1dContext, ttd: &TaskThreadData, first: usize) -> c_int {
        for fc in wrapping_iter(c.fc.iter(), first + ttd.cur.get() as usize) {
            fc.task_thread.tasks.cur_prev.set(Rav1dTaskIndex::None);
        }
        return 1;
    }
    let min_frame_idx: c_uint;
    let cur_frame_idx: c_uint;
    let first = ttd.first.load(Ordering::SeqCst);
    let mut reset_frame_idx: c_uint = ttd.reset_task_cur.swap(u32::MAX, Ordering::SeqCst);
    if reset_frame_idx < first {
        if frame_idx == u32::MAX {
            return 0 as c_int;
        }
        reset_frame_idx = u32::MAX;
    }
    if ttd.cur.get() == 0
        && c.fc[first as usize]
            .task_thread
            .tasks
            .cur_prev
            .get()
            .is_none()
    {
        return 0 as c_int;
    }
    if reset_frame_idx != u32::MAX {
        if frame_idx == u32::MAX {
            if reset_frame_idx > first.wrapping_add(ttd.cur.get()) {
                return 0 as c_int;
            }
            ttd.cur.set(reset_frame_idx.wrapping_sub(first));
            return curr_found(c, ttd, first as usize);
        }
    } else {
        if frame_idx == u32::MAX {
            return 0 as c_int;
        }
    }
    if frame_idx < first {
        frame_idx += c.fc.len() as c_uint;
    }
    min_frame_idx = cmp::min(reset_frame_idx, frame_idx);
    cur_frame_idx = first.wrapping_add(ttd.cur.get());
    if (ttd.cur.get() as usize) < c.fc.len() && cur_frame_idx < min_frame_idx {
        return 0 as c_int;
    }
    ttd.cur.set(min_frame_idx.wrapping_sub(first));
    while (ttd.cur.get() as usize) < c.fc.len() {
        if c.fc[((first + ttd.cur.get()) as usize) % c.fc.len()]
            .task_thread
            .tasks
            .head
            .load(Ordering::SeqCst)
            .is_some()
        {
            break;
        }
        ttd.cur.update(|cur| cur + 1);
    }
    return curr_found(c, ttd, first as usize);
}

#[inline]
fn reset_task_cur_async(ttd: &TaskThreadData, mut frame_idx: c_uint, n_frames: c_uint) {
    let first = ttd.first.load(Ordering::SeqCst);
    if frame_idx < first {
        frame_idx += n_frames;
    }
    let mut last_idx = frame_idx;
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

#[derive(Default)]
pub struct Rav1dTasks {
    // TODO: probably should be a VecDeque, we need to empty this and I don't think we do yet.
    tasks: RwLock<Vec<Rav1dTask>>,
    pending_tasks: Mutex<Vec<Rav1dTask>>,
    pending_tasks_merge: AtomicBool,

    pub head: Atomic<Rav1dTaskIndex>,
    // Points to the task directly before the cur pointer in the queue.
    // This cur pointer is theoretical here, we actually keep track of the
    // "prev_t" variable. This is needed to not loose the tasks in
    // [head;cur-1] when picking one for execution.
    pub cur_prev: RelaxedAtomic<Rav1dTaskIndex>,
}

impl Rav1dTasks {
    fn insert_tasks_between(
        &self,
        c: &Rav1dContext,
        first: Rav1dTaskIndex,
        last: Rav1dTaskIndex,
        a: Rav1dTaskIndex,
        b: Rav1dTaskIndex,
        cond_signal: c_int,
    ) {
        let ttd = &*c.task_thread;
        if c.flush.load(Ordering::SeqCst) {
            return;
        }
        if a.is_some() {
            assert_eq!(self.index(a).next(), b);
            self.index(a).set_next(first);
        } else {
            self.head.store(first, Ordering::SeqCst);
        }
        self.index(last).set_next(b);
        reset_task_cur(c, ttd, self.index(first).frame_idx);
        if cond_signal != 0 && ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
            ttd.cond.notify_one();
        }
    }

    fn insert_tasks(
        &self,
        c: &Rav1dContext,
        first: Rav1dTaskIndex,
        last: Rav1dTaskIndex,
        cond_signal: c_int,
    ) {
        // insert task back into task queue
        let mut prev_t = Rav1dTaskIndex::None;
        let mut t = self.head.load(Ordering::SeqCst);
        while t.is_some() {
            'next: {
                // entropy coding precedes other steps
                if self.index(t).type_0 == TaskType::TileEntropy {
                    if self.index(first).type_0 > TaskType::TileEntropy {
                        break 'next;
                    }
                    // both are entropy
                    if self.index(first).sby > self.index(t).sby {
                        break 'next;
                    }
                    if self.index(first).sby < self.index(t).sby {
                        self.insert_tasks_between(c, first, last, prev_t, t, cond_signal);
                        return;
                    }
                    // same sby
                } else {
                    if self.index(first).type_0 == TaskType::TileEntropy {
                        self.insert_tasks_between(c, first, last, prev_t, t, cond_signal);
                        return;
                    }
                    if self.index(first).sby > self.index(t).sby {
                        break 'next;
                    }
                    if self.index(first).sby < self.index(t).sby {
                        self.insert_tasks_between(c, first, last, prev_t, t, cond_signal);
                        return;
                    }
                    // same sby
                    if self.index(first).type_0 > self.index(t).type_0 {
                        break 'next;
                    }
                    if (self.index(first).type_0) < self.index(t).type_0 {
                        self.insert_tasks_between(c, first, last, prev_t, t, cond_signal);
                        return;
                    }
                    // same task type
                }

                // sort by tile-id
                assert!(
                    self.index(first).type_0 == TaskType::TileReconstruction
                        || self.index(first).type_0 == TaskType::TileEntropy
                );
                assert!(self.index(first).type_0 == self.index(t).type_0);
                assert!(self.index(t).sby == self.index(first).sby);
                let t_tile_idx = self.index(first).tile_idx;
                let p_tile_idx = self.index(t).tile_idx;
                assert!(t_tile_idx != p_tile_idx);
                if !(t_tile_idx > p_tile_idx) {
                    self.insert_tasks_between(c, first, last, prev_t, t, cond_signal);
                    return;
                }
            }
            // next:
            prev_t = t;
            t = self.index(t).next();
        }
        self.insert_tasks_between(c, first, last, prev_t, Rav1dTaskIndex::None, cond_signal);
    }

    fn push(&self, task: Rav1dTask) -> Rav1dTaskIndex {
        let mut tasks = self.tasks.try_write().unwrap();
        tasks.push(task);
        // 1-based index into tasks, so we use length after pushing
        Rav1dTaskIndex(NonZeroU32::new(tasks.len() as u32))
    }

    pub fn clear(&self) {
        self.tasks.try_write().unwrap().clear();
        self.pending_tasks.try_lock().unwrap().clear();
        self.pending_tasks_merge.store(false, Ordering::SeqCst);
        self.head.store(Default::default(), Ordering::Relaxed);
        self.cur_prev.set(Default::default());
    }

    pub fn remove(&self, t: Rav1dTaskIndex, prev_t: Rav1dTaskIndex) -> Option<Rav1dTask> {
        let next_t = self.index(t).next();
        if prev_t.is_some() {
            self.index(prev_t)
                .next
                .compare_exchange(t, next_t, Ordering::SeqCst, Ordering::SeqCst)
                .ok()?;
        } else {
            self.head
                .compare_exchange(t, next_t, Ordering::SeqCst, Ordering::SeqCst)
                .ok()?;
        }
        self.index(t).set_next(Rav1dTaskIndex::None);
        Some(self.index(t).without_next())
    }

    #[inline]
    fn index<'a>(&'a self, index: Rav1dTaskIndex) -> impl Deref<Target = Rav1dTask> + 'a {
        if let Some(index) = index.raw_index() {
            RwLockReadGuard::map(self.tasks.try_read().unwrap(), |tasks| {
                &tasks[index as usize]
            })
        } else {
            panic!("Cannot index with None");
        }
    }

    #[inline]
    fn add_pending(&self, task: Rav1dTask) {
        self.pending_tasks.lock().push(task);
        self.pending_tasks_merge.store(true, Ordering::SeqCst);
    }

    #[inline]
    fn merge_pending_frame(&self, c: &Rav1dContext) -> bool {
        let merge = self.pending_tasks_merge.swap(false, Ordering::SeqCst);
        if merge {
            let mut pending_tasks = self.pending_tasks.lock();
            let range = {
                let mut tasks = self.tasks.try_write().unwrap();
                if self.head.load(Ordering::Relaxed).is_none() {
                    tasks.clear();
                }
                let start = tasks.len() as u32;
                tasks.extend(pending_tasks.drain(..));
                start..tasks.len() as u32
            };

            for i in range {
                // 1-based index, so we have to add 1
                let task_idx = Rav1dTaskIndex(NonZeroU32::new(i + 1));
                self.insert_tasks(c, task_idx, task_idx, 0);
            }
        }
        merge
    }
}

impl Rav1dFrameContextTaskThread {
    fn insert_task(&self, c: &Rav1dContext, task: Rav1dTask, cond_signal: c_int) -> Rav1dTaskIndex {
        let idx = self.tasks.push(task);
        self.tasks.insert_tasks(c, idx, idx, cond_signal);
        idx
    }
}

/// 1-based index into the task queue vector. 0 is reserved for None.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug, Atom)]
#[repr(transparent)]
pub struct Rav1dTaskIndex(Option<NonZeroU32>);

impl Rav1dTaskIndex {
    pub const None: Self = Self(None);

    // Return the zero-based index into the task queue vector or `None`
    pub fn raw_index(self) -> Option<u32> {
        self.0.map(|i| i.get() - 1)
    }

    pub fn is_none(self) -> bool {
        self.0.is_none()
    }

    pub fn is_some(self) -> bool {
        self.0.is_some()
    }
}

impl Add<u32> for Rav1dTaskIndex {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self(self.0.and_then(|i| NonZeroU32::new(i.get() + rhs)))
    }
}

impl AddAssign<u32> for Rav1dTaskIndex {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

#[inline]
fn merge_pending(c: &Rav1dContext) -> c_int {
    let mut res = 0;
    for fc in c.fc.iter() {
        res |= fc.task_thread.tasks.merge_pending_frame(c) as c_int;
    }
    return res;
}

fn create_filter_sbrow(fc: &Rav1dFrameContext, f: &Rav1dFrameData, pass: c_int) -> Rav1dResult {
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let has_deblock = (frame_hdr.loopfilter.level_y != [0; 2]) as c_int;
    let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
    let has_cdef = seq_hdr.cdef;
    let has_resize = (frame_hdr.size.width[0] != frame_hdr.size.width[1]) as c_int;
    let has_lr = !f.lf.restore_planes.is_empty();
    if pass & 1 != 0 {
        fc.frame_thread_progress.entropy.store(0, Ordering::Relaxed);
    } else {
        let prog_sz = ((f.sbh + 31 & !(31 as c_int)) >> 5) as usize;
        let mut frame = fc.frame_thread_progress.frame.try_write().unwrap();
        frame.clear();
        frame.resize_with(prog_sz, Default::default);
        // copy_lpf is read during task selection, so we are seeing contention
        // here. This seems rare enough that it is not worth optimizing.
        let mut copy_lpf = fc.frame_thread_progress.copy_lpf.write();
        copy_lpf.clear();
        copy_lpf.resize_with(prog_sz, Default::default);
        fc.frame_thread_progress.deblock.store(0, Ordering::SeqCst);
    }
    f.frame_thread.next_tile_row[(pass & 1) as usize].set(0);
    let type_0 = if pass == 1 {
        TaskType::EntropyProgress
    } else if has_deblock != 0 {
        TaskType::DeblockCols
    } else if has_cdef != 0 || has_lr {
        TaskType::DeblockRows
    } else if has_resize != 0 {
        TaskType::SuperResolution
    } else {
        TaskType::ReconstructionProgress
    };
    let t = Rav1dTask {
        type_0,
        frame_idx: fc.index as c_uint,
        tile_idx: 0,
        sby: 0,
        recon_progress: 1,
        deblock_progress: 0,
        ..Default::default()
    };
    fc.task_thread.tasks.add_pending(t);
    Ok(())
}

pub(crate) fn rav1d_task_create_tile_sbrow(
    fc: &Rav1dFrameContext,
    f: &Rav1dFrameData,
    pass: c_int,
    _cond_signal: c_int,
) -> Rav1dResult {
    let tasks = &fc.task_thread.tasks;
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    let num_tasks = frame_hdr.tiling.cols as usize * frame_hdr.tiling.rows as usize;
    fc.task_thread.done[(pass & 1) as usize].store(0, Ordering::SeqCst);
    create_filter_sbrow(fc, f, pass)?;
    {
        let mut pending_tasks = tasks.pending_tasks.lock();
        for tile_idx in 0..num_tasks {
            let ts = &f.ts[tile_idx];
            let t = Rav1dTask {
                sby: ts.tiling.row_start >> f.sb_shift,
                recon_progress: 0,
                deblock_progress: 0,
                deps_skip: 0.into(),
                type_0: if pass != 1 {
                    TaskType::TileReconstruction
                } else {
                    TaskType::TileEntropy
                },
                frame_idx: fc.index as c_uint,
                tile_idx: tile_idx as c_uint,
                next: Default::default(),
            };
            pending_tasks.push(t);
        }
    }
    tasks.pending_tasks_merge.store(true, Ordering::SeqCst);
    fc.task_thread.init_done.store(1, Ordering::SeqCst);
    Ok(())
}

pub(crate) fn rav1d_task_frame_init(c: &Rav1dContext, fc: &Rav1dFrameContext) {
    fc.task_thread.init_done.store(0, Ordering::SeqCst);
    let init_task = Rav1dTask::init(fc.index as c_uint);
    fc.task_thread.insert_task(c, init_task, 1);
}

pub(crate) fn rav1d_task_delayed_fg(c: &Rav1dContext, out: &mut Rav1dPicture, in_0: &Rav1dPicture) {
    let ttd = &*c.task_thread;
    {
        let mut delayed_fg = ttd.delayed_fg.try_write().unwrap();
        delayed_fg.in_0 = in_0.clone();
        delayed_fg.out = out.clone();
        delayed_fg.type_0 = TaskType::FgPrep;

        // This initialization is done once per call to [`dav1d_apply_grain`].
        // Nevertheless, it is a 48 KB zero initialization that C avoids,
        // so we avoid reinitializing here if it already has the correct variant.
        //
        // NOTE: If only one bitdepth is enabled `grain` will already be
        // initialzed to the correct variant, so no update here is needed.
        #[cfg(all(feature = "bitdepth_8", feature = "bitdepth_16"))]
        match (out.p.bpc, &delayed_fg.grain) {
            (8, Grain::Bpc8(_)) | (10 | 12, Grain::Bpc16(_)) => {}
            (8, _) => delayed_fg.grain = Grain::Bpc8(Default::default()),
            (10 | 12, _) => delayed_fg.grain = Grain::Bpc16(Default::default()),
            _ => unreachable!(),
        }
    }
    let mut task_thread_lock = ttd.lock.lock();
    ttd.delayed_fg_exec.set(1);
    ttd.cond.notify_one();
    ttd.delayed_fg_cond.wait(&mut task_thread_lock);
    drop(task_thread_lock);
    ttd.delayed_fg_progress[0].store(0, Ordering::SeqCst);
    ttd.delayed_fg_progress[1].store(0, Ordering::SeqCst);
    // Release reference to in and out pictures
    let _ = mem::take(&mut *ttd.delayed_fg.try_write().unwrap());
}

#[inline]
fn ensure_progress<'l, 'ttd: 'l>(
    ttd: &'ttd TaskThreadData,
    f: &Rav1dFrameContext,
    t: &Rav1dTask,
    type_0: TaskType,
    state: &AtomicI32,
    task_thread_lock: &'l mut Option<MutexGuard<'ttd, ()>>,
) -> c_int {
    let p1 = state.load(Ordering::SeqCst);
    if p1 < t.sby {
        let t = Rav1dTask {
            type_0,
            recon_progress: 0,
            deblock_progress: t.sby,
            ..t.without_next()
        };
        f.task_thread.tasks.add_pending(t);
        *task_thread_lock = Some(ttd.lock.lock());
        return 1 as c_int;
    }
    return 0 as c_int;
}

#[inline]
fn check_tile(
    f: &Rav1dFrameData,
    task_thread: &Rav1dFrameContextTaskThread,
    t: &Rav1dTask,
    frame_mt: c_int,
) -> c_int {
    let tp = t.type_0 == TaskType::TileEntropy;
    let tile_idx = t.tile_idx as usize;
    let ts = &f.ts[tile_idx];
    let p1 = ts.progress[tp as usize].load(Ordering::SeqCst);
    if p1 < t.sby {
        return 1;
    }
    let mut error = (p1 == TILE_ERROR) as c_int;
    error |= task_thread.error.fetch_or(error, Ordering::SeqCst);
    if error == 0 && frame_mt != 0 && !tp {
        let p2 = ts.progress[1].load(Ordering::SeqCst);
        if p2 <= t.sby {
            return 1;
        }
        error = (p2 == TILE_ERROR) as c_int;
        error |= task_thread.error.fetch_or(error, Ordering::SeqCst);
    }
    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
    if error == 0 && frame_mt != 0 && !frame_hdr.frame_type.is_key_or_intra() {
        // check reference state
        let p = &f.sr_cur;
        let ss_ver = (p.p.p.layout == Rav1dPixelLayout::I420) as c_int;
        let p_b = ((t.sby + 1) << f.sb_shift + 2) as c_uint;
        let tile_sby = t.sby - (ts.tiling.row_start >> f.sb_shift);
        let lowest_px = f
            .lowest_pixel_mem
            .index(ts.lowest_pixel + tile_sby as usize);
        for n in t.deps_skip.get()..7 {
            'next: {
                let lowest = if tp {
                    // if temporal mv refs are disabled, we only need this
                    // for the primary ref; if segmentation is disabled, we
                    // don't even need that
                    p_b
                } else {
                    // +8 is postfilter-induced delay
                    let y = if lowest_px[n as usize][0] == i32::MIN {
                        i32::MIN
                    } else {
                        lowest_px[n as usize][0] + 8
                    };
                    let uv = if lowest_px[n as usize][1] == i32::MIN {
                        i32::MIN
                    } else {
                        lowest_px[n as usize][1] * ((1 as c_int) << ss_ver) + 8
                    };
                    let max = cmp::max(y, uv);
                    if max == i32::MIN {
                        break 'next;
                    }
                    iclip(max, 1 as c_int, f.refp[n as usize].p.p.h) as c_uint
                };
                let p3 = f.refp[n as usize].progress.as_ref().unwrap()[!tp as usize]
                    .load(Ordering::SeqCst);
                if p3 < lowest {
                    return 1;
                }
                task_thread
                    .error
                    .fetch_or((p3 == FRAME_ERROR) as c_int, Ordering::SeqCst);
            }
            // next:
            t.deps_skip.update(|it| it + 1);
        }
    }
    return 0;
}

#[inline]
fn get_frame_progress(fc: &Rav1dFrameContext, f: &Rav1dFrameData) -> c_int {
    // Note that `progress.is_some() == c.fc.len() > 1`.
    let frame_prog = f
        .sr_cur
        .progress
        .as_ref()
        .map(|progress| progress[1].load(Ordering::SeqCst))
        .unwrap_or(0);
    if frame_prog >= FRAME_ERROR {
        return f.sbh - 1;
    }
    let frame = fc.frame_thread_progress.frame.try_read().unwrap();
    let (idx, prog) = frame
        .iter()
        .enumerate()
        .skip(frame_prog as usize >> (f.sb_shift + 7))
        .find_map(|(i, progress)| {
            let val = !progress.load(Ordering::SeqCst);
            match val.trailing_zeros() {
                32 => None,
                progress => Some((i, progress)),
            }
        })
        .unwrap_or((frame.len(), 0));
    ((idx as u32) << 5 | prog) as c_int - 1
}

#[inline]
fn abort_frame(c: &Rav1dContext, fc: &Rav1dFrameContext, error: Rav1dResult) {
    fc.task_thread
        .error
        .store(if error == Err(EINVAL) { 1 } else { -1 }, Ordering::SeqCst);
    fc.task_thread.task_counter.store(0, Ordering::SeqCst);
    fc.task_thread.done[0].store(1, Ordering::SeqCst);
    fc.task_thread.done[1].store(1, Ordering::SeqCst);
    {
        let f = fc.data.try_read().unwrap();
        let progress = &**f.sr_cur.progress.as_ref().unwrap();
        progress[0].store(FRAME_ERROR, Ordering::SeqCst);
        progress[1].store(FRAME_ERROR, Ordering::SeqCst);
    }
    let _ = rav1d_decode_frame_exit(c, fc, error);
    fc.task_thread.cond.notify_one();
}

#[inline]
fn delayed_fg_task<'l, 'ttd: 'l>(
    ttd: &'ttd TaskThreadData,
    task_thread_lock: &'l mut Option<MutexGuard<'ttd, ()>>,
) {
    let delayed_fg_type = ttd.delayed_fg.try_read().unwrap().type_0;
    let mut row;
    let mut progmax;
    let mut done;
    match delayed_fg_type {
        TaskType::FgPrep => {
            ttd.delayed_fg_exec.set(0);
            if ttd.cond_signaled.load(Ordering::SeqCst) != 0 {
                ttd.cond.notify_one();
            }
            // re-borrow to allow independent field borrows
            let delayed_fg = &mut *ttd.delayed_fg.try_write().unwrap();
            let dsp = &Rav1dBitDepthDSPContext::get(delayed_fg.out.p.bpc)
                .as_ref()
                .unwrap()
                .fg;
            match &mut delayed_fg.grain {
                #[cfg(feature = "bitdepth_8")]
                Grain::Bpc8(grain) => {
                    rav1d_prep_grain::<BitDepth8>(
                        dsp,
                        &mut delayed_fg.out,
                        &delayed_fg.in_0,
                        grain,
                    );
                }
                #[cfg(feature = "bitdepth_16")]
                Grain::Bpc16(grain) => {
                    rav1d_prep_grain::<BitDepth16>(
                        dsp,
                        &mut delayed_fg.out,
                        &delayed_fg.in_0,
                        grain,
                    );
                }
            }
            delayed_fg.type_0 = TaskType::FgApply;
            ttd.delayed_fg_exec.set(1);
        }
        TaskType::FgApply => {}
        _ => {
            abort();
        }
    }
    row = ttd.delayed_fg_progress[0].fetch_add(1, Ordering::SeqCst);
    let _ = task_thread_lock.take();
    let delayed_fg = ttd.delayed_fg.try_read().unwrap();
    progmax = (delayed_fg.out.p.h + FG_BLOCK_SIZE as i32 - 1) / FG_BLOCK_SIZE as i32;
    loop {
        if (row + 1) < progmax {
            ttd.cond.notify_one();
        } else if row + 1 >= progmax {
            *task_thread_lock = Some(ttd.lock.lock());
            ttd.delayed_fg_exec.set(0);
            if row >= progmax {
                break;
            }
            let _ = task_thread_lock.take();
        }
        {
            let dsp = &Rav1dBitDepthDSPContext::get(delayed_fg.out.p.bpc)
                .as_ref()
                .unwrap()
                .fg;
            match &delayed_fg.grain {
                #[cfg(feature = "bitdepth_8")]
                Grain::Bpc8(grain) => {
                    rav1d_apply_grain_row::<BitDepth8>(
                        dsp,
                        &delayed_fg.out,
                        &delayed_fg.in_0,
                        grain,
                        row as usize,
                    );
                }
                #[cfg(feature = "bitdepth_16")]
                Grain::Bpc16(grain) => {
                    rav1d_apply_grain_row::<BitDepth16>(
                        dsp,
                        &delayed_fg.out,
                        &delayed_fg.in_0,
                        grain,
                        row as usize,
                    );
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
        *task_thread_lock = Some(ttd.lock.lock());
        ttd.delayed_fg_exec.set(0);
        break;
    }
    done = ttd.delayed_fg_progress[1].fetch_add(1, Ordering::SeqCst) + 1;
    progmax = ttd.delayed_fg_progress[0].load(Ordering::SeqCst);
    if !(done < progmax) {
        ttd.delayed_fg_cond.notify_one();
    }
}

pub fn rav1d_worker_task(task_thread: Arc<Rav1dTaskContextTaskThread>) {
    // The main thread will unpark us once `task_thread.c` is set.
    thread::park();
    let c = &*task_thread.c.lock().take().unwrap();
    let mut tc = Rav1dTaskContext::new(task_thread);

    // We clone the Arc here for the lifetime of this function to avoid an
    // immutable borrow of tc across the call to park
    let ttd_clone = Arc::clone(&tc.task_thread.ttd);
    let ttd = &*ttd_clone;

    fn park<'ttd>(
        c: &Rav1dContext,
        tc: &mut Rav1dTaskContext,
        ttd: &TaskThreadData,
        task_thread_lock: &mut MutexGuard<'ttd, ()>,
    ) {
        tc.task_thread.flushed.set(true);
        tc.task_thread.cond.notify_one();
        // we want to be woken up next time progress is signaled
        ttd.cond_signaled.store(0, Ordering::SeqCst);
        ttd.cond.wait(task_thread_lock);
        tc.task_thread.flushed.set(false);
        reset_task_cur(c, ttd, u32::MAX);
    }

    let mut task_thread_lock = Some(ttd.lock.lock());
    'outer: while !tc.task_thread.die.get() {
        if c.flush.load(Ordering::SeqCst) {
            park(c, &mut tc, ttd, task_thread_lock.as_mut().unwrap());
            continue 'outer;
        }

        merge_pending(c);
        if ttd.delayed_fg_exec.get() != 0 {
            // run delayed film grain first
            delayed_fg_task(ttd, &mut task_thread_lock);
            continue 'outer;
        }

        let (fc, t_idx, prev_t) = 'found: {
            if c.fc.len() > 1 {
                // run init tasks second
                'init_tasks: for fc in
                    wrapping_iter(c.fc.iter(), ttd.first.load(Ordering::SeqCst) as usize)
                {
                    let tasks = &fc.task_thread.tasks;
                    if fc.task_thread.init_done.load(Ordering::SeqCst) != 0 {
                        continue 'init_tasks;
                    }
                    let t_idx = tasks.head.load(Ordering::SeqCst);
                    if t_idx.is_none() {
                        continue 'init_tasks;
                    }
                    let t = tasks.index(t_idx);
                    if t.type_0 == TaskType::Init {
                        break 'found (fc, t_idx, Rav1dTaskIndex::None);
                    }
                    if t.type_0 == TaskType::InitCdf {
                        // XXX This can be a simple else, if adding tasks of both
                        // passes at once (in dav1d_task_create_tile_sbrow).
                        // Adding the tasks to the pending Q can result in a
                        // thread merging them before setting init_done.
                        // We will need to set init_done before adding to the
                        // pending Q, so maybe return the tasks, set init_done,
                        // and add to pending Q only then.
                        let in_cdf = fc.in_cdf();
                        let p1 = (if let Some(progress) = in_cdf.progress() {
                            progress.load(Ordering::SeqCst)
                        } else {
                            1 as c_int as c_uint
                        }) as c_int;
                        if p1 != 0 {
                            fc.task_thread
                                .error
                                .fetch_or((p1 == TILE_ERROR) as c_int, Ordering::SeqCst);
                            break 'found (fc, t_idx, Rav1dTaskIndex::None);
                        }
                    }
                }
            }
            // run decoding tasks last
            while (ttd.cur.get() as usize) < c.fc.len() {
                let first = ttd.first.load(Ordering::SeqCst);
                let fc = &c.fc[(first + ttd.cur.get()) as usize % c.fc.len()];
                let tasks = &fc.task_thread.tasks;
                tasks.merge_pending_frame(c);
                let mut prev_t = tasks.cur_prev.get();
                let mut t_idx = if prev_t.is_some() {
                    tasks.index(prev_t).next()
                } else {
                    tasks.head.load(Ordering::SeqCst)
                };
                while t_idx.is_some() {
                    let t = tasks.index(t_idx);
                    'next: {
                        if t.type_0 == TaskType::InitCdf {
                            break 'next;
                        }
                        if matches!(
                            t.type_0,
                            TaskType::TileEntropy | TaskType::TileReconstruction
                        ) {
                            // We need to block here because we are seeing rare
                            // contention. The fields we access out of
                            // `Rav1dFrameData` are probably ok to read
                            // concurrently with other tasks writing, but we
                            // haven't separated out these fields.
                            let f = fc.data.read();

                            // if not bottom sbrow of tile, this task will be re-added
                            // after it's finished
                            if check_tile(&f, &fc.task_thread, &t, (c.fc.len() > 1) as c_int) == 0 {
                                break 'found (fc, t_idx, prev_t);
                            }
                        } else if t.recon_progress != 0 {
                            // We need to block here because we are seeing rare
                            // contention.
                            let f = fc.data.read();
                            let p = t.type_0 == TaskType::EntropyProgress;
                            let error = fc.task_thread.error.load(Ordering::SeqCst);
                            let done = fc.task_thread.done[p as usize].load(Ordering::SeqCst);
                            assert!(done == 0 || error != 0, "done: {done}, error: {error}");
                            let frame_hdr = fc.frame_hdr();
                            let tile_row_base = frame_hdr.tiling.cols as c_int
                                * f.frame_thread.next_tile_row[p as usize].get();
                            if p {
                                let p1_0 = fc.frame_thread_progress.entropy.load(Ordering::SeqCst);
                                if p1_0 < t.sby {
                                    break 'next;
                                }
                                fc.task_thread
                                    .error
                                    .fetch_or((p1_0 == TILE_ERROR) as c_int, Ordering::SeqCst);
                            }
                            for tc_0 in 0..frame_hdr.tiling.cols {
                                let ts = &f.ts[(tile_row_base + tc_0 as c_int) as usize];
                                let p2 = ts.progress[p as usize].load(Ordering::SeqCst);
                                if p2 < t.recon_progress {
                                    break 'next;
                                }
                                fc.task_thread
                                    .error
                                    .fetch_or((p2 == TILE_ERROR) as c_int, Ordering::SeqCst);
                            }
                            if (t.sby + 1) < f.sbh {
                                // add sby+1 to list to replace this one
                                let next_t = Rav1dTask {
                                    sby: t.sby + 1,
                                    recon_progress: t.sby + 2,
                                    ..t.without_next()
                                };
                                let ntr = f.frame_thread.next_tile_row[p as usize].get() + 1;
                                let start = frame_hdr.tiling.row_start_sb[ntr as usize] as c_int;
                                if next_t.sby == start {
                                    f.frame_thread.next_tile_row[p as usize].set(ntr);
                                }
                                drop(t);
                                fc.task_thread.insert_task(c, next_t, 0);
                            }
                            break 'found (fc, t_idx, prev_t);
                        } else if t.type_0 == TaskType::Cdef {
                            let p1_1 = fc.frame_thread_progress.copy_lpf.try_read().unwrap()
                                [(t.sby - 1 >> 5) as usize]
                                .load(Ordering::SeqCst);
                            if p1_1 as c_uint & (1 as c_uint) << (t.sby - 1 & 31) != 0 {
                                break 'found (fc, t_idx, prev_t);
                            }
                        } else {
                            if t.deblock_progress == 0 {
                                unreachable!();
                            }
                            let p1_2 = fc.frame_thread_progress.deblock.load(Ordering::SeqCst);
                            if p1_2 >= t.deblock_progress {
                                fc.task_thread
                                    .error
                                    .fetch_or((p1_2 == TILE_ERROR) as c_int, Ordering::SeqCst);
                                break 'found (fc, t_idx, prev_t);
                            }
                        }
                    }
                    // next:
                    prev_t = t_idx;
                    t_idx = t.next();
                    tasks.cur_prev.set(prev_t);
                }
                ttd.cur.update(|cur| cur + 1);
            }
            if reset_task_cur(c, ttd, u32::MAX) != 0 {
                continue 'outer;
            }
            if merge_pending(c) != 0 {
                continue 'outer;
            }
            park(c, &mut tc, ttd, task_thread_lock.as_mut().unwrap());
            continue 'outer;
        };
        // found:
        // remove t from list
        let Some(mut t) = fc.task_thread.tasks.remove(t_idx, prev_t) else {
            // Another thread already consumed the task
            eprintln!("Task {t_idx:?} already consumed");
            continue 'outer;
        };
        if t.type_0 > TaskType::InitCdf
            && fc.task_thread.tasks.head.load(Ordering::SeqCst).is_none()
        {
            ttd.cur.update(|cur| cur + 1);
        }
        // we don't need to check cond_signaled here, since we found a task
        // after the last signal so we want to re-signal the next waiting thread
        // and again won't need to signal after that
        ttd.cond_signaled.store(1, Ordering::SeqCst);
        ttd.cond.notify_one();
        drop(task_thread_lock.take().expect("thread lock was not held"));

        'found_unlocked: loop {
            let flush = c.flush.load(Ordering::SeqCst) as i32;
            let mut error_0 = fc.task_thread.error.fetch_or(flush, Ordering::SeqCst) | flush;

            // run it
            let mut sby = t.sby;
            let mut task_type = t.type_0;
            'fallthrough: loop {
                match task_type {
                    TaskType::Init => {
                        if !(c.fc.len() > 1) {
                            unreachable!();
                        }
                        let res = rav1d_decode_frame_init(c, fc);
                        let p1_3 = (if let Some(progress) = fc.in_cdf().progress() {
                            progress.load(Ordering::SeqCst)
                        } else {
                            1 as c_int as c_uint
                        }) as c_int;
                        if res.is_err() || p1_3 == TILE_ERROR {
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.lock.lock());
                            abort_frame(c, fc, if res.is_err() { res } else { Err(EINVAL) });
                            reset_task_cur(c, ttd, t.frame_idx);
                        } else {
                            t.type_0 = TaskType::InitCdf;
                            if p1_3 != 0 {
                                continue 'found_unlocked;
                            }
                            fc.task_thread.tasks.add_pending(t);
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.lock.lock());
                        }
                        continue 'outer;
                    }
                    TaskType::InitCdf => {
                        if !(c.fc.len() > 1) {
                            unreachable!();
                        }
                        let mut res_0 = Err(EINVAL);
                        let mut f = fc.data.try_write().unwrap();
                        if fc.task_thread.error.load(Ordering::SeqCst) == 0 {
                            res_0 = rav1d_decode_frame_init_cdf(c, fc, &mut f, &fc.in_cdf());
                        }
                        let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                        if frame_hdr.refresh_context != 0 && !fc.task_thread.update_set.get() {
                            f.out_cdf.progress().unwrap().store(
                                (if res_0.is_err() {
                                    TILE_ERROR
                                } else {
                                    1 as c_int
                                }) as c_uint,
                                Ordering::SeqCst,
                            );
                        }
                        drop(f);
                        if res_0.is_ok() {
                            if !(c.fc.len() > 1) {
                                unreachable!();
                            }
                            let mut p_0 = 1;
                            while p_0 <= 2 {
                                let f = fc.data.try_read().unwrap();
                                let res_1 = rav1d_task_create_tile_sbrow(fc, &f, p_0, 0);
                                if res_1.is_err() {
                                    assert!(
                                        task_thread_lock.is_none(),
                                        "thread lock should not be held"
                                    );
                                    task_thread_lock = Some(ttd.lock.lock());
                                    // memory allocation failed
                                    fc.task_thread.done[(2 - p_0) as usize]
                                        .store(1 as c_int, Ordering::SeqCst);
                                    fc.task_thread.error.store(-(1 as c_int), Ordering::SeqCst);
                                    let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                                    fc.task_thread.task_counter.fetch_sub(
                                        frame_hdr.tiling.cols as c_int
                                            * frame_hdr.tiling.rows as c_int
                                            + f.sbh,
                                        Ordering::SeqCst,
                                    );

                                    // Note that `progress.is_some() == c.fc.len() > 1`.
                                    let progress = &**f.sr_cur.progress.as_ref().unwrap();
                                    progress[(p_0 - 1) as usize]
                                        .store(FRAME_ERROR, Ordering::SeqCst);
                                    if p_0 == 2
                                        && fc.task_thread.done[1].load(Ordering::SeqCst) != 0
                                    {
                                        if fc.task_thread.task_counter.load(Ordering::SeqCst) != 0 {
                                            unreachable!();
                                        }
                                        drop(f);
                                        let _ = rav1d_decode_frame_exit(c, fc, Err(ENOMEM));
                                        fc.task_thread.cond.notify_one();
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
                            task_thread_lock = Some(ttd.lock.lock());
                        } else {
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.lock.lock());
                            abort_frame(c, fc, res_0);
                            reset_task_cur(c, ttd, t.frame_idx);
                            fc.task_thread.init_done.store(1, Ordering::SeqCst);
                        }
                        continue 'outer;
                    }
                    TaskType::TileEntropy | TaskType::TileReconstruction => {
                        let f = fc.data.try_read().unwrap();
                        let p_1 = t.type_0 == TaskType::TileEntropy;
                        let tile_idx = t.tile_idx as usize;
                        let ts = &f.ts[tile_idx];
                        tc.ts = tile_idx;
                        tc.b.y = sby << f.sb_shift;
                        let uses_2pass = (c.fc.len() > 1) as c_int;
                        tc.frame_thread.pass = if uses_2pass == 0 {
                            0 as c_int
                        } else {
                            1 as c_int + (t.type_0 == TaskType::TileReconstruction) as c_int
                        };
                        if error_0 == 0 {
                            error_0 = match rav1d_decode_tile_sbrow(c, &mut tc, &f) {
                                Ok(()) => 0,
                                Err(()) => 1,
                            };
                        }
                        let progress = if error_0 != 0 { TILE_ERROR } else { 1 + sby };

                        // signal progress
                        fc.task_thread.error.fetch_or(error_0, Ordering::SeqCst);
                        if (sby + 1) << f.sb_shift < ts.tiling.row_end {
                            t.sby += 1;
                            t.deps_skip = 0.into();
                            if check_tile(&f, &fc.task_thread, &t, uses_2pass) == 0 {
                                ts.progress[p_1 as usize].store(progress, Ordering::SeqCst);
                                reset_task_cur_async(ttd, t.frame_idx, c.fc.len() as u32);
                                if ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
                                    ttd.cond.notify_one();
                                }
                                continue 'found_unlocked;
                            }
                            ts.progress[p_1 as usize].store(progress, Ordering::SeqCst);
                            fc.task_thread.tasks.add_pending(t);
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            drop(f);
                            task_thread_lock = Some(ttd.lock.lock());
                        } else {
                            assert!(task_thread_lock.is_none(), "thread lock should not be held");
                            task_thread_lock = Some(ttd.lock.lock());
                            ts.progress[p_1 as usize].store(progress, Ordering::SeqCst);
                            reset_task_cur(c, ttd, t.frame_idx);
                            error_0 = fc.task_thread.error.load(Ordering::SeqCst);
                            let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                            if frame_hdr.refresh_context != 0
                                && tc.frame_thread.pass <= 1
                                && fc.task_thread.update_set.get()
                                && frame_hdr.tiling.update as usize == tile_idx
                            {
                                if error_0 == 0 {
                                    rav1d_cdf_thread_update(
                                        frame_hdr,
                                        &mut f.out_cdf.cdf_write(),
                                        &f.ts[frame_hdr.tiling.update as usize]
                                            .context
                                            .try_lock()
                                            .unwrap()
                                            .cdf,
                                    );
                                }
                                if let Some(progress) = f.out_cdf.progress() {
                                    progress.store(
                                        (if error_0 != 0 { TILE_ERROR } else { 1 as c_int })
                                            as c_uint,
                                        Ordering::SeqCst,
                                    );
                                }
                            }
                            if fc.task_thread.task_counter.fetch_sub(1, Ordering::SeqCst) - 1 == 0
                                && fc.task_thread.done[0].load(Ordering::SeqCst) != 0
                                && (uses_2pass == 0
                                    || fc.task_thread.done[1].load(Ordering::SeqCst) != 0)
                            {
                                error_0 = fc.task_thread.error.load(Ordering::SeqCst);
                                drop(f);
                                let _ = rav1d_decode_frame_exit(
                                    c,
                                    fc,
                                    if error_0 == 1 {
                                        Err(EINVAL)
                                    } else if error_0 != 0 {
                                        Err(ENOMEM)
                                    } else {
                                        Ok(())
                                    },
                                );
                                fc.task_thread.cond.notify_one();
                            }
                            if !(fc.task_thread.task_counter.load(Ordering::SeqCst) >= 0) {
                                unreachable!();
                            }
                            if ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
                                ttd.cond.notify_one();
                            }
                        }
                        continue 'outer;
                    }
                    TaskType::DeblockCols => {
                        {
                            let f = fc.data.try_read().unwrap();
                            if fc.task_thread.error.load(Ordering::SeqCst) == 0 {
                                (f.bd_fn().filter_sbrow_deblock_cols)(c, &f, &mut tc, sby);
                            }
                        }
                        if ensure_progress(
                            ttd,
                            fc,
                            &t,
                            TaskType::DeblockRows,
                            &fc.frame_thread_progress.deblock,
                            &mut task_thread_lock,
                        ) != 0
                        {
                            continue 'outer;
                        }
                        task_type = TaskType::DeblockRows;
                        continue 'fallthrough;
                    }
                    TaskType::DeblockRows => {
                        let f = fc.data.try_read().unwrap();
                        if fc.task_thread.error.load(Ordering::SeqCst) == 0 {
                            (f.bd_fn().filter_sbrow_deblock_rows)(c, &f, &mut tc, sby);
                        }
                        // signal deblock progress
                        let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
                        let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                        if frame_hdr.loopfilter.level_y != [0; 2] {
                            drop(f);
                            error_0 = fc.task_thread.error.load(Ordering::SeqCst);
                            fc.frame_thread_progress.deblock.store(
                                if error_0 != 0 { TILE_ERROR } else { sby + 1 },
                                Ordering::SeqCst,
                            );
                            reset_task_cur_async(ttd, t.frame_idx, c.fc.len() as u32);
                            if ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
                                ttd.cond.notify_one();
                            }
                        } else if seq_hdr.cdef != 0 || !f.lf.restore_planes.is_empty() {
                            drop(f);
                            let copy_lpf = fc.frame_thread_progress.copy_lpf.try_read().unwrap();
                            copy_lpf[(sby >> 5) as usize]
                                .fetch_or((1 as c_uint) << (sby & 31), Ordering::SeqCst);
                            // CDEF needs the top buffer to be saved by lr_copy_lpf of the
                            // previous sbrow
                            if sby != 0 {
                                let prog_1 =
                                    copy_lpf[(sby - 1 >> 5) as usize].load(Ordering::SeqCst);
                                if !prog_1 as c_uint & (1 as c_uint) << (sby - 1 & 31) != 0 {
                                    t.type_0 = TaskType::Cdef;
                                    t.deblock_progress = 0 as c_int;
                                    t.recon_progress = t.deblock_progress;
                                    fc.task_thread.tasks.add_pending(t);
                                    assert!(
                                        task_thread_lock.is_none(),
                                        "thread lock should not be held"
                                    );
                                    task_thread_lock = Some(ttd.lock.lock());
                                    continue 'outer;
                                }
                            }
                        }
                        task_type = TaskType::Cdef;
                        continue 'fallthrough;
                    }
                    TaskType::Cdef => {
                        let f = fc.data.try_read().unwrap();
                        let seq_hdr = &***f.seq_hdr.as_ref().unwrap();
                        if seq_hdr.cdef != 0 {
                            if fc.task_thread.error.load(Ordering::SeqCst) == 0 {
                                (f.bd_fn().filter_sbrow_cdef)(c, &f, &mut tc, sby);
                            }
                            drop(f);
                            reset_task_cur_async(ttd, t.frame_idx, c.fc.len() as u32);
                            if ttd.cond_signaled.fetch_or(1, Ordering::SeqCst) == 0 {
                                ttd.cond.notify_one();
                            }
                        }
                        task_type = TaskType::SuperResolution;
                        continue 'fallthrough;
                    }
                    TaskType::SuperResolution => {
                        let f = fc.data.try_read().unwrap();
                        let frame_hdr = &***f.frame_hdr.as_ref().unwrap();
                        if frame_hdr.size.width[0] != frame_hdr.size.width[1] {
                            if fc.task_thread.error.load(Ordering::SeqCst) == 0 {
                                (f.bd_fn().filter_sbrow_resize)(c, &f, &mut tc, sby);
                            }
                        }
                        task_type = TaskType::LoopRestoration;
                        continue 'fallthrough;
                    }
                    TaskType::LoopRestoration => {
                        let f = fc.data.try_read().unwrap();
                        if fc.task_thread.error.load(Ordering::SeqCst) == 0
                            && !f.lf.restore_planes.is_empty()
                        {
                            (f.bd_fn().filter_sbrow_lr)(c, &f, &mut tc, sby);
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
                    TaskType::FgPrep | TaskType::FgApply => {
                        abort();
                    }
                }
                break 'fallthrough;
            }
            // if task completed [typically LR], signal picture progress as per below
            let f = fc.data.try_read().unwrap();
            let uses_2pass_0 = (c.fc.len() > 1) as c_int;
            let sbh = f.sbh;
            let sbsz = f.sb_step * 4;
            if t.type_0 == TaskType::EntropyProgress {
                error_0 = fc.task_thread.error.load(Ordering::SeqCst);
                let y: c_uint = if sby + 1 == sbh {
                    u32::MAX
                } else {
                    ((sby + 1) as c_uint).wrapping_mul(sbsz as c_uint)
                };
                // Note that `progress.is_some() == c.fc.len() > 1`.
                let progress = &**f.sr_cur.progress.as_ref().unwrap();
                if f.sr_cur.p.data.is_some() {
                    progress[0].store(if error_0 != 0 { FRAME_ERROR } else { y }, Ordering::SeqCst);
                }
                drop(f);
                fc.frame_thread_progress.entropy.store(
                    if error_0 != 0 { TILE_ERROR } else { sby + 1 },
                    Ordering::SeqCst,
                );
                if sby + 1 == sbh {
                    fc.task_thread.done[1].store(1, Ordering::SeqCst);
                }
                assert!(task_thread_lock.is_none(), "thread lock should not be held");
                task_thread_lock = Some(ttd.lock.lock());
                let num_tasks = fc.task_thread.task_counter.fetch_sub(1, Ordering::SeqCst) - 1;
                if (sby + 1) < sbh && num_tasks != 0 {
                    reset_task_cur(c, ttd, t.frame_idx);
                    continue 'outer;
                }
                if num_tasks == 0
                    && fc.task_thread.done[0].load(Ordering::SeqCst) != 0
                    && fc.task_thread.done[1].load(Ordering::SeqCst) != 0
                {
                    error_0 = fc.task_thread.error.load(Ordering::SeqCst);
                    let _ = rav1d_decode_frame_exit(
                        c,
                        fc,
                        if error_0 == 1 {
                            Err(EINVAL)
                        } else if error_0 != 0 {
                            Err(ENOMEM)
                        } else {
                            Ok(())
                        },
                    );
                    fc.task_thread.cond.notify_one();
                }
                reset_task_cur(c, ttd, t.frame_idx);
                continue 'outer;
            }
            // t->type != DAV1D_TASK_TYPE_ENTROPY_PROGRESS
            fc.frame_thread_progress.frame.try_read().unwrap()[(sby >> 5) as usize]
                .fetch_or((1 as c_uint) << (sby & 31), Ordering::SeqCst);
            {
                let _task_thread_lock = fc.task_thread.lock.lock();
                sby = get_frame_progress(fc, &f);
                error_0 = fc.task_thread.error.load(Ordering::SeqCst);
                let y_0: c_uint = if sby + 1 == sbh {
                    u32::MAX
                } else {
                    ((sby + 1) as c_uint).wrapping_mul(sbsz as c_uint)
                };
                // Note that `progress.is_some() == c.fc.len() > 1`.
                if let Some(progress) = &f.sr_cur.progress {
                    // upon flush, this can be free'ed already
                    if f.sr_cur.p.data.is_some() {
                        progress[1].store(
                            if error_0 != 0 { FRAME_ERROR } else { y_0 },
                            Ordering::SeqCst,
                        );
                    }
                }
            }
            drop(f);
            if sby + 1 == sbh {
                fc.task_thread.done[0].store(1, Ordering::SeqCst);
            }
            assert!(task_thread_lock.is_none(), "thread lock should not be held");
            task_thread_lock = Some(ttd.lock.lock());
            let num_tasks_0 = fc.task_thread.task_counter.fetch_sub(1, Ordering::SeqCst) - 1;
            if (sby + 1) < sbh && num_tasks_0 != 0 {
                reset_task_cur(c, ttd, t.frame_idx);
                continue 'outer;
            }
            if num_tasks_0 == 0
                && fc.task_thread.done[0].load(Ordering::SeqCst) != 0
                && (uses_2pass_0 == 0 || fc.task_thread.done[1].load(Ordering::SeqCst) != 0)
            {
                error_0 = fc.task_thread.error.load(Ordering::SeqCst);
                let _ = rav1d_decode_frame_exit(
                    c,
                    fc,
                    if error_0 == 1 {
                        Err(EINVAL)
                    } else if error_0 != 0 {
                        Err(ENOMEM)
                    } else {
                        Ok(())
                    },
                );
                fc.task_thread.cond.notify_one();
            }
            reset_task_cur(c, ttd, t.frame_idx);

            break 'found_unlocked;
        }
    }
    drop(task_thread_lock.take().expect("thread lock was not held"));
}
