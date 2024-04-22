use crate::{pure_local_cord, pure_local_cpartial_eq};
use crate::{COrd, CPartialEq, Locality};
use core::cmp::Ordering;

impl CPartialEq for () {
    const LOCALITY: Locality = Locality::PureLocal;

    fn eq_local(&self, other: &Self) -> bool {
        true
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        true
    }
}

impl COrd for () {
    fn cmp_local(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        debug_assert!(false, "unreachable");
        Ordering::Equal
    }
}
//--------

pure_local_cpartial_eq! { bool }
pure_local_cord! { bool }
pure_local_cpartial_eq! { u8 }
pure_local_cord! { u8 }
