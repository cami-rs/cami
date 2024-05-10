// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use cami::prelude::*;
use core::ops::RangeBounds;
use core::{hint, marker::PhantomData, time::Duration};
use criterion::{BenchmarkId, Criterion};
use fastrand::Rng;
//use ref_cast::RefCast;

pub fn criterion_config() -> Criterion {
    Criterion::default().warm_up_time(Duration::from_millis(200))
}

// On heap.
pub const MIN_ITEMS: usize = 4; //10;
pub const MAX_ITEMS: usize = 10; //100_000;

#[allow(unused)]
/// On heap. For example, for String, this is the maximum number of `char` - so the actual UTF-8
/// size may be a few times higher.
pub const MAX_ITEM_LEN: usize = 4; //1_000;

// For purging the L1, L2..., in bytes.
const MAX_CACHE_SIZE: usize = 2_080_000;

pub trait Random {
    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8;
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize;
}

impl Random for Rng {
    fn u8(&mut self, range: impl RangeBounds<u8>) -> u8 {
        Rng::u8(self, range)
    }
    fn usize(&mut self, range: impl RangeBounds<usize>) -> usize {
        Rng::usize(self, range)
    }
}

pub fn purge_cache<RND: Random>(rng: &mut RND) {
    let mut vec = Vec::<u8>::with_capacity(MAX_CACHE_SIZE);

    for _ in [0..MAX_CACHE_SIZE] {
        vec.push(rng.u8(..));
    }
    hint::black_box(vec);
}

/// Shortcut trait, for "output" items based on owned items, but with no specified lifetime.
pub trait OutItem: Clone + CamiOrd + Ord {}
/// Shortcut trait, for "output" items based on owned items, with a lifetime.
pub trait OutItemLifetimed<'own>: OutItem {}
/// Blanket impl, so no need to write it for any specific type.
impl<T> OutItem for T where T: Clone + CamiOrd + Ord {}
/// Blanket impl, so no need to write it for any specific type.
impl<'own, T> OutItemLifetimed<'own> for T where T: OutItem {}

/// Collection for "output" items, based on/referencing "owned" items. Used for
/// [OutCollectionIndicator::OutCollectionImpl].
///
/// When implementing [Extend] for this, do implement [Extend::extend_one] and
/// [Extend::extend_reserve], too - even though they do have a default implementation.
///
/// Not extending [core::ops::Index], because [BTreeSet] doesn't extend it either.
pub trait OutCollection<T>: Clone + Extend<T>
where
    T: OutItem,
{
    // @TODO see if RustDoc/docs.rs/libs.rs generates a correct link for
    // alloc::collections::BTreeSet. Otherwise change it to std::
    /// For example, `true` for [Vec], `false` for [alloc::collections::BTreeSet].
    const ALLOWS_MULTIPLE_EQUAL_ITEMS: bool;
    // @TODO RustDoc: InOut -> OutCollection
    /// NOT a part of public API. It may `panic`.
    /// Used by default implementations only.
    //fn as_vec(&self) -> &Vec<T>;
    /// NOT a part of public API. It may `panic`.
    /// Used by default implementations only.
    //fn as_mut_vec(&mut self) -> &mut Vec<T>;
    /// NOT a part of public API. It may `panic`.
    /// Used by default implementations only.
    //fn into_vec(self) -> Vec<T>;

    /// Prefer [InOut::with_capacity] if possible.
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn clear(&mut self);

    fn len(&self) -> usize;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;
    /// Not required - it may `panic`. If available, use it only for [Transref::Out] so we can have
    /// multiple output instances based on the same input.
    fn into_iter(self) -> impl Iterator<Item = T>;

    /// Like [Iterator::is_sorted]. BUT: For types that maintain/guarantee a sorted order, like
    /// [std::collections::BTreeSet], this must NOT (for example)
    /// - simply return `true`, nor
    /// - just call [std::collections::BTreeSet::iter] -> [Iterator::is_sorted], because that could
    /// be optimized away .
    ///
    /// Instead, it verifies the sorted order. For example: [std::collections::BTreeSet::iter] ->
    /// [core::hint::black_box] -> [Iterator::is_sorted].
    fn is_sorted(&self) -> bool;
    //fn is_sorted_by<F>(&self, compare: F) -> bool where F: FnMut(&T, &T) -> bool;
    fn sort(&mut self);
    // fn sort_by<F>(&mut self, compare: F) where F: FnMut(&T, &T) -> Ordering;
    // @TODO sort_unstable_by + const SEPARATE_UNSTABLE_SORT: bool
    /// Search; return `true` if found an equal item (or key, in case of [alloc::collections::BTreeMap] and friends.)
    fn binary_search(&self, x: &T) -> bool;
    //fn binary_search_by<'this, F>(&'this self, f: F) -> Result<usize, usize> where F: FnMut(&'this T) -> Ordering, T: 'this;
}

