/// `for` loops aren't allowed in `const fn`s,
/// so loops over integral ranges have to be written using `while` loops.
/// This approximates `for` loops for [`Range`]s.
///
/// [`Range`]: std::ops::Range
macro_rules! const_for {
    ($index:ident in $range:expr, step_by $step:expr => $block:block) => {{
        use std::ops::Range;

        let range: Range<_> = $range; // Make sure it's the right range type.
        let step = $step;
        let mut $index = range.start;
        while $index < range.end {
            $block
            $index += step;
        }
    }};
    ($index:ident in $range:expr => $block:block) => {
        const_for!($index in $range, step_by 1 => $block)
    };
}

pub(crate) use const_for;

macro_rules! const_min {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        if a < b {
            a
        } else {
            b
        }
    }};
}

/// [`std::cmp::min`] is not `const` since it would need `const` `trait` `fn`s,
/// so this implements it with a macro instead so it can remain `const`.
pub(crate) use const_min;
