use std::error::Error;
use std::ffi::{c_int, c_uint};
use std::fmt::{self, Display, Formatter};

use strum::FromRepr;

/// The `errno` values that [`Rav1dError`] is made of, from the C library.
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
mod errno {
    pub use libc::{EAGAIN, EINVAL, ENOENT, ENOMEM, ENOPROTOOPT, ERANGE};
}

/// A target without a C library, such as `wasm32-unknown-unknown`,
/// has no `errno` values. Nothing there can call the C API they are for,
/// so [`Rav1dError`] only needs them distinct, and these are Linux's.
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod errno {
    use std::ffi::c_int;

    pub const ENOENT: c_int = 2;
    pub const EAGAIN: c_int = 11;
    pub const ENOMEM: c_int = 12;
    pub const EINVAL: c_int = 22;
    pub const ERANGE: c_int = 34;
    pub const ENOPROTOOPT: c_int = 92;
}

/// Error enum return by various `rav1d` operations.
#[derive(Clone, Copy, PartialEq, Eq, FromRepr, Debug)]
#[repr(u8)]
#[non_exhaustive]
pub enum Rav1dError {
    /// This represents a generic `rav1d` error.
    /// It has nothing to do with the other `errno`-based ones.
    ///
    /// Normally `EPERM = 1`, but `dav1d` never uses `EPERM`,
    /// but does use `-1`, as opposed to the normal `DAV1D_ERR(E*)`.
    ///
    /// Also note that this forces `0` to be the niche,
    /// which is more optimal since `0` is no error for [`Dav1dResult`].
    Other = 1,

    /// No entity.
    ///
    /// No Sequence Header OBUs were found in the buffer.
    NoEntity = errno::ENOENT as u8,

    /// Try again.
    ///
    /// If this is returned by [`rav1d_send_data`], then there
    /// are decoded frames pending that first have to be retrieved via [`rav1d_get_picture`]
    /// before processing any further pending data.
    ///
    /// If this is returned by [`rav1d_get_picture`], then no decoded frames are pending
    /// currently and more data needs to be sent to the decoder.
    TryAgain = errno::EAGAIN as u8,

    /// Out of memory.
    ///
    /// Not enough memory is currently available for performing this operation.
    OutOfMemory = errno::ENOMEM as u8,

    /// Invalid argument.
    ///
    /// One of the arguments passed to the function, including the bitstream, is invalid.
    InvalidArgument = errno::EINVAL as u8,

    /// Out of range.
    ///
    /// The frame size is larger than the limit.
    OutOfRange = errno::ERANGE as u8,

    /// Unsupported bitstream.
    ///
    /// The provided bitstream is not supported by `rav1d`.
    UnsupportedBitstream = errno::ENOPROTOOPT as u8,
}

impl Rav1dError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Rav1dError::TryAgain => "Try again",
            Rav1dError::InvalidArgument => "Invalid argument",
            Rav1dError::OutOfMemory => "Not enough memory available",
            Rav1dError::UnsupportedBitstream => "Unsupported bitstream",
            Rav1dError::Other => "Other error",
            Rav1dError::NoEntity => "No sequence header found",
            Rav1dError::OutOfRange => "Out of range",
        }
    }
}

impl Display for Rav1dError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.as_str())
    }
}

impl Error for Rav1dError {}

pub type Rav1dResult<T = ()> = Result<T, Rav1dError>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Dav1dResult(pub c_int);

impl From<Rav1dResult> for Dav1dResult {
    #[inline]
    fn from(value: Rav1dResult) -> Self {
        // Doing the `-` negation on both branches
        // makes the code short and branchless.
        Dav1dResult(
            -(match value {
                Ok(()) => 0,
                Err(e) => e as c_int,
            }),
        )
    }
}

impl From<Rav1dResult<c_uint>> for Dav1dResult {
    #[inline]
    fn from(value: Rav1dResult<c_uint>) -> Self {
        Dav1dResult(match value {
            Ok(value) => value as c_int,
            Err(e) => e as c_int,
        })
    }
}

impl TryFrom<Dav1dResult> for Rav1dResult {
    type Error = Dav1dResult;

    #[inline]
    fn try_from(value: Dav1dResult) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Ok(())),
            e => {
                let e = (-e).try_into().map_err(|_| value)?;
                let e = Rav1dError::from_repr(e).ok_or(value)?;
                Ok(Err(e))
            }
        }
    }
}
