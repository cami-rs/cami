use crate::{ca_ord, ca_partial_eq};
use crate::{COrd, CPartialEq, Locality};
use core::cmp::Ordering;

/// We need this, even though we have a generic impl for slices in [crate::slices_impls].
impl CPartialEq for &str {
    const LOCALITY: Locality = Locality::Both;

    fn eq_local(&self, other: &Self) -> bool {
        self.len() == other.len()
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        self == other
    }
}

/// We need this, even though we have a generic impl for slices in [crate::slices_impls].
impl COrd for &str {
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
ca_partial_eq! {
    ::alloc::string::String
    { Locality::Both }
    [.len()]
    [(|this: &::alloc::string::String, other: &::alloc::string::String| this == other)]
    //[{|instance: &Self| instance}] //@TODO lifetime
    []
}

#[cfg(feature = "alloc")]
ca_ord! {
    ::alloc::string::String
    [{|v: &::alloc::string::String| v.len()}]
    [(|this: &::alloc::string::String, other: &::alloc::string::String| this.cmp(&other))]
}
