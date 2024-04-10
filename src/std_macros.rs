macro_rules! std_wrap {
    ([$derived:tt] $wrapper_name:ident <$generics:tt> $item_type:ty) => {
        TODO - and do NOT add Clone/Debug
    };

    ($wrapper_name:ident <$generics:tt> $T:ty) => {
        /// @TODO t (item name) as a parameter/optional
        ///
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
        #[derive(::core::clone::Clone, ::core::fmt::Debug)]
        #[repr(transparent)]
        pub struct $wrapper_name<$generics> {
            t: $T,
        }
    };
}

macro_rules! std_partial_eq {
    ($wrapper_name:ident <$generics:tt> $T:ty) => {
        impl<$generics> ::core::cmp::PartialEq for $wrapper_name<$T>
        where
            $T: $crate::CPartialEq,
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                (T::LOCALITY.no_local() || self.t.eq_local(&other.t))
                    && (T::LOCALITY.no_non_local() || self.t.eq_non_local(&other.t))
            }

            #[inline]
            fn ne(&self, other: &Self) -> bool {
                T::LOCALITY.has_local() && !self.t.eq_local(&other.t)
                    || T::LOCALITY.has_non_local() && !self.t.eq_non_local(&other.t)
            }
        }
    };
}

macro_rules! std_eq {
    ($wrapper_name:ident <$generics:tt> $T:ty) => {
        impl<$generics> ::core::cmp::Eq for $wrapper_name<$T> where $T: $crate::CPartialEq {}
    };
}

macro_rules! std_partial_ord {
    ($wrapper_name:ident <$generics:tt> $T:ty) => {
        impl<$generics> ::core::cmp::PartialOrd for $wrapper_name<$T>
        where
            $T: $crate::COrd,
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                Some(self.t.cmp_full(&other.t))
            }

            #[inline]
            fn lt(&self, other: &Self) -> bool {
                self.t.cmp_full(&other.t) == ::core::cmp::Ordering::Less
            }
            #[inline]
            fn le(&self, other: &Self) -> bool {
                self.t.cmp_full(&other.t) != ::core::cmp::Ordering::Greater
            }
            #[inline]
            fn gt(&self, other: &Self) -> bool {
                self.t.cmp_full(&other.t) == ::core::cmp::Ordering::Greater
            }
            #[inline]
            fn ge(&self, other: &Self) -> bool {
                self.t.cmp_full(&other.t) != ::core::cmp::Ordering::Less
            }
        }
    };
}

macro_rules! std_ord {
    ($wrapper_name:ident <$generics:tt> $T:ty) => {
        impl<$generics> ::core::cmp::Ord for $wrapper_name<$T>
        where
            $T: $crate::COrd,
        {
            #[inline]
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.t.cmp_full(&other.t)
            }
        }
    };
}
