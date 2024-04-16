#[cfg(feature = "unsafe")]
use core::ops::DerefPure;
use core::ops::{Deref, DerefMut};

#[macro_export]
macro_rules! ca_wrap {
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
        ca_wrap! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };
    // Default the derived trait impls
    ($($tt:tt)+) => {
        ca_wrap! {
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

#[macro_export]
macro_rules! ca_wrap_partial_eq {
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_path:path
     $(>$($generic_right:tt),+<)?

     // $locality is NOT an ident, so that we allow (const-time) expressions.
     { $t:tt @ $locality: expr }// The name of the only (wrapped) field, or 0 if tuple.

     $(where $($left:ty : $right:tt),+)?
     // Within each of the following two oval pairs (), repeat any of the THREE parts:
     // - `..._ident` for non-tuple structs, or
     // - `..._idx` for tuples, or
     // - [` ..._eq_closure`] for a boolean closure. Each closure must receive TWO parameters, for
     //   example `this` and `other`. Both parameters' type is a reference to the wrapped type. The
     //   closure compares the same chosen field in both references, and returns their equality.
     // - {` ..._get_closure`} for an accessor closure. Each closure must receive ONE parameter, for
     //   example `this` or `obj`. That parameter's type is a reference to the wrapped type. The
     //   closure returns (reference, or copy) of a chosen field, or a value based on that field if
     //   such a value is unique per the field's value.
     (
        $(
           $(
            $local_ident:ident
            $(. $($local_ident_ident:ident)? $($local_ident_idx:literal)?
             )*)?
           
           $(
            $local_idx:literal
            $(. $($local_idx_ident:ident)? $($local_idx_idx:literal)?
             )* )?

           $([$local_eq_closure:expr])?
           $({$local_get_closure:expr})?
        ),*
     )
     (
        $(
           $(
            $non_local_ident:ident
            $(. $($non_local_ident_ident:ident)? $($non_local_ident_idx:literal)?
             )*)?
           
           $(
            $non_local_idx:literal
            $(. $($non_local_idx_ident:ident)? $($non_local_idx_idx:literal)?
             )* )?

           $([$non_local_eq_closure:expr])?
           $({$non_local_get_closure:expr})?
        ),*
     )
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        $crate::CPartialEq for $struct_path $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            const LOCALITY: $crate::Locality = $locality;

            fn eq_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_local();
                true
                $(
                    //$(&& self.$t.$local_ident_first==other.$t.$local_ident_first)?
                    $(&& self.$t.$local_ident
                        $(.$($local_ident_ident)? $($local_ident_idx)?
                         )* ==
                         other.$t.$local_ident
                        $(.$($local_ident_ident)? $($local_ident_idx)?
                         )*
                    )?
                    //$(&& self.$t.$local_idx_first==other.$t.$local_idx_first)?
                    $(&& self.$t.$local_idx
                        $(.$($local_idx_ident)? $($local_idx_idx)?
                         )* ==
                         other.$t.$local_idx
                        $(.$($local_idx_ident)? $($local_idx_idx)?
                         )*
                    )?

                    $(&& $local_eq_closure(&self.$t, &other.$t))?
                    $(&& $local_get_closure(&self.$t)==$local_get_closure(&other.$t))?
                )*
            }

            fn eq_non_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_non_local();
                true
                $(
                    //$(&& self.$t.$non_local_ident_first==other.$t.$non_local_ident_first)?
                    $(&& self.$t.$non_local_ident
                        $(.$($non_local_ident_ident)? $($non_local_ident_idx)?
                         )* ==
                         other.$t.$non_local_ident
                        $(.$($non_local_ident_ident)? $($non_local_ident_idx)?
                         )*
                    )?
                    //$(&& self.$t.$non_local_idx_first==other.$t.$non_local_idx_first)?
                    $(&& self.$t.$non_local_idx
                        $(.$($non_local_idx_ident)? $($non_local_idx_idx)?
                         )* ==
                         other.$t.$non_local_idx
                        $(.$($non_local_idx_ident)? $($non_local_idx_idx)?
                         )*
                    )?

                    $(&& $non_local_eq_closure(&self.$t, &other.$t))?
                    $(&& $non_local_get_closure(&self.$t)==$non_local_get_closure(&other.$t))?
                )*
            }
        }
    };
}

