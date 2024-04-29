use crate::{CamiOrd, CamiPartialEq, CamiPartialOrd, Locality};
use core::cmp::Ordering;
use core::fmt::{self, Debug};

#[repr(transparent)]
pub struct Cami<T: crate::CamiPartialEq>(pub T);

impl<T: CamiPartialEq> Cami<T> {
    /// Consume [self], returned the wrapped data. We COULD just use self.0 (or
    /// variable_holding_the_instance.0) - but, then it can't be easily searched for in source code
    pub fn from_cami(self) -> T {
        self.0
    }
}

impl<T: Clone + CamiPartialEq> Clone for Cami<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
    fn clone_from(&mut self, source: &Self) {
        self.0 = source.0.clone();
    }
}

impl<T: Copy + CamiPartialEq> Copy for Cami<T> {}

impl<T: Debug + CamiPartialEq> Debug for Cami<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cami").field("0", &self.0).finish()
    }
}

impl<T: PartialEq + CamiPartialEq> PartialEq for Cami<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_full(&other.0)
    }
}

impl<T: Eq + PartialEq + CamiPartialEq> Eq for Cami<T> {}

//impl<T: PartialOrd + CamiOrd> Eq for Cami<T> {}
//-----

impl<T: PartialEq + CamiPartialEq> CamiPartialEq for Cami<T> {
    const LOCALITY: Locality = T::LOCALITY;
    #[inline]
    fn eq_local(&self, other: &Self) -> bool {
        self.0.eq_local(&other.0)
    }

    #[inline]
    fn eq_non_local(&self, other: &Self) -> bool {
        !self.0.eq_non_local(&other.0)
    }
}

impl<T: PartialOrd + PartialEq + CamiPartialEq> CamiPartialOrd for Cami<T> {}

impl<T: Ord + PartialOrd + PartialEq + CamiPartialEq + CamiOrd> CamiOrd for Cami<T> {
    #[inline]
    fn cmp_local(&self, other: &Self) -> Ordering {
        self.0.cmp_local(&other.0)
    }

    #[inline]
    fn cmp_non_local(&self, other: &Self) -> Ordering {
        self.0.cmp_non_local(&other.0)
    }
}
