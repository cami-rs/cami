use crate::{CamiOrd, CamiPartialEq, CamiPartialOrd, Locality};
use core::cmp::Ordering;
use core::fmt::{self, Debug};
#[cfg(feature = "transmute")]
use core::mem;
#[cfg(feature = "deref_pure")]
use core::ops::DerefPure;
use core::ops::{Deref, DerefMut};

// @TODO once agreed & futureproofed, remove `#[deprecated...]` on field `pub T`.
//
// Having an `Rhs` generic (for `CamiPartialEq`) would need a phantom data field, so we couldn't
// easily pattern match this etc.
//
// pub struct Cami<T: CamiPartialEq<Rhs>, Rhs: ?Sized = Self>(pub T);
#[repr(transparent)]
pub struct Cami<T: CamiPartialEq + ?Sized>(#[deprecated = "unstable"] pub T);
//----------

impl<T: CamiPartialEq> Cami<T> {
    pub fn new(from: T) -> Self {
        Self(from)
    }

    pub fn in_cami(&self) -> &T {
        #[allow(deprecated)]
        &self.0
    }
    /// Consume [self], return the wrapped data. We COULD just use `self.0` (or
    /// `variable_holding_the_instance.0`) - but, then it can't be easily searched for in source
    /// code.
    pub fn from_cami(self) -> T {
        #[allow(deprecated)]
        self.0
    }
}

impl<T: CamiPartialEq + Copy> Cami<T> {
    /// Take [self] by reference, return a copy of the wrapped data. We COULD just use `self.0` (or
    /// `variable_holding_the_instance.0`) - but, then it can't be easily searched for in source
    /// code.
    pub fn from_cami_copy(&self) -> T {
        #[allow(deprecated)]
        self.0
    }
}

impl<T: CamiPartialEq + Clone> Cami<T> {
    /// Take [self] by reference, return a clone of the wrapped data. We COULD just use
    /// `self.0.clone()` (or `variable_holding_the_instance.0.clone()`) - but, then it can't be
    /// easily searched for in source code.
    pub fn from_cami_clone(&self) -> T {
        #[allow(deprecated)]
        self.0.clone()
    }
}
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

pub trait IntoCamiRef {
    type Wrapped: CamiPartialEq + ?Sized;
    fn into_cami_ref(&self) -> &Cami<Self::Wrapped>;
}
#[cfg(feature = "transmute")]
impl<T: CamiPartialEq + ?Sized> IntoCamiRef for T {
    type Wrapped = Self;
    #[inline]
    fn into_cami_ref(&self) -> &Cami<Self> {
        unsafe { mem::transmute(self) }
    }
}

/// @TODO Should this rather be called `IntoSliceCami` to indicate that it's not the slice, but the
/// items, that get transmuted to `Cami`?
pub trait IntoCamiSlice {
    type Wrapped: CamiPartialEq;
    fn into_cami_slice(&self) -> &[Cami<Self::Wrapped>];
}
#[cfg(feature = "transmute")]
impl<T: CamiPartialEq> IntoCamiSlice for [T] {
    type Wrapped = T;
    #[inline]
    fn into_cami_slice(&self) -> &[Cami<T>] {
        unsafe { mem::transmute(self) }
    }
}
//----------

impl<T: Clone + CamiPartialEq> Clone for Cami<T> {
    fn clone(&self) -> Self {
        Self(
            #[allow(deprecated)]
            self.0.clone(),
        )
    }
    fn clone_from(&mut self, source: &Self) {
        #![allow(deprecated)]
        self.0 = source.0.clone();
    }
}

impl<T: Copy + CamiPartialEq> Copy for Cami<T> {}

impl<T: Debug + CamiPartialEq> Debug for Cami<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cami")
            .field(
                "0",
                #[allow(deprecated)]
                &self.0,
            )
            .finish()
    }
}
//-----

/// Simple forwarding
///
/// NO "Rhs" (right hand side) generic parameter, because then [Cami] would have to contain phantom
/// data, which would make pattern matching etc. difficult.
impl<T: CamiPartialEq> PartialEq for Cami<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let this = self.in_cami();
        let other = other.in_cami();
        // @TODO write a test that the following would return the same
        //
        // Write them not in this crate, but in Camigo crate - for example, next to the
        // implementation for `bool`.
        if false {
            return (!Self::LOCALITY.has_local() || this.eq_local(&other))
                && (!Self::LOCALITY.has_non_local() || this.eq_non_local(&other));
        }
        if Self::LOCALITY.has_local() {
            let local = this.eq_local(other);
            if local {
                Self::LOCALITY.has_non_local() || this.eq_non_local(other)
            } else {
                false
            }
        } else {
            this.eq_non_local(other)
        }
    }
}

impl<T: Eq + CamiPartialEq> Eq for Cami<T> {}

