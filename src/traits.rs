use camigo_helpers::Locality;
use core::cmp::Ordering;

/// Cache-friendly comparison.
///
/// NOT extending [PartialEq], because a type (that implements [CamiPartialEq]) may not implement
/// [PartialEq]. But, if the type does implement [PartialEq], then [CamiPartialEq::eq_full] should
/// return same result as [PartialEq::eq].
pub trait CamiPartialEq {
    /// Which of "local_*" and "non_local_*" methods apply (which ones have custom logic) here & in
    /// [CamiOrd]. Used to short circuit any unneeded parts in the default implementation of
    /// "full_*" methods here & in [CamiOrd].
    const LOCALITY: Locality;

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

/// Cache-friendly ordering. NOT extending [Ord] (or [PartialOrd]):
/// 1. because [CamiOrd] MAY be INCOMPATIBLE with those two traits ([CamiOrd::cmp_full] MAY differ
///    to [Ord::cmp].) And, it's exactly types where those two functions DO differ, where `Camigo`
///    hopes to be useful. Also
/// 2. because a type that implements [CamiOrd] may not implement [Ord] (or [PartialOrd]).
pub trait CamiOrd: CamiPartialEq {
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
    /// respects [CamiPartialOrd::LOCALITY] and calls [CamiOrd::cmp_local] and/or
    /// [CamiOrd::cmp_non_local] only when they're applicable and when they're needed.
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
