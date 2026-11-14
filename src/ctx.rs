//! The [`CaseSet`] API below is a safe and simplified version of the `case_set*` macros in `ctx.h`.
//!
//! The `case_set*` macros themselves replaced `memset`s in order to further optimize them
//! (in e3b5d4d044506f9e0e95e79b3de42fd94386cc61,
//! which performed the following optimizations:
//! 1. larger, inlinable writes for small power of 2 lengths
//! 2. aligned writes for the above
//! 3. 8-byte aligned fields [`BlockContext`]
//!   * allows 8-byte aligned writes
//!   * better cache boundary alignment
//!
//! (1) is easy to preserve in Rust, but (2) is difficult to do so without overhead,
//! as unaligned writes are UB, and so we'd need to check at runtime if they're aligned
//! (a runtime-determined `off`set is used, so we can't reasonably ensure this at compile-time).
//!
//! To more thoroughly check this, I ran the same benchmarks done in
//! e3b5d4d044506f9e0e95e79b3de42fd94386cc61, which introduced the `case_set*` macros:
//!
//! ```sh
//! cargo build --release && hyperfine './target/release/dav1d -i ./tests/large/chimera_8b_1080p.ivf -l 1000 -o /dev/null'
//! ```
//!
//! for 3 implementations:
//! 1. the original `case_set*` macros translated directly to `unsafe` Rust `fn`s
//! 2. the safe [`CaseSet`] implementation below using [`small_memset`] with its small powers of 2 optimization
//! 3. a safe [`CaseSet`] implementation using [`slice::fill`]/`memset` only
//!
//! The [`small_memset`] version was ~1.27% faster than the `case_set*` one,
//! and ~3.26% faster than the `memset` one.
//! The `case_set*` macros were also faster than `memset` in C by a similar margin,
//! meaning the `memset` option is the slowest in both C and Rust,
//! and since it was replaced with `case_set*` in C, we shouldn't use it in Rust.
//! Thus, the [`small_memset`] implementation seems optimal, as it:
//! * is the fastest of the Rust implementations
//! * is completely safe
//! * employs the same small powers of 2 optimization the `case_set*` implementation did
//! * is far simpler than the `case_set*` implementation, consisting of a `match` and array writes
//!
//! [`BlockContext`]: crate::env::BlockContext
use std::iter::zip;

use crate::disjoint_mut::{AsMutPtr, DisjointMut};

/// Perform a `memset` optimized for lengths that are small powers of 2.
///
/// For power of 2 lengths `<= UP_TO`,
/// the `memset` is done as an array write of that exactly (compile-time known) length.
/// If the length is not a power of 2 or `> UP_TO`,
/// then the `memset` is done by [`slice::fill`] (a `memset` call) if `WITH_DEFAULT` is `true`,
/// or else skipped if `WITH_DEFAULT` is `false`.
///
/// This optimizes for the common cases where `buf.len()` is a small power of 2,
/// where the array write is optimized as few and large stores as possible.
#[inline(always)]
pub fn small_memset<T: Clone + Copy, const UP_TO: usize, const WITH_DEFAULT: bool>(
    buf: &mut [T],
    val: T,
) {
    fn as_array<T: Clone + Copy, const N: usize>(buf: &mut [T]) -> &mut [T; N] {
        buf.try_into().unwrap()
    }
    match LenClass::new(buf.len()) {
        LenClass::Len01 if UP_TO >= 01 => *as_array(buf) = [val; 01],
        LenClass::Len02 if UP_TO >= 02 => *as_array(buf) = [val; 02],
        LenClass::Len04 if UP_TO >= 04 => *as_array(buf) = [val; 04],
        LenClass::Len08 if UP_TO >= 08 => *as_array(buf) = [val; 08],
        LenClass::Len16 if UP_TO >= 16 => *as_array(buf) = [val; 16],
        LenClass::Len32 if UP_TO >= 32 => *as_array(buf) = [val; 32],
        _ => {
            if WITH_DEFAULT {
                buf.fill(val)
            }
        }
    }
}

/// The lengths that [`CaseSet`] specializes for.
#[derive(Clone, Copy)]
#[repr(u8)]
enum LenClass {
    Len01,
    Len02,
    Len04,
    Len08,
    Len16,
    Len32,
    Other,
}

