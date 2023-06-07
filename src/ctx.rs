use std::iter::zip;

#[inline]
fn small_memset<T: Clone + Copy, const UP_TO: usize, const WITH_DEFAULT: bool>(
    buf: &mut [T],
    val: T,
) {
    fn as_array<T: Clone + Copy, const N: usize>(buf: &mut [T]) -> &mut [T; N] {
        buf.try_into().unwrap()
    }
    match buf.len() {
        01 if UP_TO >= 01 => *as_array(buf) = [val; 01],
        02 if UP_TO >= 02 => *as_array(buf) = [val; 02],
        04 if UP_TO >= 04 => *as_array(buf) = [val; 04],
        08 if UP_TO >= 08 => *as_array(buf) = [val; 08],
        16 if UP_TO >= 16 => *as_array(buf) = [val; 16],
        32 if UP_TO >= 32 => *as_array(buf) = [val; 32],
        64 if UP_TO >= 64 => *as_array(buf) = [val; 64],
        _ => {
            if WITH_DEFAULT {
                buf.fill(val)
            }
        }
    }
}

pub struct CaseSetter<const UP_TO: usize, const WITH_DEFAULT: bool> {
    offset: usize,
    len: usize,
}

impl<const UP_TO: usize, const WITH_DEFAULT: bool> CaseSetter<UP_TO, WITH_DEFAULT> {
    #[inline]
    pub fn set<T: Clone + Copy>(&self, buf: &mut [T], val: T) {
        small_memset::<T, UP_TO, WITH_DEFAULT>(&mut buf[self.offset..][..self.len], val);
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
