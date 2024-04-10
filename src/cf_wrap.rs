use crate::{COrd, CPartialEq, Locality};
use core::cmp::Ordering;

/// A (zero cost) wrapper & bridge that implements [CPartialEq], [PartialEq], [PartialOrd], [COrd]
/// and [Ord], forwarding to [PartialEq], [PartialOrd] and [Ord] methods of `T`.
///
/// For compatibility only - no speed/cache benefit!
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl<T: PartialEq> CPartialEq for CfWrap<T> {
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

impl<T: Ord> COrd for CfWrap<T> {
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
