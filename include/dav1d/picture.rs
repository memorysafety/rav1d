use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::dav1d::Dav1dRef;
use crate::include::dav1d::headers::DRav1d;
use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dPixelLayout;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dContentLightLevel;
use crate::include::dav1d::headers::Rav1dFrameHeader;
use crate::include::dav1d::headers::Rav1dITUTT35;
use crate::include::dav1d::headers::Rav1dMasteringDisplay;
use crate::include::dav1d::headers::Rav1dPixelLayout;
use crate::include::dav1d::headers::Rav1dSequenceHeader;
use crate::src::error::Dav1dResult;
use crate::src::error::Rav1dResult;
use crate::src::r#ref::Rav1dRef;
use libc::ptrdiff_t;
use libc::uintptr_t;
use std::ffi::c_int;
use std::ffi::c_void;
use std::ptr;
use std::ptr::addr_of_mut;

#[repr(C)]
pub struct Dav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: c_int,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub(crate) struct Rav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Rav1dPixelLayout,
    pub bpc: c_int,
}

impl From<Dav1dPictureParameters> for Rav1dPictureParameters {
    fn from(value: Dav1dPictureParameters) -> Self {
        let Dav1dPictureParameters { w, h, layout, bpc } = value;
        Self {
            w,
            h,
            layout: layout.try_into().unwrap(),
            bpc,
        }
    }
}

impl From<Rav1dPictureParameters> for Dav1dPictureParameters {
    fn from(value: Rav1dPictureParameters) -> Self {
        let Rav1dPictureParameters { w, h, layout, bpc } = value;
        Self {
            w,
            h,
            layout: layout.into(),
            bpc,
        }
    }
}