pub trait OutCollectionIndicator {
    type OutCollectionImpl<T>: OutCollection<T>
    where
        T: OutItem;
}
#[derive(Clone /*, RefCast*/)]
#[repr(transparent)]
pub struct OutCollectionVec<T>(pub Vec<T>)
where
    T: OutItem;

impl<T> Extend<T> for OutCollectionVec<T>
where
    T: OutItem,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
    fn extend_one(&mut self, item: T) {
        self.0.extend_one(item);
    }
    fn extend_reserve(&mut self, additional: usize) {
        self.0.extend_reserve(additional);
    }
}
impl<T> OutCollection<T> for OutCollectionVec<T>
where
    T: OutItem,
{
    const ALLOWS_MULTIPLE_EQUAL_ITEMS: bool = true;

    fn new() -> Self {
        Self(Vec::new())
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    fn clear(&mut self) {
        self.0.clear();
    }

    fn len(&self) -> usize {
        self.0.len()
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.0.iter()
    }
    /// Not required - it may `panic`. If available, use it only for [Transref::Out] so we can have
    /// multiple output instances based on the same input.
    fn into_iter(self) -> impl Iterator<Item = T> {
        self.0.into_iter()
    }
    /// Like [Iterator::is_sorted_by]. BUT: For types that maintain/guarantee a sorted order, like
    /// [std::collections::BTreeSet], this must NOT (for example)
    /// - simply return `true`, nor
    /// - just call [std::collections::BTreeSet::iter] -> [Iterator::is_sorted_by], because that could
    /// be optimized away .
    ///
    /// Instead, it verifies the sorted orde. For example: [std::collections::BTreeSet::iter] ->
    /// [core::hint::black_box] -> [Iterator::is_sorted_by].
    fn is_sorted(&self) -> bool {
        self.0.is_sorted()
    }
    fn sort(&mut self) {
        self.0.sort();
    }
    fn binary_search(&self, x: &T) -> bool {
        self.0.binary_search(x).is_ok()
    }
}

pub struct OutCollectionVecIndicator();
impl OutCollectionIndicator for OutCollectionVecIndicator {
    type OutCollectionImpl<T> = OutCollectionVec<T> where T: OutItem;
}

type OutCollRetriever<'own, OutCollectionIndicatorImpl, OutItemIndicatorImpl> =
    <OutCollectionIndicatorImpl as OutCollectionIndicator>::OutCollectionImpl<
        <OutItemIndicatorImpl as OutItemIndicator>::OutItemLifetimedImpl<'own>,
    >;

/*pub type OutCollectionVecItemRef<'o, T> = OutCollectionVec<&'o T>;
pub struct OutCollectionVecItemRefIndicator();
impl <T>  OutCollectionIndicator for OutCollectionVecItemRef where T: Clone + CamiOrd + Ord{
    type OutCollectionImpl<T> = OutCollectionVecItemRef<T>;
}*/

/*
/// Intended for [TransRef::Out] referencing to [TransRef::In].
#[derive(Clone /*, RefCast*/)]
#[repr(transparent)]
pub struct OutVecItemRef<'o, T>(pub Vec<&'o T>);

