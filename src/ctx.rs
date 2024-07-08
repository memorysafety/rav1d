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
//! [`BlockContext`]: crate::src::env::BlockContext
use crate::src::disjoint_mut::AsMutPtr;
use crate::src::disjoint_mut::DisjointMut;
use std::iter::zip;

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
#[inline]
pub fn small_memset<T: Clone + Copy, const N: usize, const WITH_DEFAULT: bool>(
    buf: &mut [T],
    val: T,
) {
    fn as_array<T: Clone + Copy, const N: usize>(buf: &mut [T]) -> &mut [T; N] {
        buf.try_into().unwrap()
    }
    if N == 0 {
        if WITH_DEFAULT {
            buf.fill(val)
        }
    } else {
        assert!(buf.len() == N); // Meant to be optimized out.
        *as_array(buf) = [val; N];
    }
}

pub trait CaseSetter {
    fn set<T: Clone + Copy>(&self, buf: &mut [T], val: T);

    /// # Safety
    ///
    /// Caller must ensure that no elements of the written range are concurrently
    /// borrowed (immutably or mutably) at all during the call to `set_disjoint`.
    fn set_disjoint<T, V>(&self, buf: &DisjointMut<T>, val: V)
    where
        T: AsMutPtr<Target = V>,
        V: Clone + Copy;
}

pub struct CaseSetterN<const N: usize, const WITH_DEFAULT: bool> {
    offset: usize,
    len: usize,
}

impl<const N: usize, const WITH_DEFAULT: bool> CaseSetterN<N, WITH_DEFAULT> {
    const fn len(&self) -> usize {
        if N == 0 {
            self.len
        } else {
            N
        }
    }
}

impl<const N: usize, const WITH_DEFAULT: bool> CaseSetter for CaseSetterN<N, WITH_DEFAULT> {
    #[inline]
    fn set<T: Clone + Copy>(&self, buf: &mut [T], val: T) {
        small_memset::<_, N, WITH_DEFAULT>(&mut buf[self.offset..][..self.len()], val);
    }

    /// # Safety
    ///
    /// Caller must ensure that no elements of the written range are concurrently
    /// borrowed (immutably or mutably) at all during the call to `set_disjoint`.
    #[inline]
    fn set_disjoint<T, V>(&self, buf: &DisjointMut<T>, val: V)
    where
        T: AsMutPtr<Target = V>,
        V: Clone + Copy,
    {
        let mut buf = buf.index_mut((self.offset.., ..self.len()));
        small_memset::<_, N, WITH_DEFAULT>(&mut *buf, val);
    }
}

/// Rank-2 polymorphic closures aren't a thing in Rust yet,
/// so we need to emulate this through a generic trait with a generic method.
/// Unforunately, this means we have to write the closure sugar manually.
pub trait SetCtx<T> {
    fn call<S: CaseSetter>(self, case: &S, ctx: T) -> Self;
}

/// Emulate a closure for a [`SetCtx`] `impl`.
macro_rules! set_ctx {
    (
        // `||` is used instead of just `|` due to this bug: <https://github.com/rust-lang/rustfmt/issues/6228>.
        ||
            $($lifetime:lifetime,)?
            $case:ident,
            $ctx:ident: $T:ty,
            // Note that the required trailing `,` is so `:expr` can precede `|`.
            $($up_var:ident: $up_var_ty:ty$( = $up_var_val:expr)?,)*
        || $body:block
    ) => {{
        use $crate::src::ctx::SetCtx;
        use $crate::src::ctx::CaseSetter;

        struct F$(<$lifetime>)? {
            $($up_var: $up_var_ty,)*
        }

        impl$(<$lifetime>)? SetCtx<$T> for F$(<$lifetime>)? {
            fn call<S: CaseSetter>(self, $case: &S, $ctx: $T) -> Self {
                let Self {
                    $($up_var,)*
                } = self;
                $body
                // We destructure and re-structure `Self` so that we
                // can move out of refs without using `ref`/`ref mut`,
                // which I don't know how to match on in a macro.
                Self {
                    $($up_var,)*
                }
            }
        }

        F {
            $($up_var$(: $up_var_val)?,)*
        }
    }};
}

pub(crate) use set_ctx;

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
    #[inline]
    pub fn one<T, F>(ctx: T, len: usize, offset: usize, set_ctx: F) -> F
    where
        F: SetCtx<T>,
    {
        macro_rules! set_ctx {
            ($N:literal) => {
                set_ctx.call(&CaseSetterN::<$N, WITH_DEFAULT> { offset, len }, ctx)
            };
        }
        match len {
            01 if UP_TO >= 01 => set_ctx!(01),
            02 if UP_TO >= 02 => set_ctx!(02),
            04 if UP_TO >= 04 => set_ctx!(04),
            08 if UP_TO >= 08 => set_ctx!(08),
            16 if UP_TO >= 16 => set_ctx!(16),
            32 if UP_TO >= 32 => set_ctx!(32),
            64 if UP_TO >= 64 => set_ctx!(64),
            _ => set_ctx!(0),
        }
    }

    /// Perform many case sets in one call.
    ///
    /// This allows specifying the `set_ctx` closure inline easily,
    /// and also allows you to group the same args together.
    ///
    /// The `lens`, `offsets`, and `dirs` are zipped and passed to [`CaseSet::one`],
    /// where `dirs` can be an array of any type and whose elements are passed back to the `set_ctx` closure.
    #[inline]
    pub fn many<T, F, const N: usize>(
        dirs: [T; N],
        lens: [usize; N],
        offsets: [usize; N],
        mut set_ctx: F,
    ) where
        F: SetCtx<T>,
    {
        for (dir, (len, offset)) in zip(dirs, zip(lens, offsets)) {
            set_ctx = Self::one(dir, len, offset, set_ctx);
        }
    }
}
