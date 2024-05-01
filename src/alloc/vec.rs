use crate::{Cami, CamiPartialEq};
use rust_alloc::vec::Vec;

#[cfg(feature = "wrappers")]
/// NOT a [Cami] of [Vec], BUT a [Vec] of [Cami]. If you really need [Cami] of [Vec], use
/// `Cami<Vec<T>>`.
pub type VecCami<T> = Vec<Cami<T>>;

pub trait IntoCamiVec<T>
where
    T: CamiPartialEq,
{
    #[must_use]
    fn into_cami_vec(self) -> Vec<Cami<T>>;
}
#[cfg(feature = "transmute")]
impl<T: CamiPartialEq> IntoCamiVec<T> for Vec<T> {
    #[must_use]
    #[inline]
    fn into_cami_vec(self) -> Vec<Cami<T>> {
        unsafe { core::mem::transmute(self) }
    }
}
