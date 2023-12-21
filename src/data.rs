use crate::include::common::validate::validate_input;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Rav1dData;
use crate::src::c_arc::CArc;
use crate::src::c_box::CBox;
use crate::src::c_box::FnFree;
use crate::src::c_box::Free;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dResult;
use std::ffi::c_void;
use std::ptr::NonNull;

impl From<CArc<[u8]>> for Rav1dData {
    fn from(data: CArc<[u8]>) -> Self {
        let size = data.len();
        Self {
            data: Some(data),
            m: Rav1dDataProps {
                size,
                ..Default::default()
            },
        }
    }
}

impl Rav1dData {
    pub fn create(size: usize) -> Rav1dResult<Self> {
        let data = CArc::zeroed_slice(size)?;
        Ok(data.into())
    }

    pub fn wrap(
        data: NonNull<[u8]>,
        free_callback: Option<FnFree>,
        cookie: *mut c_void,
    ) -> Rav1dResult<Self> {
        let free = validate_input!(free_callback.ok_or(EINVAL))?;
        let free = Free { free, cookie };
        let data = CBox::from_c(data, free);
        let data = CArc::wrap(data)?;
        Ok(data.into())
    }

    pub fn wrap_user_data(
        &mut self,
        user_data: NonNull<u8>,
        free_callback: Option<FnFree>,
        cookie: *mut c_void,
    ) -> Rav1dResult {
        let free = validate_input!(free_callback.ok_or(EINVAL))?;
        let free = Free { free, cookie };
        let user_data = CBox::from_c(user_data, free);
        let user_data = CArc::wrap(user_data)?;
        self.m.user_data = Some(user_data);
        Ok(())
    }
}

impl AsRef<[u8]> for Rav1dData {
    fn as_ref(&self) -> &[u8] {
        match &self.data {
            Some(data) => data.as_ref(),
            None => &[],
        }
    }
}
