use std::ptr::NonNull;

use crate::c_arc::{CArc, RawCArc};
use crate::include::dav1d::common::{Dav1dDataProps, Rav1dDataProps};

#[derive(Default)]
#[repr(C)]
pub struct Dav1dData {
    pub data: Option<NonNull<u8>>,
    pub sz: usize,
    pub r#ref: Option<RawCArc<[u8]>>, // opaque, so we can change this
    pub m: Dav1dDataProps,
}

#[derive(Clone, Default)]
#[repr(C)]
pub(crate) struct Rav1dData {
    pub data: Option<CArc<[u8]>>,
    pub m: Rav1dDataProps,
}

impl From<Dav1dData> for Rav1dData {
    fn from(value: Dav1dData) -> Self {
        let Dav1dData {
            data: _,
            sz: _,
            r#ref,
            m,
        } = value;
        Self {
            data: r#ref.map(|r#ref| {
                // SAFETY: `r#ref` is a [`RawCArc`] originally from [`CArc`].
                unsafe { CArc::from_raw(r#ref) }
            }),
            m: m.into(),
        }
    }
}

impl From<Rav1dData> for Dav1dData {
    fn from(value: Rav1dData) -> Self {
        let Rav1dData { data, m } = value;
        Self {
            // Use the stable pointer instead of going through `as_ref()`,
            // which would return a pointer derived from a shared reference,
            // making any subsequent write by the C caller UB (#1514).
            // In particular, `dav1d_data_create` hands this pointer out
            // for the caller to fill the buffer with the OBUs.
            data: data.as_ref().map(|data| data.stable_ptr().cast()),
            sz: data.as_ref().map(|data| data.len()).unwrap_or_default(),
            r#ref: data.map(|data| data.into_raw()),
            m: m.into(),
        }
    }
}
