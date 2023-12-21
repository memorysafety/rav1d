use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::dav1d::Dav1dRef;
use crate::src::r#ref::Rav1dRef;
use std::ptr::NonNull;

#[derive(Default)]
#[repr(C)]
pub struct Dav1dData {
    pub data: Option<NonNull<u8>>,
    pub sz: usize,
    pub r#ref: Option<NonNull<Dav1dRef>>,
    pub m: Dav1dDataProps,
}

#[derive(Clone, Default)]
#[repr(C)]
pub(crate) struct Rav1dData {
    pub data: Option<NonNull<u8>>,
    pub sz: usize,
    pub r#ref: Option<NonNull<Rav1dRef>>,
    pub m: Rav1dDataProps,
}

impl From<Dav1dData> for Rav1dData {
    fn from(value: Dav1dData) -> Self {
        let Dav1dData { data, sz, r#ref, m } = value;
        Self {
            data,
            sz,
            r#ref,
            m: m.into(),
        }
    }
}

impl From<Rav1dData> for Dav1dData {
    fn from(value: Rav1dData) -> Self {
        let Rav1dData { data, sz, r#ref, m } = value;
        Self {
            data,
            sz,
            r#ref,
            m: m.into(),
        }
    }
}
