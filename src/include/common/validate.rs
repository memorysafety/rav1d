use std::any::type_name;
use std::process::abort;

fn type_name_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

pub fn parent_type_name_of<T>(t: &T) -> &'static str {
    let name = type_name_of(&t);
    let name = name.strip_prefix("&").unwrap();
    let name = name.strip_suffix("::f").unwrap();
    name
}

pub fn debug_abort() {
    if cfg!(debug_assertions) {
        abort();
    }
}

macro_rules! func_name {
    () => {{
        fn f() {}
        $crate::include::common::validate::parent_type_name_of(&f)
    }};
}

pub(crate) use func_name;

pub trait ValidatedIntoResult<T, E> {
    fn into_result(self) -> Result<T, E>;
}

impl<T, E> ValidatedIntoResult<T, E> for Result<T, E> {
    fn into_result(self) -> Result<T, E> {
        self
    }
}

impl<E> ValidatedIntoResult<(), E> for (bool, E) {
    fn into_result(self) -> Result<(), E> {
        let (ok, e) = self;
        if ok {
            Ok(())
        } else {
            Err(e)
        }
    }
}

impl ValidatedIntoResult<(), ()> for bool {
    fn into_result(self) -> Result<(), ()> {
        (self, ()).into_result()
    }
}

macro_rules! validate_input {
    ($condition:expr, $block:block) => {{
        use $crate::include::common::validate::debug_abort;
        use $crate::include::common::validate::func_name;
        use $crate::include::common::validate::ValidatedIntoResult;

        // Needs to be outside of the closure.
        let func_name = func_name!();

        $condition.into_result().map_err(|e| {
            eprintln!(
                "Input validation check `{}` failed in `fn {}` in `{}:{}:{}`!",
                stringify!($condition),
                func_name,
                file!(),
                line!(),
                column!(),
            );
            $block;
            debug_abort();
            e
        })
    }};

    ($condition:expr) => {
        validate_input!($condition, {})
    };
}

pub(crate) use validate_input;
