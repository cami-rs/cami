use crate::{CfOrd, CfPartialEq, Locality};
use core::cmp::Ordering;

/// For blanket implementations for core/std types (mostly primitives), implemented in this crate
pub trait CfPartialEqProxyPureLocal {}

impl<T: CfPartialEqProxyPureLocal + PartialEq> CfPartialEq for T {
    const LOCALITY: Locality = Locality::PureLocal;

    // If unsure, then it's `false`.
    //const COMPATIBLE_WITH_PARTIAL_EQ: bool = true;

    fn eq_local(&self, other: &Self) -> bool {
        self == other
    }
    fn eq_non_local(&self, other: &Self) -> bool {
        debug_assert!(false, "unreachable");
        self == other
    }
    fn eq_full(&self, other: &Self) -> bool {
        self == other
    }
}

/// For blanket implementations for core/std types (mostly primitives), implemented in this crate
pub trait CfOrdProxyPureLocal {}

impl<T: CfOrdProxyPureLocal + CfPartialEq + Ord> CfOrd for T {
    //const COMPATIBLE_WITH_ORD: bool = true;

    fn cmp_local(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        debug_assert!(false, "unreachable");
        self.cmp(other)
    }

    fn cmp_full(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl CfPartialEqProxyPureLocal for u8 {}
impl CfOrdProxyPureLocal for u8 {}
