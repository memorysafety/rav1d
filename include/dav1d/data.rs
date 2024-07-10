use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::src::c_arc::CArc;
use crate::src::c_arc::RawCArc;
use std::ptr::NonNull;
use to_method::To as _;

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
            data: data
                .as_ref()
                .map(|data| data.as_ref().to::<NonNull<[u8]>>().cast()),
            sz: data.as_ref().map(|data| data.len()).unwrap_or_default(),
            r#ref: data.map(|data| data.into_raw()),
            m: m.into(),
        }
    }
}
