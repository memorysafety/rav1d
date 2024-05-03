pub trait OptionError<E> {
    fn err_or<T>(self, ok: T) -> Result<T, E>;
}

impl<E> OptionError<E> for Option<E> {
    #[inline]
    fn err_or<T>(self, ok: T) -> Result<T, E> {
        match self {
            Some(e) => Err(e),
            None => Ok(ok),
        }
    }
}
