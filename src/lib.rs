#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "hint_assert_unchecked", feature(hint_assert_unchecked))]
#![cfg_attr(
    not(any(feature = "unsafe", feature = "unsafe_from_rust_source")),
    deny(unsafe_code)
)]
#![cfg_attr(feature = "deref_pure_trait", feature(deref_pure_trait))]

// @TODO in tests-only => dev dependency: use David Tolnay's rust version crate:
/*#cfg[(and(feature = "nightly", arch--...-))]
const NOT_SUPPORTED: () = {
    panic!("NOT_SUPPORTED")
};*/

use core::cmp::Ordering;
pub use slice::Slice;

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod macros_c;
pub mod prelude;
mod primitives;

#[macro_use]
mod pure_local_macros;
mod pure_local_impls;
mod slice;
#[macro_use]
mod macros_s;
mod std_wrap;
mod string;

// TODO compile fail if feature = "nightly" if NOT on nightly

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Locality {
    PureNonLocal,
    PureLocal,
    Both,
}
impl Locality {
    #[inline]
    const fn has_local(&self) -> bool {
        match self {
            Locality::PureNonLocal => false,
            _ => true,
        }
    }

    #[inline]
    const fn has_non_local(&self) -> bool {
        match self {
            Locality::PureLocal => false,
            _ => true,
        }
    }

    #[inline]
    pub const fn debug_reachable_for_local(&self) {
        #[cfg(debug_assertions)]
        if !self.has_local() {
            panic!("Unreachable for 'local_*' functions because of its Locality.");
        }
    }

    #[inline]
    pub const fn debug_reachable_for_non_local(&self) {
        #[cfg(debug_assertions)]
        if !self.has_non_local() {
            panic!("Unreachable for 'non_local_*' functions because of its Locality.");
        }
    }
}

#[cfg(test)]
mod locality_tests {
    use crate::Locality;

    #[test]
    fn methods() {
        assert_eq!(Locality::PureNonLocal.has_local(), false);
        assert_eq!(Locality::PureNonLocal.has_non_local(), true);

        assert_eq!(Locality::PureLocal.has_local(), true);
        assert_eq!(Locality::PureLocal.has_non_local(), false);

        assert_eq!(Locality::Both.has_local(), true);
        assert_eq!(Locality::Both.has_non_local(), true);
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

/// NOT a part of public API.
///
/// The main benefit: With this, we don't need to capture the wrapped type in `c_partial_eq` &
/// `c_ord when we apply those macros to a (`#[repr(transparent)]`) wrapper struct or tuple. See
/// also how we needed `$t_type:ty` (in commit `06cfc12`):
/// <https://github.com/peter-kehl/camigo/blob/06cfc120812179e71a291a92b9c1034a792551eb/src/macros_c.rs#L135>.
///
/// A smaller benefit: Less duplication in `c_partial_eq` & `c_ord` macros: no need for an
/// (anonymous) filler closure.
// This has to return a reference, hence "_ref" in its name.
#[doc(hidden)]
#[inline]
pub fn always_equal_ref<T>(_instance: &T) -> &() {
    &()
}
