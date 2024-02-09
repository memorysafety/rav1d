use crate::include::common::bitdepth::BitDepth16;
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::bitdepth::DynCoef;
use crate::include::common::validate::validate_input;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::Dav1dContext;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dSettings;
use crate::include::dav1d::dav1d::Rav1dSettings;
use crate::include::dav1d::dav1d::RAV1D_DECODEFRAMETYPE_ALL;
use crate::include::dav1d::dav1d::RAV1D_DECODEFRAMETYPE_KEY;
use crate::include::dav1d::dav1d::RAV1D_INLOOPFILTER_ALL;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::cdf::rav1d_cdf_thread_unref;
use crate::src::cpu::rav1d_init_cpu;
use crate::src::cpu::rav1d_num_logical_processors;
use crate::src::decode::rav1d_decode_frame_exit;
use crate::src::error::Dav1dResult;
use crate::src::error::Rav1dError::EGeneric;
use crate::src::error::Rav1dError::EAGAIN;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dError::ENOENT;
use crate::src::error::Rav1dError::ENOMEM;
use crate::src::error::Rav1dResult;
use crate::src::fg_apply;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dContextTaskThread;
use crate::src::internal::Rav1dContextTaskType;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTask;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTaskContext_task_thread;
use crate::src::internal::TaskThreadData;
use crate::src::intra_edge::rav1d_init_mode_tree;
use crate::src::levels::BL_128X128;
use crate::src::levels::BL_64X64;
use crate::src::log::Rav1dLog as _;
use crate::src::mem::freep;
use crate::src::mem::rav1d_alloc_aligned;
use crate::src::mem::rav1d_free_aligned;
use crate::src::mem::rav1d_freep_aligned;
use crate::src::mem::rav1d_mem_pool_end;
use crate::src::mem::rav1d_mem_pool_init;
use crate::src::obu::rav1d_parse_obus;
use crate::src::picture::dav1d_default_picture_alloc;
use crate::src::picture::dav1d_default_picture_release;
use crate::src::picture::rav1d_picture_alloc_copy;
use crate::src::picture::rav1d_picture_move_ref;
use crate::src::picture::rav1d_picture_ref;
use crate::src::picture::rav1d_picture_unref_internal;
use crate::src::picture::rav1d_thread_picture_move_ref;
use crate::src::picture::rav1d_thread_picture_ref;
use crate::src::picture::rav1d_thread_picture_unref;
use crate::src::picture::PictureFlags;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::r#ref::rav1d_ref_dec;
use crate::src::refmvs::rav1d_refmvs_clear;
use crate::src::refmvs::rav1d_refmvs_dsp_init;
use crate::src::refmvs::rav1d_refmvs_init;
use crate::src::thread_task::rav1d_task_delayed_fg;
use crate::src::thread_task::rav1d_worker_task;
use crate::src::thread_task::FRAME_ERROR;
use cfg_if::cfg_if;
use libc::free;
use libc::memset;
use libc::pthread_attr_destroy;
use libc::pthread_attr_init;
use libc::pthread_attr_setstacksize;
use libc::pthread_attr_t;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::mem;
use std::mem::MaybeUninit;
use std::process::abort;
use std::ptr;
use std::ptr::NonNull;
use std::slice;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::Once;
use std::thread;
use to_method::To as _;

#[cfg(target_os = "linux")]
use libc::dlsym;

#[cfg(target_os = "linux")]
use libc::sysconf;

#[cold]
fn init_internal() {
    rav1d_init_cpu();
}

pub fn rav1d_version() -> &'static str {
    let null_termination_version = "966d63c1\0";
    &null_termination_version[..null_termination_version.len() - 1]
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_version() -> *const c_char {
    // Safety: [`rav1d_version`] has a null-terminator.
    rav1d_version().as_ptr().cast()
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
            inloop_filters: RAV1D_INLOOPFILTER_ALL,
            decode_frame_type: RAV1D_DECODEFRAMETYPE_ALL,
        }
    }
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_default_settings(s: *mut Dav1dSettings) {
    s.write(Rav1dSettings::default().into());
}

