pub trait HasFnPtr {
    type FnPtr;
}

#[repr(transparent)]
pub struct WrappedFnPtr<F>(F);

impl<F> HasFnPtr for WrappedFnPtr<F> {
    type FnPtr = F;
}

#[allow(dead_code)]
impl<F> WrappedFnPtr<F> {
    pub const fn new(fn_ptr: F) -> Self {
        Self(fn_ptr)
    }

    pub const fn get(&self) -> &F {
        &self.0
    }
}

#[allow(unused_macros)]
macro_rules! wrap_fn_ptr {
    ($vis:vis struct $Wrapper:ident(
        unsafe extern "C" fn(
            $($arg_name:ident: $arg_ty:ty),*$(,)?
        ) -> $return_ty:ty
    )) => {
        $vis type $Wrapper = WrappedFnPtr<unsafe extern "C" fn($($arg_name: $arg_ty),*) -> $return_ty>;

        impl DefaultValue for $Wrapper {
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
    };
}

#[allow(unused_imports)]
pub(crate) use wrap_fn_ptr;
