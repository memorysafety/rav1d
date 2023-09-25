use crate::include::common::bitdepth::DynCoef;
use crate::include::common::intops::iclip;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::common::Rav1dUserData;
use crate::include::dav1d::data::Dav1dData;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::Dav1dContext;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dSettings;
use crate::include::dav1d::dav1d::Rav1dEventFlags;
use crate::include::dav1d::dav1d::Rav1dLogger;
use crate::include::dav1d::dav1d::Rav1dSettings;
use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_ALL;
use crate::include::dav1d::dav1d::DAV1D_DECODEFRAMETYPE_KEY;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_ALL;
use crate::include::dav1d::dav1d::DAV1D_INLOOPFILTER_NONE;
use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::picture::Rav1dPicAllocator;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::include::stdatomic::atomic_int;
use crate::include::stdatomic::atomic_uint;
use crate::src::cdf::rav1d_cdf_thread_unref;
use crate::src::cpu::rav1d_init_cpu;
use crate::src::cpu::rav1d_num_logical_processors;
use crate::src::data::rav1d_data_create_internal;
use crate::src::data::rav1d_data_props_copy;
use crate::src::data::rav1d_data_props_set_defaults;
use crate::src::data::rav1d_data_props_unref_internal;
use crate::src::data::rav1d_data_ref;
use crate::src::data::rav1d_data_unref_internal;
use crate::src::data::rav1d_data_wrap_internal;
use crate::src::data::rav1d_data_wrap_user_data_internal;
use crate::src::decode::rav1d_decode_frame_exit;
use crate::src::internal::CodedBlockInfo;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dTask;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::TaskThreadData;
use crate::src::intra_edge::rav1d_init_mode_tree;
use crate::src::levels::Av1Block;
use crate::src::levels::BL_128X128;
use crate::src::levels::BL_64X64;
use crate::src::log::rav1d_log;
use crate::src::log::rav1d_log_default_callback;
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
use crate::src::picture::rav1d_picture_get_event_flags;
use crate::src::picture::rav1d_picture_move_ref;
use crate::src::picture::rav1d_picture_ref;
use crate::src::picture::rav1d_picture_unref_internal;
use crate::src::picture::rav1d_thread_picture_move_ref;
use crate::src::picture::rav1d_thread_picture_ref;
use crate::src::picture::rav1d_thread_picture_unref;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::picture::PICTURE_FLAG_NEW_TEMPORAL_UNIT;
use crate::src::r#ref::rav1d_ref_dec;
use crate::src::r#ref::Rav1dRef;
use crate::src::refmvs::rav1d_refmvs_clear;
use crate::src::refmvs::rav1d_refmvs_dsp_init;
use crate::src::refmvs::rav1d_refmvs_init;
use crate::src::thread_task::rav1d_task_delayed_fg;
use crate::src::thread_task::rav1d_worker_task;
use crate::src::thread_task::FRAME_ERROR;
use crate::stderr;
use cfg_if::cfg_if;
use libc::calloc;
use libc::fprintf;
use libc::free;
use libc::memcpy;
use libc::memset;
use libc::pthread_attr_destroy;
use libc::pthread_attr_init;
use libc::pthread_attr_setstacksize;
use libc::pthread_attr_t;
use libc::pthread_cond_broadcast;
use libc::pthread_cond_destroy;
use libc::pthread_cond_init;
use libc::pthread_cond_wait;
use libc::pthread_condattr_t;
use libc::pthread_join;
use libc::pthread_mutex_destroy;
use libc::pthread_mutex_init;
use libc::pthread_mutex_lock;
use libc::pthread_mutex_unlock;
use libc::pthread_mutexattr_t;
use libc::pthread_t;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::process::abort;
use std::sync::Once;

#[cfg(feature = "bitdepth_8")]
use crate::src::fg_apply_tmpl_8::rav1d_apply_grain_8bpc;

#[cfg(feature = "bitdepth_16")]
use crate::src::fg_apply_tmpl_16::rav1d_apply_grain_16bpc;

#[cfg(target_os = "linux")]
use libc::dlsym;

#[cfg(target_os = "linux")]
use libc::sysconf;

extern "C" {
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        __arg: *mut c_void,
    ) -> c_int;
}

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

