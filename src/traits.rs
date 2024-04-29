use crate::Locality;
use core::cmp::Ordering;

/// Cache-friendly comparison.
///
/// NOT extending [PartialEq], because a type (that implements [CamiPartialEq]) may not implement
/// [PartialEq]. (Or, the type may be suitable for implementing/deriving [PartialEq], but the author
/// chose not to/forgot to do so.)
///
/// But, if the type does implement [PartialEq], too, then [CamiPartialEq::eq_full] should return
/// same result as [PartialEq::eq].
///
/// If the type does implement/derive [Eq], then its [CamiPartialEq] implementation SHOULD be
/// compatible with its [Eq] implementation.
pub trait CamiPartialEq<Rhs: ?Sized = Self> {
    //@TODO GENERIC
    /// Which of "local_*" and "non_local_*" methods apply (which ones have custom logic) here & in
    /// [CamiOrd]. Used to short circuit any unneeded parts in the default implementation of
    /// "full_*" methods here & in [CamiOrd].
    const LOCALITY: Locality;

    fn eq_local(&self, other: &Rhs) -> bool;
    fn eq_non_local(&self, other: &Rhs) -> bool;

    #[inline]
    fn eq_full(&self, other: &Rhs) -> bool {
        // @TODO write a test that the following would return the same
        //
        // Write them not in this crate, but in Camigo crate - for example, next to the
        // implementation for `bool`.
        if false {
            return (!Self::LOCALITY.has_local() || self.eq_local(&other))
                && (!Self::LOCALITY.has_non_local() || self.eq_non_local(&other));
        }
        if Self::LOCALITY.has_local() {
            let local = self.eq_local(other);
            if local {
                Self::LOCALITY.has_non_local() || self.eq_non_local(other)
            } else {
                false
            }
        } else {
            self.eq_non_local(other)
        }
    }
}

pub trait CamiPartialOrd<Rhs: ?Sized = Self>: CamiPartialEq {
    // Required methods
    fn partial_cmp_local(&self, other: &Rhs) -> Option<Ordering> {
        todo!()
    }
    fn partial_cmp_non_local(&self, other: &Rhs) -> Option<Ordering> {
        todo!()
    }

    // Provided methods. If possible, do implement them, rather than relying on partial_cmp_*.
    // Implementing them may speed up [core -> primitive slice
    // sort_unstable*()](https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.sort_unstable)
    // and its
    // [binary_search*()](https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.binary_search),
    // and stable sort in std: [std -> primitive slice -> sort() and sort*()].
    #[inline]
    fn lt_local(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_local(other), Some(Ordering::Less))
    }
    #[inline]
    fn lt_non_local(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_non_local(other), Some(Ordering::Less))
    }
    #[inline]
    fn le_local(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_local(other),
            Some(Ordering::Less | Ordering::Equal)
        )
    }
    #[inline]
    fn le_non_local(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_non_local(other),
            Some(Ordering::Less | Ordering::Equal)
        )
    }
    #[inline]
    fn gt_local(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_local(other), Some(Ordering::Greater))
    }
    #[inline]
    fn gt_non_local(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_non_local(other), Some(Ordering::Greater))
    }
    #[inline]
    fn ge_local(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_local(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
    #[inline]
    fn ge_non_local(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_non_local(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
}

/// Cache-friendly ordering. NOT extending [Ord] (or [PartialOrd]):
/// 1. because [CamiOrd] MAY be INCOMPATIBLE with those two traits ([CamiOrd::cmp_full] MAY differ
///    to [Ord::cmp].) And, it's exactly types where those two functions DO differ, where `Camigo`
///    hopes to be useful. Also
/// 2. because a type that implements [CamiOrd] may not implement [Ord] (or [PartialOrd]).
///
/// Unlike `Ord`, this trait doesn't have any `max/min/clamp`-like methods.
pub trait CamiOrd: Eq + CamiPartialOrd {
    /// Comparison based on local (non-referenced) field(s) only (if any).
    ///
    /// Result must be [Ordering::Equal] or the same as the result of [cmp_full].
    ///
    /// Any implementation must NOT call [cmp_full] (whether directly or indirectly).
    fn cmp_local(&self, other: &Self) -> Ordering;

    /// Comparison based on non-local (referenced) field(s) only (if any).
    ///
    /// Any implementation must NOT call [cmp_full] (whether directly or indirectly).
    fn cmp_non_local(&self, other: &Self) -> Ordering;

    /// Full comparison.
    ///
    /// Any implementation must be equivalent to the default one. The default implementation
    /// respects [CamiPartialOrd::LOCALITY] and calls [CamiOrd::cmp_local] and/or
    /// [CamiOrd::cmp_non_local] only when they're applicable and when they're needed.
    #[inline]
    fn cmp_full(&self, other: &Self) -> Ordering {
        // @TODO apply https://rust.godbolt.org/z/698eYffTx
        if Self::LOCALITY.has_local() {
            let local = self.cmp_local(other);
            if local == Ordering::Equal {
                if Self::LOCALITY.has_non_local() {
                    self.cmp_non_local(other)
                } else {
                    Ordering::Equal
                }
            } else {
                local
            }
        } else {
            self.cmp_non_local(other)
        }
    }
}
