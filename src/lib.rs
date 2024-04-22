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

pub use locality::Locality;
pub use macros_c::always_equal_ref;
pub use slice::Slice;
pub use traits::{COrd, CPartialEq};

#[cfg(feature = "alloc")]
extern crate alloc;

mod locality;
#[macro_use]
mod macros_c;
pub mod prelude;
mod primitives;

#[macro_use]
mod pure_local_macros;
mod pure_local_impls;
mod slice;
#[macro_use]
mod macros_s;
mod std_wrap;
mod string;
mod traits;