/// [`LenClass`] of each length up to 32.
static LEN_CLASS: [LenClass; 33] = {
    let mut classes = [LenClass::Other; 33];
    classes[01] = LenClass::Len01;
    classes[02] = LenClass::Len02;
    classes[04] = LenClass::Len04;
    classes[08] = LenClass::Len08;
    classes[16] = LenClass::Len16;
    classes[32] = LenClass::Len32;
    classes
};

impl LenClass {
    /// Classify `len` through a small table instead of `match`ing on it directly.
    ///
    /// A `match` on the sparse powers of 2 is lowered to a power of 2 check
    /// plus a count of trailing zeros in front of the jump table,
    /// whereas this is a single bounds check and load.
    #[inline(always)]
    fn new(len: usize) -> Self {
        match LEN_CLASS.get(len) {
            Some(&class) => class,
            None => Self::Other,
        }
    }
}

pub struct CaseSetter<const UP_TO: usize, const WITH_DEFAULT: bool> {
    offset: usize,
    len: usize,
}

impl<const UP_TO: usize, const WITH_DEFAULT: bool> CaseSetter<UP_TO, WITH_DEFAULT> {
    #[inline(always)]
    pub fn set<T: Clone + Copy>(&self, buf: &mut [T], val: T) {
        small_memset::<T, UP_TO, WITH_DEFAULT>(&mut buf[self.offset..][..self.len], val);
    }

    /// # Safety
    ///
    /// Caller must ensure that no elements of the written range are concurrently
    /// borrowed (immutably or mutably) at all during the call to `set_disjoint`.
    /// The same case (i.e. the same length), but at a different offset.
    ///
    /// This lets several buffers that are set with the same length
    /// but at different offsets share a single dispatch on the length.
    #[inline(always)]
    pub fn with_offset(&self, offset: usize) -> Self {
        Self {
            offset,
            len: self.len,
        }
    }

    #[inline(always)]
    pub fn set_disjoint<T, V>(&self, buf: &DisjointMut<T>, val: V)
    where
        T: AsMutPtr<Target = V>,
        V: Clone + Copy,
    {
        let mut buf = buf.index_mut(self.offset..self.offset + self.len);
        small_memset::<V, UP_TO, WITH_DEFAULT>(&mut *buf, val);
    }
}

/// The entrypoint to the [`CaseSet`] API.
///
/// `UP_TO` and `WITH_DEFAULT` are made const generic parameters rather than have multiple `case_set*` `fn`s,
/// and these are put in a separate `struct` so that these 2 generic parameters
/// can be manually specified while the ones on the methods are inferred.
pub struct CaseSet<const UP_TO: usize, const WITH_DEFAULT: bool>;

impl<const UP_TO: usize, const WITH_DEFAULT: bool> CaseSet<UP_TO, WITH_DEFAULT> {
    /// Perform one case set.
    ///
    /// This API is generic over the element type (`T`) rather than hardcoding `u8`,
    /// as sometimes other types are used, though only `i8` is used currently.
    ///
    /// The `len` and `offset` are supplied here and
    /// applied to each `buf` passed to [`CaseSetter::set`] in `set_ctx`.
    #[inline(always)]
    pub fn one<T, F>(ctx: T, len: usize, offset: usize, mut set_ctx: F)
    where
        F: FnMut(&CaseSetter<UP_TO, WITH_DEFAULT>, T),
    {
        Self::one_by_ref(ctx, len, offset, &mut set_ctx);
    }

    /// Call `set_ctx` directly (and not through `impl FnMut for &mut F`)
    /// so that an `#[inline(always)]` on the closure is respected.
    #[inline(always)]
    fn one_by_ref<T, F>(ctx: T, len: usize, offset: usize, set_ctx: &mut F)
    where
        F: FnMut(&CaseSetter<UP_TO, WITH_DEFAULT>, T),
    {
        // Dispatch on `len` once here so that, once `set_ctx` is inlined into each arm,
        // every `CaseSetter::set*` call in it has a constant `len`,
        // rather than each of them `match`ing on `len` separately.
        macro_rules! arm {
            ($len:literal) => {
                set_ctx(&CaseSetter { offset, len: $len }, ctx)
            };
        }
        match LenClass::new(len) {
            LenClass::Len01 if UP_TO >= 01 => arm!(01),
            LenClass::Len02 if UP_TO >= 02 => arm!(02),
            LenClass::Len04 if UP_TO >= 04 => arm!(04),
            LenClass::Len08 if UP_TO >= 08 => arm!(08),
            LenClass::Len16 if UP_TO >= 16 => arm!(16),
            LenClass::Len32 if UP_TO >= 32 => arm!(32),
            _ => {
                if WITH_DEFAULT {
                    set_ctx(&CaseSetter { offset, len }, ctx)
                }
            }
        }
    }

