/// Iterate through `iter` starting at index `start`
/// and then wrapping around to `start` again.
pub fn wrapping_iter<I, T>(iter: I, start: usize) -> impl Iterator<Item = T>
where
    I: Iterator<Item = T> + Clone,
{
    iter.clone().skip(start).chain(iter.take(start))
}
