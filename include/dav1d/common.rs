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

impl From<Dav1dUserData> for Rav1dUserData {
    fn from(value: Dav1dUserData) -> Self {
        let Dav1dUserData { data, r#ref } = value;
        Self { data, r#ref }
    }
}

impl From<Rav1dUserData> for Dav1dUserData {
    fn from(value: Rav1dUserData) -> Self {
        let Rav1dUserData { data, r#ref } = value;
        Self { data, r#ref }
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

#[derive(Clone, Default)]
#[repr(C)]
pub(crate) struct Rav1dDataProps {
    pub timestamp: i64,
    pub duration: i64,
    pub offset: libc::off_t,
    pub size: usize,
    pub user_data: Rav1dUserData,
}

impl From<Dav1dDataProps> for Rav1dDataProps {
    fn from(value: Dav1dDataProps) -> Self {
        let Dav1dDataProps {
            timestamp,
            duration,
            offset,
            size,
            user_data,
        } = value;
        Self {
            timestamp,
            duration,
            offset,
            size,
            user_data: user_data.into(),
        }
    }
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
            user_data: user_data.into(),
        }
    }
}
