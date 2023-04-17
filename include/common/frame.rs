use crate::include::dav1d::headers::Dav1dFrameHeader;


/// Checks whether the frame type is `INTER` or `SWITCH`.
pub fn is_inter_or_switch(frame_header: &Dav1dFrameHeader) -> bool {
    // Both are defined as odd numbers {1, 3} and therefore have the LSB set.
    // See also: AV1 spec 6.8.2
    frame_header.frame_type & 1 != 0
}