impl<T: CamiPartialOrd> PartialOrd for Cami<T> {
    /// This returns [Some] only if BOTH of [CamiPartialOrd::partial_cmp_local] and
    /// [CamiPartialOrd::partial_cmp_local] (as applicable - depending on [CamiPartialEq::LOCALITY])
    /// return [Some].
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let this = self.in_cami();
        let other = other.in_cami();
        if Self::LOCALITY.has_local() {
            let local = this.partial_cmp_local(other);
            if local == None {
                return None;
            }
            if local == Some(Ordering::Equal) {
                if Self::LOCALITY.has_non_local() {
                    this.partial_cmp_non_local(other)
                } else {
                    Some(Ordering::Equal)
                }
            } else {
                local
            }
        } else {
            this.partial_cmp_non_local(other)
        }
    }

    // Provided methods
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        let this = self.in_cami();
        let other = other.in_cami();
        if Self::LOCALITY.has_local() {
            this.lt_local(other) || Self::LOCALITY.has_non_local() && this.lt_non_local(other)
        } else {
            this.lt_non_local(other)
        }
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        let this = self.in_cami();
        let other = other.in_cami();
        if Self::LOCALITY.has_local() {
            this.le_local(other) || Self::LOCALITY.has_non_local() && this.le_non_local(other)
        } else {
            this.le_non_local(other)
        }
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        let this = self.in_cami();
        let other = other.in_cami();
        if Self::LOCALITY.has_local() {
            this.gt_local(other) || Self::LOCALITY.has_non_local() && this.gt_non_local(other)
        } else {
            this.gt_non_local(other)
        }
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        let this = self.in_cami();
        let other = other.in_cami();
        if Self::LOCALITY.has_local() {
            this.ge_local(other) || Self::LOCALITY.has_non_local() && this.ge_non_local(other)
        } else {
            this.ge_non_local(other)
        }
    }
}

impl<T: CamiOrd> Ord for Cami<T> {
    /// Full comparison.
    ///
    /// It respects [CamiPartialOrd::LOCALITY] and calls [CamiOrd::cmp_local] and/or
    /// [CamiOrd::cmp_non_local] only when they're applicable and when they're needed.
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let this = self.in_cami();
        let other = other.in_cami();
        if Self::LOCALITY.has_local() {
            let local = this.cmp_local(other);
            if local == Ordering::Equal {
                if Self::LOCALITY.has_non_local() {
                    this.cmp_non_local(other)
                } else {
                    Ordering::Equal
                }
            } else {
                local
            }
        } else {
            this.cmp_non_local(other)
        }
    }
}
//-----

impl<T: CamiPartialEq + ?Sized> Deref for Cami<T> {
    type Target = T;
    fn deref(&self) -> &T {
        #[allow(deprecated)]
        &self.0
    }
}

impl<T: CamiPartialEq + ?Sized> DerefMut for Cami<T> {
    fn deref_mut(&mut self) -> &mut T {
        #[allow(deprecated)]
        &mut self.0
    }
}

#[cfg(feature = "deref_pure")]
unsafe impl<T: CamiPartialEq + ?Sized> DerefPure for Cami<T> {}
//-----

// Simple forwarding. Not really necessary: We normally don't need to wrap a `Cami` type inside one
// more level of `Cami`, but it's possible.
impl<T: CamiPartialEq> CamiPartialEq for Cami<T> {
    const LOCALITY: Locality = T::LOCALITY;
    #[inline]
    fn eq_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        self.0.eq_local(&other.0)
    }

    #[inline]
    fn eq_non_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        !self.0.eq_non_local(&other.0)
    }
}

// Simple forwarding. Not really necessary: We normally don't need to wrap a `Cami` type inside one
// more level of `Cami`, but it's possible.
impl<T: CamiPartialOrd> CamiPartialOrd for Cami<T> {
    fn partial_cmp_local(&self, other: &Self) -> Option<Ordering> {
        #![allow(deprecated)]
        self.0.partial_cmp_local(&other.0)
    }
    fn partial_cmp_non_local(&self, other: &Self) -> Option<Ordering> {
        #![allow(deprecated)]
        self.0.partial_cmp_non_local(&other.0)
    }
    #[inline]
    fn lt_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        self.0.lt_local(&other.0)
    }
    #[inline]
    fn lt_non_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        self.0.lt_non_local(&other.0)
    }
    #[inline]
    fn le_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        self.0.le_local(&other.0)
    }
    #[inline]
    fn le_non_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        self.0.le_non_local(&other.0)
    }
    #[inline]
    fn gt_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        self.0.gt_local(&other.0)
    }
    #[inline]
    fn gt_non_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        self.0.gt_non_local(&other.0)
    }
    #[inline]
    fn ge_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        self.0.ge_local(&other.0)
    }
    #[inline]
    fn ge_non_local(&self, other: &Self) -> bool {
        #![allow(deprecated)]
        self.0.ge_non_local(&other.0)
    }
}

// Simple forwarding. Not really necessary: We normally don't need to wrap a `Cami` type inside one
// more level of `Cami`, but it's possible.
impl<T: Ord + PartialOrd + PartialEq + CamiPartialEq + CamiOrd> CamiOrd for Cami<T> {
    #[inline]
    fn cmp_local(&self, other: &Self) -> Ordering {
        #![allow(deprecated)]
        self.0.cmp_local(&other.0)
    }

    #[inline]
    fn cmp_non_local(&self, other: &Self) -> Ordering {
        #![allow(deprecated)]
        self.0.cmp_non_local(&other.0)
    }
}
