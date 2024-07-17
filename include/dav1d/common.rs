use crate::src::c_arc::CArc;
use crate::src::c_arc::RawCArc;
use std::ptr::NonNull;

#[derive(Default)]
#[repr(C)]
pub struct Dav1dUserData {
    pub data: Option<NonNull<u8>>,
    pub r#ref: Option<RawCArc<u8>>, // opaque, so we can change this
}

pub(crate) type Rav1dUserData = Option<CArc<u8>>;

impl From<Dav1dUserData> for Rav1dUserData {
    fn from(value: Dav1dUserData) -> Self {
        let Dav1dUserData { data: _, r#ref } = value;
        r#ref.map(|r#ref| {
            // SAFETY: `r#ref` came from `CArc::into_raw`.
            unsafe { CArc::from_raw(r#ref) }
        })
    }
}

impl From<Rav1dUserData> for Dav1dUserData {
    fn from(value: Rav1dUserData) -> Self {
        Self {
            data: value.as_ref().map(|user_data| user_data.as_ref().into()),
            r#ref: value.map(|user_data| user_data.into_raw()),
        }
    }
}

#[derive(Default)]
#[repr(C)]
pub struct Dav1dDataProps {
    pub timestamp: i64,
    pub duration: i64,
    pub offset: libc::off_t,
    pub size: usize,
    pub user_data: Dav1dUserData,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dDataProps {
    pub timestamp: i64,
    pub duration: i64,
    pub offset: libc::off_t,
    pub size: usize,
    pub user_data: Rav1dUserData,
}

impl Default for Rav1dDataProps {
    fn default() -> Self {
        Self {
            timestamp: i64::MIN,
            duration: 0,
            offset: -1,
            size: 0,
            user_data: Default::default(),
        }
    }
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

impl From<Rav1dDataProps> for Dav1dDataProps {
    fn from(value: Rav1dDataProps) -> Self {
        let Rav1dDataProps {
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
