#[macro_export]
macro_rules! pure_local_cpartial_eq {
    ($T:ident) => {
        impl $crate::CPartialEq for $T {
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

#[macro_export]
macro_rules! pure_local_cord {
    ($T:ident) => {
        impl $crate::COrd for $T {
            fn cmp_local(&self, other: &Self) -> core::cmp::Ordering {
                self.cmp(other)
            }

            fn cmp_non_local(&self, other: &Self) -> core::cmp::Ordering {
                debug_assert!(false, "unreachable");
                self.cmp(other)
            }

            fn cmp_full(&self, other: &Self) -> core::cmp::Ordering {
                self.cmp(other)
            }
        }
    };
}
