#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "hint_assert_unchecked", feature(hint_assert_unchecked))]
#![cfg_attr(
    not(any(feature = "unsafe", feature = "unsafe_from_rust_source")),
    deny(unsafe_code)
)]
#![cfg_attr(feature = "deref_pure_trait", feature(deref_pure_trait))]

// @TODO in tests-only => dev dependency: use David Tolnay's rust version crate:
/*#cfg[(and(feature = "nightly", arch--...-))]
const NOT_SUPPORTED: () = {
    panic!("NOT_SUPPORTED")
};*/

pub use cami::{Cami, IntoCami, IntoCamiCopy};
pub use camigo_helpers::Locality;
pub use traits::{CamiOrd, CamiPartialEq, CamiPartialOrd};

pub mod prelude;
mod traits;

#[cfg(feature = "alloc")]
pub mod alloc;
mod cami;
pub mod core;
#[cfg(feature = "std")]
pub mod std;

#[cfg(feature = "alloc")]
extern crate alloc as rust_alloc;
