use crate as cami; // For macros
use crate::{Cami, CamiOrd, CamiPartialEq, CamiPartialOrd};
use cami_helpers::{cami_ord, cami_partial_eq, Locality};
use core::cmp::Ordering;
use rust_alloc::string::String;

// @TODO rename to CamiString, or: remove?
pub type StringCami = Cami<String>;

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
