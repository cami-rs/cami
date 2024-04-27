pub use crate as camigo;
use crate::{CamiOrd, CamiPartialEq};
use camigo_helpers::{cami_ord, cami_partial_eq, Locality};
use core::cmp::Ordering;

/// We need this, even though we have a generic impl for slices in [crate::slices_impls].
impl CamiPartialEq for &str {
    const LOCALITY: Locality = Locality::Both;

    fn eq_local(&self, other: &Self) -> bool {
        self.len() == other.len()
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        self == other
    }
}

/// We need this, even though we have a generic impl for slices in [crate::slices_impls].
impl CamiOrd for &str {
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
// @TODO special wrapper for &[char]?

// @TODO
// - confusion - should this be behind a feature (other than "alloc")?
// - without it, we'd need more `transmute`.
// --- even if we do have it, it doesn't "auto-magically" apply to core/std's slice::sort(). And we don't want to copy-and-paste sort()
// ----- TODO inspect & benchmark sort_by() & unstable_sort_by().
#[cfg(feature = "alloc")]
cami_partial_eq! {
    [::rust_alloc::string::String]
    { Locality::Both }
    [.len()]
    [(|this: &::rust_alloc::string::String, other: &::rust_alloc::string::String| this == other)]
    //[{|instance: &Self| instance}] //@TODO lifetime
    []
}

#[cfg(feature = "alloc")]
cami_ord! {
    ::rust_alloc::string::String
    [{|v: &::rust_alloc::string::String| v.len()}]
    [(|this: &::rust_alloc::string::String, other: &::rust_alloc::string::String| this.cmp(&other))]
}
