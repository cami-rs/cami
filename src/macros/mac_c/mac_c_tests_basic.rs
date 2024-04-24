use crate::{ca_ord, ca_partial_eq, ca_wrap_struct, ca_wrap_tuple};
use alloc::vec::Vec;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct A {
    x: i32,
    v: Vec<i32>,
}

ca_wrap_struct! {
    _CaWrap2 <A> {
        pub t : Vec<A>
    }
}

use crate::Locality;

ca_wrap_struct! { CaWrapA1 {t : A }}
ca_partial_eq! {
    CaWrapA1 {
        Locality::Both => t
    }
    [ (|this: &A, other: &A| this.x==other.x) ]
    [.v]
}
ca_ord! {
    CaWrapA1 { t }
    [ {|a: &A| a.x} ]
    [v]
}

ca_wrap_tuple! { _CaTupleGen1 <T> (pub T) where T: Clone}

ca_wrap_tuple! { CaTupleA2 (A) }
fn get_v<'a>(wrap: &'a A) -> &'a Vec<i32> {
    &wrap.v
}
ca_partial_eq! {
    <'a>
    CaTupleA2 {
        Locality::Both => 0
    }
    [ {|obj: &A| obj.x}
    ]
    // @TODO: We can't specify return lifetimes here:
    //
    // [{ |obj: &'l A| -> &'l Vec<i32> {&obj.v} }]
    //
    // Hence a separate function:
    [ {get_v} ]
    []
}
ca_ord! {
    CaTupleA2 { 0 }
    [( |this: &A, other: &A| this.x.cmp(&other.x) )]
    [v]
}
