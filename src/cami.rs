use crate::{CamiOrd, CamiPartialEq, CamiPartialOrd, Locality};
use core::cmp::Ordering;
use core::fmt::{self, Debug};

// Having an `Rhs` generic would need a phantom data field, so we couldn't easily pattern match this
// etc.
//
// pub struct Cami<T: CamiPartialEq<Rhs>, Rhs: ?Sized = Self>(pub T);
#[repr(transparent)]
pub struct Cami<T: CamiPartialEq>(pub T);

pub trait IntoCami: CamiPartialEq + Sized {
    #[inline]
    fn into_cami(self) -> Cami<Self> {
        Cami(self)
    }
}
impl<T: CamiPartialEq> IntoCami for T {}

pub trait IntoCamiCopy: CamiPartialEq + Sized + Copy {
    #[inline]
    fn into_cami_copy(&self) -> Cami<Self> {
        Cami(*self)
    }
}
impl<T: CamiPartialEq + Copy> IntoCamiCopy for T {}

pub trait IntoCamiClone: CamiPartialEq + Sized + Clone {
    #[inline]
    fn into_cami_clone(&self) -> Cami<Self> {
        Cami(self.clone())
    }
}
impl<T: CamiPartialEq + Clone> IntoCamiClone for T {}

impl<T: CamiPartialEq> Cami<T> {
    /// Consume [self], return the wrapped data. We COULD just use `self.0` (or
    /// `variable_holding_the_instance.0`) - but, then it can't be easily searched for in source
    /// code.
    pub fn from_cami(self) -> T {
        self.0
    }
}

impl<T: CamiPartialEq + Copy> Cami<T> {
    /// Take [self] by reference, return a copy of the wrapped data. We COULD just use `self.0` (or
    /// `variable_holding_the_instance.0`) - but, then it can't be easily searched for in source
    /// code.
    pub fn from_cami_copy(&self) -> T {
        self.0
    }
}

impl<T: CamiPartialEq + Clone> Cami<T> {
    /// Take [self] by reference, return a clone of the wrapped data. We COULD just use
    /// `self.0.clone()` (or `variable_holding_the_instance.0.clone()`) - but, then it can't be
    /// easily searched for in source code.
    pub fn from_cami_clone(&self) -> T {
        self.0.clone()
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
