#![cfg_attr(target_arch = "arm", feature(stdarch_arm_feature_detection))]
#![cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    feature(stdarch_riscv_feature_detection)
)]
#![allow(clippy::derivable_impls, clippy::ptr_eq)]
#![expect(
    non_upper_case_globals,
    clippy::absurd_extreme_comparisons, // error by default
    clippy::arc_with_non_send_sync,
    clippy::borrow_deref_ref,
    clippy::borrowed_box,
    clippy::cast_abs_to_unsigned,
    clippy::clone_on_copy,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::doc_overindented_list_items,
    clippy::duplicate_underscore_argument,
    clippy::eq_op, // error by default,
    clippy::erasing_op, // error by default
    clippy::explicit_auto_deref,
    clippy::identity_op,
    clippy::incompatible_msrv,
    clippy::int_plus_one,
    clippy::into_iter_on_ref,
    clippy::large_const_arrays,
    clippy::large_enum_variant,
    clippy::len_without_is_empty,
    clippy::len_zero,
    clippy::let_and_return,
    clippy::let_underscore_lock,
    clippy::manual_div_ceil,
    clippy::manual_range_contains,
    clippy::manual_saturating_arithmetic,
    clippy::module_inception,
    clippy::misrefactored_assign_op,
    clippy::needless_borrow,
    clippy::needless_late_init,
    clippy::needless_lifetimes,
    clippy::needless_option_as_deref,
    clippy::needless_range_loop,
    clippy::needless_return,
    clippy::neg_multiply,
    clippy::nonminimal_bool,
    clippy::overly_complex_bool_expr, // error by default
    clippy::option_map_unit_fn,
    clippy::partialeq_to_none,
    clippy::precedence,
    clippy::redundant_closure,
    clippy::redundant_pattern_matching,
    clippy::redundant_static_lifetimes,
    clippy::search_is_some,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unit_arg,
    clippy::uninlined_format_args,
    clippy::unnecessary_cast,
    clippy::unnecessary_fallible_conversions,
    clippy::unnecessary_map_on_constructor,
    clippy::unnecessary_mut_passed,
    clippy::unnecessary_lazy_evaluations,
    clippy::unneeded_wildcard_pattern,
    clippy::upper_case_acronyms,
    clippy::useless_conversion,
    clippy::zero_prefixed_literal,
)]
#![deny(
    unsafe_op_in_unsafe_fn,
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks
)]

#[cfg(not(any(feature = "bitdepth_8", feature = "bitdepth_16")))]
compile_error!("No bitdepths enabled. Enable one or more of the following features: `bitdepth_8`, `bitdepth_16`");

pub mod include {
    pub mod common {
        pub(crate) mod attributes;
        pub(crate) mod bitdepth;
        pub(crate) mod dump;
        pub(crate) mod intops;
        pub(crate) mod validate;
    } // mod common
    pub mod dav1d {
        pub mod common;
        pub mod data;
        pub mod dav1d;
        pub mod headers;
        pub mod picture;
    } // mod dav1d
} // mod include

pub mod align;
pub(crate) mod c_arc;
pub(crate) mod c_box;
mod cdef;
mod cdef_apply;
mod cdf;
mod const_fn;
pub mod cpu;
mod ctx;
mod cursor;
mod data;
mod decode;
mod dequant_tables;
pub(crate) mod disjoint_mut;
pub(crate) mod enum_map;
mod env;
pub(crate) mod error;
mod ffi_safe;
mod fg_apply;
mod filmgrain;
mod getbits;
pub(crate) mod pic_or_buf;
pub(crate) mod pixels;
pub(crate) mod relaxed_atomic;
pub mod send_sync_non_null;
pub(crate) mod strided;
pub(crate) mod with_offset;
pub(crate) mod wrap_fn_ptr;
// TODO(kkysen) Temporarily `pub(crate)` due to a `pub use` until TAIT.
mod extensions;
mod in_range;
mod internal;
mod intra_edge;
mod ipred;
mod ipred_prepare;
mod iter;
mod itx;
mod itx_1d;
pub(crate) mod levels;
mod lf_apply;
mod lf_mask;
pub(crate) mod log;
mod loopfilter;
mod looprestoration;
mod lr_apply;
mod mc;
mod msac;
mod obu;
mod pal;
mod picture;
mod pool;
mod qm;
mod recon;
mod refmvs;
mod scan;
mod tables;
mod thread_task;
mod warpmv;
mod wedge;

