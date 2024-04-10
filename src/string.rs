use crate::{CfOrd, CfPartialEq, Locality};
use core::cmp::Ordering;

/// We need this, even though we have a generic impl for slices in [crate::slices].
impl CfPartialEq for &str {
    const LOCALITY: Locality = Locality::Both;

    fn eq_local(&self, other: &Self) -> bool {
        self.len() == other.len()
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        self == other
    }
}

/// We need this, even though we have a generic impl for slices in [crate::slices].
impl CfOrd for &str {
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
