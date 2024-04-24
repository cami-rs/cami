pub use crate as camigo;
use crate::{COrd, CPartialEq};
use camigo_helpers::Locality;
use camigo_helpers::{pure_local_c_ord, pure_local_c_partial_eq};
use core::cmp::Ordering;

impl CPartialEq for () {
    const LOCALITY: Locality = Locality::PureLocal;

    fn eq_local(&self, other: &Self) -> bool {
        true
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        camigo_helpers::debug_fail_unreachable_for_non_local();
        true
    }
}

impl COrd for () {
    fn cmp_local(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        camigo_helpers::debug_fail_unreachable_for_non_local();
        Ordering::Equal
    }
}
//--------

pure_local_c_partial_eq! { bool }
pure_local_c_ord! { bool }

pure_local_c_partial_eq! { u8 }
pure_local_c_ord! { u8 }
