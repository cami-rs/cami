//#![no_std]
#![feature(hint_assert_unchecked)]

pub use cf_wrap::*;
use core::cmp::Ordering;
pub use slice::Slice;
pub use std_wrap::*;

mod cf_wrap;
#[macro_use]
mod pure_local_macros;
mod pure_local_impls;
mod slice;
#[macro_use]
mod std_macros;
mod std_wrap;
mod string;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Locality {
    PureNonLocal,
    PureLocal,
    Both,
}
impl Locality {
    const fn has_local(&self) -> bool {
        match self {
            Locality::PureNonLocal => false,
            _ => true,
        }
    }
    const fn no_local(&self) -> bool {
        !self.has_local()
    }

    const fn has_non_local(&self) -> bool {
        match self {
            Locality::PureLocal => false,
            _ => true,
        }
    }
    const fn no_non_local(&self) -> bool {
        !self.has_non_local()
    }
}
#[cfg(test)]
mod locality_tests {
    use crate::Locality;

    #[test]
    fn methods() {
        assert_eq!(Locality::PureNonLocal.has_local(), false);
        assert_eq!(Locality::PureNonLocal.no_local(), true);
        assert_eq!(Locality::PureNonLocal.has_non_local(), true);
        assert_eq!(Locality::PureNonLocal.no_non_local(), false);

        assert_eq!(Locality::PureLocal.has_local(), true);
        assert_eq!(Locality::PureLocal.no_local(), false);
        assert_eq!(Locality::PureLocal.has_non_local(), false);
        assert_eq!(Locality::PureLocal.no_non_local(), true);

        assert_eq!(Locality::Both.has_local(), true);
        assert_eq!(Locality::Both.no_local(), false);
        assert_eq!(Locality::Both.has_non_local(), true);
        assert_eq!(Locality::Both.no_non_local(), false);
    }
}

pub trait CPartialEq {
    const LOCALITY: Locality;

    // If unsure, then it's `false`.
    //
    //const COMPATIBLE_WITH_PARTIAL_EQ: bool;

    fn eq_local(&self, other: &Self) -> bool;

    fn eq_non_local(&self, other: &Self) -> bool;

    fn eq_full(&self, other: &Self) -> bool {
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

/** Cache-friendly ordering.
 *
 *  NOT extending [Ord], because they MAY be INCOMPATIBLE.
 */
pub trait COrd: CPartialEq {
    // If unsure, then it's `false`.
    //
    //const COMPATIBLE_WITH_ORD: bool;

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
    /// respects [COrd::LOCALITY] and calls [COrd::cmp_local] and/or [COrd::cmp_non_local] only
    /// when they're applicable and when they're needed.
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
