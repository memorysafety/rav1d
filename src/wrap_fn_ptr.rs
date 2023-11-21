use std::marker::PhantomData;

pub trait HasFnPtr {
    type FnPtr;
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WrappedFnPtr<F, T>
where
    T: Clone + Copy + PartialEq + Eq,
{
    fn_ptr: F,
    token: PhantomData<T>,
}

impl<F, T> HasFnPtr for WrappedFnPtr<F, T>
where
    T: Clone + Copy + PartialEq + Eq,
{
    type FnPtr = F;
}

impl<F, T> WrappedFnPtr<F, T>
where
    T: Clone + Copy + PartialEq + Eq,
{
    pub const fn new(fn_ptr: F) -> Self {
        Self {
            fn_ptr,
            token: PhantomData,
        }
    }

    pub const fn get(&self) -> &F {
        &self.fn_ptr
    }
}

macro_rules! wrap_fn_ptr {
    ($vis:vis unsafe extern "C" fn $name:ident(
            $($arg_name:ident: $arg_ty:ty),*$(,)?
    ) -> $return_ty:ty) => {
        $vis mod $name {
            use $crate::src::wrap_fn_ptr::WrappedFnPtr;
            use $crate::src::enum_map::DefaultValue;
            use super::*;

            #[derive(Clone, Copy, PartialEq, Eq)]
            pub struct Token;

            pub type Fn = WrappedFnPtr<unsafe extern "C" fn($($arg_name: $arg_ty),*) -> $return_ty, Token>;

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
