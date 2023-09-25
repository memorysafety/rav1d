use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::dav1d::Dav1dRef;
use crate::src::r#ref::Rav1dRef;
use std::ptr;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dData {
    pub data: *const u8,
    pub sz: usize,
    pub r#ref: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}

impl Default for Dav1dData {
    fn default() -> Self {
        Self {
            data: ptr::null(),
            sz: Default::default(),
            r#ref: ptr::null_mut(),
            m: Default::default(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dData {
    pub data: *const u8,
    pub sz: usize,
    pub r#ref: *mut Rav1dRef,
    pub m: Rav1dDataProps,
}

impl Default for Rav1dData {
    fn default() -> Self {
        Self {
            data: ptr::null(),
            sz: Default::default(),
            r#ref: ptr::null_mut(),
            m: Default::default(),
        }
    }
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

impl Rav1dData {
    pub fn into_c(self) -> Dav1dData {
        let Self { data, sz, r#ref, m } = self;
        Dav1dData {
            data,
            sz,
            r#ref,
            m: m.into_c(),
        }
    }
}
