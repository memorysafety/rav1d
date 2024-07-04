//! The [`case_set!`] macro is a safe and simplified version of the `case_set*` macros in `ctx.h`.
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
//! 
//! TODO: benchmark new implementation against old one?
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

/// Fill small ranges of buffers with a value.
/// 
/// This is effectively a specialized version [`slice::fill`] for small
/// power-of-two sized ranges of buffers.
/// 
/// `$UP_TO` is the maximum length that will be optimized, with powers of two up
/// to 64 supported.
/// 
/// If `$WITH_DEFAULT` is `true`, then the implementation falls back to
/// [`slice::fill`] for buffer lengths not a power of two or greater than
/// `$UP_TO`. Otherwise no operation is performed in these cases.
/// 
/// # Examples
/// 
/// ```
/// # use rav1d::case_set;
/// let mut buf = [0u8; 32];
/// let len = 16;
/// for offset in [0, 16] {
///     case_set!(32, len, offset, {
///         set!(&mut buf, 1u8);
///     });
/// }
/// ```
/// 
/// In the simplest case, `$len` is the length of the buffer range to fill
/// starting from `$offset`. The `$body` block is executed with `len` and
/// `offset` identifiers set to the given length and offset values. Within the
/// body a `set!` macro is available and must be called to set each buffer range
/// to a value. `set!` takes a buffer and a value and sets the range
/// `buf[offset..][..len]` to the value.
/// ```
/// # macro_rules! set {
/// #     ($buf:expr, $val:expr) => {};
/// # }
/// set!(buf, value);
/// ```
/// 
/// ## Naming parameters
/// 
/// The identifier for either or both of `len` and `offset` can be overridden by
/// specifying `identifer=value` for those parameters:
/// ```
/// # use rav1d::case_set;
/// let mut buf = [0u8; 32];
/// let outer_len = 16;
/// for outer_offset in [0, 16] {
///     case_set!(32, len=outer_len, offset=outer_offset, {
///         set!(&mut buf, (offset+len) as u8);
///     });
/// }
/// ```
/// 
/// ## `DisjointMut` buffers
/// 
/// [`DisjointMut`] buffers can be used in basically the same way as normal
/// buffers but using the `set_disjoint!` macro instead of `set!`.
/// ```
/// # use rav1d::case_set;
/// # use rav1d::src::disjoint_mut::DisjointMut;
/// let mut buf = DisjointMut::new([0u8; 32]);
/// let len = 16;
/// for offset in [0, 16] {
///     case_set!(32, len, offset, {
///         set_disjoint!(&mut buf, 1u8);
///     });
/// }
/// ```
/// 
/// ## Multiple buffer ranges
/// 
/// Multiple buffers with different lengths and offsets can be filled with the
/// same body statements. In the following example, two buffers with different
/// sizes are initialized by quarters.
/// ```
/// # use rav1d::case_set;
/// let mut buf1 = [0u8; 32];
/// let mut buf2 = [0u8; 64];
/// for offset in [0, 8, 16, 24] {
///     case_set!(16, buf=[&mut buf1[..], &mut buf2[..]], len=[8, 16], offset=[offset, offset*2], {
///         set!(buf, len as u8 >> 3);
///     });
/// }
/// ```
/// 
/// A more realistic example of filling multiple buffers with the same value is
/// initializing different struct fields at the same time (from
/// `src/decode.rs`):
/// ```ignore
/// case_set!(32, ctx=[(&t.l, 1), (&f.a[t.a], 0)], len=[bh4, bw4], offset=[by4, bx4], {
///      let (dir, dir_index) = ctx;
///      set_disjoint!(dir.seg_pred, seg_pred.into());
///      set_disjoint!(dir.skip_mode, b.skip_mode);
///      set_disjoint!(dir.intra, 0);
///      set_disjoint!(dir.skip, b.skip);
///      set_disjoint!(dir.pal_sz, 0);
/// });
/// ```
/// 
/// [`DisjointMut`]: crate::src::disjoint_mut::DisjointMut
#[macro_export]
macro_rules! case_set {
    ($UP_TO:literal, $($WITH_DEFAULT:literal,)? $ctx:ident=[$($ctx_expr:expr),*], $len:ident=[$($len_expr:expr),*], $offset:ident=[$($offset_expr:expr),*], $body:block) => {
        for ($ctx, ($len, $offset)) in core::iter::zip([$($ctx_expr,)*], core::iter::zip([$($len_expr,)*], [$($offset_expr,)*])) {
            case_set!($UP_TO, $($WITH_DEFAULT,)? $ctx=$ctx, $len=$len, $offset=$offset, $body);
        }
    };
    ($UP_TO:literal, $($WITH_DEFAULT:literal,)? $len:ident, $offset:ident, $body:block) => {
        case_set!($UP_TO, $($WITH_DEFAULT,)? ctx=(), $len=$len, $offset=$offset, $body);
    };
    ($UP_TO:literal, $($WITH_DEFAULT:literal,)? $len:ident=$len_expr:expr, $offset:ident, $body:block) => {
        case_set!($UP_TO, $($WITH_DEFAULT,)? ctx=(), $len=$len_expr, $offset=$offset, { $(($buf, $val)),* });
    };
    ($UP_TO:literal, $($WITH_DEFAULT:literal,)? $len:ident, $offset:ident=$offset_expr:expr, $body:block) => {
        case_set!($UP_TO, $($WITH_DEFAULT,)? ctx=(), $len=$len, $offset=$offset_expr, $body);
    };
    ($UP_TO:literal, $($WITH_DEFAULT:literal,)? $len:ident=$len_expr:expr, $offset:ident=$offset_expr:expr, $body:block) => {
        case_set!($UP_TO, $($WITH_DEFAULT,)? ctx=(), $len=$len_expr, $offset=$offset_expr, $body);
    };
    ($UP_TO:literal, $($WITH_DEFAULT:literal,)? $ctx:ident=$ctx_expr:expr, $len:ident=$len_expr:expr, $offset:ident=$offset_expr:expr, $body:block) => {
        let $ctx = $ctx_expr;
        let $len = $len_expr;
        let $offset = $offset_expr;
        {
            #[allow(unused_macros)]
            macro_rules! set {
                ($buf:expr, $val:expr) => {
                    assert!($offset <= $buf.len() && $offset + $len <= $buf.len());
                };
            }
            #[allow(unused_macros)]
            macro_rules! set_disjoint {
                ($buf:expr, $val:expr) => {
                    assert!($offset <= $buf.len() && $offset + $len <= $buf.len());
                };
            }
            $body
        }
        macro_rules! exec_block {
            ($N:literal, $block:block) => {
                {
                    #[allow(unused_macros)]
                    macro_rules! set {
                        ($buf:expr, $val:expr) => {
                            // SAFETY: The offset and length are checked by the
                            // assert outside of the match.
                            let buf_range = unsafe {
                                $buf.get_unchecked_mut($offset..$offset+$N)
                            };
                            *<&mut [_; $N]>::try_from(buf_range).unwrap() = [$val; $N];
                        };
                    }
                    #[allow(unused_macros)]
                    macro_rules! set_disjoint {
                        ($buf:expr, $val:expr) => {
                            // SAFETY: The offset and length are checked by the
                            // assert outside of the match.
                            let mut buf_range = unsafe {
                                $buf.index_mut_unchecked(($offset.., ..$N))
                            };
                            *<&mut [_; $N]>::try_from(&mut *buf_range).unwrap() = [$val; $N];
                        };
                    }
                    $block
                }
            };
        }
        match $len {
            01 if $UP_TO >= 01 => exec_block!(01, $body),
            02 if $UP_TO >= 02 => exec_block!(02, $body),
            04 if $UP_TO >= 04 => exec_block!(04, $body),
            08 if $UP_TO >= 08 => exec_block!(08, $body),
            16 if $UP_TO >= 16 => exec_block!(16, $body),
            32 if $UP_TO >= 32 => exec_block!(32, $body),
            64 if $UP_TO >= 64 => exec_block!(64, $body),
            _ => {
                if $($WITH_DEFAULT ||)? false {
                    #[allow(unused_macros)]
                    macro_rules! set {
                        ($buf:expr, $val:expr) => {
                            $buf[$offset..][..$len].fill($val);
                        };
                    }
                    #[allow(unused_macros)]
                    macro_rules! set_disjoint {
                        ($buf:expr, $val:expr) => {
                            $buf.index_mut($offset..$offset+$len).fill($val);
                        };
                    }
                    $body
                }
            }
        }
    };
}
pub use case_set;