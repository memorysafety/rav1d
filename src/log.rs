use std::ffi::c_char;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::ffi::CStr;
use std::fmt;
use std::fmt::Write as _;
use std::io::stderr;
use std::io::stdout;
use std::io::Write as _;
use std::ptr;

pub type Dav1dLoggerCallback = unsafe extern "C" fn(
    // The above `cookie` field.
    cookie: *mut c_void,
    // A `printf`-style format specifier.
    fmt: *const c_char,
    // `printf`-style variadic args.
    args: ...
);

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dLogger {
    /// A cookie that's passed as the first argument to the callback below.
    cookie: *mut c_void,
    /// A `printf`-style function except for an extra first argument that will always be the above `cookie`.
    callback: Option<Dav1dLoggerCallback>,
}

impl Dav1dLogger {
    /// # Safety
    ///
    /// `callback`, if non-[`None`]/`NULL` must be safe to call when:
    /// * the first argument is `cookie`
    /// * the rest of the arguments would be safe to call `printf` with
    pub const unsafe fn new(cookie: *mut c_void, callback: Option<Dav1dLoggerCallback>) -> Self {
        Self { cookie, callback }
    }
}

/// A [`Dav1dLogger`] from C that's not
/// just a [`Rav1dLogger`] converted to a [`Dav1dLogger`].
#[derive(Clone)]
pub(crate) struct OnlyDav1dLogger {
    cookie: *mut c_void,
    callback: Dav1dLoggerCallback,
}

impl fmt::Write for OnlyDav1dLogger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // `s` doesn't have a terminating nul-byte,
        // and it may have internal nul-bytes,
        // so it's easiest just to print one byte at a time.
        // This may be slow, but logging can be disabled if it's slow,
        // or the Rust API can be used instead.
        // TODO(kkysen) Replace with `c"%c"` once its stabilization reaches stable.
        let fmt = CStr::from_bytes_with_nul(b"%c\0").unwrap();
        for &byte in s.as_bytes() {
            // # Safety
            //
            // The first argument is `self.cookie`
            // and the rest are safe to call `printf` with,
            // as required by [`Self::new`].
            unsafe {
                (self.callback)(self.cookie, fmt.as_ptr(), byte as c_uint);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub(crate) enum Rav1dLogger {
    Dav1d(OnlyDav1dLogger),
    Stdout,
    #[default]
    Stderr,
}

/// Any type implementing [`Rav1dLog`] can be used with [`write!`].
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
            // [`OnlyDav1dLogger`] is trivial to [`Clone`], though, so we can just do that.
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

/// Used as a marker for [`Rav1dLogger::Stdout`].  Still a valid (i.e. safe) [`Dav1dLoggerCallback`], though.
unsafe extern "C" fn rav1d_logger_stdout(_cookie: *mut c_void, _fmt: *const c_char, ...) {}

/// Used as a marker for [`Rav1dLogger::Stderr`].  Still a valid (i.e. safe) [`Dav1dLoggerCallback`], though.
unsafe extern "C" fn rav1d_logger_stderr(_cookie: *mut c_void, _fmt: *const c_char, ...) {}

impl From<Dav1dLogger> for Option<Rav1dLogger> {
    fn from(logger: Dav1dLogger) -> Self {
        let Dav1dLogger { cookie, callback } = logger;
        let callback = callback?;
        Some(if callback == rav1d_logger_stdout {
            Rav1dLogger::Stdout
        } else if callback == rav1d_logger_stderr {
            Rav1dLogger::Stderr
        } else {
            Rav1dLogger::Dav1d(OnlyDav1dLogger { cookie, callback })
        })
    }
}

impl From<Option<Rav1dLogger>> for Dav1dLogger {
    fn from(logger: Option<Rav1dLogger>) -> Self {
        let cookie = match &logger {
            Some(Rav1dLogger::Dav1d(dav1d)) => dav1d.cookie,
            _ => ptr::null_mut(),
        };
        let callback = logger.map(|logger| match logger {
            Rav1dLogger::Dav1d(dav1d) => dav1d.callback,
            Rav1dLogger::Stdout => rav1d_logger_stdout,
            Rav1dLogger::Stderr => rav1d_logger_stderr,
        });
        Self { cookie, callback }
    }
}
