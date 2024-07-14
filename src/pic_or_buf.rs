use crate::include::dav1d::picture::Rav1dPictureDataComponent;
use crate::src::disjoint_mut::AsMutPtr;
use crate::src::disjoint_mut::DisjointMut;
use crate::src::pixels::Pixels;
use crate::src::strided::Strided;
use crate::src::strided::WithStride;
use crate::src::with_offset::WithOffset;

pub enum PicOrBuf<'a, 'buf, T: AsMutPtr<Target = u8>> {
    Pic(&'a Rav1dPictureDataComponent<'buf>),
    Buf(WithStride<&'a DisjointMut<T>>),
}

/// Manual `impl` since `T: Clone` is not required.
impl<T: AsMutPtr<Target = u8>> Clone for PicOrBuf<'_, '_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: AsMutPtr<Target = u8>> Copy for PicOrBuf<'_, '_, T> {}

impl<T: AsMutPtr<Target = u8>> Pixels for PicOrBuf<'_, '_, T> {
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

impl<T: AsMutPtr<Target = u8>> Strided for PicOrBuf<'_, '_, T> {
    fn stride(&self) -> isize {
        match self {
            Self::Pic(pic) => pic.stride(),
            Self::Buf(buf) => buf.stride(),
        }
    }
}

impl<'a, 'buf, T: AsMutPtr<Target = u8>> WithOffset<PicOrBuf<'a, 'buf, T>> {
    pub fn pic(pic: WithOffset<&'a Rav1dPictureDataComponent<'buf>>) -> Self {
        Self {
            data: PicOrBuf::Pic(pic.data),
            offset: pic.offset,
        }
    }

    pub fn buf(buf: WithOffset<WithStride<&'a DisjointMut<T>>>) -> Self {
        Self {
            data: PicOrBuf::Buf(buf.data),
            offset: buf.offset,
        }
    }
}
