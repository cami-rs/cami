macro_rules! ca_wrap {
    // Default the derived trait impls,, and the wrapped field name to `t`
    ($struct_name:ident
     $(<$($generic:tt $(: $bound:tt)?),+>)?
     : $T:ty
     $(where $($left:ty : $right:tt),+)?
    ) => {
        ca_wrap! { [
            ::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::Eq, ::core::cmp::Ord,
            ::core::cmp::PartialEq, ::core::cmp::PartialOrd
            ] $struct_name $(<$($generic $(: $bound)?),+>)? t : $T $(where $($left : $right),+)?
        }
    };
    // NOT adding Clone/Debug/Eq/Ord/PartialEq/PartialOrd to $derived
    ([$($($derived:path),+)?]
     $struct_name:ident
     $(<$($generic:tt $(: $bound:tt)?),+>)?
     $t:ident
     : $T:ty
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
        pub struct $struct_name$(<$($generic $(: $bound)?)+>)?
        $(where $($left : $right),+)? {
            $t: $T,
        }
    };
}

macro_rules! ca_wrap_partial_eq {
    // Default the wrapped field name to `t`
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_name:ident
     $(<$($generic_right:tt),+>)?
     $(where $($left:ty : $right:tt),+)?
     $locality: ident // if this were a (const) `expr`, then we need a separator afterwards
     // Use exactly one of the following two "optional" parts
     $([$($local_ident:ident),*]
       [$($non_local_ident:ident),*]
     )?
     $(($local_expr:expr)
       ($non_local_expr:expr)
     )?
    ) => {
        ca_wrap_partial_eq! {
            $(<$($generic_left $(: $bound)?),+>)?
            $struct_name t
            $(<$($generic_right),+>)?
            $(where $($left : $right),+)?
            $locality
            $([$($local_ident),*]
              [$($non_local_ident),*]
            )?
            $(($local_expr)
              ($non_local_expr)
            )?
        }
    };
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_name:ident
     $(<$($generic_right:tt),+>)?
     
     //@TODO vis modifier; private by default
     $t:ident

     $(where $($left:ty : $right:tt),+)?
     $locality: ident // if this were a (const) `expr`, then we need a separator afterwards - TODO use separator =>

     // Use exactly one of the following two "optional" parts
     //
     // @TODO allow number literals, as tuple item indices
     $([$($local_ident:ident),*]
       [$($non_local_ident:ident),*]
     )?
     $(//@TODO optional $other:ident
       //
       // Because of macro hygiene, the following two can't be :expr - then they couldn't access
       // `self`. Hence :tt.
       ($local_expr:tt)
       ($non_local_expr:tt)
     )?
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        $crate::CPartialEq for $struct_name $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            const LOCALITY: $crate::Locality = $crate::Locality::$locality;

            fn eq_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_local();
                $(
                    true $(&& self.$t.$local_ident==other.$t.$local_ident)*
                )?
                $($local_expr)?
            }

            fn eq_non_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_non_local();
                $(
                    true $(&& self.$t.$non_local_ident==other.$t.$non_local_ident)*
                )?
                $($non_local_expr)?
            }
        }
    };
}

ca_wrap! { CaWrap : u8}
ca_wrap! { CaWrap2 <A> : Vec<A> }
ca_wrap! { [Clone, Debug] CaWrap3 <T> t : T }
ca_wrap! { [Clone, Debug] CaWrap4 <T:Sized> t : T }
ca_wrap! { [Clone, Debug] CaWrap5 <T> t : T where T: 'static}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct A {
    x: i32,
    v: Vec<i32>,
}
ca_wrap! { CaWrapA1 : A }
ca_wrap_partial_eq! {
    CaWrapA1 Both
    [x]
    [v]
}

ca_wrap! { CaWrapAwithExpressions : A }
// This fails because of macro hygiene. We can't pass expressions and have them "evaluated" within
// the context that a macro generates.
ca_wrap_partial_eq! {
    CaWrapAwithExpressions Both
    (self.t.x==other.t.x)
    (self.t.v==other.t.v)
}
