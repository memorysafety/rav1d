#[cfg(feature = "bitdepth_16")]
use crate::include::common::bitdepth::BitDepth16;
#[cfg(feature = "bitdepth_8")]
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::validate::validate_input;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::Dav1dContext;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dSettings;
use crate::include::dav1d::dav1d::Rav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Rav1dInloopFilterType;
use crate::include::dav1d::dav1d::Rav1dSettings;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::c_box::FnFree;
use crate::src::cpu::rav1d_init_cpu;
use crate::src::cpu::rav1d_num_logical_processors;
use crate::src::decode::rav1d_decode_frame_exit;
use crate::src::error::Dav1dResult;
use crate::src::error::Rav1dError::EGeneric;
use crate::src::error::Rav1dError::EAGAIN;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dResult;
use crate::src::extensions::OptionError as _;
use crate::src::fg_apply;
use crate::src::internal::Rav1dBitDepthDSPContext;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dContextTaskThread;
use crate::src::internal::Rav1dContextTaskType;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTaskContext_task_thread;
use crate::src::internal::TaskThreadData;
use crate::src::iter::wrapping_iter;
use crate::src::log::Rav1dLog as _;
use crate::src::obu::rav1d_parse_obus;
use crate::src::obu::rav1d_parse_sequence_header;
use crate::src::picture::rav1d_picture_alloc_copy;
use crate::src::picture::PictureFlags;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::thread_task::rav1d_task_delayed_fg;
use crate::src::thread_task::rav1d_worker_task;
use crate::src::thread_task::FRAME_ERROR;
use parking_lot::Condvar;
use parking_lot::Mutex;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem;
use std::process::abort;
use std::ptr;
use std::ptr::NonNull;
use std::slice;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Once;
use std::thread;
use to_method::To as _;

#[cold]
fn init_internal() {
    rav1d_init_cpu();
}

const DAV1D_VERSION: &CStr = c"966d63c1";
const RAV1D_VERSION: &str = match DAV1D_VERSION.to_str() {
    Ok(version) => version,
    Err(_) => unreachable!(),
};

pub const fn rav1d_version() -> &'static str {
    RAV1D_VERSION
}

#[no_mangle]
#[cold]
pub extern "C" fn dav1d_version() -> *const c_char {
    DAV1D_VERSION.as_ptr()
}

pub const DAV1D_API_VERSION_MAJOR: u8 = 7;
pub const DAV1D_API_VERSION_MINOR: u8 = 0;
pub const DAV1D_API_VERSION_PATCH: u8 = 0;

/// Get the `dav1d` library C API version.
///
/// Return a value in the format `0x00XXYYZZ`, where `XX` is the major version,
/// `YY` the minor version, and `ZZ` the patch version.
#[no_mangle]
#[cold]
pub extern "C" fn dav1d_version_api() -> c_uint {
    u32::from_be_bytes([
        0,
        DAV1D_API_VERSION_MAJOR,
        DAV1D_API_VERSION_MINOR,
        DAV1D_API_VERSION_PATCH,
    ])
}