pub use crate::error::Dav1dResult;

use crate::c_arc::RawArc;
use crate::c_box::FnFree;
use crate::cpu::rav1d_init_cpu;
use crate::cpu::rav1d_num_logical_processors;
use crate::decode::rav1d_decode_frame_exit;
use crate::error::Rav1dError::EGeneric;
use crate::error::Rav1dError::EAGAIN;
use crate::error::Rav1dError::EINVAL;
use crate::error::Rav1dResult;
use crate::extensions::OptionError as _;
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
use crate::internal::Rav1dBitDepthDSPContext;
use crate::internal::Rav1dContext;
use crate::internal::Rav1dContextFrameThread;
use crate::internal::Rav1dContextTaskThread;
use crate::internal::Rav1dContextTaskType;
use crate::internal::Rav1dFrameContext;
use crate::internal::Rav1dState;
use crate::internal::Rav1dTaskContext;
use crate::internal::Rav1dTaskContextTaskThread;
use crate::internal::TaskThreadData;
use crate::iter::wrapping_iter;
use crate::log::Rav1dLog as _;
use crate::log::Rav1dLogger;
use crate::obu::rav1d_parse_obus;
use crate::obu::rav1d_parse_sequence_header;
use crate::picture::rav1d_picture_alloc_copy;
use crate::picture::PictureFlags;
use crate::send_sync_non_null::SendSyncNonNull;
use crate::thread_task::rav1d_task_delayed_fg;
use crate::thread_task::rav1d_worker_task;
use crate::thread_task::FRAME_ERROR;
use parking_lot::Mutex;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::ptr::NonNull;
use std::slice;
use std::sync::atomic::AtomicBool;
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
            logger: Some(Rav1dLogger::default()),
            strict_std_compliance: false,
            output_invisible_frames: false,
            inloop_filters: Rav1dInloopFilterType::all(),
            decode_frame_type: Rav1dDecodeFrameType::All,
        }
    }
}

