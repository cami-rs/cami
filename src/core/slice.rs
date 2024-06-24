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

// @TODO (not just here, but in the whole crate): Find use cases where we benefit from PartialOrd,
// but we do NOT need (full) Ord

impl<T> CamiPartialOrd for &[T]
where
    T: PartialOrd,
{
    #[must_use]
    #[inline]
    fn partial_cmp_local(&self, other: &Self) -> Option<Ordering> {
        // @TODO benchmark if this is faster: Some(self.len().cmp(&other.len()))
        self.len().partial_cmp(&other.len())
    }
    #[must_use]
    #[inline]
    fn partial_cmp_non_local(&self, other: &Self) -> Option<Ordering> {
        // @TODO benchmark if this is faster: Some(self.cmp(other))
        self.partial_cmp(other)
    }

    #[must_use]
    #[inline]
    fn lt_local(&self, other: &Self) -> bool {
        self.len() < other.len()
    }
    #[must_use]
    #[inline]
    fn lt_non_local(&self, other: &Self) -> bool {
        self < other
    }

    #[must_use]
    #[inline]
    fn le_local(&self, other: &Self) -> bool {
        self.len() <= other.len()
    }
    #[must_use]
    #[inline]
    fn le_non_local(&self, other: &Self) -> bool {
        self <= other
    }

    #[must_use]
    #[inline]
    fn gt_local(&self, other: &Self) -> bool {
        self.len() > other.len()
    }
    #[must_use]
    #[inline]
    fn gt_non_local(&self, other: &Self) -> bool {
        self > other
    }

    #[must_use]
    #[inline]
    fn ge_local(&self, other: &Self) -> bool {
        self.len() >= other.len()
    }
    #[must_use]
    #[inline]
    fn ge_non_local(&self, other: &Self) -> bool {
        self >= other
    }
}

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

    /// If T itself were [CamiOrd] (and we use min_specialization - on `nightly` only as of mid
    /// 2024), here we COULD compare [CamiOrd::cmp_local] of all items first, and only then compare
    /// them by [CamiOrd::cmp_non_local]. But, if all items don't git into CPU cache(s), then by the
    /// end of the first run ([CamiOrd::cmp_local]) we'd invalidate the earlier items.
    ///
    /// But/And, if `T`'s impl of [Ord::cmp] is based on [CamiOrd], and if the items between `self`
    /// and `other` vary, then that may short circuit early enough.
    #[must_use]
    #[inline]
    fn cmp_non_local(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

// @TODO search for SliceCami (traits containing this in their name), and update them to use `SliceCami`
//
// @TODO rename to CamiSlice? Or: remove?
pub type SliceCami<'a, T> = Cami<&'a [T]>;

/// We need this for `&str`, even though we have a generic impl for slices.
impl CamiPartialEq for &str {
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

impl CamiPartialOrd for &str {
    #[must_use]
    #[inline]
    fn partial_cmp_local(&self, other: &Self) -> Option<Ordering> {
        // @TODO benchmark if this is faster: Some(self.len().cmp(&other.len()))
        self.len().partial_cmp(&other.len())
    }
    #[must_use]
    #[inline]
    fn partial_cmp_non_local(&self, other: &Self) -> Option<Ordering> {
        // @TODO benchmark if this is faster: Some(self.cmp(other))
        self.partial_cmp(other)
    }

    #[must_use]
    #[inline]
    fn lt_local(&self, other: &Self) -> bool {
        self.len() < other.len()
    }
    #[must_use]
    #[inline]
    fn lt_non_local(&self, other: &Self) -> bool {
        self < other
    }

    #[must_use]
    #[inline]
    fn le_local(&self, other: &Self) -> bool {
        self.len() <= other.len()
    }
    #[must_use]
    #[inline]
    fn le_non_local(&self, other: &Self) -> bool {
        self <= other
    }

    #[must_use]
    #[inline]
    fn gt_local(&self, other: &Self) -> bool {
        self.len() > other.len()
    }
    #[must_use]
    #[inline]
    fn gt_non_local(&self, other: &Self) -> bool {
        self > other
    }

    #[must_use]
    #[inline]
    fn ge_local(&self, other: &Self) -> bool {
        self.len() >= other.len()
    }
    #[must_use]
    #[inline]
    fn ge_non_local(&self, other: &Self) -> bool {
        self >= other
    }
}

/// We need this, even though we have a generic impl for slices in [crate::slices_impls].
impl CamiOrd for &str {
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
