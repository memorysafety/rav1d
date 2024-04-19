use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;

pub trait EnumKey<const N: usize>: Sized + Copy {
    const VALUES: [Self; N];

    fn as_usize(self) -> usize;
}

/// This is a `const` version of [`Default::default`].
/// `trait` `fn`s can't be `const` (yet),
/// but we can make this an associated `const`.
pub trait DefaultValue {
    const DEFAULT: Self;
}

impl<T> DefaultValue for Option<T> {
    const DEFAULT: Self = None;
}

/// A map from an `enum` key `K` to `V`s.
/// `N` is the number of possible `enum` values.
pub struct EnumMap<K, V, const N: usize>
where
    K: EnumKey<N>,
{
    array: [V; N],
    _phantom: PhantomData<K>,
}

// Has to be a macro until we have `#![feature(generic_const_exprs)]`.
macro_rules! enum_map_ty {
    ($K:ty, $V:ty) => {
        $crate::src::enum_map::EnumMap<$K, $V, { <$K as ::strum::EnumCount>::COUNT }>
    }
}

pub(crate) use enum_map_ty;

impl<K, V, const N: usize> EnumMap<K, V, N>
where
    K: EnumKey<N>,
{
    /// Create an [`EnumMap`] from an existing array
    /// where the array's indices correspond to `K`'s values `as usize`.
    pub const fn new(array: [V; N]) -> Self {
        Self {
            array,
            _phantom: PhantomData,
        }
    }
}

impl<K, V, const N: usize> EnumMap<K, V, N>
where
    K: EnumKey<N>,
    V: DefaultValue,
{
    /// Create an [`EnumMap`] with default values when `V: ` [`DefaultValue`].
    #[allow(dead_code)] // TODO(kkysen) remove when used
    const fn default() -> Self {
        Self {
            array: [V::DEFAULT; N],
            _phantom: PhantomData,
        }
    }
}

impl<K, V, const N: usize> Index<K> for EnumMap<K, V, N>
where
    K: EnumKey<N>,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.array[index.as_usize()]
    }
}

impl<K, V, const N: usize> IndexMut<K> for EnumMap<K, V, N>
where
    K: EnumKey<N>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.array[index.as_usize()]
    }
}

/// Create an [`EnumMap`] where `V: ` [`DefaultValue`]
/// using a `match` from `K` to `V`.
///
/// The [`DefaultValue::DEFAULT`] is not actually ever used,
/// but needs to exist to be able to
/// create the array safely and at `const` time.
///
/// [`MaybeUninit`] can do this without [`DefaultValue`]
/// by using `unsafe` initialization,
/// but it also doesn't yet work in `const` contexts.
///
/// [`MaybeUninit`]: std::mem::MaybeUninit
macro_rules! enum_map {
    ($K:ty => $V:ty; match key { $($t:tt)* }) => {{
        use $crate::src::enum_map::EnumKey;
        use $crate::src::enum_map::EnumMap;

        let mut a = [<$V>::DEFAULT; <$K>::VALUES.len()];
        let mut i = 0;
        while i < <$K>::VALUES.len() {
            let key = <$K>::VALUES[i];
            use $K::*;
            let value = match key { $($t)* };
            a[key as usize] = value;
            i += 1;
        }
        EnumMap::<$K, $V, { <$K>::VALUES.len() }>::new(a)
    }};
}

pub(crate) use enum_map;