impl<'o, T: 'o + Clone> OutCollection<&'o T> for OutVecItemRef<'o, T> where
    T: Clone + CamiOrd + Ord {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    /*
    fn len(&self) -> usize {
        self.0.len()
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a &'o T>
    where
        T: 'a,
        'o: 'a,
    {
        self.0.iter().map(|ref_ref| *ref_ref)
    }
    fn into_iter(self) -> impl Iterator<Item = T> {
        if true {
            unimplemented!("Use .iter() if possible. Otherwise:We could implement it for Clone only. But instead, use a seed that is Clone, clone the seed, and get a new collection.");
        }
        // without this, rustc complained that "() is not an iterator" - even though
        // `unimplemented!(...)` does panic!, so it should return never type `!`
        Vec::<T>::new().into_iter()
    }

    fn push(&mut self, _item: T) {
        unimplemented!("Use .extend(iter) instead.")
    }
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter.map(|rf| &rf ));
    }
    */
}
impl<'o, T: 'o + Clone> Out<&'o T> for OutVecItemRef<'o, T> {}

pub trait OutItemRef<'o, T: 'o + Clone>: Out<&'o T> {}

impl<'o, T: 'o + Clone> OutItemRef<'o, T> for OutVecItemRef<'o, T> {}

// Previous `TransRef` is at
// https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/DropCk.20.26.20GAT.20.28Generic.20Associative.20Types.29
pub trait TransRef<'slice, InItem: Clone, OutItem: 'slice + Clone> {
    type In: In<InItem>;
    // NOT needed: where T: 'own
    type Own<'own>;

    type OutSeed;
    // Self::Out DOES need: where T: 'out
    type Out<'out>: Out<OutItem> + 'out
    where
        OutItem: 'out;
    type OutRef<'out>: OutItemRef<'slice, OutItem> + 'out
    where
        OutItem: 'out,
        'slice: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed);
    fn reserve_own<'own>() -> Self::Own<'own> {
        todo!()
    }

    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out;

    fn ini_out_move_seed<'out>(out: &mut Self::Out<'out>, mut out_seed: Self::OutSeed)
    where
        Self::Out<'out>: 'out,
    {
        Self::ini_out_mut_seed(out, &mut out_seed);
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        Self::Out<'out>: 'out;

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        out: &mut Self::Out<'out>,
        own: &'ownref Self::Own<'own>,
    ) where
        Self::Out<'out>: 'out,
        'ownref: 'slice;
}
*/
/*pub trait TransRefVecInnerHolder<'out, 'own: 'out, InItem, OutItem>
where
    OutItem: 'out,
{
    type TransRefImpl: TransRef<OutItem, In = Vec<InItem>, Own<'own> = Vec<InItem>, Out<'out> = Vec<OutItem>>
    where
        OutItem: 'out,
        <Self as TransRefVecInnerHolder<'out, 'own, InItem, OutItem>>::TransRefImpl: 'out;
}
pub trait TransRefVecOuterHolder<InItem, OutItem> {
    type TransRefInnerHolder<'out, 'own: 'out>: TransRefVecInnerHolder<'out, 'own, InItem, OutItem>
    where
        OutItem: 'out;
}*/
/*
pub struct VecVecToVecSlice();
//
impl<'slice, Item: Clone> TransRef<'slice, Vec<Item>, &'slice [Item]> for VecVecToVecSlice
//impl<'t, T: 't> TransRef<'t, T> for VecVecToVecSlice<'t>
// where Self: 't,
{
    //impl<T> TransRef<T> for VecVecToVecSlice<T> {
    type In = OutCollectionVec<Vec<Item>>;
    type Own<'own> = Vec<Vec<Item>>;
    type OutSeed = usize;
    // Surprisingly, no lifetime here - no Vec<&'out
    //
    type Out<'out> = OutCollectionVec<&'slice [Item]> where Self::Out<'out>: 'out, 'slice: 'out, Item: 'out;

    type OutRef<'out> = OutVecItemRef<'slice, &'slice [Item]> where Self::Out<'out>: 'out, 'slice: 'out, Item: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        let len = input.len();
        (input.0, len)
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out,
        'slice: 'out,
        Item: 'out,
    {
        OutCollectionVec::new()
    }

    // @TODO Delay reservation?
    fn ini_out_move_seed<'out>(out: &mut Self::Out<'out>, out_seed: Self::OutSeed)
    where
        Self::Out<'out>: 'out,
        'slice: 'out,
        Item: 'out,
    {
        out.reserve(out_seed);
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        Self::Out<'out>: 'out,
        'slice: 'out,
        Item: 'out,
    {
        out.reserve(*out_seed);
    }

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        out: &mut Self::Out<'out>,
        own: &'ownref Self::Own<'own>,
    ) where
        Self::Out<'out>: 'out,
        'slice: 'out,
        Item: 'out,
        'ownref: 'slice, //////
    {
        out.extend(own.iter().map(|v| &v[..]));
    }
}

//pub struct VecToVecCloned<T>(PhantomData<T>);
pub struct VecToVecCloned();

//impl<'t, T: 't + Clone /*'t, T: 't + Clone*/> TransRef<'t, T> for VecToVecCloned<T> where Self : 't{
impl<'slice, Item: Clone + 'slice> TransRef<'slice, Item, Item> for VecToVecCloned {
    type In = OutCollectionVec<Item>;
    type Own<'own> = Vec<Item>;
    type OutSeed = ();
    type Out<'out> = OutCollectionVec<Item> where Self::Out<'out>: 'out, Item: 'out;

    type OutRef<'out> = OutVecItemRef<'slice, Item> where Self::Out<'out>: 'out, 'slice: 'out, Item: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        (input.0, ())
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        OutCollectionVec::new()
    }

    /// Delay allocation a.s.a.p. - lazy.
    fn ini_out_move_seed<'out>(_out: &mut Self::Out<'out>, _out_seed: Self::OutSeed)
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
    }
    fn ini_out_mut_seed<'out, 'outref>(
        _out: &'outref mut Self::Out<'out>,
        _out_seed: &'outref mut Self::OutSeed,
    ) where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
    }

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        out: &mut Self::Out<'out>,
        own: &'ownref Self::Own<'own>,
    ) where
        Self::Out<'out>: 'out,
        Item: 'out,
        'ownref: 'slice,
    {
        out.reserve(own.len());
        out.extend_from_slice(&own[..]);
    }
}

