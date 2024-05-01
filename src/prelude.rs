pub use crate::{
    Cami, CamiOrd, CamiPartialEq, CamiPartialOrd, IntoCami, IntoCamiClone, IntoCamiCopy,
    IntoRefCami, IntoSliceCami, Locality,
};

#[cfg(feature = "alloc")]
mod a_prelude;

#[cfg(feature = "alloc")]
pub use a_prelude::*;

mod c_prelude;
pub use c_prelude::*;

// @TODO same two `pub use` for core & std
