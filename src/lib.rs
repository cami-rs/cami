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

pub use camigo_helpers::Locality;
pub use impls::*;
pub use macros::mac_c::always_equal_ref;
pub use traits::{COrd, CPartialEq};

mod impls;
#[macro_use]
mod macros;
pub mod prelude;
mod std_wrap;
mod traits;

#[cfg(feature = "alloc")]
extern crate alloc;