#[repr(C)]
pub struct Dav1dPicture {
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub data: [*mut c_void; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Dav1dPictureParameters,
    pub m: Dav1dDataProps,
    pub content_light: *mut Rav1dContentLightLevel,
    pub mastering_display: *mut Rav1dMasteringDisplay,
    pub itut_t35: *mut Dav1dITUTT35,
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: *mut Dav1dRef,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub content_light_ref: *mut Dav1dRef,
    pub mastering_display_ref: *mut Dav1dRef,
    pub itut_t35_ref: *mut Dav1dRef,
    pub reserved_ref: [uintptr_t; 4],
    pub r#ref: *mut Dav1dRef,
    pub allocator_data: *mut c_void,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dPicture {
    pub seq_hdr: *mut Rav1dSequenceHeader,
    pub frame_hdr: *mut Rav1dFrameHeader,
    pub data: [*mut c_void; 3],
    pub stride: [ptrdiff_t; 2],
    pub p: Rav1dPictureParameters,
    pub m: Rav1dDataProps,
    pub content_light: *mut Rav1dContentLightLevel,
    pub mastering_display: *mut Rav1dMasteringDisplay,
    pub itut_t35: *mut Rav1dITUTT35,
    pub reserved: [uintptr_t; 4],
    pub frame_hdr_ref: *mut Rav1dRef,
    pub seq_hdr_ref: *mut Rav1dRef,
    pub content_light_ref: *mut Rav1dRef,
    pub mastering_display_ref: *mut Rav1dRef,
    pub itut_t35_ref: *mut Rav1dRef,
    pub reserved_ref: [uintptr_t; 4],
    pub r#ref: *mut Rav1dRef,
    pub allocator_data: *mut c_void,
}

impl From<Dav1dPicture> for Rav1dPicture {
    fn from(value: Dav1dPicture) -> Self {
        let Dav1dPicture {
            seq_hdr,
            frame_hdr,
            data,
            stride,
            p,
            m,
            content_light,
            mastering_display,
            itut_t35,
            reserved,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref,
            r#ref,
            allocator_data,
        } = value;
        assert_eq!(seq_hdr.is_null(), seq_hdr_ref.is_null());
        assert_eq!(frame_hdr.is_null(), frame_hdr_ref.is_null());
        Self {
            // `.update_rav1d()` happens in `#[no_mangle] extern "C"`/`DAV1D_API` calls
            seq_hdr: if seq_hdr.is_null() {
                ptr::null_mut()
            } else {
                unsafe {
                    addr_of_mut!(
                        (*(seq_hdr_ref.read())
                            .data
                            .cast::<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>())
                        .rav1d
                    )
                }
            },
            // `.update_rav1d()` happens in `#[no_mangle] extern "C"`/`DAV1D_API` calls
            frame_hdr: if frame_hdr.is_null() {
                ptr::null_mut()
            } else {
                unsafe {
                    addr_of_mut!(
                        (*(frame_hdr_ref.read())
                            .data
                            .cast::<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>())
                        .rav1d
                    )
                }
            },
            data,
            stride,
            p: p.into(),
            m: m.into(),
            content_light,
            mastering_display,
            // `.update_rav1d()` happens in `#[no_mangle] extern "C"`/`DAV1D_API` calls
            itut_t35: if itut_t35.is_null() {
                ptr::null_mut()
            } else {
                unsafe {
                    addr_of_mut!(
                        (*(itut_t35_ref.read())
                            .data
                            .cast::<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>())
                        .rav1d
                    )
                }
            },
            reserved,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref,
            r#ref,
            allocator_data,
        }
    }
}

impl From<Rav1dPicture> for Dav1dPicture {
    fn from(value: Rav1dPicture) -> Self {
        let Rav1dPicture {
            seq_hdr,
            frame_hdr,
            data,
            stride,
            p,
            m,
            content_light,
            mastering_display,
            itut_t35,
            reserved,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref,
            r#ref,
            allocator_data,
        } = value;
        assert_eq!(seq_hdr.is_null(), seq_hdr_ref.is_null());
        assert_eq!(frame_hdr.is_null(), frame_hdr_ref.is_null());
        Self {
            // `DRav1d::from_rav1d` is called right after [`parse_seq_hdr`].
            seq_hdr: if seq_hdr.is_null() {
                ptr::null_mut()
            } else {
                unsafe {
                    addr_of_mut!(
                        (*(seq_hdr_ref.read())
                            .data
                            .cast::<DRav1d<Rav1dSequenceHeader, Dav1dSequenceHeader>>())
                        .dav1d
                    )
                }
            },
            // `DRav1d::from_rav1d` is called in [`parse_frame_hdr`].
            frame_hdr: if frame_hdr.is_null() {
                ptr::null_mut()
            } else {
                unsafe {
                    addr_of_mut!(
                        (*(frame_hdr_ref.read())
                            .data
                            .cast::<DRav1d<Rav1dFrameHeader, Dav1dFrameHeader>>())
                        .dav1d
                    )
                }
            },
            data,
            stride,
            p: p.into(),
            m: m.into(),
            content_light,
            mastering_display,
            // `DRav1d::from_rav1d` is called in [`rav1d_parse_obus`].
            itut_t35: if itut_t35.is_null() {
                ptr::null_mut()
            } else {
                unsafe {
                    addr_of_mut!(
                        (*(itut_t35_ref.read())
                            .data
                            .cast::<DRav1d<Rav1dITUTT35, Dav1dITUTT35>>())
                        .dav1d
                    )
                }
            },
            reserved,
            frame_hdr_ref,
            seq_hdr_ref,
            content_light_ref,
            mastering_display_ref,
            itut_t35_ref,
            reserved_ref,
            r#ref,
            allocator_data,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Dav1dPicAllocator {
    pub cookie: *mut c_void,
    pub alloc_picture_callback:
        Option<unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> Dav1dResult>,
    pub release_picture_callback:
        Option<unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> ()>,
}

#[derive(Clone)]
#[repr(C)]
pub(crate) struct Rav1dPicAllocator {
    pub cookie: *mut c_void,
    pub alloc_picture_callback:
        Option<unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> Dav1dResult>,
    pub release_picture_callback:
        Option<unsafe extern "C" fn(*mut Dav1dPicture, *mut c_void) -> ()>,
}

impl From<Dav1dPicAllocator> for Rav1dPicAllocator {
    fn from(value: Dav1dPicAllocator) -> Self {
        let Dav1dPicAllocator {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        } = value;
        Self {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        }
    }
}

impl From<Rav1dPicAllocator> for Dav1dPicAllocator {
    fn from(value: Rav1dPicAllocator) -> Self {
        let Rav1dPicAllocator {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        } = value;
        Self {
            cookie,
            alloc_picture_callback,
            release_picture_callback,
        }
    }
}

impl Rav1dPicAllocator {
    pub unsafe fn alloc_picture(&mut self, p: *mut Rav1dPicture) -> Rav1dResult {
        let mut p_c = p.read().into();
        let result = self
            .alloc_picture_callback
            .expect("non-null function pointer")(&mut p_c, self.cookie);
        p.write(p_c.into());
        result.try_into().unwrap()
    }

    pub unsafe fn release_picture(&mut self, p: *mut Rav1dPicture) {
        let mut p_c = p.read().into();
        self.release_picture_callback
            .expect("non-null function pointer")(&mut p_c, self.cookie);
        p.write(p_c.into());
    }
}
