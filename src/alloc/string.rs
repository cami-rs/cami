use crate as camigo; // For macros
use crate::{Cami, CamiOrd, CamiPartialEq, CamiPartialOrd};
use camigo_helpers::{cami_ord, cami_partial_eq, Locality};
use core::cmp::Ordering;
use rust_alloc::string::String;

pub type StringCami = Cami<String>;

/// We need this, even though we have a generic impl for slices in [crate::slices_impls].
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
// @TODO special wrapper for &[char]?

// @TODO
// - confusion - should this be behind a feature (other than "alloc")?
// - without it, we'd need more `transmute`.
// --- even if we do have it, it doesn't "auto-magically" apply to core/std's slice::sort(). And we don't want to copy-and-paste sort()
// ----- TODO inspect & benchmark sort_by() & unstable_sort_by().
#[cfg(feature = "alloc")]
cami_partial_eq! {
    {String}
    (Locality::Both)
    [.len()]
    [(|this| this)]
    //[{|instance: &Self| instance}] //@TODO lifetime
    []
}

#[cfg(feature = "alloc")]
cami_ord! {
    String
    [{|v: &String| v.len()}]
    [(|this: &String, other: &String| this.cmp(other))]
}
