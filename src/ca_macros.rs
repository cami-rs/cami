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
     // @TODO test with moving the "where" part before field_vis
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

macro_rules! ca_wrap_struct_partial_eq {
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_name:ident
     $(<$($generic_right:tt),+>)?

     $t:tt // Name of the only (wrapped) field, or 0 if tuple

     $(where $($left:ty : $right:tt),+)?
     $locality: ident // if this were a (const) `expr`, then we need a separator afterwards - TODO use separator =>

     // Within each of the following two square pairs [], use exactly one of the two repeated parts.
     $([$($local_ident:ident),* $($local_idx:literal),*]
       [$($non_local_ident:ident),* $($non_local_idx:literal),*]
     )?
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        $crate::CPartialEq for $struct_name $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            const LOCALITY: $crate::Locality = $crate::Locality::$locality;

            fn eq_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_local();
                $(
                    true
                    $(&& self.$t.$local_ident==other.$t.$local_ident)*
                    $(&& self.$t.$local_idx==other.$t.$local_idx)*
                )?
            }

            fn eq_non_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_non_local();
                $(
                    true
                    $(&& self.$t.$non_local_ident==other.$t.$non_local_ident)*
                    $(&& self.$t.$non_local_idx==other.$t.$non_local_idx)*
                )?
            }
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
ca_wrap_struct! { CaWrapA1 {t : A }}
ca_wrap_struct_partial_eq! {
    CaWrapA1 t Both
    [x]
    [v]
}

ca_wrap_tuple! { CaTupleGen1 <T> (pub T) where T: Sized}

ca_wrap_tuple! { CaTupleA2 (A) }
ca_wrap_struct_partial_eq! {
    CaTupleA2 0 Both
    [x]
    [v]
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
