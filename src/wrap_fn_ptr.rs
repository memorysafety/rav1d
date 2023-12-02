/// Declare a newtype wrapper for a `fn` ptr
/// and define related, useful items for that `fn` ptr (see below).
/// Given a `fn` signature with no body,
/// this generates a `mod` with the name of the `fn` provided that contains:
///
/// * `type FnPtr`:
///     The raw, inner `fn` ptr (according to the provided signature) contained by `Fn`.
///
/// * `type Fn`:
///     A newtype wrapping `FnPtr`.
///     It defines `const fn new(FnPtr) -> Self` to construct it
///     and `const fn get(&self) -> &FnPtr` to read the `FnPtr`.
///
///     These methods are marked `pub(super)`
///     as they are meant to be used in the module calling [`wrap_fn_ptr!`].
///
///     It is meant for a `fn call` method to also be implemented
///     for this type to allow users to call the `fn`
///     in a type-safe (e.x. [`BitDepth`]-wise)
///     and generally safer (memory safety-wise) way.
///
/// * `impl ` [`DefaultValue`] ` for Fn`:
///     A `const`-compatible default implementation of `Fn`
///     that just calls [`unimplemented!`].
///     This lets `Fn` be used by [`enum_map!`] without wrapping it in an [`Option`],
///     and removes any need for an [`Option::unwrap`] check,
///     as the check is moved to inside the `fn` call.
///
/// * `decl_fn!`:
///     A macro that, given a `fn $fn_name:ident`,
///     declares an `extern "C" fn` with the `fn` signature provided.
///     This macro can and is meant to be used with [`bd_fn!`].
///
/// This ensures that the `fn` signature is consistent between all of these
/// and reduces the need to repeat the `fn` signature many times.
///
/// [`BitDepth`]: crate::include::common::bitdepth::BitDepth
/// [`DefaultValue`]: crate::src::enum_map::DefaultValue
/// [`enum_map!`]: crate::src::enum_map::enum_map
/// [`bd_fn!`]: crate::include::common::bitdepth::bd_fn
macro_rules! wrap_fn_ptr {
    ($vis:vis unsafe extern "C" fn $name:ident(
            $($arg_name:ident: $arg_ty:ty),*$(,)?
    ) -> $return_ty:ty) => {
        $vis mod $name {
            use $crate::src::enum_map::DefaultValue;
            use super::*;

            pub type FnPtr = unsafe extern "C" fn($($arg_name: $arg_ty),*) -> $return_ty;

            /// A newtype wrapped [`FnPtr`].
            ///
            /// This allows us to add a safer
            /// (type-safe for sure, and increasingly fully safe)
            /// interface for calling a `fn` ptr.
            #[derive(Clone, Copy, PartialEq, Eq)]
            #[repr(transparent)]
            pub struct Fn(FnPtr);

            impl Fn {
                pub(super) const fn new(fn_ptr: FnPtr) -> Self {
                    Self(fn_ptr)
                }

                pub(super) const fn get(&self) -> &FnPtr {
                    &self.0
                }
            }

            impl DefaultValue for Fn {
                const DEFAULT: Self = {
                    extern "C" fn default_unimplemented(
                        $($arg_name: $arg_ty,)*
                    ) -> $return_ty {
                        $(let _ = $arg_name;)*
                        unimplemented!()
                    }
                    Self::new(default_unimplemented)
                };
            }

            #[cfg(feature = "asm")]
            #[allow(unused_macros)]
            macro_rules! decl_fn {
                (fn $fn_name:ident) => {{
                    extern "C" {
                        fn $fn_name($($arg_name: $arg_ty,)*) -> $return_ty;
                    }

                    self::$name::Fn::new($fn_name)
                }};
            }

            #[cfg(feature = "asm")]
            #[allow(unused_imports)]
            pub(crate) use decl_fn;
        }
    };
}

pub(crate) use wrap_fn_ptr;
