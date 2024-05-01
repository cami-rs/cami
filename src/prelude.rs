pub use crate::{
    Cami, CamiOrd, CamiPartialEq, CamiPartialOrd, IntoCami, IntoCamiClone, IntoCamiCopy,
    IntoRefCami, IntoSliceCami, Locality,
};

#[cfg(all(feature = "wrappers", feature = "alloc"))]
mod a_prelude;

#[cfg(all(feature = "wrappers", feature = "alloc"))]
pub use a_prelude::*;

#[cfg(feature = "wrappers")]
mod c_prelude;
#[cfg(feature = "wrappers")]
pub use c_prelude::*;

// @TODO same two `pub use` for core & std
