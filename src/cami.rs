use crate::{CamiOrd, CamiPartialEq, CamiPartialOrd, Locality};
use core::cmp::Ordering;
use core::fmt::{self, Debug};

// Having an `Rhs` generic would need a phantom data field, so we couldn't easily pattern match this
// etc.
//
// pub struct Cami<T: CamiPartialEq<Rhs>, Rhs: ?Sized = Self>(pub T);
#[repr(transparent)]
pub struct Cami<T: CamiPartialEq>(pub T);

//----------
pub trait IntoCami {
    type Wrapped: CamiPartialEq;
    fn into_cami(self) -> Cami<Self::Wrapped>;
}
impl<T: CamiPartialEq> IntoCami for T {
    type Wrapped = Self;
    fn into_cami(self) -> Cami<Self> {
        Cami(self)
    }
}

pub trait IntoCamiCopy {
    type Wrapped: CamiPartialEq;
    fn into_cami_copy(&self) -> Cami<Self::Wrapped>;
}
impl<T: CamiPartialEq + Copy> IntoCamiCopy for T {
    type Wrapped = Self;
    #[inline]
    fn into_cami_copy(&self) -> Cami<Self> {
        Cami(*self)
    }
}

pub trait IntoCamiClone {
    type Wrapped: CamiPartialEq;
    fn into_cami_clone(&self) -> Cami<Self::Wrapped>;
}
impl<T: CamiPartialEq + Clone> IntoCamiClone for T {
    type Wrapped = Self;
    #[inline]
    fn into_cami_clone(&self) -> Cami<Self> {
        Cami(self.clone())
    }
}
//----------

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
//----------

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

// Simple forwarding. Not really necessary: We normally don't need to wrap a `Cami` type inside one
// more level of `Cami`, but it's possible.
impl<T: CamiPartialEq> CamiPartialEq for Cami<T> {
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

// Simple forwarding. Not really necessary: We normally don't need to wrap a `Cami` type inside one
// more level of `Cami`, but it's possible.
impl<T: CamiPartialOrd> CamiPartialOrd for Cami<T> {
    fn partial_cmp_local(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp_local(&other.0)
    }
    fn partial_cmp_non_local(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp_non_local(&other.0)
    }
    #[inline]
    fn lt_local(&self, other: &Self) -> bool {
        self.0.lt_local(&other.0)
    }
    #[inline]
    fn lt_non_local(&self, other: &Self) -> bool {
        self.0.lt_non_local(&other.0)
    }
    #[inline]
    fn le_local(&self, other: &Self) -> bool {
        self.0.le_local(&other.0)
    }
    #[inline]
    fn le_non_local(&self, other: &Self) -> bool {
        self.0.le_non_local(&other.0)
    }
    #[inline]
    fn gt_local(&self, other: &Self) -> bool {
        self.0.gt_local(&other.0)
    }
    #[inline]
    fn gt_non_local(&self, other: &Self) -> bool {
        self.0.gt_non_local(&other.0)
    }
    #[inline]
    fn ge_local(&self, other: &Self) -> bool {
        self.0.ge_local(&other.0)
    }
    #[inline]
    fn ge_non_local(&self, other: &Self) -> bool {
        self.0.ge_non_local(&other.0)
    }
}

// Simple forwarding. Not really necessary: We normally don't need to wrap a `Cami` type inside one
// more level of `Cami`, but it's possible.
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