#[cold]
pub(crate) unsafe fn rav1d_default_settings(s: *mut Rav1dSettings) {
    (*s).n_threads = 0 as c_int;
    (*s).max_frame_delay = 0 as c_int;
    (*s).apply_grain = 1 as c_int;
    (*s).allocator.cookie = 0 as *mut c_void;
    (*s).allocator.alloc_picture_callback = Some(
        dav1d_default_picture_alloc
            as unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> c_int,
    );
    (*s).allocator.release_picture_callback = Some(
        dav1d_default_picture_release as unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> (),
    );
    (*s).logger.cookie = 0 as *mut c_void;
    (*s).logger.callback = Some(
        rav1d_log_default_callback
            as unsafe extern "C" fn(*mut c_void, *const c_char, ::core::ffi::VaList) -> (),
    );
    (*s).operating_point = 0 as c_int;
    (*s).all_layers = 1 as c_int;
    (*s).frame_size_limit = 0 as c_int as c_uint;
    (*s).strict_std_compliance = 0 as c_int;
    (*s).output_invisible_frames = 0 as c_int;
    (*s).inloop_filters = DAV1D_INLOOPFILTER_ALL;
    (*s).decode_frame_type = DAV1D_DECODEFRAMETYPE_ALL;
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_default_settings(s: *mut Dav1dSettings) {
    s.write(
        {
            let mut s = s.read().into_rust();
            rav1d_default_settings(&mut s);
            s
        }
        .into_c(),
    );
}

#[cold]
unsafe extern "C" fn get_stack_size_internal(_thread_attr: *const pthread_attr_t) -> usize {
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

#[cold]
unsafe extern "C" fn get_num_threads(
    c: *mut Rav1dContext,
    s: *const Rav1dSettings,
    n_tc: *mut c_uint,
    n_fc: *mut c_uint,
) {
    static fc_lut: [u8; 49] = [
        1, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6,
        6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    ];
    *n_tc = (if (*s).n_threads != 0 {
        (*s).n_threads
    } else {
        iclip(rav1d_num_logical_processors(c), 1 as c_int, 256 as c_int)
    }) as c_uint;
    *n_fc = if (*s).max_frame_delay != 0 {
        cmp::min((*s).max_frame_delay as c_uint, *n_tc)
    } else {
        (if *n_tc < 50 as c_uint {
            fc_lut[(*n_tc).wrapping_sub(1 as c_int as c_uint) as usize] as c_int
        } else {
            8 as c_int
        }) as c_uint
    };
}

#[cold]
pub(crate) unsafe fn rav1d_get_frame_delay(s: *const Rav1dSettings) -> c_int {
    let mut n_tc: c_uint = 0;
    let mut n_fc: c_uint = 0;
    if s.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"s != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"dav1d_get_frame_delay\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    if !((*s).n_threads >= 0 && (*s).n_threads <= 256) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"s->n_threads >= 0 && s->n_threads <= DAV1D_MAX_THREADS\0" as *const u8
                as *const c_char,
            (*::core::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"dav1d_get_frame_delay\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    if !((*s).max_frame_delay >= 0 && (*s).max_frame_delay <= 256) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"s->max_frame_delay >= 0 && s->max_frame_delay <= DAV1D_MAX_FRAME_DELAY\0" as *const u8
                as *const c_char,
            (*::core::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"dav1d_get_frame_delay\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    get_num_threads(0 as *mut Rav1dContext, s, &mut n_tc, &mut n_fc);
    return n_fc as c_int;
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_get_frame_delay(s: *const Dav1dSettings) -> c_int {
    rav1d_get_frame_delay(&s.read().into_rust())
}

#[cold]
pub(crate) unsafe fn rav1d_open(c_out: *mut *mut Rav1dContext, s: *const Rav1dSettings) -> c_int {
    unsafe extern "C" fn error(
        c: *mut Rav1dContext,
        c_out: *mut *mut Rav1dContext,
        thread_attr: *mut pthread_attr_t,
    ) -> c_int {
        if !c.is_null() {
            close_internal(c_out, 0 as c_int);
        }
        pthread_attr_destroy(thread_attr);
        return -(12 as c_int);
    }

    static initted: Once = Once::new();
    initted.call_once(|| init_internal());
    if c_out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"c_out != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if s.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"s != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if !((*s).n_threads >= 0 && (*s).n_threads <= 256) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"s->n_threads >= 0 && s->n_threads <= DAV1D_MAX_THREADS\0" as *const u8
                as *const c_char,
            (*::core::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if !((*s).max_frame_delay >= 0 && (*s).max_frame_delay <= 256) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"s->max_frame_delay >= 0 && s->max_frame_delay <= DAV1D_MAX_FRAME_DELAY\0" as *const u8
                as *const c_char,
            (*::core::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if ((*s).allocator.alloc_picture_callback).is_none() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"s->allocator.alloc_picture_callback != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if ((*s).allocator.release_picture_callback).is_none() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"s->allocator.release_picture_callback != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if !((*s).operating_point >= 0 && (*s).operating_point <= 31) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"s->operating_point >= 0 && s->operating_point <= 31\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"dav1d_open\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if !((*s).decode_frame_type as c_uint >= DAV1D_DECODEFRAMETYPE_ALL as c_int as c_uint
        && (*s).decode_frame_type as c_uint <= DAV1D_DECODEFRAMETYPE_KEY as c_int as c_uint)
    {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8
                as *const c_char,
            b"s->decode_frame_type >= DAV1D_DECODEFRAMETYPE_ALL && s->decode_frame_type <= DAV1D_DECODEFRAMETYPE_KEY\0"
                as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 11], &[c_char; 11]>(b"dav1d_open\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    let mut thread_attr: pthread_attr_t = std::mem::zeroed();
    if pthread_attr_init(&mut thread_attr) != 0 {
        return -(12 as c_int);
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
    (*c).allocator = (*s).allocator.clone();
    (*c).logger = (*s).logger.clone();
    (*c).apply_grain = (*s).apply_grain;
    (*c).operating_point = (*s).operating_point;
    (*c).all_layers = (*s).all_layers;
    (*c).frame_size_limit = (*s).frame_size_limit;
    (*c).strict_std_compliance = (*s).strict_std_compliance;
    (*c).output_invisible_frames = (*s).output_invisible_frames;
    (*c).inloop_filters = (*s).inloop_filters;
    (*c).decode_frame_type = (*s).decode_frame_type;
    rav1d_data_props_set_defaults(&mut (*c).cached_error_props);
    if rav1d_mem_pool_init(&mut (*c).seq_hdr_pool) != 0
        || rav1d_mem_pool_init(&mut (*c).frame_hdr_pool) != 0
        || rav1d_mem_pool_init(&mut (*c).segmap_pool) != 0
        || rav1d_mem_pool_init(&mut (*c).refmvs_pool) != 0
        || rav1d_mem_pool_init(&mut (*c).cdf_pool) != 0
    {
        return error(c, c_out, &mut thread_attr);
    }
    if (*c).allocator.alloc_picture_callback
        == Some(
            dav1d_default_picture_alloc
                as unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> c_int,
        )
        && (*c).allocator.release_picture_callback
            == Some(
                dav1d_default_picture_release
                    as unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> (),
            )
    {
        if !((*c).allocator.cookie).is_null() {
            return error(c, c_out, &mut thread_attr);
        }
        if rav1d_mem_pool_init(&mut (*c).picture_pool) != 0 {
            return error(c, c_out, &mut thread_attr);
        }
        (*c).allocator.cookie = (*c).picture_pool as *mut c_void;
    } else if (*c).allocator.alloc_picture_callback
        == Some(
            dav1d_default_picture_alloc
                as unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> c_int,
        )
        || (*c).allocator.release_picture_callback
            == Some(
                dav1d_default_picture_release
                    as unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> (),
            )
    {
        return error(c, c_out, &mut thread_attr);
    }
    if (::core::mem::size_of::<usize>() as c_ulong) < 8 as c_ulong
        && ((*s).frame_size_limit).wrapping_sub(1 as c_int as c_uint) >= (8192 * 8192) as c_uint
    {
        (*c).frame_size_limit = (8192 * 8192) as c_uint;
        if (*s).frame_size_limit != 0 {
            rav1d_log(
                c,
                b"Frame size limit reduced from %u to %u.\n\0" as *const u8 as *const c_char,
                (*s).frame_size_limit,
                (*c).frame_size_limit,
            );
        }
    }
    (*c).flush = &mut (*c).flush_mem;
    *(*c).flush = 0 as c_int;
    get_num_threads(c, s, &mut (*c).n_tc, &mut (*c).n_fc);
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
    (*c).tc = rav1d_alloc_aligned(
        ::core::mem::size_of::<Rav1dTaskContext>().wrapping_mul((*c).n_tc as usize),
        64 as c_int as usize,
    ) as *mut Rav1dTaskContext;
    if ((*c).tc).is_null() {
        return error(c, c_out, &mut thread_attr);
    }
    memset(
        (*c).tc as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<Rav1dTaskContext>().wrapping_mul((*c).n_tc as usize),
    );
    if (*c).n_tc > 1 as c_uint {
        if pthread_mutex_init(&mut (*c).task_thread.lock, 0 as *const pthread_mutexattr_t) != 0 {
            return error(c, c_out, &mut thread_attr);
        }
        if pthread_cond_init(&mut (*c).task_thread.cond, 0 as *const pthread_condattr_t) != 0 {
            pthread_mutex_destroy(&mut (*c).task_thread.lock);
            return error(c, c_out, &mut thread_attr);
        }
        if pthread_cond_init(
            &mut (*c).task_thread.delayed_fg.cond,
            0 as *const pthread_condattr_t,
        ) != 0
        {
            pthread_cond_destroy(&mut (*c).task_thread.cond);
            pthread_mutex_destroy(&mut (*c).task_thread.lock);
            return error(c, c_out, &mut thread_attr);
        }
        (*c).task_thread.cur = (*c).n_fc;
        *&mut (*c).task_thread.reset_task_cur = u32::MAX;
        *&mut (*c).task_thread.cond_signaled = 0 as c_int;
        (*c).task_thread.inited = 1 as c_int;
    }
    if (*c).n_fc > 1 as c_uint {
        (*c).frame_thread.out_delayed = calloc(
            (*c).n_fc as usize,
            ::core::mem::size_of::<Rav1dThreadPicture>(),
        ) as *mut Rav1dThreadPicture;
        if ((*c).frame_thread.out_delayed).is_null() {
            return error(c, c_out, &mut thread_attr);
        }
    }
    let mut n: c_uint = 0 as c_int as c_uint;
    while n < (*c).n_fc {
        let f: *mut Rav1dFrameContext =
            &mut *((*c).fc).offset(n as isize) as *mut Rav1dFrameContext;
        if (*c).n_tc > 1 as c_uint {
            if pthread_mutex_init(&mut (*f).task_thread.lock, 0 as *const pthread_mutexattr_t) != 0
            {
                return error(c, c_out, &mut thread_attr);
            }
            if pthread_cond_init(&mut (*f).task_thread.cond, 0 as *const pthread_condattr_t) != 0 {
                pthread_mutex_destroy(&mut (*f).task_thread.lock);
                return error(c, c_out, &mut thread_attr);
            }
            if pthread_mutex_init(
                &mut (*f).task_thread.pending_tasks.lock,
                0 as *const pthread_mutexattr_t,
            ) != 0
            {
                pthread_cond_destroy(&mut (*f).task_thread.cond);
                pthread_mutex_destroy(&mut (*f).task_thread.lock);
                return error(c, c_out, &mut thread_attr);
            }
        }
        (*f).c = c;
        (*f).task_thread.ttd = &mut (*c).task_thread;
        (*f).lf.last_sharpness = -(1 as c_int);
        rav1d_refmvs_init(&mut (*f).rf);
        n = n.wrapping_add(1);
    }
    let mut m: c_uint = 0 as c_int as c_uint;
    while m < (*c).n_tc {
        let t: *mut Rav1dTaskContext = &mut *((*c).tc).offset(m as isize) as *mut Rav1dTaskContext;
        (*t).f = &mut *((*c).fc).offset(0) as *mut Rav1dFrameContext;
        (*t).task_thread.ttd = &mut (*c).task_thread;
        (*t).c = c;
        memset(
            ((*t).c2rust_unnamed.cf_16bpc).as_mut_ptr() as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<[i32; 1024]>(),
        );
        if (*c).n_tc > 1 as c_uint {
            if pthread_mutex_init(
                &mut (*t).task_thread.td.lock,
                0 as *const pthread_mutexattr_t,
            ) != 0
            {
                return error(c, c_out, &mut thread_attr);
            }
            if pthread_cond_init(
                &mut (*t).task_thread.td.cond,
                0 as *const pthread_condattr_t,
            ) != 0
            {
                pthread_mutex_destroy(&mut (*t).task_thread.td.lock);
                return error(c, c_out, &mut thread_attr);
            }
            if pthread_create(
                &mut (*t).task_thread.td.thread,
                &mut thread_attr,
                Some(rav1d_worker_task as unsafe extern "C" fn(*mut c_void) -> *mut c_void),
                t as *mut c_void,
            ) != 0
            {
                pthread_cond_destroy(&mut (*t).task_thread.td.cond);
                pthread_mutex_destroy(&mut (*t).task_thread.td.lock);
                return error(c, c_out, &mut thread_attr);
            }
            (*t).task_thread.td.inited = 1 as c_int;
        }
        m = m.wrapping_add(1);
    }
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
    return 0 as c_int;
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_open(
    c_out: *mut *mut Dav1dContext,
    s: *const Dav1dSettings,
) -> c_int {
    rav1d_open(c_out, &s.read().into_rust())
}

unsafe extern "C" fn dummy_free(data: *const u8, user_data: *mut c_void) {
    if !(!data.is_null() && user_data.is_null()) {
        unreachable!();
    }
}

pub(crate) unsafe fn rav1d_parse_sequence_header(
    out: *mut Rav1dSequenceHeader,
    ptr: *const u8,
    sz: usize,
) -> c_int {
    let mut current_block: u64;
    let mut buf: Rav1dData = {
        let init = Rav1dData {
            data: 0 as *const u8,
            sz: 0,
            r#ref: 0 as *mut Rav1dRef,
            m: Rav1dDataProps {
                timestamp: 0,
                duration: 0,
                offset: 0,
                size: 0,
                user_data: Rav1dUserData {
                    data: 0 as *const u8,
                    r#ref: 0 as *mut Rav1dRef,
                },
            },
        };
        init
    };
    let mut res;
    if out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"out != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 28], &[c_char; 28]>(b"dav1d_parse_sequence_header\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    let mut s: Rav1dSettings = Rav1dSettings {
        n_threads: 0,
        max_frame_delay: 0,
        apply_grain: 0,
        operating_point: 0,
        all_layers: 0,
        frame_size_limit: 0,
        allocator: Rav1dPicAllocator {
            cookie: 0 as *mut c_void,
            alloc_picture_callback: None,
            release_picture_callback: None,
        },
        logger: Rav1dLogger {
            cookie: 0 as *mut c_void,
            callback: None,
        },
        strict_std_compliance: 0,
        output_invisible_frames: 0,
        inloop_filters: DAV1D_INLOOPFILTER_NONE,
        decode_frame_type: DAV1D_DECODEFRAMETYPE_ALL,
        reserved: [0; 16],
    };
    rav1d_default_settings(&mut s);
    s.n_threads = 1 as c_int;
    s.logger.callback = None;
    let mut c: *mut Rav1dContext = 0 as *mut Rav1dContext;
    res = rav1d_open(&mut c, &mut s);
    if res < 0 {
        return res;
    }
    if !ptr.is_null() {
        res = rav1d_data_wrap_internal(
            &mut buf,
            ptr,
            sz,
            Some(dummy_free as unsafe extern "C" fn(*const u8, *mut c_void) -> ()),
            0 as *mut c_void,
        );
        if res < 0 {
            current_block = 10647346020414903899;
        } else {
            current_block = 5399440093318478209;
        }
    } else {
        current_block = 5399440093318478209;
    }
    loop {
        match current_block {
            10647346020414903899 => {
                rav1d_data_unref_internal(&mut buf);
                break;
            }
            _ => {
                if buf.sz > 0 {
                    res = rav1d_parse_obus(c, &mut buf, 1 as c_int);
                    if res < 0 {
                        current_block = 10647346020414903899;
                        continue;
                    }
                    if !(res as usize <= buf.sz) {
                        unreachable!();
                    }
                    buf.sz = (buf.sz as c_ulong).wrapping_sub(res as c_ulong) as usize as usize;
                    buf.data = (buf.data).offset(res as isize);
                    current_block = 5399440093318478209;
                } else if ((*c).seq_hdr).is_null() {
                    res = -(2 as c_int);
                    current_block = 10647346020414903899;
                } else {
                    memcpy(
                        out as *mut c_void,
                        (*c).seq_hdr as *const c_void,
                        ::core::mem::size_of::<Dav1dSequenceHeader>(),
                    );
                    res = 0 as c_int;
                    current_block = 10647346020414903899;
                }
            }
        }
    }
    rav1d_close(&mut c);
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_parse_sequence_header(
    out: *mut Dav1dSequenceHeader,
    ptr: *const u8,
    sz: usize,
) -> c_int {
    let mut out_rust = out.read().into_rust();
    let result = rav1d_parse_sequence_header(&mut out_rust, ptr, sz);
    out.write(out_rust.into_c());
    result
}

unsafe extern "C" fn has_grain(pic: *const Rav1dPicture) -> c_int {
    let fgdata: *const Dav1dFilmGrainData = &mut (*(*pic).frame_hdr).film_grain.data;
    return ((*fgdata).num_y_points != 0
        || (*fgdata).num_uv_points[0] != 0
        || (*fgdata).num_uv_points[1] != 0
        || (*fgdata).clip_to_restricted_range != 0 && (*fgdata).chroma_scaling_from_luma != 0)
        as c_int;
}

unsafe extern "C" fn output_image(c: *mut Rav1dContext, out: *mut Rav1dPicture) -> c_int {
    let mut res = 0;
    let in_0: *mut Rav1dThreadPicture = if (*c).all_layers != 0 || (*c).max_spatial_id == 0 {
        &mut (*c).out
    } else {
        &mut (*c).cache
    };
    if (*c).apply_grain == 0 || has_grain(&mut (*in_0).p) == 0 {
        rav1d_picture_move_ref(out, &mut (*in_0).p);
        rav1d_thread_picture_unref(in_0);
    } else {
        res = rav1d_apply_grain(c, out, &mut (*in_0).p);
        rav1d_thread_picture_unref(in_0);
    }
    if (*c).all_layers == 0 && (*c).max_spatial_id != 0 && !((*c).out.p.data[0]).is_null() {
        rav1d_thread_picture_move_ref(in_0, &mut (*c).out);
    }
    return res;
}

unsafe extern "C" fn output_picture_ready(c: *mut Rav1dContext, drain: c_int) -> c_int {
    if (*c).cached_error != 0 {
        return 1 as c_int;
    }
    if (*c).all_layers == 0 && (*c).max_spatial_id != 0 {
        if !((*c).out.p.data[0]).is_null() && !((*c).cache.p.data[0]).is_null() {
            if (*c).max_spatial_id == (*(*c).cache.p.frame_hdr).spatial_id
                || (*c).out.flags as c_uint & PICTURE_FLAG_NEW_TEMPORAL_UNIT as c_int as c_uint != 0
            {
                return 1 as c_int;
            }
            rav1d_thread_picture_unref(&mut (*c).cache);
            rav1d_thread_picture_move_ref(&mut (*c).cache, &mut (*c).out);
            return 0 as c_int;
        } else {
            if !((*c).cache.p.data[0]).is_null() && drain != 0 {
                return 1 as c_int;
            } else {
                if !((*c).out.p.data[0]).is_null() {
                    rav1d_thread_picture_move_ref(&mut (*c).cache, &mut (*c).out);
                    return 0 as c_int;
                }
            }
        }
    }
    return !((*c).out.p.data[0]).is_null() as c_int;
}

unsafe extern "C" fn drain_picture(c: *mut Rav1dContext, out: *mut Rav1dPicture) -> c_int {
    let mut drain_count: c_uint = 0 as c_int as c_uint;
    let mut drained = 0;
    loop {
        let next: c_uint = (*c).frame_thread.next;
        let f: *mut Rav1dFrameContext =
            &mut *((*c).fc).offset(next as isize) as *mut Rav1dFrameContext;
        pthread_mutex_lock(&mut (*c).task_thread.lock);
        while (*f).n_tile_data > 0 {
            pthread_cond_wait(
                &mut (*f).task_thread.cond,
                &mut (*(*f).task_thread.ttd).lock,
            );
        }
        let out_delayed: *mut Rav1dThreadPicture =
            &mut *((*c).frame_thread.out_delayed).offset(next as isize) as *mut Rav1dThreadPicture;
        if !((*out_delayed).p.data[0]).is_null()
            || ::core::intrinsics::atomic_load_seqcst(
                &mut (*f).task_thread.error as *mut atomic_int,
            ) != 0
        {
            let mut first: c_uint =
                ::core::intrinsics::atomic_load_seqcst(&mut (*c).task_thread.first);
            if first.wrapping_add(1 as c_uint) < (*c).n_fc {
                ::core::intrinsics::atomic_xadd_seqcst(&mut (*c).task_thread.first, 1 as c_uint);
            } else {
                ::core::intrinsics::atomic_store_seqcst(
                    &mut (*c).task_thread.first,
                    0 as c_int as c_uint,
                );
            }
            let fresh0 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
                &mut (*c).task_thread.reset_task_cur,
                *&mut first,
                u32::MAX,
            );
            *&mut first = fresh0.0;
            fresh0.1;
            if (*c).task_thread.cur != 0 && (*c).task_thread.cur < (*c).n_fc {
                (*c).task_thread.cur = ((*c).task_thread.cur).wrapping_sub(1);
            }
            drained = 1 as c_int;
        } else if drained != 0 {
            pthread_mutex_unlock(&mut (*c).task_thread.lock);
            break;
        }
        (*c).frame_thread.next = ((*c).frame_thread.next).wrapping_add(1);
        if (*c).frame_thread.next == (*c).n_fc {
            (*c).frame_thread.next = 0 as c_int as c_uint;
        }
        pthread_mutex_unlock(&mut (*c).task_thread.lock);
        let error = (*f).task_thread.retval;
        if error != 0 {
            (*f).task_thread.retval = 0 as c_int;
            rav1d_data_props_copy(&mut (*c).cached_error_props, &mut (*out_delayed).p.m);
            rav1d_thread_picture_unref(out_delayed);
            return error;
        }
        if !((*out_delayed).p.data[0]).is_null() {
            let progress: c_uint = ::core::intrinsics::atomic_load_relaxed(
                &mut *((*out_delayed).progress).offset(1) as *mut atomic_uint,
            );
            if ((*out_delayed).visible || (*c).output_invisible_frames != 0)
                && progress != FRAME_ERROR
            {
                rav1d_thread_picture_ref(&mut (*c).out, out_delayed);
                (*c).event_flags = ::core::mem::transmute::<c_uint, Dav1dEventFlags>(
                    (*c).event_flags as c_uint
                        | rav1d_picture_get_event_flags(out_delayed) as c_uint,
                );
            }
            rav1d_thread_picture_unref(out_delayed);
            if output_picture_ready(c, 0 as c_int) != 0 {
                return output_image(c, out);
            }
        }
        drain_count = drain_count.wrapping_add(1);
        if !(drain_count < (*c).n_fc) {
            break;
        }
    }
    if output_picture_ready(c, 1 as c_int) != 0 {
        return output_image(c, out);
    }
    return -(11 as c_int);
}

unsafe extern "C" fn gen_picture(c: *mut Rav1dContext) -> c_int {
    let mut res;
    let in_0: *mut Rav1dData = &mut (*c).in_0;
    if output_picture_ready(c, 0 as c_int) != 0 {
        return 0 as c_int;
    }
    while (*in_0).sz > 0 {
        res = rav1d_parse_obus(c, in_0, 0 as c_int);
        if res < 0 {
            rav1d_data_unref_internal(in_0);
        } else {
            if !(res as usize <= (*in_0).sz) {
                unreachable!();
            }
            (*in_0).sz = ((*in_0).sz as c_ulong).wrapping_sub(res as c_ulong) as usize as usize;
            (*in_0).data = ((*in_0).data).offset(res as isize);
            if (*in_0).sz == 0 {
                rav1d_data_unref_internal(in_0);
            }
        }
        if output_picture_ready(c, 0 as c_int) != 0 {
            break;
        }
        if res < 0 {
            return res;
        }
    }
    return 0 as c_int;
}

pub(crate) unsafe fn rav1d_send_data(c: *mut Rav1dContext, in_0: *mut Rav1dData) -> c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"c != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 16], &[c_char; 16]>(b"dav1d_send_data\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if in_0.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"in != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 16], &[c_char; 16]>(b"dav1d_send_data\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if !(((*in_0).data).is_null() || (*in_0).sz != 0) {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"in->data == NULL || in->sz\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 16], &[c_char; 16]>(b"dav1d_send_data\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if !((*in_0).data).is_null() {
        (*c).drain = 0 as c_int;
    }
    if !((*c).in_0.data).is_null() {
        return -(11 as c_int);
    }
    rav1d_data_ref(&mut (*c).in_0, in_0);
    let res = gen_picture(c);
    if res == 0 {
        rav1d_data_unref_internal(in_0);
    }
    return res;
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_send_data(c: *mut Rav1dContext, in_0: *mut Dav1dData) -> c_int {
    let mut in_rust = in_0.read().into_rust();
    let result = rav1d_send_data(c, &mut in_rust);
    in_0.write(in_rust.into_c());
    result
}

pub(crate) unsafe fn rav1d_get_picture(c: *mut Rav1dContext, out: *mut Rav1dPicture) -> c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"c != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"dav1d_get_picture\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"out != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"dav1d_get_picture\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    let drain = (*c).drain;
    (*c).drain = 1 as c_int;
    let res = gen_picture(c);
    if res < 0 {
        return res;
    }
    if (*c).cached_error != 0 {
        let res_0 = (*c).cached_error;
        (*c).cached_error = 0 as c_int;
        return res_0;
    }
    if output_picture_ready(c, ((*c).n_fc == 1 as c_uint) as c_int) != 0 {
        return output_image(c, out);
    }
    if (*c).n_fc > 1 as c_uint && drain != 0 {
        return drain_picture(c, out);
    }
    return -(11 as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_get_picture(c: *mut Dav1dContext, out: *mut Dav1dPicture) -> c_int {
    let mut out_rust = out.read().into_rust();
    let result = rav1d_get_picture(c, &mut out_rust);
    out.write(out_rust.into_c());
    result
}

pub(crate) unsafe fn rav1d_apply_grain(
    c: *mut Rav1dContext,
    out: *mut Rav1dPicture,
    in_0: *const Rav1dPicture,
) -> c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"c != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"rav1d_apply_grain\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"out != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"rav1d_apply_grain\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if in_0.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"in != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 18], &[c_char; 18]>(b"rav1d_apply_grain\0")).as_ptr(),
        );
        return -(22 as c_int);
    }
    if has_grain(in_0) == 0 {
        rav1d_picture_ref(out, in_0);
        return 0 as c_int;
    }
    let res = rav1d_picture_alloc_copy(c, out, (*in_0).p.w, in_0);
    if res < 0 {
        rav1d_picture_unref_internal(out);
        return res;
    } else {
        if (*c).n_tc > 1 as c_uint {
            rav1d_task_delayed_fg(c, out, in_0);
        } else {
            match (*out).p.bpc {
                #[cfg(feature = "bitdepth_8")]
                8 => {
                    rav1d_apply_grain_8bpc(&mut (*((*c).dsp).as_mut_ptr().offset(0)).fg, out, in_0);
                }
                #[cfg(feature = "bitdepth_16")]
                10 | 12 => {
                    rav1d_apply_grain_16bpc(
                        &mut (*((*c).dsp)
                            .as_mut_ptr()
                            .offset((((*out).p.bpc >> 1) - 4) as isize))
                        .fg,
                        out,
                        in_0,
                    );
                }
                _ => {
                    abort();
                }
            }
        }
        return 0 as c_int;
    };
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain(
    c: *mut Dav1dContext,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) -> c_int {
    let mut out_rust = out.read().into_rust();
    let in_rust = in_0.read().into_rust();
    let result = rav1d_apply_grain(c, &mut out_rust, &in_rust);
    out.write(out_rust.into_c());
    result
}

pub(crate) unsafe fn rav1d_flush(c: *mut Rav1dContext) {
    rav1d_data_unref_internal(&mut (*c).in_0);
    if !((*c).out.p.frame_hdr).is_null() {
        rav1d_thread_picture_unref(&mut (*c).out);
    }
    if !((*c).cache.p.frame_hdr).is_null() {
        rav1d_thread_picture_unref(&mut (*c).cache);
    }
    (*c).drain = 0 as c_int;
    (*c).cached_error = 0 as c_int;
    let mut i = 0;
    while i < 8 {
        if !((*c).refs[i as usize].p.p.frame_hdr).is_null() {
            rav1d_thread_picture_unref(&mut (*((*c).refs).as_mut_ptr().offset(i as isize)).p);
        }
        rav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(i as isize)).segmap);
        rav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(i as isize)).refmvs);
        rav1d_cdf_thread_unref(&mut *((*c).cdf).as_mut_ptr().offset(i as isize));
        i += 1;
    }
    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
    (*c).seq_hdr = 0 as *mut Dav1dSequenceHeader;
    rav1d_ref_dec(&mut (*c).seq_hdr_ref);
    (*c).mastering_display = 0 as *mut Dav1dMasteringDisplay;
    (*c).content_light = 0 as *mut Dav1dContentLightLevel;
    (*c).itut_t35 = 0 as *mut Dav1dITUTT35;
    rav1d_ref_dec(&mut (*c).mastering_display_ref);
    rav1d_ref_dec(&mut (*c).content_light_ref);
    rav1d_ref_dec(&mut (*c).itut_t35_ref);
    rav1d_data_props_unref_internal(&mut (*c).cached_error_props);
    if (*c).n_fc == 1 as c_uint && (*c).n_tc == 1 as c_uint {
        return;
    }
    ::core::intrinsics::atomic_store_seqcst((*c).flush, 1 as c_int);
    if (*c).n_tc > 1 as c_uint {
        pthread_mutex_lock(&mut (*c).task_thread.lock);
        let mut i_0: c_uint = 0 as c_int as c_uint;
        while i_0 < (*c).n_tc {
            let tc: *mut Rav1dTaskContext =
                &mut *((*c).tc).offset(i_0 as isize) as *mut Rav1dTaskContext;
            while !(*tc).task_thread.flushed {
                pthread_cond_wait(&mut (*tc).task_thread.td.cond, &mut (*c).task_thread.lock);
            }
            i_0 = i_0.wrapping_add(1);
        }
        let mut i_1: c_uint = 0 as c_int as c_uint;
        while i_1 < (*c).n_fc {
            let ref mut fresh1 = (*((*c).fc).offset(i_1 as isize)).task_thread.task_head;
            *fresh1 = 0 as *mut Rav1dTask;
            let ref mut fresh2 = (*((*c).fc).offset(i_1 as isize)).task_thread.task_tail;
            *fresh2 = 0 as *mut Rav1dTask;
            let ref mut fresh3 = (*((*c).fc).offset(i_1 as isize)).task_thread.task_cur_prev;
            *fresh3 = 0 as *mut Rav1dTask;
            let ref mut fresh4 = (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks
                .head;
            *fresh4 = 0 as *mut Rav1dTask;
            let ref mut fresh5 = (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks
                .tail;
            *fresh5 = 0 as *mut Rav1dTask;
            *&mut (*((*c).fc).offset(i_1 as isize))
                .task_thread
                .pending_tasks
                .merge = 0 as c_int;
            i_1 = i_1.wrapping_add(1);
        }
        *&mut (*c).task_thread.first = 0 as c_int as c_uint;
        (*c).task_thread.cur = (*c).n_fc;
        ::core::intrinsics::atomic_store_seqcst(&mut (*c).task_thread.reset_task_cur, u32::MAX);
        ::core::intrinsics::atomic_store_seqcst(&mut (*c).task_thread.cond_signaled, 0 as c_int);
        pthread_mutex_unlock(&mut (*c).task_thread.lock);
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
            rav1d_decode_frame_exit(&mut *f, -(1 as c_int));
            (*f).n_tile_data = 0 as c_int;
            (*f).task_thread.retval = 0 as c_int;
            let out_delayed: *mut Rav1dThreadPicture = &mut *((*c).frame_thread.out_delayed)
                .offset(next as isize)
                as *mut Rav1dThreadPicture;
            if !((*out_delayed).p.frame_hdr).is_null() {
                rav1d_thread_picture_unref(out_delayed);
            }
            n = n.wrapping_add(1);
            next = next.wrapping_add(1);
        }
        (*c).frame_thread.next = 0 as c_int as c_uint;
    }
    ::core::intrinsics::atomic_store_seqcst((*c).flush, 0 as c_int);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_flush(c: *mut Dav1dContext) {
    rav1d_flush(c)
}

#[cold]
pub(crate) unsafe fn rav1d_close(c_out: *mut *mut Rav1dContext) {
    if c_out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"c_out != ((void*)0)\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 12], &[c_char; 12]>(b"dav1d_close\0")).as_ptr(),
        );
        return;
    }
    close_internal(c_out, 1 as c_int);
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_close(c_out: *mut *mut Dav1dContext) {
    rav1d_close(c_out)
}

#[cold]
unsafe extern "C" fn close_internal(c_out: *mut *mut Rav1dContext, flush: c_int) {
    let c: *mut Rav1dContext = *c_out;
    if c.is_null() {
        return;
    }
    if flush != 0 {
        rav1d_flush(c);
    }
    if !((*c).tc).is_null() {
        let ttd: *mut TaskThreadData = &mut (*c).task_thread;
        if (*ttd).inited != 0 {
            pthread_mutex_lock(&mut (*ttd).lock);
            let mut n: c_uint = 0 as c_int as c_uint;
            while n < (*c).n_tc && (*((*c).tc).offset(n as isize)).task_thread.td.inited != 0 {
                (*((*c).tc).offset(n as isize)).task_thread.die = true;
                n = n.wrapping_add(1);
            }
            pthread_cond_broadcast(&mut (*ttd).cond);
            pthread_mutex_unlock(&mut (*ttd).lock);
            let mut n_0: c_uint = 0 as c_int as c_uint;
            while n_0 < (*c).n_tc {
                let pf: *mut Rav1dTaskContext =
                    &mut *((*c).tc).offset(n_0 as isize) as *mut Rav1dTaskContext;
                if (*pf).task_thread.td.inited == 0 {
                    break;
                }
                pthread_join((*pf).task_thread.td.thread, 0 as *mut *mut c_void);
                pthread_cond_destroy(&mut (*pf).task_thread.td.cond);
                pthread_mutex_destroy(&mut (*pf).task_thread.td.lock);
                n_0 = n_0.wrapping_add(1);
            }
            pthread_cond_destroy(&mut (*ttd).delayed_fg.cond);
            pthread_cond_destroy(&mut (*ttd).cond);
            pthread_mutex_destroy(&mut (*ttd).lock);
        }
        rav1d_free_aligned((*c).tc as *mut c_void);
    }
    let mut n_1: c_uint = 0 as c_int as c_uint;
    while !((*c).fc).is_null() && n_1 < (*c).n_fc {
        let f: *mut Rav1dFrameContext =
            &mut *((*c).fc).offset(n_1 as isize) as *mut Rav1dFrameContext;
        if (*c).n_fc > 1 as c_uint {
            freep(
                &mut (*f).tile_thread.lowest_pixel_mem as *mut *mut [[c_int; 2]; 7] as *mut c_void,
            );
            freep(&mut (*f).frame_thread.b as *mut *mut Av1Block as *mut c_void);
            rav1d_freep_aligned(&mut (*f).frame_thread.pal_idx as *mut *mut u8 as *mut c_void);
            rav1d_freep_aligned(&mut (*f).frame_thread.cf as *mut *mut DynCoef as *mut c_void);
            freep(&mut (*f).frame_thread.tile_start_off as *mut *mut c_int as *mut c_void);
            rav1d_freep_aligned(
                &mut (*f).frame_thread.pal as *mut *mut [[u16; 8]; 3] as *mut c_void,
            );
            freep(&mut (*f).frame_thread.cbi as *mut *mut CodedBlockInfo as *mut c_void);
        }
        if (*c).n_tc > 1 as c_uint {
            pthread_mutex_destroy(&mut (*f).task_thread.pending_tasks.lock);
            pthread_cond_destroy(&mut (*f).task_thread.cond);
            pthread_mutex_destroy(&mut (*f).task_thread.lock);
        }
        freep(&mut (*f).frame_thread.frame_progress as *mut *mut atomic_uint as *mut c_void);
        freep(&mut (*f).task_thread.tasks as *mut *mut Rav1dTask as *mut c_void);
        freep(
            &mut *((*f).task_thread.tile_tasks).as_mut_ptr().offset(0) as *mut *mut Rav1dTask
                as *mut c_void,
        );
        rav1d_free_aligned((*f).ts as *mut c_void);
        rav1d_free_aligned((*f).ipred_edge[0] as *mut c_void);
        free((*f).a as *mut c_void);
        free((*f).tile as *mut c_void);
        free((*f).lf.mask as *mut c_void);
        free((*f).lf.lr_mask as *mut c_void);
        free((*f).lf.level as *mut c_void);
        free((*f).lf.tx_lpf_right_edge[0] as *mut c_void);
        free((*f).lf.start_of_tile_row as *mut c_void);
        rav1d_refmvs_clear(&mut (*f).rf);
        rav1d_free_aligned((*f).lf.cdef_line_buf as *mut c_void);
        rav1d_free_aligned((*f).lf.lr_line_buf as *mut c_void);
        n_1 = n_1.wrapping_add(1);
    }
    rav1d_free_aligned((*c).fc as *mut c_void);
    if (*c).n_fc > 1 as c_uint && !((*c).frame_thread.out_delayed).is_null() {
        let mut n_2: c_uint = 0 as c_int as c_uint;
        while n_2 < (*c).n_fc {
            if !((*((*c).frame_thread.out_delayed).offset(n_2 as isize))
                .p
                .frame_hdr)
                .is_null()
            {
                rav1d_thread_picture_unref(
                    &mut *((*c).frame_thread.out_delayed).offset(n_2 as isize),
                );
            }
            n_2 = n_2.wrapping_add(1);
        }
        free((*c).frame_thread.out_delayed as *mut c_void);
    }
    let mut n_3 = 0;
    while n_3 < (*c).n_tile_data {
        rav1d_data_unref_internal(&mut (*((*c).tile).offset(n_3 as isize)).data);
        n_3 += 1;
    }
    free((*c).tile as *mut c_void);
    let mut n_4 = 0;
    while n_4 < 8 {
        rav1d_cdf_thread_unref(&mut *((*c).cdf).as_mut_ptr().offset(n_4 as isize));
        if !((*c).refs[n_4 as usize].p.p.frame_hdr).is_null() {
            rav1d_thread_picture_unref(&mut (*((*c).refs).as_mut_ptr().offset(n_4 as isize)).p);
        }
        rav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(n_4 as isize)).refmvs);
        rav1d_ref_dec(&mut (*((*c).refs).as_mut_ptr().offset(n_4 as isize)).segmap);
        n_4 += 1;
    }
    rav1d_ref_dec(&mut (*c).seq_hdr_ref);
    rav1d_ref_dec(&mut (*c).frame_hdr_ref);
    rav1d_ref_dec(&mut (*c).mastering_display_ref);
    rav1d_ref_dec(&mut (*c).content_light_ref);
    rav1d_ref_dec(&mut (*c).itut_t35_ref);
    rav1d_mem_pool_end((*c).seq_hdr_pool);
    rav1d_mem_pool_end((*c).frame_hdr_pool);
    rav1d_mem_pool_end((*c).segmap_pool);
    rav1d_mem_pool_end((*c).refmvs_pool);
    rav1d_mem_pool_end((*c).cdf_pool);
    rav1d_mem_pool_end((*c).picture_pool);
    rav1d_freep_aligned(c_out as *mut c_void);
}

pub(crate) unsafe fn rav1d_get_event_flags(
    c: *mut Rav1dContext,
    flags: *mut Rav1dEventFlags,
) -> c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"c != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"dav1d_get_event_flags\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    if flags.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"flags != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 22], &[c_char; 22]>(b"dav1d_get_event_flags\0"))
                .as_ptr(),
        );
        return -(22 as c_int);
    }
    *flags = (*c).event_flags;
    (*c).event_flags = 0 as Dav1dEventFlags;
    return 0 as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_get_event_flags(
    c: *mut Dav1dContext,
    flags: *mut Dav1dEventFlags,
) -> c_int {
    rav1d_get_event_flags(c, flags)
}

pub(crate) unsafe fn rav1d_get_decode_error_data_props(
    c: *mut Rav1dContext,
    out: *mut Rav1dDataProps,
) -> c_int {
    if c.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"c != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 34], &[c_char; 34]>(
                b"dav1d_get_decode_error_data_props\0",
            ))
            .as_ptr(),
        );
        return -(22 as c_int);
    }
    if out.is_null() {
        fprintf(
            stderr,
            b"Input validation check '%s' failed in %s!\n\0" as *const u8 as *const c_char,
            b"out != NULL\0" as *const u8 as *const c_char,
            (*::core::mem::transmute::<&[u8; 34], &[c_char; 34]>(
                b"dav1d_get_decode_error_data_props\0",
            ))
            .as_ptr(),
        );
        return -(22 as c_int);
    }
    rav1d_data_props_unref_internal(out);
    *out = (*c).cached_error_props.clone();
    rav1d_data_props_set_defaults(&mut (*c).cached_error_props);
    return 0 as c_int;
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_get_decode_error_data_props(
    c: *mut Dav1dContext,
    out: *mut Dav1dDataProps,
) -> c_int {
    let mut out_rust = out.read().into_rust();
    let result = rav1d_get_decode_error_data_props(c, &mut out_rust);
    out.write(out_rust.into_c());
    result
}

