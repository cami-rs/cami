#[cfg(feature = "unsafe")]
use core::ops::DerefPure;
use core::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! ca_struct {
    // An INTERNAL rule
    (@[$($($derived:path),+)?]
     $struct_vis:vis
     $struct_name:ident
     $(<$($generic:tt $(: $bound:tt)?),+>)?
     $(where $($left:ty : $right:tt),+)?
     {
     $field_vis:vis
     $t:ident
     : $T:ty
     }

    ) => {
        /// A zero cost (transparent) wrapper struct around a given type. For use with other macros
        /// from this crate.
        $(#[derive($($derived),+)])?
        #[repr(transparent)]
        $struct_vis struct $struct_name $(<$($generic $(: $bound)?),+>)?
        $(where $($left : $right),+)?
        {
            $field_vis $t: $T
        }
    };
    // The following prevents recursion on incorrect macro invocation
    (@
     $($tt:tt)+
    ) => {
        INCORRECT_MACRO_INVOCATION
        $($tt:tt)+
    };
    // NOT adding Clone/Debug/Eq/Ord/PartialEq/PartialOrd to $derived
    ([$($($derived:path),+)?]
     $($tt:tt)+
    ) => {
        ca_struct! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };
    // Default the derived trait impls
    ($($tt:tt)+) => {
        ca_struct! {
            @
            [
            ::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::Eq, ::core::cmp::Ord,
            ::core::cmp::PartialEq, ::core::cmp::PartialOrd
            ]
            $($tt)+
        }
    };
}

#[macro_export]
macro_rules! ca_tuple {
    // An INTERNAL rule
    (@
     [$($($derived:path),+)?]
     $struct_vis:vis
     $struct_name:ident
     $(<$($generic:tt $(: $bound:tt)?),+>)?
     (
     $field_vis:vis
     $T:ty
     )
     $(where $($left:ty : $right:tt),+)?
    ) => {
        /// A zero cost (transparent) wrapper struct around a given type. For use with other macros
        /// from this crate.
        $(#[derive($($derived),+)])?
        #[repr(transparent)]
        $struct_vis struct $struct_name $(<$($generic $(: $bound)?),+>)?
        (
            $field_vis $T
        )
        $(where $($left : $right),+)?
        ;
    };
    // The following prevents recursion on incorrect macro invocation
    (@
     $($tt:tt)+
    ) => {
        INCORRECT_MACRO_INVOCATION
        $($tt:tt)+
    };
    // NOT adding Clone/Debug/Eq/Ord/PartialEq/PartialOrd to $derived
    ([$($($derived:path),+)?]
     $($tt:tt)+
    ) => {
        ca_tuple! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };
    // Default the derived trait impls
    ($($tt:tt)+) => {
        ca_tuple! {
            @
            [
            ::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::Eq, ::core::cmp::Ord,
            ::core::cmp::PartialEq, ::core::cmp::PartialOrd
            ]
            $($tt)+
        }
    };
}

#[macro_export]
macro_rules! ca_partial_eq {
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_name:ident
     $(<$($generic_right:tt),+>)?

     $t:tt // The name of the only (wrapped) field, or 0 if tuple.

     $(where $($left:ty : $right:tt),+)?
     // $locality is NOT an ident, so that we allow (const-time) expressions.
     //
     // Because it's an `expr`, we need `=>` afterwards.
     $locality: expr
     =>
     // Within each of the following two square pairs [], use exactly one of the three repeated
     // parts:
     // - `..._ident` parts for non-tuple structs, and
     // - `..._idx` parts for tuples.
     // - `@` followed with ` ..._eq_expr` parts for compound expressions. Those must use `self` and
     //   `other`, and they muse yield a bool.

     // TODO instead of :ident, consider :tt, and see if that covers expressions/function calls. Or,
     // have that as a 3rd repetitive part $($local_expr:tt),*
     [$($local_ident:ident),* $($local_idx:literal),* $(@ $($local_eq_closure:expr)*)?]
     [$($non_local_ident:ident),* $($non_local_idx:literal),* $(@ $($non_local_eq_closure:expr)*)?]
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        $crate::CPartialEq for $struct_name $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            const LOCALITY: $crate::Locality = $locality;

            fn eq_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_local();
                true
                $(&& self.$t.$local_ident==other.$t.$local_ident)*
                $(&& self.$t.$local_idx==other.$t.$local_idx)*
                $(&& $($local_eq_closure(&self.$t, &other.$t))*)?
            }

            fn eq_non_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_non_local();
                true
                $(&& self.$t.$non_local_ident==other.$t.$non_local_ident)*
                $(&& self.$t.$non_local_idx==other.$t.$non_local_idx)*
                $(&& $($non_local_eq_closure(&self.$t, &other.$t))*)?
            }
        }
    };
}

