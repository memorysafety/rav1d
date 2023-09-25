use crate::include::dav1d::dav1d::Dav1dRef;
use crate::src::r#ref::Rav1dRef;
use std::ptr;

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: *const u8,
    pub r#ref: *mut Dav1dRef,
}

impl Default for Dav1dUserData {
    fn default() -> Self {
        Self {
            data: ptr::null(),
            r#ref: ptr::null_mut(),
        }
    }
}

impl Dav1dUserData {
    pub(crate) fn into_rust(self) -> Rav1dUserData {
        let Self { data, r#ref } = self;
        Rav1dUserData { data, r#ref }
    }
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dUserData {
    pub data: *const u8,
    pub r#ref: *mut Rav1dRef,
}

impl Default for Rav1dUserData {
    fn default() -> Self {
        Self {
            data: ptr::null(),
            r#ref: ptr::null_mut(),
        }
    }
}

impl Rav1dUserData {
    pub fn into_c(self) -> Dav1dUserData {
        let Self { data, r#ref } = self;
        Dav1dUserData { data, r#ref }
    }
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Dav1dDataProps {
    pub timestamp: i64,
    pub duration: i64,
    pub offset: libc::off_t,
    pub size: usize,
    pub user_data: Dav1dUserData,
}

impl Dav1dDataProps {
    pub(crate) fn into_rust(self) -> Rav1dDataProps {
        let Self {
            timestamp,
            duration,
            offset,
            size,
            user_data,
        } = self;
        Rav1dDataProps {
            timestamp,
            duration,
            offset,
            size,
            user_data: user_data.into_rust(),
        }
    }
}

#[derive(Clone, Default)]
#[repr(C)]
pub(crate) struct Rav1dDataProps {
    pub timestamp: i64,
    pub duration: i64,
    pub offset: libc::off_t,
    pub size: usize,
    pub user_data: Rav1dUserData,
}

impl Rav1dDataProps {
    pub fn into_c(self) -> Dav1dDataProps {
        let Self {
            timestamp,
            duration,
            offset,
            size,
            user_data,
        } = self;
        Dav1dDataProps {
            timestamp,
            duration,
            offset,
            size,
            user_data: user_data.into_c(),
        }
    }
}
