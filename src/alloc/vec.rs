use crate::{Cami, CamiPartialEq};
use rust_alloc::vec::Vec;

#[cfg(feature = "wrappers")]
/// NOT a [Cami] of [Vec], BUT a [Vec] of [Cami]. If you really need [Cami] of [Vec], use
/// `Cami<Vec<T>>`.
pub type VecCami<T> = Vec<Cami<T>>;

pub trait IntoVecCami<T>
where
    T: CamiPartialEq,
{
    #[must_use]
    fn into_vec_cami(self) -> Vec<Cami<T>>;
    #[must_use]
    fn into_ref_vec_cami(&self) -> &Vec<Cami<T>>;
    #[must_use]
    fn into_mut_vec_cami(&mut self) -> &mut Vec<Cami<T>>;
}
#[cfg(feature = "transmute")]
impl<T: CamiPartialEq> IntoVecCami<T> for Vec<T> {
    #[must_use]
    #[inline]
    fn into_vec_cami(self) -> Vec<Cami<T>> {
        unsafe { core::mem::transmute(self) }
    }
    #[must_use]
    #[inline]
    fn into_ref_vec_cami(&self) -> &Vec<Cami<T>> {
        unsafe { core::mem::transmute(self) }
    }
    #[must_use]
    #[inline]
    fn into_mut_vec_cami(&mut self) -> &mut Vec<Cami<T>> {
        unsafe { core::mem::transmute(self) }
    }
}