//pub struct VecToVecMoved<T>(PhantomData<T>);
pub struct VecToVecMoved<'slice, Item>(PhantomData<(&'slice (), Item, Item)>);
//pub struct VecToVecMoved();
//impl<'t, T: 't> TransRef<'t, T> for VecToVecMoved<T> where Self: 't{
//impl<'out, 'own: 'out, T> TransRef<T> for VecToVecMoved<'out, 'own> {
impl<'slice, Item: 'slice + Clone> TransRef<'slice, Item, Item> for VecToVecMoved<'slice, Item> {
    type In = OutCollectionVec<Item>;
    type Own<'own> = Vec<Item>;
    type OutSeed = Vec<Item>;
    type Out<'out> = OutCollectionVec<Item> where Self::Out<'out>: 'out, Item: 'out;

    type OutRef<'out> = OutVecItemRef<'slice, Item> where Self::Out<'out>: 'out, 'slice: 'out, Item: 'out;

    fn ini_own_and_seed<'own>(_input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        todo!("transmute")
        //(Vec::new(), input)
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        OutCollectionVec(Vec::new())
    }

    fn ini_out_move_seed<'out>(out: &mut Self::Out<'out>, mut out_seed: Self::OutSeed)
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        mem::swap(&mut out.0, &mut out_seed);
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        mem::swap(&mut out.0, out_seed);
    }

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        _out: &mut Self::Out<'out>,
        _own: &'ownref Self::Own<'own>,
    ) where
        Self::Out<'out>: 'out,
        Item: 'out,
        'ownref: 'slice,
    {
    }
}

pub struct VecToVecMovedHolder();
impl<Item: Clone> TransRefHolder<Item, Item> for VecToVecMovedHolder {
    type TransRefImpl<'slice> = VecToVecMoved<'slice, Item> where Item: 'slice;
}

//pub struct VecToVecMovedInnerHolder<'out, 'own: 'out>(PhantomData<(&'out (), &'own ())>);
/*pub struct VecToVecMovedInnerHolder();

impl<'out, 'own: 'out, InItem, OutItem> TransRefVecInnerHolder<'out, 'own, InItem, OutItem>
    for VecToVecMovedInnerHolder
/*<'out, 'own>*/
where
    OutItem: 'out,
{
    type TransRefImpl = VecToVecMoved;
}

