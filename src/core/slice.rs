use crate::{Cami, CamiOrd, CamiPartialEq, CamiPartialOrd, Locality};
use core::cmp::Ordering;

/// Used, for example, for multi-dimensional slices (or arrays/vectors). We also have a similar
/// implementation for `&str`.
impl<T> CamiPartialEq for &[T]
where
    T: PartialEq,
{
    const LOCALITY: Locality = Locality::Both;

    #[must_use]
    #[inline]
    fn eq_local(&self, other: &Self) -> bool {
        self.len() == other.len()
    }

    #[must_use]
    #[inline]
    fn eq_non_local(&self, other: &Self) -> bool {
        self == other
    }
}

// @TODO (not just here, but in the whole crate): Find use cases when we benefit from PartialOrd,
// but we do NOT need (full) Ord

impl<T> CamiPartialOrd for &[T] where T: PartialOrd {}

/// Used, for example, for multi-dimensional slices (or arrays/vectors). We also have a similar
/// implementation for `&str`.
impl<T> CamiOrd for &[T]
where
    T: Ord,
{
    #[must_use]
    #[inline]
    fn cmp_local(&self, other: &Self) -> Ordering {
        self.len().cmp(&other.len())
    }

    #[must_use]
    #[inline]
    fn cmp_non_local(&self, other: &Self) -> Ordering {
        self.cmp(&other)
    }
}

#[cfg(feature = "wrappers")]
pub type SliceCami<'a, T> = Cami<&'a [T]>;