#[cold]
unsafe fn get_stack_size_internal(_thread_attr: *const pthread_attr_t) -> usize {
    if 0 != 0 {
        // TODO(perl): migrate the compile-time guard expression for this:
        // #if defined(__linux__) && defined(HAVE_DLSYM) && defined(__GLIBC__)
        cfg_if! {
            if #[cfg(target_os = "linux")] {
                let get_minstack: Option<unsafe extern "C" fn(*const pthread_attr_t) -> usize> =
                    ::core::mem::transmute::<
                        *mut c_void,
                        Option<unsafe extern "C" fn(*const pthread_attr_t) -> usize>,
                    >(dlsym(
                        0 as *mut c_void,
                        b"__pthread_get_minstack\0" as *const u8 as *const c_char,
                    ));
                if get_minstack.is_some() {
                    return (get_minstack.expect("non-null function pointer")(_thread_attr))
                        .wrapping_sub(sysconf(75) as usize);
                }
            }
        }
    }
    return 0;
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
pub(crate) unsafe fn rav1d_get_frame_delay(s: &Rav1dSettings) -> Rav1dResult<usize> {
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
    unsafe fn error(
        c: *mut Rav1dContext,
        c_out: &mut *mut Rav1dContext,
        thread_attr: *mut pthread_attr_t,
    ) -> Rav1dResult {
        if !c.is_null() {
            close_internal(c_out, 0 as c_int);
        }
        pthread_attr_destroy(thread_attr);
        return Err(ENOMEM);
    }

    static initted: Once = Once::new();
    initted.call_once(|| init_internal());
    validate_input!((s.n_threads >= 0 && s.n_threads <= 256, EINVAL))?;
    validate_input!((s.max_frame_delay >= 0 && s.max_frame_delay <= 256, EINVAL))?;
    validate_input!((s.operating_point >= 0 && s.operating_point <= 31, EINVAL))?;
    validate_input!((
        s.decode_frame_type >= RAV1D_DECODEFRAMETYPE_ALL
            && s.decode_frame_type <= RAV1D_DECODEFRAMETYPE_KEY,
        EINVAL
    ))?;
    let mut thread_attr: pthread_attr_t = std::mem::zeroed();
    if pthread_attr_init(&mut thread_attr) != 0 {
        return Err(ENOMEM);
    }
    let stack_size: usize = 1024 * 1024 * get_stack_size_internal(&mut thread_attr);
    pthread_attr_setstacksize(&mut thread_attr, stack_size);
    *c_out = rav1d_alloc_aligned(::core::mem::size_of::<Rav1dContext>(), 64) as *mut Rav1dContext;
    let c: *mut Rav1dContext = *c_out;
    if c.is_null() {
        return error(c, c_out, &mut thread_attr);
    }
    memset(
        c as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Rav1dContext>(),
    );
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
    (*c).cached_error_props = Default::default();
    if rav1d_mem_pool_init(&mut (*c).segmap_pool).is_err()
        || rav1d_mem_pool_init(&mut (*c).refmvs_pool).is_err()
        || rav1d_mem_pool_init(&mut (*c).cdf_pool).is_err()
    {
        return error(c, c_out, &mut thread_attr);
    }
    if (*c).allocator.alloc_picture_callback == dav1d_default_picture_alloc
        && (*c).allocator.release_picture_callback == dav1d_default_picture_release
    {
        if !((*c).allocator.cookie).is_null() {
            return error(c, c_out, &mut thread_attr);
        }
        if rav1d_mem_pool_init(&mut (*c).picture_pool).is_err() {
            return error(c, c_out, &mut thread_attr);
        }
        (*c).allocator.cookie = (*c).picture_pool as *mut c_void;
    } else if (*c).allocator.alloc_picture_callback == dav1d_default_picture_alloc
        || (*c).allocator.release_picture_callback == dav1d_default_picture_release
    {
        return error(c, c_out, &mut thread_attr);
    }
    if (::core::mem::size_of::<usize>() as c_ulong) < 8 as c_ulong
        && (s.frame_size_limit).wrapping_sub(1 as c_int as c_uint) >= (8192 * 8192) as c_uint
    {
        (*c).frame_size_limit = (8192 * 8192) as c_uint;
        if s.frame_size_limit != 0 {
            writeln!(
                (*c).logger,
                "Frame size limit reduced from {} to {}.",
                s.frame_size_limit,
                (*c).frame_size_limit,
            );
        }
    }
    (*c).flush = AtomicI32::new(0);
    let NumThreads { n_tc, n_fc } = get_num_threads(s);
    (*c).n_tc = n_tc as c_uint;
    (*c).n_fc = n_fc as c_uint;
    (*c).fc = rav1d_alloc_aligned(
        ::core::mem::size_of::<Rav1dFrameContext>().wrapping_mul((*c).n_fc as usize),
        32 as c_int as usize,
    ) as *mut Rav1dFrameContext;
    if ((*c).fc).is_null() {
        return error(c, c_out, &mut thread_attr);
    }
    memset(
        (*c).fc as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Rav1dFrameContext>().wrapping_mul((*c).n_fc as usize),
    );
    let ttd = TaskThreadData {
        cond: Condvar::new(),
        first: AtomicU32::new(0),
        cur: AtomicU32::new((*c).n_fc),
        reset_task_cur: AtomicU32::new(u32::MAX),
        cond_signaled: AtomicI32::new(0),
        delayed_fg_progress: [AtomicI32::new(0), AtomicI32::new(0)],
        delayed_fg_cond: Condvar::new(),
        delayed_fg: Mutex::new(mem::zeroed()),
    };
    (&mut (*c).task_thread as *mut Arc<TaskThreadData>).write(Arc::new(ttd));
    ptr::addr_of_mut!((*c).frame_thread.out_delayed).write(if (*c).n_fc > 1 {
        (0..(*c).n_fc).map(|_| Default::default()).collect()
    } else {
        Box::new([])
    });
    let mut n: c_uint = 0 as c_int as c_uint;
    while n < (*c).n_fc {
        let f: *mut Rav1dFrameContext =
            &mut *((*c).fc).offset(n as isize) as *mut Rav1dFrameContext;
        if (*c).n_tc > 1 as c_uint {
            (*f).task_thread.lock = Mutex::new(());
            (*f).task_thread.cond = Condvar::new();
            (*f).task_thread.pending_tasks = Default::default();
        }
        (&mut (*f).task_thread.ttd as *mut Arc<TaskThreadData>)
            .write(Arc::clone(&(*c).task_thread));
        (*f).lf.last_sharpness = -(1 as c_int);
        rav1d_refmvs_init(&mut (*f).rf);
        n = n.wrapping_add(1);
    }
    (*c).tc = (0..(*c).n_tc)
        .map(|_| {
            let thread_data = Arc::new(Rav1dTaskContext_task_thread::new(Arc::clone(
                &(*c).task_thread,
            )));
            if (*c).n_tc > 1 {
                // TODO(SJC): can be removed when c is not a raw pointer
                let context_borrow = &*c;
                let thread_data_copy = Arc::clone(&thread_data);
                let handle = thread::spawn(|| rav1d_worker_task(context_borrow, thread_data_copy));
                Rav1dContextTaskThread {
                    task: Rav1dContextTaskType::Worker(handle),
                    thread_data,
                }
            } else {
                Rav1dContextTaskThread {
                    task: Rav1dContextTaskType::Single(Mutex::new(Box::new(
                        Rav1dTaskContext::new(&mut *((*c).fc).offset(0), Arc::clone(&thread_data)),
                    ))),
                    thread_data,
                }
            }
        })
        .collect();
    rav1d_refmvs_dsp_init(&mut (*c).refmvs_dsp);
    (*c).intra_edge.root[BL_128X128 as c_int as usize] =
        &mut (*((*c).intra_edge.branch_sb128).as_mut_ptr().offset(0)).node;
    rav1d_init_mode_tree(
        (*c).intra_edge.root[BL_128X128 as c_int as usize],
        &mut (*c).intra_edge.tip_sb128,
        true,
    );
    (*c).intra_edge.root[BL_64X64 as c_int as usize] =
        &mut (*((*c).intra_edge.branch_sb64).as_mut_ptr().offset(0)).node;
    rav1d_init_mode_tree(
        (*c).intra_edge.root[BL_64X64 as c_int as usize],
        &mut (*c).intra_edge.tip_sb64,
        false,
    );
    pthread_attr_destroy(&mut thread_attr);
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

unsafe extern "C" fn dummy_free(data: *const u8, user_data: *mut c_void) {
    if !(!data.is_null() && user_data.is_null()) {
        unreachable!();
    }
}

pub(crate) unsafe fn rav1d_parse_sequence_header(
    ptr: *const u8,
    sz: usize,
) -> Rav1dResult<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>> {
    let s = Rav1dSettings {
        n_threads: 1,
        logger: None,
        ..Default::default()
    };
    let mut c: *mut Rav1dContext = 0 as *mut Rav1dContext;
    rav1d_open(&mut c, &s)?;
    || -> Rav1dResult<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>> {
        let Rav1dData {
            mut data,
            m: mut props,
        } = match NonNull::new(ptr.cast_mut()) {
            None => Default::default(),
            Some(ptr) => Rav1dData::wrap(
                slice::from_raw_parts(ptr.as_ptr(), sz).into(),
                Some(dummy_free),
                ptr::null_mut(),
            )?,
        };
        if let Some(data) = &mut data {
            while !data.is_empty() {
                let len = rav1d_parse_obus(&mut *c, data, &mut props, true)?;
                data.slice_in_place(len..);
            }
        }

        if (*c).seq_hdr.is_none() {
            return Err(ENOENT);
        }

        (*c).seq_hdr
            .take()
            .and_then(Arc::into_inner)
            .map(Ok)
            .unwrap()
    }()
    .inspect_err(|_| {
        rav1d_close(&mut c);
    })
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_parse_sequence_header(
    out: *mut Dav1dSequenceHeader,
    ptr: *const u8,
    sz: usize,
) -> Dav1dResult {
    (|| {
        validate_input!((!out.is_null(), EINVAL))?;
        let seq_hdr = rav1d_parse_sequence_header(ptr, sz)?;
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
    unsafe fn has_grain(&self) -> bool {
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
        rav1d_picture_move_ref(out, &mut (*r#in).p);
    } else {
        res = rav1d_apply_grain(c, out, &(*r#in).p);
    }
    rav1d_thread_picture_unref(&mut *r#in);

    if !c.all_layers && c.max_spatial_id && !(c.out.p.data.data[0]).is_null() {
        rav1d_thread_picture_move_ref(r#in, &mut c.out);
    }
    res
}

unsafe fn output_picture_ready(c: &mut Rav1dContext, drain: bool) -> bool {
    if c.cached_error.is_err() {
        return true;
    }
    if !c.all_layers && c.max_spatial_id {
        if !c.out.p.data.data[0].is_null() && !c.cache.p.data.data[0].is_null() {
            if c.max_spatial_id == (c.cache.p.frame_hdr.as_ref().unwrap().spatial_id != 0)
                || c.out.flags.contains(PictureFlags::NEW_TEMPORAL_UNIT)
            {
                return true;
            }
            rav1d_thread_picture_unref(&mut c.cache);
            rav1d_thread_picture_move_ref(&mut c.cache, &mut c.out);
            return false;
        } else {
            if !c.cache.p.data.data[0].is_null() && drain {
                return true;
            } else {
                if !c.out.p.data.data[0].is_null() {
                    rav1d_thread_picture_move_ref(&mut c.cache, &mut c.out);
                    return false;
                }
            }
        }
    }
    !c.out.p.data.data[0].is_null()
}

unsafe fn drain_picture(c: &mut Rav1dContext, out: &mut Rav1dPicture) -> Rav1dResult {
    let mut drain_count: c_uint = 0 as c_int as c_uint;
    let mut drained = 0;
    loop {
        let next: c_uint = c.frame_thread.next;
        let f: *mut Rav1dFrameContext =
            &mut *(c.fc).offset(next as isize) as *mut Rav1dFrameContext;
        let mut task_thread_lock = c.task_thread.delayed_fg.lock().unwrap();
        while !(*f).tiles.is_empty() {
            task_thread_lock = (*f).task_thread.cond.wait(task_thread_lock).unwrap();
        }
        let out_delayed = &mut c.frame_thread.out_delayed[next as usize];
        if !out_delayed.p.data.data[0].is_null()
            || (*f).task_thread.error.load(Ordering::SeqCst) != 0
        {
            let first: c_uint = c.task_thread.first.load(Ordering::SeqCst);
            if first.wrapping_add(1 as c_uint) < c.n_fc {
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
            if c.task_thread.cur.load(Ordering::Relaxed) != 0
                && c.task_thread.cur.load(Ordering::Relaxed) < c.n_fc
            {
                c.task_thread.cur.fetch_sub(1, Ordering::Relaxed);
            }
            drained = 1 as c_int;
        } else if drained != 0 {
            break;
        }
        c.frame_thread.next = (c.frame_thread.next).wrapping_add(1);
        if c.frame_thread.next == c.n_fc {
            c.frame_thread.next = 0 as c_int as c_uint;
        }
        drop(task_thread_lock);
        let error = (*f).task_thread.retval;
        if error.is_err() {
            (*f).task_thread.retval = Ok(());
            *c.cached_error_props.get_mut().unwrap() = out_delayed.p.m.clone();
            rav1d_thread_picture_unref(out_delayed);
            return error;
        }
        if !(out_delayed.p.data.data[0]).is_null() {
            let progress = out_delayed.progress.as_ref().unwrap()[1].load(Ordering::Relaxed);
            if (out_delayed.visible || c.output_invisible_frames) && progress != FRAME_ERROR {
                rav1d_thread_picture_ref(&mut c.out, out_delayed);
                c.event_flags |= out_delayed.flags.into();
            }
            rav1d_thread_picture_unref(out_delayed);
            if output_picture_ready(c, false) {
                return output_image(c, out);
            }
        }
        drain_count = drain_count.wrapping_add(1);
        if !(drain_count < c.n_fc) {
            break;
        }
    }
    if output_picture_ready(c, true) {
        return output_image(c, out);
    }
    return Err(EAGAIN);
}

unsafe fn gen_picture(c: &mut Rav1dContext) -> Rav1dResult {
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
        let len = rav1d_parse_obus(c, &r#in, &props, false);
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

pub(crate) unsafe fn rav1d_send_data(c: &mut Rav1dContext, in_0: &mut Rav1dData) -> Rav1dResult {
    validate_input!((
        in_0.data.as_ref().map_or(true, |data| !data.is_empty()),
        EINVAL
    ))?;
    if in_0.data.is_some() {
        c.drain = 0 as c_int;
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
    let drain = mem::replace(&mut c.drain, 1);
    gen_picture(c)?;
    mem::replace(&mut c.cached_error, Ok(()))?;
    if output_picture_ready(c, c.n_fc == 1) {
        return output_image(c, out);
    }
    if c.n_fc > 1 && drain != 0 {
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
        let mut out_rust = MaybeUninit::zeroed().assume_init(); // TODO(kkysen) Temporary until we return it directly.
        let result = rav1d_get_picture(c, &mut out_rust);
        out.write(out_rust.into());
        result
    })()
    .into()
}

pub(crate) unsafe fn rav1d_apply_grain(
    c: &mut Rav1dContext,
    out: &mut Rav1dPicture,
    in_0: &Rav1dPicture,
) -> Rav1dResult {
    if !in_0.has_grain() {
        rav1d_picture_ref(out, in_0);
        return Ok(());
    }
    let res = rav1d_picture_alloc_copy(c, out, in_0.p.w, in_0);
    if res.is_err() {
        rav1d_picture_unref_internal(out);
        return res;
    } else {
        if c.n_tc > 1 as c_uint {
            rav1d_task_delayed_fg(c, out, in_0);
        } else {
            match out.p.bpc {
                #[cfg(feature = "bitdepth_8")]
                8 => {
                    fg_apply::rav1d_apply_grain::<BitDepth8>(
                        &mut (*(c.dsp).as_mut_ptr().offset(0)).fg,
                        out,
                        in_0,
                    );
                }
                #[cfg(feature = "bitdepth_16")]
                10 | 12 => {
                    fg_apply::rav1d_apply_grain::<BitDepth16>(
                        &mut (*(c.dsp).as_mut_ptr().offset(((out.p.bpc >> 1) - 4) as isize)).fg,
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
        let mut out_rust = MaybeUninit::zeroed().assume_init(); // TODO(kkysen) Temporary until we return it directly.
        let in_rust = in_0.into();
        let result = rav1d_apply_grain(c, &mut out_rust, &in_rust);
        out.write(out_rust.into());
        result
    })()
    .into()
}

pub(crate) unsafe fn rav1d_flush(c: *mut Rav1dContext) {
    let _ = mem::take(&mut (*c).in_0);
    if (*c).out.p.frame_hdr.is_some() {
        rav1d_thread_picture_unref(&mut (*c).out);
    }
    if (*c).cache.p.frame_hdr.is_some() {
        rav1d_thread_picture_unref(&mut (*c).cache);
    }
    (*c).drain = 0 as c_int;
    (*c).cached_error = Ok(());
    let mut i = 0;
    while i < 8 {
        if (*c).refs[i as usize].p.p.frame_hdr.is_some() {
            rav1d_thread_picture_unref(&mut (*((*c).refs).as_mut_ptr().offset(i as isize)).p);
        }
        rav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(i as isize)).segmap);
        rav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(i as isize)).refmvs);
        rav1d_cdf_thread_unref(&mut *((*c).cdf).as_mut_ptr().offset(i as isize));
        i += 1;
    }
    let _ = mem::take(&mut (*c).frame_hdr); // TODO(kkysen) Why wasn't [`rav1d_ref_dec`] called on it?
    let _ = mem::take(&mut (*c).seq_hdr);
    let _ = mem::take(&mut (*c).content_light);
    let _ = mem::take(&mut (*c).mastering_display);
    let _ = mem::take(&mut (*c).itut_t35);
    let _ = mem::take(&mut (*c).cached_error_props);
    if (*c).n_fc == 1 as c_uint && (*c).n_tc == 1 as c_uint {
        return;
    }
    (*c).flush.store(1, Ordering::SeqCst);
    if (*c).n_tc > 1 as c_uint {
        let mut task_thread_lock = (*c).task_thread.delayed_fg.lock().unwrap();
        for tc in (*c).tc.iter() {
            while !tc.flushed() {
                task_thread_lock = tc.thread_data.cond.wait(task_thread_lock).unwrap();
            }
        }
        let mut i_1: c_uint = 0 as c_int as c_uint;
        while i_1 < (*c).n_fc {
            let ref mut fresh1 = (*((*c).fc).offset(i_1 as isize)).task_thread.task_head;
            *fresh1 = 0 as *mut Rav1dTask;
            let ref mut fresh2 = (*((*c).fc).offset(i_1 as isize)).task_thread.task_tail;
            *fresh2 = 0 as *mut Rav1dTask;
            let ref mut fresh3 = (*((*c).fc).offset(i_1 as isize)).task_thread.task_cur_prev;
            *fresh3 = 0 as *mut Rav1dTask;
            *(*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks
                .get_mut()
                .unwrap() = Default::default();
            *&mut (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks_merge = AtomicI32::new(0);
            i_1 = i_1.wrapping_add(1);
        }
        (*c).task_thread.first.store(0, Ordering::SeqCst);
        (*c).task_thread.cur.store((*c).n_fc, Ordering::SeqCst);
        (*c).task_thread
            .reset_task_cur
            .store(u32::MAX, Ordering::SeqCst);
        (*c).task_thread.cond_signaled.store(0, Ordering::SeqCst);
    }
    if (*c).n_fc > 1 as c_uint {
        let mut n: c_uint = 0 as c_int as c_uint;
        let mut next: c_uint = (*c).frame_thread.next;
        while n < (*c).n_fc {
            if next == (*c).n_fc {
                next = 0 as c_int as c_uint;
            }
            let f: *mut Rav1dFrameContext =
                &mut *((*c).fc).offset(next as isize) as *mut Rav1dFrameContext;
            rav1d_decode_frame_exit(&*c, &mut *f, Err(EGeneric));
            (*f).task_thread.retval = Ok(());
            let out_delayed = &mut (*c).frame_thread.out_delayed[next as usize];
            if out_delayed.p.frame_hdr.is_some() {
                rav1d_thread_picture_unref(out_delayed);
            }
            n = n.wrapping_add(1);
            next = next.wrapping_add(1);
        }
        (*c).frame_thread.next = 0 as c_int as c_uint;
    }
    (*c).flush.store(0, Ordering::SeqCst);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_flush(c: *mut Dav1dContext) {
    rav1d_flush(c)
}

#[cold]
pub(crate) unsafe fn rav1d_close(c_out: &mut *mut Rav1dContext) {
    close_internal(c_out, 1 as c_int);
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
unsafe fn close_internal(c_out: &mut *mut Rav1dContext, flush: c_int) {
    let c: *mut Rav1dContext = *c_out;
    if c.is_null() {
        return;
    }
    if flush != 0 {
        rav1d_flush(c);
    }
    c.drop_in_place();
    rav1d_freep_aligned(c_out as *mut _ as *mut c_void);
}

impl Drop for Rav1dContext {
    fn drop(&mut self) {
        // SAFETY: TODO(SJC): This is not safe and will be made safe once we
        // remove all pointers from the structure. We can't make the drop
        // function unsafe because the Drop trait requires a safe function.
        unsafe {
            if self.n_tc > 1 {
                let ttd: &TaskThreadData = &*self.task_thread;
                let task_thread_lock = ttd.delayed_fg.lock().unwrap();
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
            let mut n_1: c_uint = 0 as c_int as c_uint;
            while !(self.fc).is_null() && n_1 < self.n_fc {
                let f: *mut Rav1dFrameContext =
                    &mut *(self.fc).offset(n_1 as isize) as *mut Rav1dFrameContext;
                if self.n_fc > 1 as c_uint {
                    freep(
                        &mut (*f).tile_thread.lowest_pixel_mem as *mut *mut [[c_int; 2]; 7]
                            as *mut c_void,
                    );
                    let _ = mem::take(&mut (*f).frame_thread.b); // TODO: remove when context is owned
                    rav1d_freep_aligned(
                        &mut (*f).frame_thread.pal_idx as *mut *mut u8 as *mut c_void,
                    );
                    rav1d_freep_aligned(
                        &mut (*f).frame_thread.cf as *mut *mut DynCoef as *mut c_void,
                    );
                    freep(&mut (*f).frame_thread.tile_start_off as *mut *mut c_int as *mut c_void);
                    rav1d_freep_aligned(
                        &mut (*f).frame_thread.pal as *mut *mut [[u16; 8]; 3] as *mut c_void,
                    );
                    let _ = mem::take(&mut (*f).frame_thread.cbi); // TODO: remove when context is owned
                }
                if self.n_tc > 1 as c_uint {
                    let _ = mem::take(&mut (*f).task_thread.pending_tasks); // TODO: remove when context is owned
                }
                mem::take(&mut (*f).frame_thread.frame_progress); // TODO: remove when context is owned
                mem::take(&mut (*f).frame_thread.copy_lpf_progress); // TODO: remove when context is owned
                freep(&mut (*f).task_thread.tasks as *mut *mut Rav1dTask as *mut c_void);
                freep(&mut *((*f).task_thread.tile_tasks).as_mut_ptr().offset(0)
                    as *mut *mut Rav1dTask as *mut c_void);
                rav1d_free_aligned((*f).ts as *mut c_void);
                rav1d_free_aligned((*f).ipred_edge[0] as *mut c_void);
                free((*f).a as *mut c_void);
                let _ = mem::take(&mut (*f).tiles);
                free((*f).lf.mask as *mut c_void);
                free((*f).lf.lr_mask as *mut c_void);
                let _ = mem::take(&mut (*f).lf.level);
                free((*f).lf.tx_lpf_right_edge[0] as *mut c_void);
                free((*f).lf.start_of_tile_row as *mut c_void);
                rav1d_refmvs_clear(&mut (*f).rf);
                rav1d_free_aligned((*f).lf.cdef_line_buf as *mut c_void);
                rav1d_free_aligned((*f).lf.lr_line_buf as *mut c_void);
                n_1 = n_1.wrapping_add(1);
            }
            rav1d_free_aligned(self.fc as *mut c_void);
            if self.n_fc > 1 as c_uint && !self.frame_thread.out_delayed.is_empty() {
                let mut n_2: c_uint = 0 as c_int as c_uint;
                while n_2 < self.n_fc {
                    if self.frame_thread.out_delayed[n_2 as usize]
                        .p
                        .frame_hdr
                        .is_some()
                    {
                        rav1d_thread_picture_unref(
                            &mut self.frame_thread.out_delayed[n_2 as usize],
                        );
                    }
                    n_2 = n_2.wrapping_add(1);
                }
                let _ = mem::take(&mut self.frame_thread.out_delayed);
            }
            let _ = mem::take(&mut self.tiles);
            let mut n_4 = 0;
            while n_4 < 8 {
                rav1d_cdf_thread_unref(&mut *(self.cdf).as_mut_ptr().offset(n_4 as isize));
                if self.refs[n_4 as usize].p.p.frame_hdr.is_some() {
                    rav1d_thread_picture_unref(
                        &mut (*(self.refs).as_mut_ptr().offset(n_4 as isize)).p,
                    );
                }
                rav1d_ref_dec(&mut (*(self.refs).as_mut_ptr().offset(n_4 as isize)).refmvs);
                rav1d_ref_dec(&mut (*(self.refs).as_mut_ptr().offset(n_4 as isize)).segmap);
                n_4 += 1;
            }
            let _ = mem::take(&mut self.seq_hdr);
            let _ = mem::take(&mut self.frame_hdr);
            let _ = mem::take(&mut self.mastering_display);
            let _ = mem::take(&mut self.content_light);
            let _ = mem::take(&mut self.itut_t35);
            rav1d_mem_pool_end(self.segmap_pool);
            rav1d_mem_pool_end(self.refmvs_pool);
            rav1d_mem_pool_end(self.cdf_pool);
            rav1d_mem_pool_end(self.picture_pool);
        }
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
        out.write(mem::take(&mut *((*c).cached_error_props).get_mut().unwrap()).into());
        Ok(())
    })()
    .into()
}

pub(crate) unsafe fn rav1d_picture_unref(p: &mut Rav1dPicture) {
    rav1d_picture_unref_internal(p);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_unref(p: *mut Dav1dPicture) {
    if validate_input!(!p.is_null()).is_err() {
        return;
    }
    let mut p_rust = p.read().into();
    rav1d_picture_unref(&mut p_rust);
    p.write(p_rust.into());
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_create(buf: *mut Dav1dData, sz: usize) -> *mut u8 {
    || -> Rav1dResult<*mut u8> {
        let buf = validate_input!(NonNull::new(buf).ok_or(EINVAL))?;
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
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    user_data: *mut c_void,
) -> Dav1dResult {
    || -> Rav1dResult {
        let buf = validate_input!(NonNull::new(buf).ok_or(EINVAL))?;
        let ptr = validate_input!(NonNull::new(ptr.cast_mut()).ok_or(EINVAL))?;
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
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
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
