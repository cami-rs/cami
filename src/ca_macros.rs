macro_rules! ca_wrap_struct {
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
        /// @TODO replace $item_type and $crate in this doc:
        ///
        /// A (zero cost/low cost) wrapper & bridge that implements [::core::cmp::PartialEq]
        /// forwarding to [$crate::CPartialEq] and [::core::cmp::Ord] forwarding to [$crate::COrd]
        /// of `$item_type`.
        ///
        /// These implementations are useful, and for many data types it may speed up searches etc.
        /// (anything based on comparison).
        ///
        /// NO cache-specific benefit for [$crate::Slice]'s cache-aware methods (`binary_search_cf`
        /// etc.) themselves!
        ///
        /// Usable for BENCHMARKING. It allows us to run slice's `binary_search`:
        /// `<[$item_type]>::binary_search(self, given)` using the full comparison, and benchmark
        /// against cache-aware [$crate::Slice]'s `binary_search_cf` etc.
        ///
        /// [::core::cmp::PartialEq] is implemented NOT by forwarding to [::core::cmp::PartialEq]'s
        /// `eq` and `ne` of `$item_type`, but by forwarding to[$crate::COrd]'s `cmp_local`] and
        /// `cmp_non_local`` of `$item_type` instead. (Hence `$item_type` itself doesn't need to be
        /// [::core::cmp::PartialEq] or [::core::cmp::Ord].)
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
        ca_wrap_struct! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };
    // Default the derived trait impls
    ($($tt:tt)+) => {
        ca_wrap_struct! {
            @
            [
            ::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::Eq, ::core::cmp::Ord,
            ::core::cmp::PartialEq, ::core::cmp::PartialOrd
            ]
            $($tt)+
        }
    };
}

macro_rules! ca_wrap_tuple {
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
        /// @TODO replace $item_type and $crate in this doc:
        ///
        /// A (zero cost/low cost) wrapper & bridge that implements [::core::cmp::PartialEq]
        /// forwarding to [$crate::CPartialEq] and [::core::cmp::Ord] forwarding to [$crate::COrd]
        /// of `$item_type`.
        ///
        /// These implementations are useful, and for many data types it may speed up searches etc.
        /// (anything based on comparison).
        ///
        /// NO cache-specific benefit for [$crate::Slice]'s cache-aware methods (`binary_search_cf`
        /// etc.) themselves!
        ///
        /// Usable for BENCHMARKING. It allows us to run slice's `binary_search`:
        /// `<[$item_type]>::binary_search(self, given)` using the full comparison, and benchmark
        /// against cache-aware [$crate::Slice]'s `binary_search_cf` etc.
        ///
        /// [::core::cmp::PartialEq] is implemented NOT by forwarding to [::core::cmp::PartialEq]'s
        /// `eq` and `ne` of `$item_type`, but by forwarding to[$crate::COrd]'s `cmp_local`] and
        /// `cmp_non_local`` of `$item_type` instead. (Hence `$item_type` itself doesn't need to be
        /// [::core::cmp::PartialEq] or [::core::cmp::Ord].)
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
        ca_wrap_tuple! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };
    // Default the derived trait impls
    ($($tt:tt)+) => {
        ca_wrap_tuple! {
            @
            [
            ::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::Eq, ::core::cmp::Ord,
            ::core::cmp::PartialEq, ::core::cmp::PartialOrd
            ]
            $($tt)+
        }
    };
}

macro_rules! ca_wrap_cpartial_eq {
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
     // Within each of the following two square pairs [], use exactly one of the two repeated parts:
     // - the `..._ident` parts for non-tuple structs, and
     // - the `..._idx` parts for tuples.
     [$($local_ident:ident),* $($local_idx:literal),*]
     [$($non_local_ident:ident),* $($non_local_idx:literal),*]
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
            }

            fn eq_non_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_non_local();
                true
                $(&& self.$t.$non_local_ident==other.$t.$non_local_ident)*
                $(&& self.$t.$non_local_idx==other.$t.$non_local_idx)*
            }
        }
    };
}

macro_rules! ca_wrap_cord {
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
                Self::LOCALITY.debug_reachable_for_local();
                let result = ::core::cmp::Ordering::Equal;
                // LLVM should be able to optimize away the first comparison of
                // result==::core::cmp::Ordering::Equal
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    let result = self.$t.$local_ident.cmp(other.$t.$local_ident);
                )*
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    let result = self.$t.$local_idx.cmp(other.$t.$local_idx);
                )*
                $result
            }

            fn cmp_non_local(&self, other: &Self) -> ::core::cmp::Ordering {
                Self::LOCALITY.debug_reachable_for_non_local();
                let result = ::core::cmp::Ordering::Equal;
                // LLVM should be able to optimize away the first comparison of
                // result==::core::cmp::Ordering::Equal
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    let result = self.$t.$non_ident.cmp(other.$t.$non_local_ident);
                )*
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    let result = self.$t.$non_local_idx.cmp(other.$t.$non_local_idx);
                )*
                $result
            }
            // NOT re-implemeting cmp_full(...), but using its default impl.
        }
    };
}

ca_wrap_struct! { pub CaWrap {t : u8}}
ca_wrap_struct! { CaWrap2 <A> {pub t : Vec<A> }}
ca_wrap_struct! { [Clone, Debug] CaWrap3 <T> {t : T }}
ca_wrap_struct! { [Clone, Debug] CaWrap4 <T:Sized> {t : T }}
ca_wrap_struct! {
    [Clone, Debug]
    CaWrap5 <T>
    where T: 'static {
        t : T
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct A {
    x: i32,
    v: Vec<i32>,
}

mod test_macros {
    use crate::ca_macros::A;
    use crate::Locality::Both;

    ca_wrap_struct! { CaWrapA1 {t : A }}
    ca_wrap_cpartial_eq! {
        CaWrapA1 t Both =>
        [x]
        [v]
    }

    ca_wrap_tuple! { CaTupleGen1 <T> (pub T) where T: Sized}

    ca_wrap_tuple! { CaTupleA2 (A) }
    ca_wrap_cpartial_eq! {
        CaTupleA2 0 Both =>
        [x]
        [v]
    }
}
/*
ca_wrap_struct! { CaWrapAwithExpressions : A }

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
