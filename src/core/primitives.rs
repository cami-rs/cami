///! Not strictly necessary. Mostly needed so that a blanket `impl` works for tuples containing any
///! types that implement [crate::CamiPartialEq] & [crate::CamiOrd].
pub use crate as cami; // for macros
use crate::prelude::*;
use cami_helpers::{pure_local_c_ord, pure_local_c_partial_eq};
use core::cmp::Ordering;
#[cfg(feature = "transmute")]
use core::mem;

impl CamiPartialEq for () {
    const LOCALITY: Locality = Locality::PureLocal;

    fn eq_local(&self, _other: &Self) -> bool {
        true
    }

    fn eq_non_local(&self, _other: &Self) -> bool {
        cami_helpers::debug_fail_unreachable_for_non_local();
        true
    }
}

impl CamiPartialOrd for () {
    #[must_use]
    #[inline]
    fn partial_cmp_local(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
    #[must_use]
    #[inline]
    fn partial_cmp_non_local(&self, _other: &Self) -> Option<Ordering> {
        cami_helpers::debug_fail_unreachable_for_non_local();
        Some(Ordering::Equal)
    }
}

impl CamiOrd for () {
    fn cmp_local(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }

    fn cmp_non_local(&self, _other: &Self) -> Ordering {
        cami_helpers::debug_fail_unreachable_for_non_local();
        Ordering::Equal
    }
}
//--------

/// This exists, so that it has consistent [CamiPartialEq], [CamiPartialOrd], [CamiOrd] and
/// [PartialEq] based on [pub fn total_cmp(&self, other: &Self) ->
/// Ordering](https://doc.rust-lang.org/nightly/core/primitive.f32.html#method.total_cmp). Those
/// implementations do NOT always agree with [PartialEq] (and [PartialOrd]) of [f32].
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct F32Total(f32);

impl F32Total {
    #[must_use]
    #[inline]
    pub fn new(from: f32) -> Self {
        Self(from)
    }
}

impl PartialEq for F32Total {
    #[must_use]
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_eq()
    }
    #[must_use]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_ne()
    }
}
impl Eq for F32Total {}
impl CamiPartialEq for F32Total {
    const LOCALITY: Locality = Locality::PureLocal;
    #[must_use]
    #[inline]
    fn eq_local(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_eq()
    }

    #[must_use]
    #[inline]
    fn eq_non_local(&self, _other: &Self) -> bool {
        cami_helpers::debug_fail_unreachable_for_non_local();
        true
    }
}

impl CamiPartialOrd for F32Total {
    #[must_use]
    #[inline]
    fn partial_cmp_local(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.total_cmp(&other.0))
    }
    #[must_use]
    #[inline]
    fn partial_cmp_non_local(&self, _other: &Self) -> Option<Ordering> {
        cami_helpers::debug_fail_unreachable_for_non_local();
        Some(Ordering::Equal)
    }
    // NOT specializing the rest of the methods. We can't use f32 standard/classic comparison (with
    // operators <, >...), because that is incompatible with toal_cmp
}

impl CamiOrd for F32Total {
    #[must_use]
    #[inline]
    fn cmp_local(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }

    #[must_use]
    #[inline]
    fn cmp_non_local(&self, _other: &Self) -> Ordering {
        cami_helpers::debug_fail_unreachable_for_non_local();
        Ordering::Equal
    }
}
pub type F32Cami = Cami<F32Total>;

