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

// TODO compile fail if feature = "nightly" if NOT on nightly

/// NOT a part of public API. Only for use by macro-generated code. Subject to change.
///
/// The main benefit: With this, we don't need to capture the wrapped type in `c_partial_eq` &
/// `c_ord when we apply those macros to a (`#[repr(transparent)]`) wrapper struct or tuple. See
/// also how we needed `$t_type:ty` (in commit `06cfc12`):
/// <https://github.com/peter-kehl/camigo/blob/06cfc120812179e71a291a92b9c1034a792551eb/src/macros_c.rs#L135>.
///
/// A smaller benefit: Less duplication in `c_partial_eq` & `c_ord` macros: no need for an
/// (anonymous) filler closure.
// This has to return a reference, hence "_ref" in its name.
#[doc(hidden)]
#[inline]
pub fn always_equal_ref<T>(_instance: &T) -> &() {
    &()
}