pub(crate) unsafe fn rav1d_picture_unref(p: *mut Rav1dPicture) {
    rav1d_picture_unref_internal(p);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_unref(p: *mut Dav1dPicture) {
    let mut p_rust = p.read().into_rust();
    rav1d_picture_unref(&mut p_rust);
    p.write(p_rust.into_c());
}

pub(crate) unsafe fn rav1d_data_create(buf: *mut Rav1dData, sz: usize) -> *mut u8 {
    return rav1d_data_create_internal(buf, sz);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_create(buf: *mut Dav1dData, sz: usize) -> *mut u8 {
    let mut buf_rust = buf.read().into_rust();
    let result = rav1d_data_create(&mut buf_rust, sz);
    buf.write(buf_rust.into_c());
    result
}

pub(crate) unsafe fn rav1d_data_wrap(
    buf: *mut Rav1dData,
    ptr: *const u8,
    sz: usize,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    user_data: *mut c_void,
) -> c_int {
    return rav1d_data_wrap_internal(buf, ptr, sz, free_callback, user_data);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap(
    buf: *mut Dav1dData,
    ptr: *const u8,
    sz: usize,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    user_data: *mut c_void,
) -> c_int {
    let mut buf_rust = buf.read().into_rust();
    let result = rav1d_data_wrap(&mut buf_rust, ptr, sz, free_callback, user_data);
    buf.write(buf_rust.into_c());
    result
}

pub(crate) unsafe fn rav1d_data_wrap_user_data(
    buf: *mut Rav1dData,
    user_data: *const u8,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    cookie: *mut c_void,
) -> c_int {
    return rav1d_data_wrap_user_data_internal(buf, user_data, free_callback, cookie);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_user_data(
    buf: *mut Dav1dData,
    user_data: *const u8,
    free_callback: Option<unsafe extern "C" fn(*const u8, *mut c_void) -> ()>,
    cookie: *mut c_void,
) -> c_int {
    let mut buf_rust = buf.read().into_rust();
    let result = rav1d_data_wrap_user_data(&mut buf_rust, user_data, free_callback, cookie);
    buf.write(buf_rust.into_c());
    result
}

pub(crate) unsafe fn rav1d_data_unref(buf: *mut Rav1dData) {
    rav1d_data_unref_internal(buf);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_unref(buf: *mut Dav1dData) {
    let mut buf_rust = buf.read().into_rust();
    let result = rav1d_data_unref(&mut buf_rust);
    buf.write(buf_rust.into_c());
    result
}

#[no_mangle]
pub(crate) unsafe extern "C" fn rav1d_data_props_unref(props: *mut Rav1dDataProps) {
    rav1d_data_props_unref_internal(props);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_unref(props: *mut Dav1dDataProps) {
    let mut props_rust = props.read().into_rust();
    rav1d_data_props_unref(&mut props_rust);
    props.write(props_rust.into_c());
}
