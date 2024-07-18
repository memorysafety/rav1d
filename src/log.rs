use crate::src::send_sync_non_null::SendSyncNonNull;
use std::ffi::c_char;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::fmt;
use std::fmt::Write as _;
use std::io::stderr;
use std::io::stdout;
use std::io::Write as _;

pub type Dav1dLoggerCallback = unsafe extern "C" fn(
    // [`Dav1dLogger::cookie`].
    cookie: Option<SendSyncNonNull<c_void>>,
    // A `printf`-style format specifier.
    fmt: *const c_char,
    // `printf`-style variadic args.
    args: ...
);

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dLogger {
    /// A cookie that's passed as the first argument to the callback below.
    ///
    /// # Safety
    ///
    /// All accesses to [`Self::cookie`] must be thread-safe
    /// (i.e. [`Self::cookie`] must be [`Send`]` + `[`Sync`]).
    ///
    /// If used from Rust, [`Self::cookie`] is a [`SendSyncNonNull`],
    /// whose constructors ensure this [`Send`]` + `[`Sync`] safety.
    cookie: Option<SendSyncNonNull<c_void>>,

    /// A `printf`-style function except for an extra first argument that will always be the above `cookie`.
    callback: Option<Dav1dLoggerCallback>,
}

impl Dav1dLogger {
    /// # Safety
    ///
    /// `callback`, if non-[`None`]/`NULL` must be safe to call when:
    /// * the first argument is `cookie`
    /// * the rest of the arguments would be safe to call `printf` with
    ///
    /// See [`Self::cookie`]'s safety requirements.
    pub const unsafe fn new(
        cookie: Option<SendSyncNonNull<c_void>>,
        callback: Option<Dav1dLoggerCallback>,
    ) -> Self {
        Self { cookie, callback }
    }
}

impl fmt::Write for Dav1dLogger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let callback = self.callback.unwrap();

        // `s` doesn't have a terminating nul-byte,
        // and it may have internal nul-bytes,
        // so it's easiest just to print one byte at a time.
        // This may be slow, but logging can be disabled if it's slow,
        // or the Rust API can be used instead.
        let fmt = c"%c";
        for &byte in s.as_bytes() {
            // SAFETY:
            //
            // The first argument is `self.cookie`
            // and the rest are safe to call `printf` with,
            // as required by [`Self::new`].
            unsafe {
                callback(self.cookie, fmt.as_ptr(), byte as c_uint);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub(crate) enum Rav1dLogger {
    Dav1d(Dav1dLogger),
    Stdout,
    #[default]
    Stderr,
}

/// Any type implementing [`Rav1dLog`] can be used with [`write!`].
///
/// [`Rav1dLog`] is very similar to [`fmt::Write`] and [`io::Write`]
/// in that they can both be called by [`write!`] as `.write_fmt(format_args!(...))`.
///
/// The difference, and the reason we don't use either of those,
/// is that [`Rav1dLog::write_fmt`] takes `&self` instead of `&mut self`,
/// and this makes ownership and borrowing much simpler and more flexible.
/// Furthermore, this returns `()` instead of a [`Result`]
/// so that call sites don't have to propagate or `.unwrap()` it,
/// bloating call sites for non-essential logging code.
///
/// [`io::Write`]: std::io::Write
pub trait Rav1dLog {
    fn write_fmt(&self, args: fmt::Arguments);
}

impl Rav1dLog for Rav1dLogger {
    /// Logging doesn't have to be fast when it's on,
    /// but we don't want it slow things down when it's off,
    /// so ensure the logging code isn't inlined everywhere, bloating call sites.
    #[inline(never)]
    fn write_fmt(&self, args: fmt::Arguments) {
        match self {
            // The `dav1d.clone()` is because [`fmt::Write::write_fmt`] takes `&mut`
            // even though we don't need it to.
            // [`Dav1dLogger`] is trivial to [`Clone`], though, so we can just do that.
            Self::Dav1d(dav1d) => dav1d.clone().write_fmt(args).unwrap(),
            Self::Stdout => stdout().write_fmt(args).unwrap(),
            Self::Stderr => stderr().write_fmt(args).unwrap(),
        }
    }
}

impl Rav1dLog for Option<Rav1dLogger> {
    /// When a logger isn't set, we don't want to have to run any code here,
    /// so force this to be inlined so a [`None`] can be seen.
    #[inline(always)]
    fn write_fmt(&self, args: fmt::Arguments) {
        if let Some(logger) = self {
            logger.write_fmt(args);
        }
    }
}

mod marker {
    use super::*;

    type Callback = extern "C" fn(cookie: *mut c_void, fmt: *const c_char);

    const fn cast(callback: Callback) -> Dav1dLoggerCallback {
        // SAFETY: It should always be safe to ignore variadic args.
        // Declaring a variadic `fn` is unstable, though, which is why we avoid that.
        unsafe { std::mem::transmute(callback) }
    }

    pub const STDOUT: Dav1dLoggerCallback = {
        /// Create an empty [`Dav1dLoggerCallback`] for use as a marker `fn`
        /// for special `fn`s stored in [`Dav1dLogger::callback`].
        extern "C" fn stdout(_cookie: *mut c_void, _fmt: *const c_char) {
            // The `fn` needs a unique body so that
            // multiple ones don't get optimized into the same `fn`.
            unimplemented!(stringify!(stdout));
        }
        cast(stdout)
    };

    pub const STDERR: Dav1dLoggerCallback = {
        /// Create an empty [`Dav1dLoggerCallback`] for use as a marker `fn`
        /// for special `fn`s stored in [`Dav1dLogger::callback`].
        extern "C" fn stderr(_cookie: *mut c_void, _fmt: *const c_char) {
            // The `fn` needs a unique body so that
            // multiple ones don't get optimized into the same `fn`.
            unimplemented!(stringify!(stderr));
        }
        cast(stderr)
    };
}

impl From<Dav1dLogger> for Option<Rav1dLogger> {
    fn from(logger: Dav1dLogger) -> Self {
        let Dav1dLogger { cookie, callback } = logger;
        Some(match callback {
            None => return None,
            Some(cb) if cb == marker::STDOUT => Rav1dLogger::Stdout,
            Some(cb) if cb == marker::STDERR => Rav1dLogger::Stderr,
            _ => Rav1dLogger::Dav1d(Dav1dLogger { cookie, callback }),
        })
    }
}

impl From<Option<Rav1dLogger>> for Dav1dLogger {
    fn from(logger: Option<Rav1dLogger>) -> Self {
        let cookie = match &logger {
            Some(Rav1dLogger::Dav1d(dav1d)) => dav1d.cookie,
            _ => None,
        };
        let callback = logger.and_then(|logger| match logger {
            Rav1dLogger::Dav1d(dav1d) => dav1d.callback,
            Rav1dLogger::Stdout => Some(marker::STDOUT),
            Rav1dLogger::Stderr => Some(marker::STDERR),
        });
        Self { cookie, callback }
    }
}
