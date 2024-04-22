use crate::{c_ord, c_partial_eq, c_wrap, c_wrap_tuple};
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
        Locality::Both => t
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

c_wrap_tuple! { CaTupleA2 (A) }
fn get_v<'a>(wrap: &'a A) -> &'a Vec<i32> {
    &wrap.v
}
c_partial_eq! {
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
c_ord! {
    CaTupleA2 { 0 }
    [( |this: &A, other: &A| this.x.cmp(&other.x) )]
    [v]
}