/// # Safety
///
/// * `s` must be valid to [`ptr::write`] to.
///   The former contents of `s` are not [`drop`]ped and it may be uninitialized.
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_default_settings(s: NonNull<Dav1dSettings>) {
    let settings = Rav1dSettings::default().into();
    // SAFETY: `s` is safe to `ptr::write` to.
    unsafe { s.as_ptr().write(settings) };
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
        rav1d_num_logical_processors().get().clamp(1, 256)
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

/// # Safety
///
/// * `s`, if [`NonNull`], must valid to [`ptr::read`] from.
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_get_frame_delay(s: Option<NonNull<Dav1dSettings>>) -> Dav1dResult {
    (|| {
        let s = validate_input!(s.ok_or(EINVAL))?;
        // SAFETY: `s` is safe to `ptr::read`.
        let s = unsafe { s.as_ptr().read() };
        let s = s.try_into()?;
        rav1d_get_frame_delay(&s).map(|frame_delay| frame_delay as c_uint)
    })()
    .into()
}

#[cold]
pub(crate) fn rav1d_open(s: &Rav1dSettings) -> Rav1dResult<Arc<Rav1dContext>> {
    static initted: Once = Once::new();
    initted.call_once(|| init_internal());

    validate_input!((s.n_threads >= 0 && s.n_threads <= 256, EINVAL))?;
    validate_input!((s.max_frame_delay >= 0 && s.max_frame_delay <= 256, EINVAL))?;
    validate_input!((s.operating_point <= 31, EINVAL))?;
    validate_input!((
        !s.allocator.is_default() || s.allocator.cookie.is_none(),
        EINVAL
    ))?;

    // On 32-bit systems, extremely large frame sizes can cause overflows in
    // `rav1d_decode_frame` alloc size calculations. Prevent that from occuring
    // by enforcing a maximum frame size limit, chosen to roughly correspond to
    // the largest size possible to decode without exhausting virtual memory.
    let frame_size_limit;
    if mem::size_of::<usize>() < 8 && s.frame_size_limit.wrapping_sub(1) >= 8192 * 8192 {
        frame_size_limit = 8192 * 8192;
        if s.frame_size_limit != 0 {
            writeln!(
                s.logger,
                "Frame size limit reduced from {} to {}.",
                s.frame_size_limit, frame_size_limit,
            );
        }
    } else {
        frame_size_limit = s.frame_size_limit;
    }

    let NumThreads { n_tc, n_fc } = get_num_threads(s);

    let ttd = TaskThreadData {
        cur: (n_fc as u32).into(),
        reset_task_cur: AtomicU32::new(u32::MAX),
        ..Default::default()
    };
    // TODO fallible allocation
    let task_thread = Arc::new(ttd);

    let fc = (0..n_fc)
        .map(|i| {
            let mut fc = Rav1dFrameContext::default(i);
            fc.task_thread.finished = AtomicBool::new(true);
            fc.task_thread.ttd = Arc::clone(&task_thread);
            let f = fc.data.get_mut();
            f.lf.last_sharpness = u8::MAX;
            fc
        })
        // TODO fallible allocation
        .collect();

    let state = Mutex::new(Rav1dState {
        frame_thread: Rav1dContextFrameThread {
            out_delayed: if n_fc > 1 {
                (0..n_fc).map(|_| Default::default()).collect()
            } else {
                Box::new([])
            },
            ..Default::default()
        },
        ..Default::default()
    });

    let tc = (0..n_tc)
        .map(|n| {
            let task_thread = Arc::clone(&task_thread);
            let thread_data = Arc::new(Rav1dTaskContextTaskThread::new(task_thread));
            let thread_data_copy = Arc::clone(&thread_data);
            let task = if n_tc > 1 {
                let handle = thread::Builder::new()
                    // Don't set stack size like `dav1d` does.
                    // See <https://github.com/memorysafety/rav1d/issues/889>.
                    .name(format!("rav1d-worker-{n}"))
                    .spawn(|| rav1d_worker_task(thread_data_copy))
                    .unwrap();
                Rav1dContextTaskType::Worker(handle)
            } else {
                Rav1dContextTaskType::Single(Mutex::new(Box::new(Rav1dTaskContext::new(
                    thread_data_copy,
                ))))
            };
            Rav1dContextTaskThread { task, thread_data }
        })
        // TODO fallible allocation
        .collect();

    let c = Rav1dContext {
        allocator: s.allocator.clone(),
        logger: s.logger.clone(),
        apply_grain: s.apply_grain,
        operating_point: s.operating_point,
        all_layers: s.all_layers,
        frame_size_limit,
        strict_std_compliance: s.strict_std_compliance,
        output_invisible_frames: s.output_invisible_frames,
        inloop_filters: s.inloop_filters,
        decode_frame_type: s.decode_frame_type,
        fc,
        task_thread,
        state,
        tc,
        ..Default::default()
    };

    // TODO fallible allocation
    let mut c = Arc::new(c);

    if c.allocator.is_default() {
        let c = Arc::get_mut(&mut c).unwrap();
        // SAFETY: When `allocator.is_default()`, `allocator.cookie` should be a `&c.picture_pool`.
        // See `Rav1dPicAllocator::cookie` docs for more, including an analysis of the lifetime.
        // Note also that we must do this after we created the `Arc` so that `c` has a stable address.
        c.allocator.cookie = Some(SendSyncNonNull::from_ref(&c.picture_pool).cast::<c_void>());
    }
    let c = c;

    for tc in c.tc.iter() {
        if let Rav1dContextTaskType::Worker(handle) = &tc.task {
            // Unpark each thread once we set its `thread_data.c`.
            *tc.thread_data.c.lock() = Some(Arc::clone(&c));
            handle.thread().unpark();
        }
    }

    Ok(c)
}

/// # Safety
///
/// * `c_out`, if [`NonNull`], is valid to [`ptr::write`] to.
/// * `s`, if [`NonNull`], is valid to [`ptr::read`] from.
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_open(
    c_out: Option<NonNull<Option<Dav1dContext>>>,
    s: Option<NonNull<Dav1dSettings>>,
) -> Dav1dResult {
    (|| {
        let mut c_out = validate_input!(c_out.ok_or(EINVAL))?;
        let s = validate_input!(s.ok_or(EINVAL))?;
        // SAFETY: `c_out` is safe to write to.
        let c_out = unsafe { c_out.as_mut() };
        // SAFETY: `s` is safe to read from.
        let s = unsafe { s.as_ptr().read() };
        let s = s.try_into()?;
        let c = rav1d_open(&s).inspect_err(|_| {
            *c_out = None;
        })?;
        *c_out = Some(RawArc::from_arc(c));
        Ok(())
    })()
    .into()
}

/// # Safety
///
/// * `out`, if [`NonNull`], is valid to [`ptr::write`] to.
/// * `ptr`, if [`NonNull`], is the start of a `&[u8]` slice of length `sz`.
#[no_mangle]
pub unsafe extern "C" fn dav1d_parse_sequence_header(
    out: Option<NonNull<Dav1dSequenceHeader>>,
    ptr: Option<NonNull<u8>>,
    sz: usize,
) -> Dav1dResult {
    (|| {
        let out = validate_input!(out.ok_or(EINVAL))?;
        let ptr = validate_input!(ptr.ok_or(EINVAL))?;
        validate_input!((sz > 0 && sz <= usize::MAX / 2, EINVAL))?;
        // SAFETY: `ptr` is the start of a `&[u8]` slice of length `sz`.
        let data = unsafe { slice::from_raw_parts(ptr.as_ptr(), sz) };
        let seq_hdr = rav1d_parse_sequence_header(data)?.dav1d;
        // SAFETY: `out` is safe to write to.
        unsafe { out.as_ptr().write(seq_hdr) };
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

fn output_image(c: &Rav1dContext, state: &mut Rav1dState, out: &mut Rav1dPicture) -> Rav1dResult {
    let mut res = Ok(());

    let use_cache = !c.all_layers && state.max_spatial_id != 0;
    let r#in = if !use_cache {
        &mut state.out
    } else {
        &mut state.cache
    };
    if !c.apply_grain || !r#in.p.has_grain() {
        *out = mem::take(&mut r#in.p);
    } else {
        res = rav1d_apply_grain(c, out, &r#in.p);
    }
    let _ = mem::take(r#in);

    if use_cache && state.out.p.data.is_some() {
        state.cache = mem::take(&mut state.out);
    }
    res
}

fn output_picture_ready(c: &Rav1dContext, state: &mut Rav1dState, drain: bool) -> bool {
    if state.cached_error.is_some() {
        return true;
    }
    if !c.all_layers && state.max_spatial_id != 0 {
        if state.out.p.data.is_some() && state.cache.p.data.is_some() {
            if state.max_spatial_id == state.cache.p.frame_hdr.as_ref().unwrap().spatial_id
                || state.out.flags.contains(PictureFlags::NEW_TEMPORAL_UNIT)
            {
                return true;
            }
            state.cache = mem::take(&mut state.out);
            return false;
        } else {
            if state.cache.p.data.is_some() && drain {
                return true;
            } else {
                if state.out.p.data.is_some() {
                    state.cache = mem::take(&mut state.out);
                    return false;
                }
            }
        }
    }
    state.out.p.data.is_some()
}

fn drain_picture(c: &Rav1dContext, state: &mut Rav1dState, out: &mut Rav1dPicture) -> Rav1dResult {
    let mut drained = false;
    for _ in 0..c.fc.len() {
        let next = state.frame_thread.next;
        let fc = &c.fc[next as usize];
        let mut task_thread_lock = c.task_thread.lock.lock();
        while !fc.task_thread.finished.load(Ordering::SeqCst) {
            fc.task_thread.cond.wait(&mut task_thread_lock);
        }
        let out_delayed = &mut state.frame_thread.out_delayed[next as usize];
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
            let cur = c.task_thread.cur.get();
            if cur != 0 && (cur as usize) < c.fc.len() {
                c.task_thread.cur.set(cur - 1);
            }
            drained = true;
        } else if drained {
            break;
        }
        state.frame_thread.next = (state.frame_thread.next + 1) % c.fc.len() as u32;
        drop(task_thread_lock);
        mem::take(&mut *fc.task_thread.retval.try_lock().unwrap())
            .err_or(())
            .inspect_err(|_| {
                state.cached_error_props = out_delayed.p.m.clone();
                let _ = mem::take(out_delayed);
            })?;
        if out_delayed.p.data.is_some() {
            let progress = out_delayed.progress.as_ref().unwrap()[1].load(Ordering::Relaxed);
            if (out_delayed.visible || c.output_invisible_frames) && progress != FRAME_ERROR {
                state.out = out_delayed.clone();
                state.event_flags |= out_delayed.flags.into();
            }
            let _ = mem::take(out_delayed);
            if output_picture_ready(c, state, false) {
                return output_image(c, state, out);
            }
        }
    }
    if output_picture_ready(c, state, true) {
        return output_image(c, state, out);
    }
    Err(EAGAIN)
}

fn gen_picture(c: &Rav1dContext, state: &mut Rav1dState) -> Rav1dResult {
    if output_picture_ready(c, state, false) {
        return Ok(());
    }
    // Take so we don't have 2 `&mut`s.
    let Rav1dData {
        data: r#in,
        m: props,
    } = mem::take(&mut state.in_0);
    let Some(mut r#in) = r#in else { return Ok(()) };
    while !r#in.is_empty() {
        let len = rav1d_parse_obus(c, state, &r#in, &props);
        if let Ok(len) = len {
            r#in.slice_in_place(len..);
        }
        // Note that [`output_picture_ready`] doesn't read [`Rav1dContext::in_0`].
        if output_picture_ready(c, state, false) {
            // Restore into `c` when there's still data left.
            if !r#in.is_empty() {
                state.in_0 = Rav1dData {
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

pub(crate) fn rav1d_send_data(c: &Rav1dContext, in_0: &mut Rav1dData) -> Rav1dResult {
    let state = &mut *c.state.try_lock().unwrap();
    if in_0.data.is_some() {
        let sz = in_0.data.as_ref().unwrap().len();
        validate_input!((sz > 0 && sz <= usize::MAX / 2, EINVAL))?;
        state.drain = false;
    }
    if state.in_0.data.is_some() {
        return Err(EAGAIN);
    }
    state.in_0 = in_0.clone();
    let res = gen_picture(c, state);
    if res.is_ok() {
        let _ = mem::take(in_0);
    }
    res
}

/// # Safety
///
/// * `c`, if [`NonNull`], must be from [`dav1d_open`] and not be passed to [`dav1d_close`] yet.
/// * `r#in`, if [`NonNull`], must be valid to [`ptr::read`] from and [`ptr::write`] to.
#[no_mangle]
pub unsafe extern "C" fn dav1d_send_data(
    c: Option<Dav1dContext>,
    r#in: Option<NonNull<Dav1dData>>,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        let r#in = validate_input!(r#in.ok_or(EINVAL))?;
        // SAFETY: `c` is from `dav1d_open` and thus from `RawArc::from_arc`.
        // It has not yet been passed to `dav1d_close` and thus not to `RawArc::into_arc` yet.
        let c = unsafe { c.as_ref() };
        // SAFETY: `r#in` is safe to read from.
        let in_c = unsafe { r#in.as_ptr().read() };
        let mut in_rust = in_c.into();
        let result = rav1d_send_data(c, &mut in_rust);
        let in_c = in_rust.into();
        // SAFETY: `r#in` is safe to write to.
        unsafe { r#in.as_ptr().write(in_c) };
        result
    })()
    .into()
}

pub(crate) fn rav1d_get_picture(c: &Rav1dContext, out: &mut Rav1dPicture) -> Rav1dResult {
    let state = &mut *c.state.try_lock().unwrap();
    let drain = mem::replace(&mut state.drain, true);
    gen_picture(c, state)?;
    mem::take(&mut state.cached_error).err_or(())?;
    if output_picture_ready(c, state, c.fc.len() == 1) {
        return output_image(c, state, out);
    }
    if c.fc.len() > 1 && drain {
        return drain_picture(c, state, out);
    }
    Err(EAGAIN)
}

/// # Safety
///
/// * `c`, if [`NonNull`], must be from [`dav1d_open`] and not be passed to [`dav1d_close`] yet.
/// * `out`, if [`NonNull`], must be valid to [`ptr::write`] to.
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_picture(
    c: Option<Dav1dContext>,
    out: Option<NonNull<Dav1dPicture>>,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        let out = validate_input!(out.ok_or(EINVAL))?;
        // SAFETY: `c` is from `dav1d_open` and thus from `RawArc::from_arc`.
        // It has not yet been passed to `dav1d_close` and thus not to `RawArc::into_arc` yet.
        let c = unsafe { c.as_ref() };
        let mut out_rust = Default::default(); // TODO(kkysen) Temporary until we return it directly.
        let result = rav1d_get_picture(c, &mut out_rust);
        let out_c = out_rust.into();
        // SAFETY: `out` is safe to write to.
        unsafe { out.as_ptr().write(out_c) };
        result
    })()
    .into()
}

pub(crate) fn rav1d_apply_grain(
    c: &Rav1dContext,
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
            let bpc = out.p.bpc;
            let dsp = Rav1dBitDepthDSPContext::get(bpc).unwrap();
            let fg = &dsp.fg;
            use fg_apply::rav1d_apply_grain;
            match bpc {
                #[cfg(feature = "bitdepth_8")]
                8 => rav1d_apply_grain::<BitDepth8>(fg, out, in_0),
                #[cfg(feature = "bitdepth_16")]
                10 | 12 => rav1d_apply_grain::<BitDepth16>(fg, out, in_0),
                _ => {}
            }
        }
        return Ok(());
    };
}

/// # Safety
///
/// * `c`, if [`NonNull`], must be from [`dav1d_open`] and not be passed to [`dav1d_close`] yet.
/// * `out`, if [`NonNull`], must be valid to [`ptr::write`] to.
/// * `r#in`, if [`NonNull`], must be valid to [`ptr::read`] from.
#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain(
    c: Option<Dav1dContext>,
    out: Option<NonNull<Dav1dPicture>>,
    r#in: Option<NonNull<Dav1dPicture>>,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        let out = validate_input!(out.ok_or(EINVAL))?;
        let r#in = validate_input!(r#in.ok_or(EINVAL))?;
        // SAFETY: `c` is from `dav1d_open` and thus from `RawArc::from_arc`.
        // It has not yet been passed to `dav1d_close` and thus not to `RawArc::into_arc` yet.
        let c = unsafe { c.as_ref() };
        // SAFETY: `r#in` is safe to read from.
        let in_c = unsafe { r#in.as_ptr().read() };
        // Don't `.update_rav1d()` [`Rav1dSequenceHeader`] because it's meant to be read-only.
        // Don't `.update_rav1d()` [`Rav1dFrameHeader`] because it's meant to be read-only.
        // Don't `.update_rav1d()` [`Rav1dITUTT35`] because we never read it.
        let mut out_rust = Default::default(); // TODO(kkysen) Temporary until we return it directly.
        let in_rust = in_c.into();
        let result = rav1d_apply_grain(c, &mut out_rust, &in_rust);
        let out_c = out_rust.into();
        // SAFETY: `out` is safe to write to.
        unsafe { out.as_ptr().write(out_c) };
        result
    })()
    .into()
}

pub(crate) fn rav1d_flush(c: &Rav1dContext) {
    let state = &mut *c.state.try_lock().unwrap();

    let old_state = mem::take(state);
    state.tiles = old_state.tiles;
    state.n_tiles = old_state.n_tiles;
    state.frame_thread = old_state.frame_thread;
    state.operating_point_idc = old_state.operating_point_idc;
    state.max_spatial_id = old_state.max_spatial_id;
    state.frame_flags = old_state.frame_flags;
    state.event_flags = old_state.event_flags;

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
        for fc in c.fc.iter() {
            fc.task_thread.tasks.clear();
        }
        c.task_thread.first.store(0, Ordering::SeqCst);
        c.task_thread.cur.set(c.fc.len() as u32);
        c.task_thread
            .reset_task_cur
            .store(u32::MAX, Ordering::SeqCst);
        c.task_thread.cond_signaled.store(0, Ordering::SeqCst);
    }
    if c.fc.len() > 1 {
        for fc in wrapping_iter(c.fc.iter(), state.frame_thread.next as usize) {
            let _ = rav1d_decode_frame_exit(c, fc, Err(EGeneric));
            *fc.task_thread.retval.try_lock().unwrap() = None;
            let out_delayed = &mut state.frame_thread.out_delayed[fc.index];
            if out_delayed.p.frame_hdr.is_some() {
                let _ = mem::take(out_delayed);
            }
        }
        state.frame_thread.next = 0;
    }
    c.flush.store(false, Ordering::SeqCst);
}

/// # Safety
///
/// * `c` must be from [`dav1d_open`] and not be passed to [`dav1d_close`] yet.
#[no_mangle]
pub unsafe extern "C" fn dav1d_flush(c: Dav1dContext) {
    // SAFETY: `c` is from `dav1d_open` and thus from `RawArc::from_arc`.
    // It has not yet been passed to `dav1d_close` and thus not to `RawArc::into_arc` yet.
    let c = unsafe { c.as_ref() };
    rav1d_flush(c)
}

#[cold]
pub(crate) fn rav1d_close(c: Arc<Rav1dContext>) {
    let c = &*c;
    rav1d_flush(c);
    c.tell_worker_threads_to_die();
}

/// # Safety
///
/// * `c_out`, if [`NonNull`], must be safe to [`ptr::read`] from and [`ptr::write`] to.
///   The `Dav1dContext` pointed to by `c_out` must be from [`dav1d_open`].
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_close(c_out: Option<NonNull<Option<Dav1dContext>>>) {
    let Ok(mut c_out) = validate_input!(c_out.ok_or(())) else {
        return;
    };
    // SAFETY: `c_out` is safe to read from and write to.
    let c_out = unsafe { c_out.as_mut() };
    mem::take(c_out).map(|c| {
        // SAFETY: `c` is from `dav1d_open` and thus from `RawArc::from_arc`.
        let c = unsafe { c.into_arc() };
        rav1d_close(c);
    });
}

impl Rav1dContext {
    fn tell_worker_threads_to_die(&self) {
        if self.tc.is_empty() {
            return;
        }
        let ttd = &*self.task_thread;
        let _task_thread_lock = ttd.lock.lock();
        for tc in self.tc.iter() {
            tc.thread_data.die.set(true);
        }
        ttd.cond.notify_all();
    }
}

/// # Safety
///
/// * `c`, if [`NonNull`], must be from [`dav1d_open`] and not be passed to [`dav1d_close`] yet.
/// * `flags`, if [`NonNull`], must be valid to [`ptr::write`] to.
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_event_flags(
    c: Option<Dav1dContext>,
    flags: Option<NonNull<Dav1dEventFlags>>,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        let flags = validate_input!(flags.ok_or(EINVAL))?;
        // SAFETY: `c` is from `dav1d_open` and thus from `RawArc::from_arc`.
        // It has not yet been passed to `dav1d_close` and thus not to `RawArc::into_arc` yet.
        let c = unsafe { c.as_ref() };
        let state = &mut *c.state.try_lock().unwrap();
        let flags_rust = mem::take(&mut state.event_flags);
        let flags_c = flags_rust.into();
        // SAFETY: `flags` is safe to write to.
        unsafe { flags.as_ptr().write(flags_c) };
        Ok(())
    })()
    .into()
}