/// For types OTHER than defined by `ca_wrap!` or `ca_wrap_tuple!`.`
///
/// See [ca_wrap_partial_eq].
#[macro_export]
macro_rules! ca_partial_eq {
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_path:path
     $(>$($generic_right:tt),+<)?

     $(where $($left:ty : $right:tt),+)?
     { $locality: expr }
     [$($local_ident:ident),* $($local_idx:literal),* $(@ $($local_eq_closure:expr),+)? $(=> $($local_get_closure:expr),+)?]
     [$($non_local_ident:ident),* $($non_local_idx:literal),* $(@ $($non_local_eq_closure:expr),+)? $(=> $($non_local_get_closure:expr),+)?]
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        $crate::CPartialEq for $struct_path $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            const LOCALITY: $crate::Locality = $locality;

            fn eq_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_local();
                true
                $(&& self.$local_ident==other.$local_ident)*
                $(&& self.$local_idx==other.$local_idx)*
                $(&& $($local_eq_closure(&self, &other))+)?
                $(&& $($local_get_closure(&self)==$local_get_closure(&other))+)?
            }

            fn eq_non_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_non_local();
                true
                $(&& self.$non_local_ident==other.$non_local_ident)*
                $(&& self.$non_local_idx==other.$non_local_idx)*
                $(&& $($non_local_eq_closure(&self, &other))+)?
                $(&& $($non_local_get_closure(&self)==$non_local_get_closure(&other))+)?
            }
        }
    };
}

#[macro_export]
macro_rules! ca_wrap_ord {
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_path:path
     $(>$($generic_right:tt),+<)?

     { $t:tt }// The name of the only (wrapped) field, or 0 if tuple.

     $(where $($left:ty : $right:tt),+)?
     // Within each of the following two square pairs [], use exactly one of the two repeated parts:
     // - the `..._ident` parts for non-tuple structs, and
     // - the `..._idx` parts for tuples.
     [$($local_ident:ident),* $($local_idx:literal),*]
     [$($non_local_ident:ident),* $($non_local_idx:literal),*]
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        $crate::COrd for $struct_path $(<$($generic_right),+>)?
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
    fn from(_value: CaWrap) -> Self {
        panic!()
    }
}
impl From<&str> for CaWrap {
    fn from(_value: &str) -> Self {
        panic!()
    }
}

ca_wrap! {
    pub CaWrap {
        t : u8
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

fn _into() {
    let _caw: CaWrap = "".into();
    let _caw: CaWrap = <&str>::into("");
}
fn _from() {
    let _caw = CaWrap::from("");
}

fn _deref(caw: &CaWrap) {
    let _ = caw.len();
}

ca_wrap! { [Clone, Debug] _CaWrap3 <T> {t : T }}
ca_wrap! { [Clone, Debug] _CaWrap4 <T:Sized> {t : T }}
ca_wrap! {
    [Clone, Debug]
    _CaWrap5 <T>
    where T: 'static {
        t : T
    }
}
ca_wrap! { pub CaWrapPub {pub t : u8}}

#[cfg(test)]
mod test_macros {
    #[cfg(feature = "alloc")]
    mod with_alloc {
        use alloc::vec::Vec;

        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        struct A {
            x: i32,
            v: Vec<i32>,
        }

        ca_wrap! {
            _CaWrap2 <A> {
                pub t : Vec<A>
            }
        }

        use crate::Locality;

        ca_wrap! { CaWrapA1 {t : A }}
        ca_wrap_partial_eq! {
            CaWrapA1 {
                t @ Locality::Both
            }
            ([|this: &A, other: &A| this.x==other.x]
            )
            (v)
        }
        ca_wrap_ord! {
            CaWrapA1 { t }
            [x]
            [v]
        }

        ca_wrap_tuple! { CaTupleGen1 <T> (pub T) where T: Sized}

        mod tuple_2 {
            use crate::Locality;
            use crate::macros::test_macros::with_alloc::A;
            use alloc::vec::Vec;
            //use alloc::string::String;

            ca_wrap_tuple! { CaTupleA2 (A) }
            fn get_v<'a>(wrap: &'a A) -> &'a Vec<i32> {
                &wrap.v
            }
            ca_wrap_partial_eq! {
                <'a>
                CaTupleA2 {
                    0 @ Locality::Both
                }
                ( {|obj: &A| obj.x}
                )
                // We can't specify return lifetimes here:
                //
                // [@ |obj: &'l A| -> &'l Vec<i32> {&obj.v}]
                //
                // Hence a separate function:
                ( {get_v} )
            }
            ca_wrap_ord! {
                CaTupleA2 { 0 }
                [x]
                [v]
            }
        }

        mod party {
            use crate::Locality;
            //use alloc::vec::Vec;
            use alloc::string::String;
            
            type Amount = u16;

            #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
            struct Food {
                name: String,
                amount: Amount
            }

            #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
            struct FoodList {
                common: Food,
                gluten_free: Food,
                vegan: Food
            }

            ca_wrap! {
                pub FoodListCa {
                    t : FoodList
                }
            }
            ca_wrap_partial_eq! {
                FoodListCa {
                    t @ Locality::Both
                }
                (
                    common.amount,
                    {|food_list: &FoodList| food_list.gluten_free.amount},
                    [|this: &FoodList, other: &FoodList| this.vegan.name==other.vegan.name]
                )
                // @TODO empty, or have a special rule to capture that:
                (   common.name, gluten_free.name,
                    [|this: &FoodList, other: &FoodList| this.vegan.name==other.vegan.name]
                )
            }
        }
    }
}