impl F32Cami {
    #[must_use]
    #[inline]
    pub fn into_f32(&self) -> f32 {
        self.in_cami().0
    }
}
//--------
impl IntoCami for f32 {
    type Wrapped = F32Total;
    #[must_use]
    #[inline]
    fn into_cami(self) -> F32Cami {
        Cami::new(F32Total(self))
    }
}
impl IntoCamiCopy for f32 {
    type Wrapped = F32Total;
    #[must_use]
    #[inline]
    fn into_cami_copy(&self) -> F32Cami {
        Cami::new(F32Total(*self))
    }
}
impl IntoCamiClone for f32 {
    type Wrapped = F32Total;
    #[must_use]
    #[inline]
    fn into_cami_clone(&self) -> F32Cami {
        Cami::new(F32Total(self.clone()))
    }
}
//--------
#[cfg(feature = "transmute")]
impl IntoRefCami for f32 {
    type Wrapped = F32Total;
    #[must_use]
    #[inline]
    fn into_ref_cami(&self) -> &F32Cami {
        unsafe { mem::transmute(self) }
    }
    #[must_use]
    #[inline]
    fn into_mut_cami(&mut self) -> &mut F32Cami {
        unsafe { mem::transmute(self) }
    }
}
#[cfg(feature = "transmute")]
impl IntoSliceCami for [f32] {
    type Wrapped = F32Total;
    #[must_use]
    #[inline]
    fn into_slice_cami(&self) -> &[F32Cami] {
        unsafe { mem::transmute(self) }
    }
    #[must_use]
    #[inline]
    fn into_slice_mut_cami(&mut self) -> &mut [F32Cami] {
        unsafe { mem::transmute(self) }
    }
}
//--------

pure_local_c_partial_eq! { bool }
pure_local_c_ord! { bool }
pub type BoolCami = Cami<bool>;

pure_local_c_partial_eq! { u8 }
pure_local_c_ord! { u8 }
pub type U8Cami = Cami<u8>;
// TODO other types

//--------

// Blanket impl for references.
impl<T> CamiPartialEq for &T
where
    T: PartialEq,
{
    const LOCALITY: Locality = Locality::PureLocal;

    #[must_use]
    #[inline]
    fn eq_local(&self, other: &Self) -> bool {
        self == other
    }

    #[must_use]
    #[inline]
    fn eq_non_local(&self, _other: &Self) -> bool {
        true
    }
}

// @TODO (not just here, but in the whole crate): Find use cases when we benefit from PartialOrd,
// but we do NOT need (full) Ord

impl<T> CamiPartialOrd for &T
where
    T: PartialOrd,
{
    #[must_use]
    #[inline]
    fn partial_cmp_local(&self, other: &Self) -> Option<Ordering> {
        // @TODO benchmark if this is faster: Some(self.len().cmp(&other.len()))
        self.partial_cmp(other)
    }
    #[must_use]
    #[inline]
    fn partial_cmp_non_local(&self, _other: &Self) -> Option<Ordering> {
        // @TODO benchmark if this is faster: Some(self.cmp(other))
        Some(Ordering::Equal)
    }

    #[must_use]
    #[inline]
    fn lt_local(&self, other: &Self) -> bool {
        self < other
    }
    #[must_use]
    #[inline]
    fn lt_non_local(&self, _other: &Self) -> bool {
        true
    }

    #[must_use]
    #[inline]
    fn le_local(&self, other: &Self) -> bool {
        self <= other
    }
    #[must_use]
    #[inline]
    fn le_non_local(&self, _other: &Self) -> bool {
        true
    }

    #[must_use]
    #[inline]
    fn gt_local(&self, other: &Self) -> bool {
        self > other
    }
    #[must_use]
    #[inline]
    fn gt_non_local(&self, _other: &Self) -> bool {
        true
    }

    #[must_use]
    #[inline]
    fn ge_local(&self, other: &Self) -> bool {
        self >= other
    }
    #[must_use]
    #[inline]
    fn ge_non_local(&self, _other: &Self) -> bool {
        true
    }
}

/// Used, for example, for multi-dimensional slices (or arrays/vectors). We also have a similar
/// implementation for `&str`.
impl<T> CamiOrd for &T
where
    T: Ord,
{
    #[must_use]
    #[inline]
    fn cmp_local(&self, other: &Self) -> Ordering {
        self.cmp(&other)
    }

    #[must_use]
    #[inline]
    fn cmp_non_local(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

// @TODO search for RefCami (traits containing this in their name), and update them to use `RefCami`
pub type RefCami<'a, T> = Cami<&'a T>;
