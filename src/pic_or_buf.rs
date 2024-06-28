use crate::include::dav1d::picture::Rav1dPictureDataComponent;
use crate::src::pixels::Pixels;
use crate::src::strided::Strided;

pub enum PicOrBuf<'a, B> {
    Pic(&'a Rav1dPictureDataComponent),
    Buf(B),
}

impl<'a, B: Pixels> Pixels for PicOrBuf<'a, B> {
    fn byte_len(&self) -> usize {
        match self {
            Self::Pic(pic) => pic.byte_len(),
            Self::Buf(buf) => buf.byte_len(),
        }
    }

    fn as_byte_mut_ptr(&self) -> *mut u8 {
        match self {
            Self::Pic(pic) => pic.as_byte_mut_ptr(),
            Self::Buf(buf) => buf.as_byte_mut_ptr(),
        }
    }
}

impl<'a, B: Strided> Strided for PicOrBuf<'a, B> {
    fn stride(&self) -> isize {
        match self {
            Self::Pic(pic) => pic.stride(),
            Self::Buf(buf) => buf.stride(),
        }
    }
}
