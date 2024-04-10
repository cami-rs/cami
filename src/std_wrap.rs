use crate::{CfOrd, CfPartialEq};
use core::cmp::Ordering;

/// A (zero cost/low cost) wrapper & bridge that implements [PartialEq] forwarding to [CfPartialEq]
/// and [Ord] forwarding to [CfOrd] of `T`.
///
/// These implementations are useful, and for many data types it may speed up searches etc.
///
/// NO cache benefit for [Slice]'s cache-aware methods (`binary_search_cf` etc.) themselves!
///
/// Usable for BENCHMARKING. It allows us to run slice's `binary_search`:
/// `<[T]>::binary_search(self, given)` using the full comparison, and benchmark against cache-aware
/// [Slice]'s `binary_search_cf` etc.
///
/// [PartialEq] is implemented NOT by forwarding to [PartialEq::eq] and [PartialEq::ne] of `T`, but
/// by forwarding to[CfOrd::cmp_local] and [CfOrd::cmp_non_local] of `T` instead. (Hence `T` itself
/// doesn't need to be [PartialEq] or [Ord].)
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct StdWrapXX<T> {
    t: T,
}

wrapper!{ StdWrap <T> T}

impl<T: CfPartialEq> PartialEq for StdWrap<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (T::LOCALITY.no_local() || self.t.eq_local(&other.t))
            && (T::LOCALITY.no_non_local() || self.t.eq_non_local(&other.t))
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        T::LOCALITY.has_local() && !self.t.eq_local(&other.t)
            || T::LOCALITY.has_non_local() && !self.t.eq_non_local(&other.t)
    }
}
impl<T: CfOrd> Eq for StdWrap<T> {}

impl<T: CfOrd> PartialOrd for StdWrap<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.t.cmp_full(&other.t))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.t.cmp_full(&other.t) == Ordering::Less
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.t.cmp_full(&other.t) != Ordering::Greater
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.t.cmp_full(&other.t) == Ordering::Greater
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.t.cmp_full(&other.t) != Ordering::Less
    }
}
impl<T: CfOrd> Ord for StdWrap<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp_full(&other.t)
    }
    /*
    // Provided methods
    fn max(self, other: Self) -> Self
       where Self: Sized {}
    fn min(self, other: Self) -> Self
       where Self: Sized {}
    fn clamp(self, min: Self, max: Self) -> Self
       where Self: Sized + PartialOrd {}
    */
}