pub struct VecToVecMovedOuterHolder();
impl<InItem, OutItem> TransRefVecOuterHolder<InItem, OutItem> for VecToVecMovedOuterHolder {
    //type TransRefInnerHolder<'out, 'own: 'out> = VecToVecMovedInnerHolder<'out, 'own> where T: 'out;
    type TransRefInnerHolder<'out, 'own: 'out> = VecToVecMovedInnerHolder where OutItem: 'out;
}*/

pub trait TransRefHolder<InItem: Clone, OutItem: Clone> {
    type TransRefImpl<'slice>: TransRef<'slice, InItem, OutItem>
    where
        OutItem: 'slice;
    //where OutItem: 'out;
}
*/

pub trait OutItemIndicator {
    type OutItemLifetimedImpl<'own, T>: OutItemLifetimed<'own> + 'own
    where
        T: OutItem + 'own; //where Self::OutItemLifetimedImpl<'own> : 'own;//+ 'own;//where T: 'own;

    /*fn generate_out_item<'own, OwnItem>(
        own_item: &'own OwnItem,
    ) -> Self::OutItemLifetimedImpl<'own>;*/
}
/*
pub trait OutItemIndicator2<'own> {
    type OutItemLifetimedImpl: OutItemLifetimed<'own> + 'own;
}
pub trait OutItemIndicator2Indicator {
    type OutItemIndicatorImpl<'own>: OutItemIndicator2<'own>;
}

pub struct OutItemIndicatorNonRef<T>(PhantomData<T>);
impl<T> OutItemIndicator for OutItemIndicatorNonRef<T>
where
    T: OutItem,
{
    type OutItemLifetimedImpl<'own> = T where Self::OutItemLifetimedImpl<'own>: 'own; //T: 'own;

    /*fn generate_out_item<'own, OwnItem>(
        _own_item: &'own OwnItem,
    ) -> Self::OutItemLifetimedImpl<'own> {
        todo!()
    }*/
}

pub struct OutItemIndicator2NonRef<T>(PhantomData<T>);
impl<'own, T> OutItemIndicator2<'own> for OutItemIndicator2NonRef<T>
where
    T: OutItem + 'own,
{
    type OutItemLifetimedImpl = T;
}
pub struct OutItemIndicator2IndicatorNonRef<T>(PhantomData<T>);
impl<T> OutItemIndicator2Indicator for OutItemIndicator2IndicatorNonRef<T>
where
    T: OutItem,
{
    type OutItemIndicatorImpl<'own> = OutItemIndicator2NonRef<T> where Self::OutItemIndicatorImpl<'own>: 'own; //OutItemIndicator2NonRef<T> : 'own; //T: 'own;
}

pub struct OutItemIndicatorSlice<T: ?Sized>(PhantomData<T>);
impl<T> OutItemIndicator for OutItemIndicatorSlice<T>
where
    T: OutItem + ?Sized,
{
    type OutItemLifetimedImpl<'own> = &'own [T] where T: 'own;

    /*fn generate_out_item<'own, OwnItem>(
        _own_item: &'own OwnItem,
    ) -> Self::OutItemLifetimedImpl<'own> {
        todo!()
    }*/
}
*/
//-----
pub trait OutItemIndicator3<'own, T>
where
    T: OutItem + 'own,
{
    type OutItemLifetimedImpl: OutItemLifetimed<'own> + 'own;
}
pub trait OutItemIndicator3Indicator<T>
where
    T: OutItem,
{
    type OutItemIndicatorImpl<'own /*, T*/>: OutItemIndicator3<'own, T>
    where
        T: 'own;
}
pub struct OutItemIndicator3NonRef<T>(PhantomData<T>);
impl<'own, T> OutItemIndicator3<'own, T> for OutItemIndicator3NonRef<T>
where
    T: OutItem + 'own, // + 'own, // TODO OutItemLifetimed<'own> ???
{
    type OutItemLifetimedImpl = T;
}
pub struct OutItemIndicator3NonRefIndicator<T>(PhantomData<T>);
impl<T> OutItemIndicator3Indicator<T> for OutItemIndicator3NonRefIndicator<T>
where
    T: OutItem,
{
    type OutItemIndicatorImpl<'own> = OutItemIndicator3NonRef<T> where T: 'own;
}