    /// Perform many case sets in one call.
    ///
    /// This allows specifying the `set_ctx` closure inline easily,
    /// and also allows you to group the same args together.
    ///
    /// The `lens`, `offsets`, and `dirs` are zipped and passed to [`CaseSet::one`],
    /// where `dirs` can be an array of any type and whose elements are passed back to the `set_ctx` closure.
    #[inline(always)]
    pub fn many<T, F, const N: usize>(
        dirs: [T; N],
        lens: [usize; N],
        offsets: [usize; N],
        mut set_ctx: F,
    ) where
        F: FnMut(&CaseSetter<UP_TO, WITH_DEFAULT>, T),
    {
        let mut iter = zip(dirs, zip(lens, offsets));
        // Manually unroll the first iterations (`N` is 2 in practice),
        // so that each `dir` gets its own specialized copy of `set_ctx`,
        // like the `case_set` macro in C does.
        if let Some((dir, (len, offset))) = iter.next() {
            Self::one_by_ref(dir, len, offset, &mut set_ctx);
        }
        if let Some((dir, (len, offset))) = iter.next() {
            Self::one_by_ref(dir, len, offset, &mut set_ctx);
        }
        for (dir, (len, offset)) in iter {
            Self::one_by_ref(dir, len, offset, &mut set_ctx);
        }
    }

    /// Like [`Self::many`], but each length is given as its base-2 logarithm,
    /// i.e. the length is `1 << log2_len`.
    ///
    /// Block dimensions are always powers of 2 and their logarithms are readily available,
    /// so dispatching on the logarithm is a single dense jump table,
    /// instead of first having to check that the length is a power of 2.
    #[inline(always)]
    pub fn many_log2<T, F, const N: usize>(
        dirs: [T; N],
        log2_lens: [u8; N],
        offsets: [usize; N],
        mut set_ctx: F,
    ) where
        F: FnMut(&CaseSetter<UP_TO, WITH_DEFAULT>, T),
    {
        let mut iter = zip(dirs, zip(log2_lens, offsets));
        if let Some((dir, (log2_len, offset))) = iter.next() {
            Self::one_by_ref_log2(dir, log2_len, offset, &mut set_ctx);
        }
        if let Some((dir, (log2_len, offset))) = iter.next() {
            Self::one_by_ref_log2(dir, log2_len, offset, &mut set_ctx);
        }
        for (dir, (log2_len, offset)) in iter {
            Self::one_by_ref_log2(dir, log2_len, offset, &mut set_ctx);
        }
    }

    /// Like [`Self::one`], but the length is given as its base-2 logarithm,
    /// i.e. the length is `1 << log2_len`.
    #[inline(always)]
    pub fn one_log2<T, F>(ctx: T, log2_len: u8, offset: usize, mut set_ctx: F)
    where
        F: FnMut(&CaseSetter<UP_TO, WITH_DEFAULT>, T),
    {
        Self::one_by_ref_log2(ctx, log2_len, offset, &mut set_ctx);
    }

    #[inline(always)]
    fn one_by_ref_log2<T, F>(ctx: T, log2_len: u8, offset: usize, set_ctx: &mut F)
    where
        F: FnMut(&CaseSetter<UP_TO, WITH_DEFAULT>, T),
    {
        macro_rules! arm {
            ($len:literal) => {
                set_ctx(&CaseSetter { offset, len: $len }, ctx)
            };
        }
        match log2_len {
            0 if UP_TO >= 01 => arm!(01),
            1 if UP_TO >= 02 => arm!(02),
            2 if UP_TO >= 04 => arm!(04),
            3 if UP_TO >= 08 => arm!(08),
            4 if UP_TO >= 16 => arm!(16),
            5 if UP_TO >= 32 => arm!(32),
            _ => {
                if WITH_DEFAULT {
                    let len = 1usize.checked_shl(log2_len.into()).unwrap();
                    set_ctx(&CaseSetter { offset, len }, ctx)
                }
            }
        }
    }
}
