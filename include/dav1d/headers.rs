#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dSegmentationData {
    pub delta_q: libc::c_int,
    pub delta_lf_y_v: libc::c_int,
    pub delta_lf_y_h: libc::c_int,
    pub delta_lf_u: libc::c_int,
    pub delta_lf_v: libc::c_int,
    pub ref_0: libc::c_int,
    pub skip: libc::c_int,
    pub globalmv: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [libc::c_int; 2],
    pub ref_delta: [libc::c_int; 8],
}