pub struct OutItemIndicator3Slice<T>(PhantomData<T>);
impl<'own, T> OutItemIndicator3<'own, T> for OutItemIndicator3Slice<T>
where
    T: OutItem + ?Sized + 'own, // + 'own, // TODO OutItemLifetimed<'own> ???
{
    type OutItemLifetimedImpl = &'own [T];
}
pub struct OutItemIndicator3SliceIndicator<T>(PhantomData<T>);
impl<T> OutItemIndicator3Indicator<T> for OutItemIndicator3SliceIndicator<T>
where
    T: OutItem,
{
    type OutItemIndicatorImpl<'own> = OutItemIndicator3Slice<T> where T: 'own; // where T: 'own;
}
//------

pub fn bench_vec_sort_bin_search<
    OwnItem,
    OutItemIndicatorImpl: OutItemIndicator,
    OutCollectionIndicatorImpl: OutCollectionIndicator,
    //#[allow(non_camel_case_types)] TRANS_REF_OUTER_HOLDER,
    //#[allow(non_camel_case_types)] TRANS_REF_HOLDER,
    RND,
    #[allow(non_camel_case_types)] ID_STATE,
>(
    c: &mut Criterion,
    rnd: &mut RND,
    group_name: impl Into<String>,
    id_state: &mut ID_STATE,
    generate_id_postfix: fn(&ID_STATE) -> String,
    generate_own_item: fn(&mut RND, &mut ID_STATE) -> OwnItem,
    //generate_out_item: for<'own> fn(&OwnItem) -> OutItemIndicatorImpl::OutItemImpl<'own>,
) where
    //TRANS_REF_HOLDER: TransRefHolder<OwnItem, OutItem>,
    /*TRANS_REF_OUTER_HOLDER: for<'out, 'own> TransRefVecOuterHolder<
        IN_ITEM,
        T,
        TransRefInnerHolder<'out, 'own>: TransRefVecInnerHolder<'out, 'own, IN_ITEM, T>,
    >,

    for<'out, 'own> <TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>>::TransRefInnerHolder<'out, 'own>:
        TransRef<T>,*/
    RND: Random,
{
    let mut group = c.benchmark_group(group_name);

    let num_items = rnd.usize(MIN_ITEMS..MAX_ITEMS);

    let mut in_items = Vec::with_capacity(1);
    //let mut in_items = TRANS_REF::TransRefImpl::In::with_capacity(num_items);
    for _ in 0..num_items {
        let item = generate_own_item(rnd, id_state);
        in_items.push(item);
    }
    if !<OutCollRetriever<
            '_,
            OutCollectionIndicatorImpl,
            OutItemIndicatorImpl,
        >>::ALLOWS_MULTIPLE_EQUAL_ITEMS {
            todo!("out -> .clone() -> check if already in an extra BTreeSet, if not, add there & to the result out collection.");
    }

    {
        /*type OutColl = <OutCollectionIndicatorImpl as OutCollectionIndicator
        >::OutCollectionImpl< <OutItemIndicatorImpl as OutItemIndicator>::OutItemImpl<'_>
                            >;*/
        //type O<'own> = OutColl<'own, OutCollectionIndicatorImpl, OutItemIndicatorImpl>;
        let mut unsorted_items = <OutCollRetriever<
            '_,
            OutCollectionIndicatorImpl,
            OutItemIndicatorImpl,
        >>::with_capacity(1);

        /*<
           <
            <
                TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>
            >
            ::TransRefInnerHolder<'_, '_> as TransRefVecInnerHolder<'_, '_,IN_ITEM, T>
           >::TransRefImpl as TransRef<T>
        >*/
        /*<<TRANS_REF_HOLDER as TransRefHolder<OwnItem, OutItem>>::TransRefImpl<'_> as TransRef<
            '_,
            OwnItem,
            OutItem,
        >>::ini_out_mut_seed(&mut unsorted_items, &mut out_seed);*/

        /*<
           <
            <
                TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>
            >
            ::TransRefInnerHolder<'_, '_> as TransRefVecInnerHolder<'_, '_,IN_ITEM, T>
           >::TransRefImpl as TransRef<T>
        >*/
        /*<<TRANS_REF_HOLDER as TransRefHolder<OwnItem, OutItem>>::TransRefImpl<'_> as TransRef<
            '_,
            OwnItem,
            OutItem,
        >>::set_out(&mut unsorted_items, &own_items);*/
        // CANNOT: let unsorted_items = unsorted_items; // Prevent mutation by mistake.

        //for size in [K, 2 * K, 4 * K, 8 * K, 16 * K].iter() {
        let id_string = format!(
            "{num_items} items, each len max {MAX_ITEM_LEN}.{}",
            generate_id_postfix(id_state)
        );
        //#[cfg(do_later)]
        if false {
            let mut sorted_lexi = <OutCollRetriever<
                '_,
                OutCollectionIndicatorImpl,
                OutItemIndicatorImpl,
            >>::with_capacity(1);
            //let mut sorted_lexi = unsorted_items.clone();
            group.bench_with_input(
                BenchmarkId::new("std sort lexi.          ", id_string.clone()),
                hint::black_box(&unsorted_items),
                |b, unsorted_items| {
                    b.iter(|| {
                        //sorted_lexi = hint::black_box(unsorted_items.clone());
                        // @TODO ^^^--> .clone()  \----> change to:
                        //
                        // .sorted_lexi.extend( it().map(|it_ref| it_ref.clone()))
                        sorted_lexi.clear();
                        sorted_lexi.extend(unsorted_items.iter().cloned());

                        //sorted_lexi.sort_by(<OutItemIndicatorImpl as OutItemIndicator>::OutItemLifetimedImpl::cmp);
                        sorted_lexi.sort();
                    })
                },
            );
            purge_cache(rnd);
            group.bench_with_input(
                BenchmarkId::new("std bin search (lexi)   ", id_string.clone()),
                hint::black_box(&unsorted_items),
                |b, unsorted_items| {
                    b.iter(|| {
                        let sorted = hint::black_box(&sorted_lexi);
                        for item in hint::black_box(unsorted_items.iter()) {
                            assert!(hint::black_box(sorted.binary_search(&item)));
                        }
                    })
                },
            );
        }
        //#[cfg(do_later)]
        {
            purge_cache(rnd);
            #[cfg(not(feature = "transmute"))]
            let unsorted_items = {
                let mut unsorted_items_cami = Vec::with_capacity(unsorted_items.len());
                unsorted_items_cami
                    .extend(unsorted_items.iter().map(|v| Cami::<TO>::new(v.clone())));
                unsorted_items_cami
            };

            let mut sorted_non_lexi = Vec::new();
            group.bench_with_input(
                BenchmarkId::new("std sort non-lexi.      ", id_string.clone()),
                hint::black_box(&unsorted_items),
                |b, unsorted_items| {
                    b.iter(|| {
                        #[cfg(feature = "transmute")]
                        let _ = {
                            // @TODO replace .clone() by: Vec::with_capacity(), .iter() -> extend -> .into_vec_cami()
                            let unsorted_items = (*unsorted_items).clone();

                            // @TODO TODO
                            //sorted_non_lexi = hint::black_box(unsorted_items).into_vec().into_vec_cami();
                        };
                        #[cfg(not(feature = "transmute"))]
                        let _ = {
                            sorted_non_lexi = hint::black_box(unsorted_items.clone());
                        };
                        sorted_non_lexi.sort();
                    })
                },
            );
            purge_cache(rnd);
            group.bench_with_input(
                BenchmarkId::new("std bin search (non-lexi)", id_string),
                hint::black_box(&unsorted_items),
                //hint::black_box( unsorted_items.into_ref_vec_cami() ),
                |b, unsorted_items| {
                    b.iter(|| {
                        let sorted = hint::black_box(&sorted_non_lexi);
                        // The following `unsorted_items.into_iter()` is cheap (no consuming of any `Vec`), because `unsorted_items`` is a reference to a Vec.
                        for item in hint::black_box(unsorted_items.iter()) {
                            #[cfg(feature = "transmute")]
                            let _ = {
                                hint::black_box(sorted.binary_search(item.into_ref_cami()))
                                    .unwrap();
                            };
                            #[cfg(not(feature = "transmute"))]
                            let _ = {
                                hint::black_box(sorted.binary_search(item)).unwrap();
                            };
                        }
                    })
                },
            );
        }
    }
    group.finish();
}
