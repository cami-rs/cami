use crate::{CfOrd, CfPartialEq, Locality};
use core::cmp::Ordering;

/// Used, for example, for multi-dimensional slices (or arrays/vectors). We also have a similar implementation for `&str` in [crate::string].
impl<T> CfPartialEq for &[T]
where
    T: PartialEq,
{
    const LOCALITY: Locality = Locality::Both;

    fn eq_local(&self, other: &Self) -> bool {
        self.len() == other.len()
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        self == other
    }
}

// @TODO (not just here, but in the whole crate): Find use cases when we benefit from PartialOrd, but we do NOT need (full) Ord

/// Used, for example, for multi-dimensional slices (or arrays/vectors). We also have a similar implementation for `&str` in [crate::string].
impl<T> CfOrd for &[T]
where
    T: Ord,
{
    fn cmp_local(&self, other: &Self) -> Ordering {
        self.len().cmp(&other.len())
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        self.cmp(&other)
    }

    fn cmp_full(&self, other: &Self) -> Ordering {
        self.len().cmp(&other.len()).then(self.cmp(&other))
    }
}