#[macro_export]
macro_rules! ca_ord {
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_name:ident
     $(<$($generic_right:tt),+>)?

     $t:tt // The name of the only (wrapped) field, or 0 if tuple.

     $(where $($left:ty : $right:tt),+)?
     // Within each of the following two square pairs [], use exactly one of the two repeated parts:
     // - the `..._ident` parts for non-tuple structs, and
     // - the `..._idx` parts for tuples.
     [$($local_ident:ident),* $($local_idx:literal),*]
     [$($non_local_ident:ident),* $($non_local_idx:literal),*]
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        $crate::COrd for $struct_name $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            fn cmp_local(&self, other: &Self) -> ::core::cmp::Ordering {
                use crate::CPartialEq;
                Self::LOCALITY.debug_reachable_for_local();
                let result = ::core::cmp::Ordering::Equal;
                // LLVM should be able to optimize away the first comparison of
                // result==::core::cmp::Ordering::Equal
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    let result = (&self.$t.$local_ident).cmp(&other.$t.$local_ident);
                )*
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    let result = (&self.$t.$local_idx).cmp(&other.$t.$local_idx);
                )*
                result
            }

            fn cmp_non_local(&self, other: &Self) -> ::core::cmp::Ordering {
                use crate::CPartialEq;
                Self::LOCALITY.debug_reachable_for_non_local();
                let result = ::core::cmp::Ordering::Equal;
                // LLVM should be able to optimize away the first comparison of
                // result==::core::cmp::Ordering::Equal
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    let result = (&self.$t.$non_local_ident).cmp(&other.$t.$non_local_ident);
                )*
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    let result = (&self.$t.$non_local_idx).cmp(&other.$t.$non_local_idx);
                )*
                result
            }
            // NOT re-implemeting cmp_full(...), but using its default impl.
        }
    };
}

// @TODO
impl From<CaWrap> for &str {
    fn from(value: CaWrap) -> Self {
        panic!()
    }
}
impl From<&str> for CaWrap {
    fn from(value: &str) -> Self {
        panic!()
    }
}

// @TODO
impl Deref for CaWrap {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        panic!()
    }
}
impl DerefMut for CaWrap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!()
    }
}
#[cfg(feature = "unsafe")]
unsafe impl DerefPure for CaWrap {}

fn into() {
    let caw: CaWrap = "".into();
    let caw: CaWrap = <&str>::into("");
}
fn from() {
    let caw = CaWrap::from("");
}

fn deref(caw: &CaWrap) {
    let _ = caw.len();
}

ca_struct! { pub CaWrap {t : u8}}
ca_struct! { [Clone, Debug] CaWrap3 <T> {t : T }}
ca_struct! { [Clone, Debug] CaWrap4 <T:Sized> {t : T }}
ca_struct! {
    [Clone, Debug]
    CaWrap5 <T>
    where T: 'static {
        t : T
    }
}
ca_struct! { pub CaWrapPub {pub t : u8}}

mod test_macros {
    #[cfg(feature = "alloc")]
    mod with_alloc {
        use alloc::vec::Vec;

        ca_struct! { CaWrap2 <A> {pub t : Vec<A> }}

        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        struct A {
            x: i32,
            v: Vec<i32>,
        }

        use crate::Locality::Both;

        ca_struct! { CaWrapA1 {t : A }}
        ca_partial_eq! {
            CaWrapA1 t Both =>
            [x]
            [v]
        }
        ca_ord! {
            CaWrapA1 t
            [x]
            [v]
        }

        ca_tuple! { CaTupleGen1 <T> (pub T) where T: Sized}

        ca_tuple! { CaTupleA2 (A) }
        ca_partial_eq! {
            CaTupleA2 0 Both =>
            [@ |left: &A, right: &A| left.x == right.x]
            [@ |left: &A, right: &A| left.v == right.v]
        }
        ca_ord! {
            CaTupleA2 0
            [x]
            [v]
        }
    }
}
/*
ca_struct! { CaWrapAwithExpressions : A }

// This failed because of macro hygiene. We can't pass expressions and have them "evaluated" within
// the context that a macro generates.
//
// See https://github.com/peter-kehl/camigo/commit/dbebbe10c0c341b55f2e5f51ae81e52b095dd049 for
// ca_wrap_struct_partial_eq back then.
ca_wrap_struct_partial_eq! {
    CaWrapAwithExpressions Both
    (self.t.x==other.t.x)
    (self.t.v==other.t.v)
}*/
