macro_rules! pure_local_cf_partial_eq {
    ($T:ident) => {
        impl $crate::CfPartialEq for $T {
            const LOCALITY: $crate::Locality = $crate::Locality::PureLocal;

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
    };
}

/*impl<T: CfPartialEqProxyPureLocal + PartialEq> CfPartialEq for T {
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
}*/
macro_rules! pure_local_cf_ord {
    ($T:ident) => {
        impl $crate::CfOrd for $T {
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
    };
}

/*impl<T: CfOrdProxyPureLocal + CfPartialEq + Ord> CfOrd for T {
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
}*/
