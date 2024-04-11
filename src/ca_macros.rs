macro_rules! ca_wrap {
    ($struct_name:ident $(<$($generics:tt),+>)? : $T:ty $(where $($left:ty : $right:tt),+)?) => {
        ca_wrap! { [
            ::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::Eq, ::core::cmp::Ord,
            ::core::cmp::PartialEq, ::core::cmp::PartialOrd
            ] $struct_name $(<$($generics),+>)? t : $T $(where $($left : $right),+)?
        }
    };
    // NOT adding Clone/Debug/Eq/Ord/PartialEq/PartialOrd to $derived
    ([$($($derived:path),+)?] $struct_name:ident $(<$($generics:tt),+>)? $t:ident : $T:ty $(where $($left:ty : $right:tt),+)?) => {
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
        pub struct $struct_name$(<$($generics)+>)?
        $(where $($left : $right),+)? {
            $t: $T,
        }
    };
}

ca_wrap! { CaWrap : u8}
ca_wrap! { CaWrap2 <A> : Vec<A> }
ca_wrap! { [Clone, Debug] CaWrap3 <T> t : T }
ca_wrap! { [Clone, Debug] CaWrap4 <T> t : T where T: 'static}