/// # Safety
///
/// * `c`, if [`NonNull`], must be from [`dav1d_open`] and not be passed to [`dav1d_close`] yet.
/// * `out`, if [`NonNull`], is valid to [`ptr::write`] to.
#[no_mangle]
pub unsafe extern "C" fn dav1d_get_decode_error_data_props(
    c: Option<Dav1dContext>,
    out: Option<NonNull<Dav1dDataProps>>,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        let out = validate_input!(out.ok_or(EINVAL))?;
        // SAFETY: `c` is from `dav1d_open` and thus from `RawArc::from_arc`.
        // It has not yet been passed to `dav1d_close` and thus not to `RawArc::into_arc` yet.
        let c = unsafe { c.as_ref() };
        let state = &mut *c.state.try_lock().unwrap();
        let props_rust = mem::take(&mut state.cached_error_props);
        let props_c = props_rust.into();
        // SAFETY: `out` is safety to write to.
        unsafe { out.as_ptr().write(props_c) };
        Ok(())
    })()
    .into()
}

/// # Safety
///
/// * `p`, if [`NonNull`], must be valid to [`ptr::read`] from and [`ptr::write`] to.
#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_unref(p: Option<NonNull<Dav1dPicture>>) {
    let Ok(p) = validate_input!(p.ok_or(())) else {
        return;
    };
    // SAFETY: `p` is safe to read from.
    let p_c = unsafe { p.as_ptr().read() };
    let mut p_rust = p_c.to::<Rav1dPicture>();
    let _ = mem::take(&mut p_rust);
    let p_c = p_rust.into();
    // SAFETY: `p` is safe to write to.
    unsafe { p.as_ptr().write(p_c) };
}