impl Default for Rav1dSettings {
    fn default() -> Self {
        Self {
            n_threads: 0,
            max_frame_delay: 0,
            apply_grain: true,
            operating_point: 0,
            all_layers: true,
            frame_size_limit: 0,
            allocator: Default::default(),
            logger: Default::default(),
            strict_std_compliance: false,
            output_invisible_frames: false,
            inloop_filters: Rav1dInloopFilterType::all(),
            decode_frame_type: Rav1dDecodeFrameType::All,
        }
    }
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_default_settings(s: *mut Dav1dSettings) {
    s.write(Rav1dSettings::default().into());
}

struct NumThreads {
    n_tc: usize,
    n_fc: usize,
}

#[cold]
fn get_num_threads(s: &Rav1dSettings) -> NumThreads {
    let n_tc = if s.n_threads != 0 {
        s.n_threads as usize
    } else {
        rav1d_num_logical_processors().clamp(1, 256)
    };
    let n_fc = if s.max_frame_delay != 0 {
        cmp::min(s.max_frame_delay as usize, n_tc)
    } else {
        cmp::min((n_tc as f64).sqrt().ceil() as usize, 8)
    };
    NumThreads { n_fc, n_tc }
}

#[cold]
pub(crate) fn rav1d_get_frame_delay(s: &Rav1dSettings) -> Rav1dResult<usize> {
    validate_input!((s.n_threads >= 0 && s.n_threads <= 256, EINVAL))?;
    validate_input!((s.max_frame_delay >= 0 && s.max_frame_delay <= 256, EINVAL))?;
    let NumThreads { n_tc: _, n_fc } = get_num_threads(s);
    Ok(n_fc)
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_get_frame_delay(s: *const Dav1dSettings) -> Dav1dResult {
    (|| {
        validate_input!((!s.is_null(), EINVAL))?;
        rav1d_get_frame_delay(&s.read().try_into()?).map(|frame_delay| frame_delay as c_uint)
    })()
    .into()
}

#[cold]
pub(crate) unsafe fn rav1d_open(c_out: &mut *mut Rav1dContext, s: &Rav1dSettings) -> Rav1dResult {
    unsafe fn error(c: *mut Rav1dContext, c_out: &mut *mut Rav1dContext) -> Rav1dResult {
        if !c.is_null() {
            close_internal(c_out, false);
        }
        return Err(ENOMEM);
    }

    static initted: Once = Once::new();
    initted.call_once(|| init_internal());
    validate_input!((s.n_threads >= 0 && s.n_threads <= 256, EINVAL))?;
    validate_input!((s.max_frame_delay >= 0 && s.max_frame_delay <= 256, EINVAL))?;
    validate_input!((s.operating_point <= 31, EINVAL))?;
    let c = Box::new(Default::default());
    let c = Box::into_raw(c);
    *c_out = c;
    let c: *mut Rav1dContext = *c_out;
    if c.is_null() {
        return error(c, c_out);
    }
    (*c).allocator = s.allocator.clone();
    (*c).logger = s.logger.clone();
    (*c).apply_grain = s.apply_grain;
    (*c).operating_point = s.operating_point;
    (*c).all_layers = s.all_layers;
    (*c).frame_size_limit = s.frame_size_limit;
    (*c).strict_std_compliance = s.strict_std_compliance;
    (*c).output_invisible_frames = s.output_invisible_frames;
    (*c).inloop_filters = s.inloop_filters;
    (*c).decode_frame_type = s.decode_frame_type;

    if (*c).allocator.is_default() {
        if !(*c).allocator.cookie.is_null() {
            return error(c, c_out);
        }
        // SAFETY: When `allocator.is_default()`, `allocator.cookie` should be a `&c.picture_pool`.
        // See `Rav1dPicAllocator::cookie` docs for more, including an analysis of the lifetime.
        (*c).allocator.cookie = ptr::from_ref(&(*c).picture_pool)
            .cast::<c_void>()
            .cast_mut();
    }

    // On 32-bit systems, extremely large frame sizes can cause overflows in
    // `rav1d_decode_frame` alloc size calculations. Prevent that from occuring
    // by enforcing a maximum frame size limit, chosen to roughly correspond to
    // the largest size possible to decode without exhausting virtual memory.
    if mem::size_of::<usize>() < 8 && s.frame_size_limit.wrapping_sub(1) >= 8192 * 8192 {
        (*c).frame_size_limit = 8192 * 8192;
        if s.frame_size_limit != 0 {
            writeln!(
                (*c).logger,
                "Frame size limit reduced from {} to {}.",
                s.frame_size_limit,
                (*c).frame_size_limit,
            );
        }
    }

    let NumThreads { n_tc, n_fc } = get_num_threads(s);
    // TODO fallible allocation
    (*c).fc = (0..n_fc).map(|i| Rav1dFrameContext::default(i)).collect();
    let ttd = TaskThreadData {
        lock: Mutex::new(()),
        cond: Condvar::new(),
        first: AtomicU32::new(0),
        cur: AtomicU32::new(n_fc as u32),
        reset_task_cur: AtomicU32::new(u32::MAX),
        cond_signaled: AtomicI32::new(0),
        delayed_fg_exec: AtomicI32::new(0),
        delayed_fg_cond: Condvar::new(),
        delayed_fg_progress: [AtomicI32::new(0), AtomicI32::new(0)],
        delayed_fg: Default::default(),
    };
    (*c).task_thread = Arc::new(ttd);
    (*c).frame_thread.out_delayed = if n_fc > 1 {
        (0..n_fc).map(|_| Default::default()).collect()
    } else {
        Box::new([])
    };
    for fc in (*c).fc.iter_mut() {
        fc.task_thread.finished = AtomicBool::new(true);
        fc.task_thread.ttd = Arc::clone(&(*c).task_thread);
        let f = fc.data.get_mut();
        f.lf.last_sharpness = u8::MAX;
    }
    (*c).tc = (0..n_tc)
        .map(|n| {
            let thread_data = Arc::new(Rav1dTaskContext_task_thread::new(Arc::clone(
                &(*c).task_thread,
            )));
            if n_tc > 1 {
                // TODO(SJC): can be removed when c is not a raw pointer
                let context_borrow = &*c;
                let thread_data_copy = Arc::clone(&thread_data);
                let handle = thread::Builder::new()
                    // Don't set stack size like `dav1d` does.
                    // See <https://github.com/memorysafety/rav1d/issues/889>.
                    .name(format!("rav1d-worker-{n}"))
                    .spawn(|| rav1d_worker_task(context_borrow, thread_data_copy))
                    .unwrap();
                Rav1dContextTaskThread {
                    task: Rav1dContextTaskType::Worker(handle),
                    thread_data,
                }
            } else {
                Rav1dContextTaskThread {
                    task: Rav1dContextTaskType::Single(Mutex::new(Box::new(
                        Rav1dTaskContext::new(Arc::clone(&thread_data)),
                    ))),
                    thread_data,
                }
            }
        })
        .collect();
    Ok(())
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_open(
    c_out: *mut *mut Dav1dContext,
    s: *const Dav1dSettings,
) -> Dav1dResult {
    (|| {
        validate_input!((!c_out.is_null(), EINVAL))?;
        validate_input!((!s.is_null(), EINVAL))?;
        rav1d_open(&mut *c_out, &s.read().try_into()?)
    })()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_parse_sequence_header(
    out: *mut Dav1dSequenceHeader,
    ptr: *const u8,
    sz: usize,
) -> Dav1dResult {
    (|| {
        validate_input!((!out.is_null(), EINVAL))?;
        validate_input!((!ptr.is_null(), EINVAL))?;
        validate_input!((sz > 0 && sz <= usize::MAX / 2, EINVAL))?;
        let seq_hdr = rav1d_parse_sequence_header(slice::from_raw_parts(ptr, sz))?;
        out.write(seq_hdr.dav1d);
        Ok(())
    })()
    .into()
}

impl Rav1dFilmGrainData {
    fn has_grain(&self) -> bool {
        self.num_y_points != 0
            || self.num_uv_points[0] != 0
            || self.num_uv_points[1] != 0
            || self.clip_to_restricted_range && self.chroma_scaling_from_luma
    }
}

impl Rav1dPicture {
    fn has_grain(&self) -> bool {
        self.frame_hdr.as_ref().unwrap().film_grain.data.has_grain()
    }
}

unsafe fn output_image(c: &mut Rav1dContext, out: &mut Rav1dPicture) -> Rav1dResult {
    let mut res = Ok(());

    let r#in: *mut Rav1dThreadPicture = if c.all_layers || !c.max_spatial_id {
        &mut c.out
    } else {
        &mut c.cache
    };
    if !c.apply_grain || !(*r#in).p.has_grain() {
        *out = mem::take(&mut (*r#in).p);
    } else {
        res = rav1d_apply_grain(c, out, &(*r#in).p);
    }
    let _ = mem::take(&mut *r#in);

    if !c.all_layers && c.max_spatial_id && c.out.p.data.is_some() {
        *r#in = mem::take(&mut c.out);
    }
    res
}

fn output_picture_ready(c: &mut Rav1dContext, drain: bool) -> bool {
    if c.cached_error.is_some() {
        return true;
    }
    if !c.all_layers && c.max_spatial_id {
        if c.out.p.data.is_some() && c.cache.p.data.is_some() {
            if c.max_spatial_id == (c.cache.p.frame_hdr.as_ref().unwrap().spatial_id != 0)
                || c.out.flags.contains(PictureFlags::NEW_TEMPORAL_UNIT)
            {
                return true;
            }
            c.cache = mem::take(&mut c.out);
            return false;
        } else {
            if c.cache.p.data.is_some() && drain {
                return true;
            } else {
                if c.out.p.data.is_some() {
                    c.cache = mem::take(&mut c.out);
                    return false;
                }
            }
        }
    }
    c.out.p.data.is_some()
}

unsafe fn drain_picture(c: &mut Rav1dContext, out: &mut Rav1dPicture) -> Rav1dResult {
    let mut drained = false;
    for _ in 0..c.fc.len() {
        let next = c.frame_thread.next;
        let fc = &c.fc[next as usize];
        let mut task_thread_lock = c.task_thread.lock.lock();
        while !fc.task_thread.finished.load(Ordering::SeqCst) {
            fc.task_thread.cond.wait(&mut task_thread_lock);
        }
        let out_delayed = &mut c.frame_thread.out_delayed[next as usize];
        if out_delayed.p.data.is_some() || fc.task_thread.error.load(Ordering::SeqCst) != 0 {
            let first = c.task_thread.first.load(Ordering::SeqCst);
            if first as usize + 1 < c.fc.len() {
                c.task_thread.first.fetch_add(1, Ordering::SeqCst);
            } else {
                c.task_thread.first.store(0, Ordering::SeqCst);
            }
            let _ = c.task_thread.reset_task_cur.compare_exchange(
                first,
                u32::MAX,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            let cur = c.task_thread.cur.load(Ordering::Relaxed);
            if cur != 0 && (cur as usize) < c.fc.len() {
                c.task_thread.cur.store(cur - 1, Ordering::Relaxed);
            }
            drained = true;
        } else if drained {
            break;
        }
        c.frame_thread.next = (c.frame_thread.next + 1) % c.fc.len() as u32;
        drop(task_thread_lock);
        mem::take(&mut *fc.task_thread.retval.try_lock().unwrap())
            .err_or(())
            .inspect_err(|_| {
                *c.cached_error_props.get_mut() = out_delayed.p.m.clone();
                let _ = mem::take(out_delayed);
            })?;
        if out_delayed.p.data.is_some() {
            let progress = out_delayed.progress.as_ref().unwrap()[1].load(Ordering::Relaxed);
            if (out_delayed.visible || c.output_invisible_frames) && progress != FRAME_ERROR {
                c.out = out_delayed.clone();
                c.event_flags |= out_delayed.flags.into();
            }
            let _ = mem::take(out_delayed);
            if output_picture_ready(c, false) {
                return output_image(c, out);
            }
        }
    }
    if output_picture_ready(c, true) {
        return output_image(c, out);
    }
    Err(EAGAIN)
}

fn gen_picture(c: &mut Rav1dContext) -> Rav1dResult {
    if output_picture_ready(c, false) {
        return Ok(());
    }
    // Take so we don't have 2 `&mut`s.
    let Rav1dData {
        data: r#in,
        m: props,
    } = mem::take(&mut c.in_0);
    let Some(mut r#in) = r#in else { return Ok(()) };
    while !r#in.is_empty() {
        let len = rav1d_parse_obus(c, &r#in, &props);
        if let Ok(len) = len {
            r#in.slice_in_place(len..);
        }
        // Note that [`output_picture_ready`] doesn't read [`Rav1dContext::in_0`].
        if output_picture_ready(c, false) {
            // Restore into `c` when there's still data left.
            if !r#in.is_empty() {
                c.in_0 = Rav1dData {
                    data: Some(r#in),
                    m: props,
                }
            }
            break;
        }
        len?;
    }
    Ok(())
}

pub(crate) fn rav1d_send_data(c: &mut Rav1dContext, in_0: &mut Rav1dData) -> Rav1dResult {
    if in_0.data.is_some() {
        let sz = in_0.data.as_ref().unwrap().len();
        validate_input!((sz > 0 && sz <= usize::MAX / 2, EINVAL))?;
        c.drain = false;
    }
    if c.in_0.data.is_some() {
        return Err(EAGAIN);
    }
    c.in_0 = in_0.clone();
    let res = gen_picture(c);
    if res.is_ok() {
        let _ = mem::take(in_0);
    }
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_send_data(
    c: *mut Rav1dContext,
    in_0: *mut Dav1dData,
) -> Dav1dResult {
    (|| {
        validate_input!((!c.is_null(), EINVAL))?;
        validate_input!((!in_0.is_null(), EINVAL))?;
        let mut in_rust = in_0.read().into();
        let result = rav1d_send_data(&mut *c, &mut in_rust);
        in_0.write(in_rust.into());
        result
    })()
    .into()
}

pub(crate) unsafe fn rav1d_get_picture(
    c: &mut Rav1dContext,
    out: &mut Rav1dPicture,
) -> Rav1dResult {
    let drain = mem::replace(&mut c.drain, true);
    gen_picture(c)?;
    mem::take(&mut c.cached_error).err_or(())?;
    if output_picture_ready(c, c.fc.len() == 1) {
        return output_image(c, out);
    }
    if c.fc.len() > 1 && drain {
        return drain_picture(c, out);
    }
    Err(EAGAIN)
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_get_picture(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
) -> Dav1dResult {
    (|| {
        validate_input!((!c.is_null(), EINVAL))?;
        validate_input!((!out.is_null(), EINVAL))?;
        let c = &mut *c;
        let mut out_rust = Default::default(); // TODO(kkysen) Temporary until we return it directly.
        let result = rav1d_get_picture(c, &mut out_rust);
        out.write(out_rust.into());
        result
    })()
    .into()
}

pub(crate) fn rav1d_apply_grain(
    c: &mut Rav1dContext,
    out: &mut Rav1dPicture,
    in_0: &Rav1dPicture,
) -> Rav1dResult {
    if !in_0.has_grain() {
        *out = in_0.clone();
        return Ok(());
    }
    let res = rav1d_picture_alloc_copy(&c.logger, out, in_0.p.w, in_0);
    if res.is_err() {
        let _ = mem::take(out);
        return res;
    } else {
        if c.tc.len() > 1 {
            rav1d_task_delayed_fg(c, out, in_0);
        } else {
            match out.p.bpc {
                #[cfg(feature = "bitdepth_8")]
                bpc @ 8 => {
                    fg_apply::rav1d_apply_grain::<BitDepth8>(
                        &Rav1dBitDepthDSPContext::get(bpc).as_ref().unwrap().fg,
                        out,
                        in_0,
                    );
                }
                #[cfg(feature = "bitdepth_16")]
                bpc @ 10 | bpc @ 12 => {
                    fg_apply::rav1d_apply_grain::<BitDepth16>(
                        &Rav1dBitDepthDSPContext::get(bpc).as_ref().unwrap().fg,
                        out,
                        in_0,
                    );
                }
                _ => {
                    abort();
                }
            }
        }
        return Ok(());
    };
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) -> Dav1dResult {
    (|| {
        validate_input!((!c.is_null(), EINVAL))?;
        validate_input!((!out.is_null(), EINVAL))?;
        validate_input!((!in_0.is_null(), EINVAL))?;
        let c = &mut *c;
        let in_0 = in_0.read();
        // Don't `.update_rav1d()` [`Rav1dSequenceHeader`] because it's meant to be read-only.
        // Don't `.update_rav1d()` [`Rav1dFrameHeader`] because it's meant to be read-only.
        // Don't `.update_rav1d()` [`Rav1dITUTT35`] because we never read it.
        let mut out_rust = Default::default(); // TODO(kkysen) Temporary until we return it directly.
        let in_rust = in_0.into();
        let result = rav1d_apply_grain(c, &mut out_rust, &in_rust);
        out.write(out_rust.into());
        result
    })()
    .into()
}

pub(crate) fn rav1d_flush(c: &mut Rav1dContext) {
    let _ = mem::take(&mut c.in_0);
    let _ = mem::take(&mut c.out);
    let _ = mem::take(&mut c.cache);
    c.drain = false;
    c.cached_error = None;
    let _ = mem::take(&mut c.refs);
    let _ = mem::take(&mut c.cdf);
    let _ = mem::take(&mut c.frame_hdr);
    let _ = mem::take(&mut c.seq_hdr);
    let _ = mem::take(&mut c.content_light);
    let _ = mem::take(&mut c.mastering_display);
    let _ = mem::take(&mut c.itut_t35);
    let _ = mem::take(&mut c.cached_error_props);
    if c.fc.len() == 1 && c.tc.len() == 1 {
        return;
    }
    c.flush.store(true, Ordering::SeqCst);
    if c.tc.len() > 1 {
        let mut task_thread_lock = c.task_thread.lock.lock();
        for tc in c.tc.iter() {
            while !tc.flushed() {
                tc.thread_data.cond.wait(&mut task_thread_lock);
            }
        }
        for fc in c.fc.iter_mut() {
            fc.task_thread.tasks.clear();
        }
        c.task_thread.first.store(0, Ordering::SeqCst);
        c.task_thread.cur.store(c.fc.len() as u32, Ordering::SeqCst);
        c.task_thread
            .reset_task_cur
            .store(u32::MAX, Ordering::SeqCst);
        c.task_thread.cond_signaled.store(0, Ordering::SeqCst);
    }
    if c.fc.len() > 1 {
        for fc in wrapping_iter(c.fc.iter(), c.frame_thread.next as usize) {
            let _ = rav1d_decode_frame_exit(c, fc, Err(EGeneric));
            *fc.task_thread.retval.try_lock().unwrap() = None;
            let out_delayed = &mut c.frame_thread.out_delayed[fc.index];
            if out_delayed.p.frame_hdr.is_some() {
                let _ = mem::take(out_delayed);
            }
        }
        c.frame_thread.next = 0;
    }
    c.flush.store(false, Ordering::SeqCst);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_flush(c: *mut Dav1dContext) {
    rav1d_flush(&mut *c)
}

#[cold]
pub(crate) unsafe fn rav1d_close(c_out: &mut *mut Rav1dContext) {
    close_internal(c_out, true);
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_close(c_out: *mut *mut Dav1dContext) {
    if validate_input!(!c_out.is_null()).is_err() {
        return;
    }
    rav1d_close(&mut *c_out)
}

#[cold]
unsafe fn close_internal(c_out: &mut *mut Rav1dContext, flush: bool) {
    let c: *mut Rav1dContext = *c_out;
    if c.is_null() {
        return;
    }
    *c_out = ptr::null_mut();
    let mut c = Box::from_raw(c);
    if flush {
        rav1d_flush(&mut c);
    }
}

impl Drop for Rav1dContext {
    fn drop(&mut self) {
        if self.tc.len() > 1 {
            let ttd: &TaskThreadData = &*self.task_thread;
            let task_thread_lock = ttd.lock.lock();
            for tc in self.tc.iter() {
                tc.thread_data.die.store(true, Ordering::Relaxed);
            }
            ttd.cond.notify_all();
            drop(task_thread_lock);
            let tc = mem::take(&mut self.tc);
            for task_thread in tc.into_vec() {
                if let Rav1dContextTaskType::Worker(handle) = task_thread.task {
                    handle.join().expect("Could not join task thread");
                }
            }
        }
        let _ = mem::take(&mut self.fc);
        let _ = mem::take(&mut self.frame_thread.out_delayed);
        let _ = mem::take(&mut self.tiles);
        let _ = mem::take(&mut self.refs);
        let _ = mem::take(&mut self.seq_hdr);
        let _ = mem::take(&mut self.frame_hdr);
        let _ = mem::take(&mut self.mastering_display);
        let _ = mem::take(&mut self.content_light);
        let _ = mem::take(&mut self.itut_t35);
        let _ = mem::take(&mut self.picture_pool);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_get_event_flags(
    c: *mut Dav1dContext,
    flags: *mut Dav1dEventFlags,
) -> Dav1dResult {
    (|| {
        validate_input!((!c.is_null(), EINVAL))?;
        validate_input!((!flags.is_null(), EINVAL))?;
        flags.write(mem::take(&mut (*c).event_flags).into());
        Ok(())
    })()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_get_decode_error_data_props(
    c: *mut Dav1dContext,
    out: *mut Dav1dDataProps,
) -> Dav1dResult {
    (|| {
        validate_input!((!c.is_null(), EINVAL))?;
        validate_input!((!out.is_null(), EINVAL))?;
        out.write(mem::take(&mut *((*c).cached_error_props).get_mut()).into());
        Ok(())
    })()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_unref(p: *mut Dav1dPicture) {
    if validate_input!(!p.is_null()).is_err() {
        return;
    }
    let mut p_rust = p.read().to::<Rav1dPicture>();
    let _ = mem::take(&mut p_rust);
    p.write(p_rust.into());
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_create(buf: *mut Dav1dData, sz: usize) -> *mut u8 {
    || -> Rav1dResult<*mut u8> {
        let buf = validate_input!(NonNull::new(buf).ok_or(EINVAL))?;
        validate_input!((sz <= usize::MAX / 2, EINVAL))?;
        let data = Rav1dData::create(sz)?;
        let data = data.to::<Dav1dData>();
        let ptr = data
            .data
            .map(|ptr| ptr.as_ptr())
            .unwrap_or_else(ptr::null_mut);
        buf.as_ptr().write(data);
        Ok(ptr)
    }()
    .unwrap_or_else(|_| ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap(
    buf: *mut Dav1dData,
    ptr: *const u8,
    sz: usize,
    free_callback: Option<FnFree>,
    user_data: *mut c_void,
) -> Dav1dResult {
    || -> Rav1dResult {
        let buf = validate_input!(NonNull::new(buf).ok_or(EINVAL))?;
        let ptr = validate_input!(NonNull::new(ptr.cast_mut()).ok_or(EINVAL))?;
        validate_input!((sz <= usize::MAX / 2, EINVAL))?;
        let data = slice::from_raw_parts(ptr.as_ptr(), sz).into();
        let data = Rav1dData::wrap(data, free_callback, user_data)?;
        buf.as_ptr().write(data.into());
        Ok(())
    }()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_user_data(
    buf: *mut Dav1dData,
    user_data: *const u8,
    free_callback: Option<FnFree>,
    cookie: *mut c_void,
) -> Dav1dResult {
    || -> Rav1dResult {
        let buf = validate_input!(NonNull::new(buf).ok_or(EINVAL))?;
        // Note that `dav1d` doesn't do this check, but they do for the similar [`dav1d_data_wrap`].
        let user_data = validate_input!(NonNull::new(user_data.cast_mut()).ok_or(EINVAL))?;
        let mut data = buf.as_ptr().read().to::<Rav1dData>();
        data.wrap_user_data(user_data, free_callback, cookie)?;
        buf.as_ptr().write(data.into());
        Ok(())
    }()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_unref(buf: *mut Dav1dData) {
    let buf = validate_input!(NonNull::new(buf).ok_or(()));
    let Ok(mut buf) = buf else { return };
    let _ = mem::take(buf.as_mut()).to::<Rav1dData>();
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_unref(props: *mut Dav1dDataProps) {
    let props = validate_input!(NonNull::new(props).ok_or(()));
    let Ok(mut props) = props else { return };
    let _ = mem::take(props.as_mut()).to::<Rav1dDataProps>();
}
