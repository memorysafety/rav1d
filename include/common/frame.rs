use crate::include::dav1d::headers::Rav1dFrameHeader;

/// Checks whether the frame type is `INTER` or `SWITCH`.
pub(crate) fn is_inter_or_switch(frame_header: &Rav1dFrameHeader) -> bool {
    // Both are defined as odd numbers {1, 3} and therefore have the LSB set.
    // See also: AV1 spec 6.8.2
    frame_header.frame_type & 1 != 0
}

/// Checks whether Dav1dFrameType == KEY || == INTRA
/// See also: AV1 spec 6.8.2
pub(crate) fn is_key_or_intra(frame_header: &Rav1dFrameHeader) -> bool {
    !is_inter_or_switch(frame_header)
}
