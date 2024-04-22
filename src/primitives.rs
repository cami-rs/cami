use crate::{COrd, CPartialEq, Locality};
use core::cmp::Ordering;

impl CPartialEq for bool {
    const LOCALITY: Locality = Locality::PureLocal;

    fn eq_local(&self, other: &Self) -> bool {
        self == other
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        self == other
    }
}

impl COrd for bool {
    fn cmp_local(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        self.cmp(&other)
    }
}
//---------

impl CPartialEq for () {
    const LOCALITY: Locality = Locality::PureLocal;

    fn eq_local(&self, other: &Self) -> bool {
        self == other
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        self == other
    }
}

impl COrd for () {
    fn cmp_local(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        self.cmp(&other)
    }
}
