use crate::{CfOrd, CfPartialEq, Locality};
use core::cmp::Ordering;

impl CfPartialEq for &str {
    const LOCALITY: Locality = Locality::Both;
    //const COMPATIBLE_WITH_PARTIAL_EQ: bool = true;

    fn eq_local(&self, other: &Self) -> bool {
        self.len() == other.len()
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        self == other
    }
}

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
