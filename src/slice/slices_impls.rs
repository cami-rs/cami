use crate::{COrd, CPartialEq, Locality};
use core::cmp::Ordering;

/// Used, for example, for multi-dimensional slices (or arrays/vectors). We also have a similar
/// implementation for `&str` in [crate::string].
impl<T> CPartialEq for &[T]
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

// @TODO (not just here, but in the whole crate): Find use cases when we benefit from PartialOrd,
// but we do NOT need (full) Ord

/// Used, for example, for multi-dimensional slices (or arrays/vectors). We also have a similar
/// implementation for `&str` in [crate::string].
impl<T> COrd for &[T]
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

// @TODO
// - confusion - should this be behind a feature (other than "alloc")?
// - without it, we'd need more `transmute`.
// --- even if we do have it, it doesn't "auto-magically" apply to core/std's slice::sort(). And we don't want to copy-and-paste sort()
// ----- TODO inspect & benchmark sort_by() & unstable_sort_by().
#[cfg(feature = "alloc")]
c_partial_eq! {
    ::alloc::string::String
    { Locality::Both }
    //[{|v: &String| v.len()}]
    [.len()] // @TODO
    [(|this: &::alloc::string::String, other: &::alloc::string::String| this == other)]
    [ {|instance: &Self| true} ]
}

#[cfg(feature = "alloc")]
c_ord! {
    ::alloc::string::String
    [{|v: &::alloc::string::String| v.len()}]
    [(|this: &::alloc::string::String, other: &::alloc::string::String| this.cmp(&other))]
}
