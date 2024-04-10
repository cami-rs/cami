use crate::{CfOrd, CfPartialEq, Locality};
use core::cmp::Ordering;

/// A (zero cost) wrapper & bridge that implements [CfPartialEq], [PartialEq], [PartialOrd], [CfOrd]
/// and [Ord] forwarding to [PartialEq], [PartialOrd] and [Ord] methods of `T`.
///
/// For compatibility only - no speed benefit!
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct CfWrap<T> {
    t: T,
}

#[inline]
fn unreachable() {
    debug_assert!(
        false,
        "unreachable because of its LOCALITY==Locality::PureNonLocal"
    );
}

impl<T: PartialEq> CfPartialEq for CfWrap<T> {
    const LOCALITY: Locality = Locality::PureNonLocal;
    //const COMPATIBLE_WITH_PARTIAL_EQ: bool = true;

    fn eq_local(&self, other: &Self) -> bool {
        unreachable();
        self.t == other.t
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl<T: Ord> CfOrd for CfWrap<T> {
    //const COMPATIBLE_WITH_ORD: bool = true;

    fn cmp_local(&self, other: &Self) -> Ordering {
        unreachable();
        self.t.cmp(&other.t)
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        unreachable();
        self.t.cmp(&other.t)
    }

    fn cmp_full(&self, other: &Self) -> Ordering {
        unreachable();
        self.t.cmp(&other.t)
    }
}

impl<T: PartialOrd> PartialOrd for CfWrap<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }

    fn lt(&self, other: &Self) -> bool {
        self.t.lt(&other.t)
    }
    fn le(&self, other: &Self) -> bool {
        self.t.le(&other.t)
    }
    fn gt(&self, other: &Self) -> bool {
        self.t.gt(&other.t)
    }
    fn ge(&self, other: &Self) -> bool {
        self.t.ge(&other.t)
    }
}
impl<T: Ord> Ord for CfWrap<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.cmp(&other.t)
    }
    // Default implementations for the rest of methods.
}
