#![deny(unsafe_op_in_unsafe_fn)]

use std::ffi::c_void;
use std::ptr::NonNull;

use crate::c_arc::CArc;
use crate::c_box::{CBox, CRef, FnFree, Free};
use crate::error::{Rav1dError, Rav1dResult};
use crate::include::common::validate::validate_input;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Rav1dData;
use crate::send_sync_non_null::SendSyncNonNull;

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

    #[expect(unused, reason = "should be used soon")]
    pub fn wrap_rust(data: Box<dyn AsRef<[u8]>>) -> Rav1dResult<Self> {
        let data = CRef::Rust(data);
        let data = CArc::wrap(data)?;
        Ok(data.into())
    }

    /// # Safety
    ///
    /// See [`CBox::new`]'s safety for `data`, `free_callback`, `cookie`.
    pub unsafe fn wrap_c(
        data: NonNull<[u8]>,
        free_callback: Option<FnFree>,
        cookie: Option<SendSyncNonNull<c_void>>,
    ) -> Rav1dResult<Self> {
        let free = validate_input!(free_callback.ok_or(Rav1dError::InvalidArgument))?;
        let free = Free { free, cookie };
        // SAFETY: Preconditions delegate to `CBox::new`'s safety.
        let data = unsafe { CBox::new(data, free) };
        let data = CRef::C(data);
        let data = CArc::wrap(data)?;
        Ok(data.into())
    }

    /// # Safety
    ///
    /// See [`CBox::new`]'s safety for `user_data`, `free_callback`, `cookie`.
    pub unsafe fn wrap_user_data(
        &mut self,
        user_data: NonNull<u8>,
        free_callback: Option<FnFree>,
        cookie: Option<SendSyncNonNull<c_void>>,
    ) -> Rav1dResult {
        let free = validate_input!(free_callback.ok_or(Rav1dError::InvalidArgument))?;
        let free = Free { free, cookie };
        // SAFETY: Preconditions delegate to `CBox::new`'s safety.
        let user_data = unsafe { CBox::new(user_data, free) };
        let user_data = CRef::C(user_data);
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
