//! Not strictly necessary. Mostly needed so that a blanket `impl` works for tuples containing any
//! types that implement [crate::CamiPartialEq] & [crate::CamiOrd].
pub use crate as camigo; // for macros
use crate::{
    Cami, CamiOrd, CamiPartialEq, CamiPartialOrd, IntoCami, IntoCamiClone, IntoCamiCopy,
    IntoCamiRef, IntoCamiSlice, Locality,
};
use camigo_helpers::{pure_local_c_ord, pure_local_c_partial_eq};
use core::cmp::Ordering;

impl CamiPartialEq for () {
    const LOCALITY: Locality = Locality::PureLocal;

    fn eq_local(&self, other: &Self) -> bool {
        true
    }

    fn eq_non_local(&self, other: &Self) -> bool {
        camigo_helpers::debug_fail_unreachable_for_non_local();
        true
    }
}

impl CamiPartialOrd for () {}

impl CamiOrd for () {
    fn cmp_local(&self, other: &Self) -> Ordering {
        Ordering::Equal
    }

    fn cmp_non_local(&self, other: &Self) -> Ordering {
        camigo_helpers::debug_fail_unreachable_for_non_local();
        Ordering::Equal
    }
}
//--------

/// This exists, so that it has consistent [CamiPartialEq], [CamiPartialOrd], [CamiOrd] and
/// [PartialEq] based on [pub fn total_cmp(&self, other: &Self) ->
/// Ordering](https://doc.rust-lang.org/nightly/core/primitive.f32.html#method.total_cmp). Those
/// implementations do NOT always agree with [PartialEq] (and [PartialOrd]) of [f32].
#[cfg(feature = "wrappers")]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct F32Total(f32);

impl F32Total {
    pub fn new(from: f32) -> Self {
        Self(from)
    }
}

impl PartialEq for F32Total {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_eq()
    }
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_ne()
    }
}
impl Eq for F32Total {}
impl CamiPartialEq for F32Total {
    const LOCALITY: Locality = Locality::PureLocal;
    #[inline]
    fn eq_local(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_eq()
    }

    #[inline]
    fn eq_non_local(&self, other: &Self) -> bool {
        camigo_helpers::debug_fail_unreachable_for_non_local();
        true
    }
}

impl CamiPartialOrd for F32Total {}

impl CamiOrd for F32Total {
    #[inline]
    fn cmp_local(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }

    #[inline]
    fn cmp_non_local(&self, other: &Self) -> Ordering {
        camigo_helpers::debug_fail_unreachable_for_non_local();
        Ordering::Equal
    }
}
#[cfg(feature = "wrappers")]
pub type F32Cami = Cami<F32Total>;
impl F32Cami {
    pub fn into_f32(&self) -> f32 {
        self.in_cami().0
    }
}
//--------
impl IntoCami for f32 {
    type Wrapped = F32Total;
    #[inline]
    fn into_cami(self) -> F32Cami {
        Cami::new(F32Total(self))
    }
}
impl IntoCamiCopy for f32 {
    type Wrapped = F32Total;
    #[inline]
    fn into_cami_copy(&self) -> F32Cami {
        Cami::new(F32Total(*self))
    }
}
impl IntoCamiClone for f32 {
    type Wrapped = F32Total;
    #[inline]
    fn into_cami_clone(&self) -> F32Cami {
        Cami::new(F32Total(self.clone()))
    }
}
//--------
impl IntoCamiRef for f32 {
    type Wrapped = F32Total;
    #[inline]
    fn into_cami_ref(&self) -> &F32Cami {
        todo!()
    }
}
impl IntoCamiSlice for [f32] {
    type Wrapped = F32Total;
    #[inline]
    fn into_cami_slice(&self) -> &[F32Cami] {
        todo!()
    }
}
//--------

pure_local_c_partial_eq! { bool }
pure_local_c_ord! { bool }
#[cfg(feature = "wrappers")]
pub type BoolCami = Cami<bool>;

pure_local_c_partial_eq! { u8 }
pure_local_c_ord! { u8 }
#[cfg(feature = "wrappers")]
pub type U8Cami = Cami<u8>;