/// # Safety
///
/// * `buf`, if [`NonNull`], is valid to [`ptr::write`] to.
///   After this call, `buf.data` will be an allocated slice of length `sz`.
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_create(buf: Option<NonNull<Dav1dData>>, sz: usize) -> *mut u8 {
    || -> Rav1dResult<*mut u8> {
        let buf = validate_input!(buf.ok_or(EINVAL))?;
        validate_input!((sz <= usize::MAX / 2, EINVAL))?;
        let data = Rav1dData::create(sz)?;
        let data = data.to::<Dav1dData>();
        let ptr = data
            .data
            .map(|ptr| ptr.as_ptr())
            .unwrap_or_else(ptr::null_mut);
        // SAFETY: `buf` is safe to write to.
        unsafe { buf.as_ptr().write(data) };
        Ok(ptr)
    }()
    .unwrap_or_else(|_| ptr::null_mut())
}

/// # Safety
///
/// * `buf`, if [`NonNull`], is valid to [`ptr::write`] to.
/// * `ptr`, if [`NonNull`], is the start of a `&[u8]` slice of length `sz`.
/// * `ptr`'s slice must be valid to dereference until `free_callback` is called on it, which must deallocate it.
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap(
    buf: Option<NonNull<Dav1dData>>,
    ptr: Option<NonNull<u8>>,
    sz: usize,
    free_callback: Option<FnFree>,
    user_data: Option<SendSyncNonNull<c_void>>,
) -> Dav1dResult {
    || -> Rav1dResult {
        let buf = validate_input!(buf.ok_or(EINVAL))?;
        let ptr = validate_input!(ptr.ok_or(EINVAL))?;
        validate_input!((sz <= usize::MAX / 2, EINVAL))?;
        // SAFETY: `ptr` is the start of a `&[u8]` slice of length `sz`.
        let data = unsafe { slice::from_raw_parts(ptr.as_ptr(), sz) };
        // SAFETY: `ptr`, and thus `data`, is valid to dereference until `free_callback` is called on it, which deallocates it.
        let data = unsafe { Rav1dData::wrap(data.into(), free_callback, user_data) }?;
        let data_c = data.into();
        // SAFETY: `buf` is safe to write to.
        unsafe { buf.as_ptr().write(data_c) };
        Ok(())
    }()
    .into()
}

