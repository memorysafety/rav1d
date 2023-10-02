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

macro_rules! validate_input {
    ($condition:expr, $error:expr, $block:block) => {{
        match $condition {
            true => Ok(()),
            false => {
                let func_name = $crate::include::common::validate::func_name!();
                eprintln!(
                    "Input validation check '{}' failed in {}!",
                    stringify!($condition),
                    func_name
                );
                $block;
                $crate::include::common::validate::debug_abort();
                Err($error)
            }
        }
    }};

    ($condition:expr, $error:expr) => {
        validate_input!($condition, $error, {})
    };

    ($condition:expr) => {
        validate_input!($condition, ())
    };
}

pub(crate) use validate_input;
