/// `for` loops aren't allowed in `const fn`s,
/// so loops over integral ranges have to be written using `while` loops.
/// This approximates `for` loops for [`Range`]s.
///
/// [`Range`]: std::ops::Range
macro_rules! const_for {
    ($index:ident in $range:expr => $block:block) => {{
        use std::ops::Range;

        let range: Range<_> = $range; // Make sure it's the right range type.
        let mut $index = range.start;
        while $index < range.end {
            $block
            $index += 1;
        }
    }};
}

pub(crate) use const_for;
