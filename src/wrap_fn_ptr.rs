/// A `trait` to extract the `fn` ptr type of a [`WrappedFnPtr`].
pub trait HasFnPtr {
    type FnPtr;
}

/// A newtype wrapped `fn` ptr.
///
/// This allows us to add a safer (type-safe for sure, and increasingly fully safe)
/// wrapper around calling a `fn` ptr.
#[repr(transparent)]
pub struct WrappedFnPtr<F>(F);

impl<F> HasFnPtr for WrappedFnPtr<F> {
    type FnPtr = F;
}

impl<F> WrappedFnPtr<F> {
    pub const fn new(fn_ptr: F) -> Self {
        Self(fn_ptr)
    }

    pub const fn get(&self) -> &F {
        &self.0
    }
}

/// Declare a newtype wrapper for a `fn` ptr
/// and define related, useful items for that `fn` ptr (see below).
/// Given a `fn` signature with no body,
/// this generates a `mod` with the name of the `fn` provided that contains:
///
/// * `type Fn`:
///     A [`WrappedFnPtr`] wrapping the `fn` ptr signature provided.
///     This is a newtype wrapper for the purpose of implementing methods on.
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
/// [`DefaultValue`]: crate::src::enum_map::DefaultValue
/// [`enum_map!`]: crate::src::enum_map::enum_map
/// [`bd_fn!`]: crate::include::common::bitdepth::bd_fn
macro_rules! wrap_fn_ptr {
    ($vis:vis unsafe extern "C" fn $name:ident(
            $($arg_name:ident: $arg_ty:ty),*$(,)?
    ) -> $return_ty:ty) => {
        $vis mod $name {
            use $crate::src::wrap_fn_ptr::WrappedFnPtr;
            use $crate::src::enum_map::DefaultValue;
            use super::*;

            pub type Fn = WrappedFnPtr<unsafe extern "C" fn($($arg_name: $arg_ty),*) -> $return_ty>;

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

                    $name::Fn::new($fn_name)
                }};
            }

            #[cfg(feature = "asm")]
            #[allow(unused_imports)]
            pub(crate) use decl_fn;
        }
    };
}

pub(crate) use wrap_fn_ptr;
