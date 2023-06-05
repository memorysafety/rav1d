use std::iter::zip;

pub struct CaseSetter<const UP_TO: usize, const WITH_DEFAULT: bool> {
    offset: usize,
    len: usize,
}

impl<const UP_TO: usize, const WITH_DEFAULT: bool> CaseSetter<UP_TO, WITH_DEFAULT> {
    #[inline]
    pub fn set<T: Clone + Copy>(&self, buf: &mut [T], val: T) {
        buf[self.offset..][..self.len].fill(val);
    }
}

pub struct CaseSet<const UP_TO: usize, const WITH_DEFAULT: bool>;

impl<const UP_TO: usize, const WITH_DEFAULT: bool> CaseSet<UP_TO, WITH_DEFAULT> {
    #[inline]
    pub fn one<T, F>(ctx: T, len: usize, offset: usize, mut set_ctx: F)
    where
        F: FnMut(&CaseSetter<UP_TO, WITH_DEFAULT>, T),
    {
        set_ctx(&CaseSetter { offset, len }, ctx);
    }

    #[inline]
    pub fn many<T, F, const N: usize>(
        dirs: [T; N],
        lens: [usize; N],
        offsets: [usize; N],
        mut set_ctx: F,
    ) where
        F: FnMut(&CaseSetter<UP_TO, WITH_DEFAULT>, T),
    {
        for (dir, (len, offset)) in zip(dirs, zip(lens, offsets)) {
            Self::one(dir, len, offset, &mut set_ctx);
        }
    }
}
