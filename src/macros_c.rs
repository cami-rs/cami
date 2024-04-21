#[cfg(feature = "unsafe")]
use core::ops::DerefPure;
use core::ops::{Deref, DerefMut};

#[cfg(test)]
pub mod tests {
    #[cfg(feature = "alloc")]
    pub mod party;
}

#[macro_export]
macro_rules! c_wrap {
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
        c_wrap! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };
    // Default the derived trait impls
    ($($tt:tt)+) => {
        c_wrap! {
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
macro_rules! c_wrap_tuple {
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
        c_wrap_tuple! {
            @
            [$($($derived),+)?]
            $($tt)+
        }
    };
    // Default the derived trait impls
    ($($tt:tt)+) => {
        c_wrap_tuple! {
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
macro_rules! c_partial_eq {
    (
     $(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_path:path
     $(>$($generic_right:tt),+<)?

     // $locality is NOT an ident, so that we allow (const-time) expressions.
     { $locality: expr
       // Only for 1-field wrapper types (newtype):
       //
       // The name of the only (wrapped) field, or 0 if tuple, for example if the struct has been
       // defined by `c_wrap!` or `c_wrap_tuple!`.` Otherwise $t is empty.
       => $t:tt : $t_type:ty
     }

     $(where $($left:ty : $right:tt),+)?
     // The following two or three square pairs [] represent local fields, non-local fields, and
     // optional: fields that themselves implement [CPartialEq]. And of those two or three kinds of
     // fields may be "deep fields".
     //
     // Within each square pair [], repeat any of the four parts (or three parts in case of "deep
     // fields"):
     // - `..._ident` for non-tuple structs, or
     // - `..._idx` for tuples, or
     // - (` ..._eq_closure`) for a boolean closure - except for "deep fields". Each closure must
     //   receive two parameters, for example `this` and `other`. Both parameters' type is a
     //   reference to the wrapped type (if you provided `$t`), or `Self` (if no `$t`). The closure
     //   compares the same chosen field in both references, and returns their equality.
     // - {` ..._get_closure`} for an accessor closure. Each closure must receive one parameter, for
     //   example `this` or `obj`. That parameter's type is a reference to the wrapped type (if you
     //   provided `$t`), or `Self` (if no `$t`). The closure returns (reference, or copy) of a
     //   chosen field, or a value based on that field if such a value is unique per the field's
     //   value.
    [
        $( $local:tt )*
    ]
    [
        $( $non_local:tt )*
    ]
    $(
    [
        $( $deep:tt )*
    ]
    )?
    ) => {
        $crate::c_partial_eq_full_squares! {
            $(<$($generic_left $(: $bound)?),+>)?
            $struct_path
            $(>$($generic_right),+<)?

            { $locality
              => $t
            }

            $(where $($left : $right),+)?
            [
                // Injecting a const `true`-generating closure. Without this, handling empty square
                // pair [] was extremely difficult because of "ambiguity: multiple successful
                // parses" (because we need a zero-or-more repetitive block that can match empty
                // content).
                {|_instance: &$t_type| &true},
                $( $local )*
            ]
            [
                {|_instance: &$t_type| &true},
                $( $non_local )*
            ]
            $(
            [
                {|_instance: &$t_type| &true},
                $( $deep )*
            ]
            )?
        }
    };

    (
     $(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_path:path
     $(>$($generic_right:tt),+<)?

     { $locality: expr
     }

     $(where $($left:ty : $right:tt),+)?
    [
        $( $local:tt )*
    ]
    [
        $( $non_local:tt )*
    ]
    $(
    [
        $( $deep:tt )*
    ]
    )?
    ) => {
        $crate::c_partial_eq_full_squares! {
            $(<$($generic_left $(: $bound)?),+>)?
            $struct_path
            $(>$($generic_right),+<)?

            { $locality
            }

            $(where $($left : $right),+)?
            [
                {|_instance: &Self| &true},
                $( $local )*
            ]
            [
                {|_instance: &Self| &true},
                $( $non_local )*
            ]
            $(
            [
                {|_instance: &Self| &true},
                $( $deep )*
            ]
            )?
        }
    };
}

#[macro_export]
macro_rules! c_partial_eq_full_squares {
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_path:path
     $(>$($generic_right:tt),+<)?

     { $locality: expr
       $( => $t:tt )?
     }

     $(where $($left:ty : $right:tt),+)?
     [
        $(
           $(($local_eq_closure:expr)
            )?

           $({$local_get_closure:expr}
            )?

           // This is necessary only to match fields/chains of fields that have the first/top level
           // field a numeric index to a tuple. (We can't match it with :literal, because then the
           // generated code fails to compile due to scope/context mixed in.)
           $(
            $( .
               $local_dotted:tt
               $( (
                   // This does NOT match "expressions" passed to functions. It's here ONLY to
                   // capture a pair of PARENS with NO parameters within.
                   $( $local_within_parens:tt )?
                  )
               )?
            )+
           )?

           $(
               $local_ident:ident
               $( (
                   // This does NOT match "expressions" passed to functions. It's here ONLY to
                   // capture a pair of PARENS with NO parameters within.
                   $( $local_after_ident_within_parens:tt )?
                  )
               )?
               // Same as "local_dotted" part above.
               $( .
                  $( $local_after_ident_dotted:tt )?
                  $( (
                       // This does NOT match "expressions" passed to functions. It's here ONLY to
                       // capture a pair of PARENS with NO parameters within.
                       $( $local_after_ident_dotted_within_parens:tt )?
                     )
                  )?
               )*
           )?
        ),*
     ]
     [
        $(
           $(($non_local_eq_closure:expr)
            )?

           $({$non_local_get_closure:expr}
            )?

           $(
            $( .
               $non_local_dotted:tt
               $( (
                   $( $non_local_within_parens:tt )?
                  )
               )?
            )+
           )?

           $(
               $non_local_ident:ident
               $( (
                   $( $non_local_after_ident_within_parens:tt )?
                  )
               )?
               $( .
                  $( $non_local_after_ident_dotted:tt )?
                  $( (
                       $( $non_local_after_ident_dotted_within_parens:tt )?
                     )
                  )?
               )*
           )?
        ),*
     ]
     $(
     [
        $(
           $({$deep_get_closure:expr}
            )?

           $(
            $( .
               $deep_dotted:tt
               $( (
                   $( $deep_within_parens:tt )?
                  )
               )?
            )+
           )?

           $(
               $deep_ident:ident
               $( (
                   $( $deep_after_ident_within_parens:tt )?
                  )
               )?
               $( .
                  $( $deep_after_ident_dotted:tt )?
                  $( (
                       $( $deep_after_ident_dotted_within_parens:tt )?
                     )
                  )?
               )*
           )?
        ),*
     ]
     )?
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        $crate::CPartialEq for $struct_path $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            const LOCALITY: $crate::Locality = $locality;

            fn eq_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_local();
                let this = &self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                true
                $(
                    $(&& $local_eq_closure(&this, &other)
                     )?

                    $(&& $local_get_closure(&this)==$local_get_closure(&other)
                     )?

                    $(&& this  $( .
                                  $local_dotted
                                  $( (
                                       $( $local_within_parens )?
                                     )
                                   )?
                                )+
                        ==
                         other $( .
                                  $local_dotted
                                  $( (
                                       $( $local_within_parens )?
                                     )
                                   )?
                                )+
                    )?

                    $(&& this  .
                               $local_ident
                               $( (
                                    $( $local_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $local_after_ident_dotted )?
                                  $( (
                                       $( $local_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        ==
                         other  .
                               $local_ident
                               $( (
                                    $( $local_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $local_after_ident_dotted )?
                                  $( (
                                       $( $local_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                    )?
                )*
                $(
                $(
                    $(&& $deep_get_closure(&this).eq_local($deep_get_closure(&other))
                     )?

                    $(&& this  $( .
                                  $deep_dotted
                                  $( (
                                       $( $deep_within_parens )?
                                     )
                                   )?
                                )+
                        .eq_local( &
                         other $( .
                                  $deep_dotted
                                  $( (
                                       $( $deep_within_parens )?
                                     )
                                   )?
                                )+
                        )
                    )?

                    $(&& this  .
                               $deep_ident
                               $( (
                                    $( $deep_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $deep_after_ident_dotted )?
                                  $( (
                                       $( $deep_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        .eq_local( &
                         other  .
                               $deep_ident
                               $( (
                                    $( $deep_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $deep_after_ident_dotted )?
                                  $( (
                                       $( $deep_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        )
                    )?
                )*
                )?
            }

            fn eq_non_local(&self, other: &Self) -> bool {
                Self::LOCALITY.debug_reachable_for_non_local();
                let this = &self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                true
                $(
                    $(&& $non_local_eq_closure(&this, &other)
                     )?

                    $(&& $non_local_get_closure(&this)==$non_local_get_closure(&other)
                     )?

                    $(&& this  $( .
                                  $non_local_dotted
                                  $( (
                                       $( $non_local_within_parens )?
                                     )
                                   )?
                                )+
                        ==
                         other $( .
                                  $non_local_dotted
                                  $( (
                                       $( $non_local_within_parens )?
                                     )
                                   )?
                                )+
                    )?

                    $(&& this  .
                               $non_local_ident
                               $( (
                                    $( $non_local_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $non_local_after_ident_dotted )?
                                  $( (
                                       $( $non_local_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        ==
                         other  .
                               $non_local_ident
                               $( (
                                    $( $non_local_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $non_local_after_ident_dotted )?
                                  $( (
                                       $( $non_local_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                    )?
                )*
                $(
                $(
                    $(&& $deep_get_closure(&this).eq_non_local($deep_get_closure(&other))
                     )?

                    $(&& this  $( .
                                  $deep_dotted
                                  $( (
                                       $( $deep_within_parens )?
                                     )
                                   )?
                                )+
                        .eq_non_local( &
                         other $( .
                                  $deep_dotted
                                  $( (
                                       $( $deep_within_parens )?
                                     )
                                   )?
                                )+
                        )
                    )?

                    $(&& this  .
                               $deep_ident
                               $( (
                                    $( $deep_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $deep_after_ident_dotted )?
                                  $( (
                                       $( $deep_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        .eq_non_local( &
                         other  .
                               $deep_ident
                               $( (
                                    $( $deep_after_ident_within_parens )?
                                  )
                               )?
                               $( .
                                  $( $deep_after_ident_dotted )?
                                  $( (
                                       $( $deep_after_ident_dotted_within_parens )?
                                     )
                                   )?
                                )*
                        )
                    )?
                )*
                )?
            }
        }
    };
}

// @TODO ca_* ---> c_*
/// Like [c_partial_eq], but for [COrd].
#[macro_export]
macro_rules! c_ord {
    ($(<$($generic_left:tt $(: $bound:tt)?),+>)?
     $struct_path:path
     $(>$($generic_right:tt),+<)?

     $({
       // The name of the only (wrapped) field, or 0 if tuple, for example if the struct has been
       // defined by `c_wrap!` or `c_wrap_tuple!`.` Otherwise $t is empty.
       $t:tt
     })?

     $(where $($left:ty : $right:tt),+)?
     // TODO update this doc.
     //
     // Within each of the following two square pairs [], repeat any of the THREE parts:
     // - `..._ident` for non-tuple structs, or
     // - `..._idx` for tuples, or
     // - (` ..._cmp_closure`) for a boolean closure. Each closure must receive TWO parameters, for
     //   example `this` and `other`. Both parameters' type is a reference to the wrapped type (if
     //   you provided `$t`), or `Self` (if no `$t`). The closure compares the same chosen field in
     //   both references, and returns their .cmp(&...).
     // - {` ..._get_closure`} for an accessor closure. Each closure must receive ONE parameter, for
     //   example `this` or `obj`. That parameter's type is a reference to the wrapped type (if you
     //   provided `$t`), or `Self` (if no `$t`). The closure returns (reference, or copy) of a
     //   chosen field, or a value based on that field if such a value is unique per the field's
     //   value.
     [
        $(
           $(
            $local_ident:ident
            $(. $($local_ident_ident:ident)? $($local_ident_idx:literal)?
             )*)?

           $(
            $local_idx:literal
            $(. $($local_idx_ident:ident)? $($local_idx_idx:literal)?
             )* )?

           $(($local_cmp_closure:expr))?
           $({$local_get_closure:expr})?
        ),*
     ]
     [
        $(
           $(
            $non_local_ident:ident
            $(. $($non_local_ident_ident:ident)? $($non_local_ident_idx:literal)?
             )*)?

           $(
            $non_local_idx:literal
            $(. $($non_local_idx_ident:ident)? $($non_local_idx_idx:literal)?
             )* )?

           $(($non_local_cmp_closure:expr))?
           $({$non_local_get_closure:expr})?
        ),*
     ]
    ) => {
        impl $(<$($generic_left $(: $bound)?)+>)?
        $crate::COrd for $struct_path $(<$($generic_right),+>)?
        $(where $($left : $right),+)? {
            fn cmp_local(&self, other: &Self) -> ::core::cmp::Ordering {
                use crate::CPartialEq;
                Self::LOCALITY.debug_reachable_for_local();
                let this = &self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                let result = ::core::cmp::Ordering::Equal;
                // LLVM should be able to optimize away the first comparison of
                // result==::core::cmp::Ordering::Equal
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    $(let result =
                         this.$local_ident
                        $(.$($local_ident_ident)? $($local_ident_idx)?
                         )* .cmp(
                         &other.$local_ident
                        $(.$($local_ident_ident)? $($local_ident_idx)?
                         )*
                        );
                    )?
                    $(let result =
                         this.$local_idx
                        $(.$($local_idx_ident)? $($local_idx_idx)?
                         )* .cmp(
                         &other.$local_idx
                        $(.$($local_idx_ident)? $($local_idx_idx)?
                         )*
                        );
                    )?
                    $(let result =
                        $local_cmp_closure(&this, &other);
                    )?
                    $(let result =
                        $local_get_closure(&this).cmp(&$local_get_closure(&other));
                    )?
                )*
                result
            }

            fn cmp_non_local(&self, other: &Self) -> ::core::cmp::Ordering {
                use crate::CPartialEq;
                Self::LOCALITY.debug_reachable_for_non_local();
                let this = &self;
                $( let this = &this.$t;
                   let other = &other.$t;
                )?
                let result = ::core::cmp::Ordering::Equal;
                $(
                    if result!=::core::cmp::Ordering::Equal {
                        return result;
                    }
                    $(let result =
                         this.$non_local_ident
                        $(.$($non_local_ident_ident)? $($non_local_ident_idx)?
                         )* .cmp(
                         &other.$non_local_ident
                        $(.$($non_local_ident_ident)? $($non_local_ident_idx)?
                         )*
                        );
                    )?
                    $(let result =
                         this.$non_local_idx
                        $(.$($non_local_idx_ident)? $($non_local_idx_idx)?
                         )* .cmp(
                         &other.$non_local_idx
                        $(.$($non_local_idx_ident)? $($non_local_idx_idx)?
                         )*
                        );
                    )?
                    $(let result =
                        $non_local_cmp_closure(&this, &other);
                    )?
                    $(let result =
                        $non_local_get_closure(&this).cmp(&$non_local_get_closure(&other));
                    )?
                )*
                result
            }
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

c_wrap! {
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

c_wrap! { [Clone, Debug] _CaWrap3 <T> {t : T }}
c_wrap! { [Clone, Debug] _CaWrap4 <T:Sized> {t : T }}
c_wrap! {
    [Clone, Debug]
    _CaWrap5 <T>
    where T: 'static {
        t : T
    }
}
c_wrap! { pub CaWrapPub {pub t : u8}}

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

        c_wrap! {
            _CaWrap2 <A> {
                pub t : Vec<A>
            }
        }

        use crate::Locality;

        c_wrap! { CaWrapA1 {t : A }}
        c_partial_eq! {
            CaWrapA1 {
                Locality::Both => t : A
            }
            [ (|this: &A, other: &A| this.x==other.x) ]
            [.v]
        }
        c_ord! {
            CaWrapA1 { t }
            [ {|a: &A| a.x} ]
            [v]
        }

        c_wrap_tuple! { _CaTupleGen1 <T> (pub T) where T: Clone}

        mod tuple_2 {
            use crate::macros_c::test_macros::with_alloc::A;
            use crate::Locality;
            use alloc::vec::Vec;
            //use alloc::string::String;

            c_wrap_tuple! { CaTupleA2 (A) }
            fn get_v<'a>(wrap: &'a A) -> &'a Vec<i32> {
                &wrap.v
            }
            c_partial_eq! {
                <'a>
                CaTupleA2 {
                    Locality::Both => 0 : A
                }
                [ {|obj: &A| obj.x}
                ]
                // We can't specify return lifetimes here:
                //
                // [{ |obj: &'l A| -> &'l Vec<i32> {&obj.v} }]
                //
                // Hence a separate function:
                [ {get_v} ]
                []
            }
            c_ord! {
                CaTupleA2 { 0 }
                [( |this: &A, other: &A| this.x.cmp(&other.x) )]
                [v]
            }
        }
    }
}
