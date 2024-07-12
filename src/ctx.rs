//! The [`case_set!`] macro is a safe and simplified version of the `case_set*`
//! macros in `ctx.h`.
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
//! We also want to avoid multiple switches when setting a group of buffers as
//! the C implementation did, which was implemented in
//! https://github.com/memorysafety/rav1d/pull/1293.
//!
//! # Benchmarks
//!
//! Comparing this implementation to the previous implementation of `CaseSet` we
//! see an 8.2-10.5% speedup for a single buffer, a 5.9-7.0% speedup for
//! multiple buffers, and a minor improvement to multiple [`DisjointMut`]
//! buffers (which happened to be well-optimized in the previous
//! implementation).
//!
//! [`BlockContext`]: crate::src::env::BlockContext
//! [`DisjointMut`]: crate::src::disjoint_mut::DisjointMut

/// Fill small ranges of buffers with a value.
///
/// This is effectively a specialized version [`slice::fill`] for small
/// power-of-two sized ranges of buffers.
///
/// `$UP_TO` is the maximum length that will be optimized, with powers of two up
/// to 64 supported. If the buffer length is not a power of two or greater than
/// `$UP_TO`, this macro will do nothing. See [`case_set_with_default!`] to fill
/// buffers with non-comforming lengths if needed.
///
/// # Examples
///
/// ```
/// # use rav1d::case_set;
/// let mut buf = [0u8; 32];
/// let len = 16;
/// for offset in [0, 16] {
///     case_set!(up_to = 32, len, offset, {
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
///     case_set!(
///         up_to = 32,
///         len=outer_len,
///         offset=outer_offset,
///         {
///             set!(&mut buf, (offset+len) as u8);
///         }
///     );
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
///     case_set!(up_to = 32, len, offset, {
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
///     case_set!(
///         up_to = 16,
///         buf = [&mut buf1[..], &mut buf2[..]],
///         len = [8, 16],
///         offset = [offset, offset*2],
///         {
///             set!(buf, len as u8 >> 3);
///         }
///     );
/// }
/// ```
///
/// A more realistic example of filling multiple buffers with the same value is
/// initializing different struct fields at the same time (from
/// `src/decode.rs`):
/// ```ignore
/// case_set!(
///     up_to = 32,
///     ctx = [(&t.l, 1), (&f.a[t.a], 0)],
///     len = [bh4, bw4],
///     offset = [by4, bx4],
///     {
///         let (dir, dir_index) = ctx;
///         set_disjoint!(dir.seg_pred, seg_pred.into());
///         set_disjoint!(dir.skip_mode, b.skip_mode);
///         set_disjoint!(dir.intra, 0);
///         set_disjoint!(dir.skip, b.skip);
///         set_disjoint!(dir.pal_sz, 0);
///     }
/// );
/// ```
///
/// [`DisjointMut`]: crate::src::disjoint_mut::DisjointMut
macro_rules! case_set {
    (up_to=$UP_TO:literal, $(@DEFAULT=$WITH_DEFAULT:literal,)? $ctx:ident=[$($ctx_expr:expr),* $(,)?], $len:ident=[$($len_expr:expr),* $(,)?], $offset:ident=[$($offset_expr:expr),* $(,)?], $body:block) => {
        let ctxs = [$($ctx_expr,)*];
        let lens = [$($len_expr,)*];
        let offsets = [$($offset_expr,)*];
        assert_eq!(ctxs.len(), lens.len());
        assert_eq!(ctxs.len(), offsets.len());
        for (i, ctx) in ctxs.into_iter().enumerate() {
            case_set!(up_to=$UP_TO, $(@DEFAULT=$WITH_DEFAULT,)? $ctx=ctx, $len=lens[i], $offset=offsets[i], $body);
        }
    };
    (up_to=$UP_TO:literal, $(@DEFAULT=$WITH_DEFAULT:literal,)? $len:ident, $offset:ident, $body:block) => {
        case_set!(up_to=$UP_TO, $(@DEFAULT=$WITH_DEFAULT,)? _ctx=(), $len=$len, $offset=$offset, $body);
    };
    (up_to=$UP_TO:literal, $(@DEFAULT=$WITH_DEFAULT:literal,)? $len:ident=$len_expr:expr, $offset:ident=$offset_expr:expr, $body:block) => {
        case_set!(up_to=$UP_TO, $(@DEFAULT=$WITH_DEFAULT,)? _ctx=(), $len=$len_expr, $offset=$offset_expr, $body);
    };
    (up_to=$UP_TO:literal, $(@DEFAULT=$WITH_DEFAULT:literal,)? $ctx:ident=$ctx_expr:expr, $len:ident=$len_expr:expr, $offset:ident=$offset_expr:expr, $body:block) => {
        #[allow(unused_mut)]
        let mut $ctx = $ctx_expr;
        let $len = $len_expr;
        let $offset = $offset_expr;
        {
            #[allow(unused_macros)]
            macro_rules! set {
                ($buf:expr, $val:expr) => {{
                    assert!($offset <= $buf.len() && $offset + $len <= $buf.len());
                }};
            }
            #[allow(unused_imports)]
            use set as set_disjoint;
            #[allow(unused)]
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
                        ($buf:expr, $val:expr) => {{
                            // SAFETY: The offset and length are checked by the
                            // assert outside of the match.
                            let mut buf_range = unsafe {
                                $buf.index_mut_unchecked(($offset.., ..$N))
                            };
                            *<&mut [_; $N]>::try_from(&mut *buf_range).unwrap() = [$val; $N];
                        }};
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
                        ($buf:expr, $val:expr) => {{
                            // SAFETY: The offset and length are checked by the
                            // assert outside of the match.
                            let buf_range = unsafe {
                                $buf.get_unchecked_mut($offset..$offset+$len)
                            };
                            buf_range.fill($val);
                        }};
                    }
                    #[allow(unused_macros)]
                    macro_rules! set_disjoint {
                        ($buf:expr, $val:expr) => {{
                            // SAFETY: The offset and length are checked by the
                            // assert outside of the match.
                            let mut buf_range = unsafe {
                                $buf.index_mut_unchecked(($offset.., ..$len))
                            };
                            buf_range.fill($val);
                        }};
                    }
                    $body
                }
            }
        }
    };
}
pub(crate) use case_set;

/// Fill small ranges of buffers with a value.
///
/// `$UP_TO` is the maximum length that will be optimized, with powers of two up
/// to 64 supported. If the buffer length is not a power of two or greater than
/// `$UP_TO`, this macro will still fill the buffer with a slower fallback.
///
/// See [`case_set!`] for examples and more documentation.
macro_rules! case_set_with_default {
    (up_to=$UP_TO:literal, $($tt:tt)*) => {
        $crate::src::ctx::case_set!(up_to=$UP_TO, @DEFAULT=true, $($tt)*);
    };
}
pub(crate) use case_set_with_default;