/// # Safety
///
/// * `buf`, if [`NonNull`], is valid to [`ptr::read`] from and [`ptr::write`] to.
/// * `user_data`, if [`NonNull`], is valid to dereference until `free_callback` is called on it, which must deallocate it.
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_user_data(
    buf: Option<NonNull<Dav1dData>>,
    user_data: Option<NonNull<u8>>,
    free_callback: Option<FnFree>,
    cookie: Option<SendSyncNonNull<c_void>>,
) -> Dav1dResult {
    || -> Rav1dResult {
        let buf = validate_input!(buf.ok_or(EINVAL))?;
        // Note that `dav1d` doesn't do this check, but they do for the similar [`dav1d_data_wrap`].
        let user_data = validate_input!(user_data.ok_or(EINVAL))?;
        // SAFETY: `buf` is safe to read from.
        let data_c = unsafe { buf.as_ptr().read() };
        let mut data = data_c.to::<Rav1dData>();
        // SAFETY: `user_data` is valid to dereference until `free_callback` is called on it, which deallocates it.
        unsafe { data.wrap_user_data(user_data, free_callback, cookie) }?;
        let data_c = data.into();
        // SAFETY: `buf` is safe to write to.
        unsafe { buf.as_ptr().write(data_c) };
        Ok(())
    }()
    .into()
}

/// # Safety
///
/// * `buf`, if [`NonNull`], is safe to [`ptr::read`] from and [`ptr::write`] from.
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_unref(buf: Option<NonNull<Dav1dData>>) {
    let buf = validate_input!(buf.ok_or(()));
    let Ok(mut buf) = buf else { return };
    // SAFETY: `buf` is safe to read from and write to.
    let buf = unsafe { buf.as_mut() };
    let _ = mem::take(buf).to::<Rav1dData>();
}

/// # Safety
///
/// * `props`, if [`NonNull`], is safe to [`ptr::read`] from and [`ptr::write`] from.
#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_unref(props: Option<NonNull<Dav1dDataProps>>) {
    let props = validate_input!(props.ok_or(()));
    let Ok(mut props) = props else { return };
    // SAFETY: `props` is safe to read from and write to.
    let props = unsafe { props.as_mut() };
    let _ = mem::take(props).to::<Rav1dDataProps>();
}
