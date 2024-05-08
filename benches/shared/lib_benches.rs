// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use camigo::prelude::*;
use core::mem;
use core::{hint, time::Duration};
use core::{marker::PhantomData, ops::RangeBounds};
use criterion::{BenchmarkId, Criterion};
use fastrand::Rng;
use ref_cast::RefCast;

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

/// Collection for [TransRef::In] & [TransRef::Out].
/// This trait doesn't inherit from [core::ops::Index] nor from [core::iter::Extend], because we
/// couldn't implement them for Rust/3rd party types.
pub trait InOut<T>: Sized + Clone {
    /// NOT a part of public API. It may `panic`.
    /// Used by default implementations only.
    fn as_vec(&self) -> &Vec<T>;
    /// NOT a part of public API. It may `panic`.
    /// Used by default implementations only.
    fn as_mut_vec(&mut self) -> &mut Vec<T>;
    /// NOT a part of public API. It may `panic`.
    /// Used by default implementations only.
    fn into_vec(self) -> Vec<T>;

    /// Prefer [InOut::with_capacity] if possible.
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;

    fn len(&self) -> usize {
        self.as_vec().len()
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.as_vec().iter()
    }
    /// Not required - it may `panic`. If available, use it only for [Transref::Out] so we can have
    /// multiple output instances based on the same input.
    fn into_iter(self) -> impl Iterator<Item = T> {
        self.into_vec().into_iter()
    }

    fn reserve(&mut self, additional: usize) {
        self.as_mut_vec().reserve(additional);
    }
    fn push(&mut self, item: T) {
        self.as_mut_vec().push(item);
    }
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.as_mut_vec().extend(iter)
    }
    fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.as_mut_vec().extend_from_slice(other);
    }
    fn clear(&mut self) {
        self.as_mut_vec().clear();
    }
}
pub trait In<T>: InOut<T> {}
pub trait Out<T>: InOut<T> {}

#[derive(Clone, RefCast)]
#[repr(transparent)]
pub struct InOutVec<T: Clone>(pub Vec<T>);

impl<T: Clone> InOut<T> for InOutVec<T> {
    fn as_vec(&self) -> &Vec<T> {
        &self.0
    }
    fn as_mut_vec(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
    fn into_vec(self) -> Vec<T> {
        self.0
    }

    fn new() -> Self {
        Self(Vec::new())
    }
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}
impl<T: Clone> In<T> for InOutVec<T> {}
impl<T: Clone> Out<T> for InOutVec<T> {}

/// Intended for [TransRef::Out] referencing to [TransRef::In].
#[derive(Clone, RefCast)]
#[repr(transparent)]
pub struct OutVecItemRef<'o, T: Clone>(pub Vec<&'o T>);

impl<'o, T: 'o + Clone> InOut<&'o T> for OutVecItemRef<'o, T> {
    fn as_vec(&self) -> &Vec<&'o T> {
        &self.0
    }
    fn as_mut_vec(&mut self) -> &mut Vec<&'o T> {
        &mut self.0
    }
    fn into_vec(self) -> Vec<&'o T> {
        self.0
    }

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

pub struct VecVecToVecSlice();
//
impl<'slice, Item: Clone> TransRef<'slice, Vec<Item>, &'slice [Item]> for VecVecToVecSlice
//impl<'t, T: 't> TransRef<'t, T> for VecVecToVecSlice<'t>
// where Self: 't,
{
    //impl<T> TransRef<T> for VecVecToVecSlice<T> {
    type In = InOutVec<Vec<Item>>;
    type Own<'own> = Vec<Vec<Item>>;
    type OutSeed = usize;
    // Surprisingly, no lifetime here - no Vec<&'out
    //
    type Out<'out> = InOutVec<&'slice [Item]> where Self::Out<'out>: 'out, 'slice: 'out, Item: 'out;

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
        InOutVec::new()
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
    type In = InOutVec<Item>;
    type Own<'own> = Vec<Item>;
    type OutSeed = ();
    type Out<'out> = InOutVec<Item> where Self::Out<'out>: 'out, Item: 'out;

    type OutRef<'out> = OutVecItemRef<'slice, Item> where Self::Out<'out>: 'out, 'slice: 'out, Item: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        (input.0, ())
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        InOutVec::new()
    }

    /// Delay allocation a.s.a.p. - lazy.
    fn ini_out_move_seed<'out>(_out: &mut Self::Out<'out>, _out_seed: Self::OutSeed)
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
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
    type In = InOutVec<Item>;
    type Own<'own> = Vec<Item>;
    type OutSeed = Vec<Item>;
    type Out<'out> = InOutVec<Item> where Self::Out<'out>: 'out, Item: 'out;

    type OutRef<'out> = OutVecItemRef<'slice, Item> where Self::Out<'out>: 'out, 'slice: 'out, Item: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        todo!("transmute")
        //(Vec::new(), input)
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        InOutVec(Vec::new())
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

pub fn bench_vec_sort_bin_search<
    InItem: Clone,
    OutItem: Clone,
    //#[allow(non_camel_case_types)] TRANS_REF_OUTER_HOLDER,
    #[allow(non_camel_case_types)] TRANS_REF_HOLDER,
    RND,
    #[allow(non_camel_case_types)] ID_STATE,
>(
    c: &mut Criterion,
    rnd: &mut RND,
    group_name: impl Into<String>,
    id_state: &mut ID_STATE,
    id_postfix: fn(&ID_STATE) -> String,
    generate: fn(&mut RND, &mut ID_STATE) -> InItem,
) where
    OutItem: CamiOrd + Ord + Clone,
    TRANS_REF_HOLDER: TransRefHolder<InItem, OutItem>,
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

    let mut in_items =
        <<TRANS_REF_HOLDER as TransRefHolder<InItem, OutItem>>::TransRefImpl<'_> as TransRef<
            '_,
            InItem,
            OutItem,
        >>::In::with_capacity(1);
    //let mut in_items = TRANS_REF::TransRefImpl::In::with_capacity(num_items);
    for _ in 0..num_items {
        let item = generate(rnd, id_state);
        in_items.push(item);
    }

    let (own_items, mut out_seed) =
    /*<
       <
        <
            TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>
        >
        ::TransRefInnerHolder<'_, '_> as TransRefVecInnerHolder<'_, '_, IN_ITEM, T>
       >::TransRefImpl as TransRef<T>
    >*/
     <
        <TRANS_REF_HOLDER as TransRefHolder<InItem, OutItem>
        >::TransRefImpl<'_>
           as TransRef<'_, InItem, OutItem>
    >::ini_own_and_seed(in_items);

    //#[cfg(off)]
    let own_items = /*<<<TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>>::TransRefInnerHolder<
        '_, '_,
    > as TransRefVecInnerHolder<'_, '_, IN_ITEM, T>>::TransRefImpl as TransRef<T>>*/ <
        <TRANS_REF_HOLDER as TransRefHolder<InItem, OutItem>
        >::TransRefImpl<'_>
           as TransRef<'_, InItem, OutItem>
    >::reserve_own(
    );

    {
        let mut unsorted_items = /*<
       <
        <
            TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>
        >
        ::TransRefInnerHolder<'_, '_> as TransRefVecInnerHolder<'_, '_,IN_ITEM, T>
       >::TransRefImpl as TransRef<T>
    >*/
     <
        <TRANS_REF_HOLDER as TransRefHolder<InItem, OutItem>
        >::TransRefImpl<'_>
           as TransRef<'_, InItem, OutItem>
    >::reserve_out();
        /*<
           <
            <
                TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>
            >
            ::TransRefInnerHolder<'_, '_> as TransRefVecInnerHolder<'_, '_,IN_ITEM, T>
           >::TransRefImpl as TransRef<T>
        >*/
        <<TRANS_REF_HOLDER as TransRefHolder<InItem, OutItem>>::TransRefImpl<'_> as TransRef<
            '_,
            InItem,
            OutItem,
        >>::ini_out_mut_seed(&mut unsorted_items, &mut out_seed);

        /*<
           <
            <
                TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>
            >
            ::TransRefInnerHolder<'_, '_> as TransRefVecInnerHolder<'_, '_,IN_ITEM, T>
           >::TransRefImpl as TransRef<T>
        >*/
        <<TRANS_REF_HOLDER as TransRefHolder<InItem, OutItem>>::TransRefImpl<'_> as TransRef<
            '_,
            InItem,
            OutItem,
        >>::set_out(&mut unsorted_items, &own_items);
        // CANNOT: let unsorted_items = unsorted_items; // Prevent mutation by mistake.

        //for size in [K, 2 * K, 4 * K, 8 * K, 16 * K].iter() {
        let id_string = format!(
            "{num_items} items, each len max {MAX_ITEM_LEN}.{}",
            id_postfix(id_state)
        );
        //#[cfg(do_later)]
        if false {
            let mut sorted_lexi = <
        <TRANS_REF_HOLDER as TransRefHolder<InItem, OutItem>
        >::TransRefImpl<'_>
           as TransRef<'_, InItem, OutItem>
    >::reserve_out();
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
                        sorted_lexi.as_mut_vec().sort();
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
                            hint::black_box(sorted.as_vec().binary_search(item)).unwrap();
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
                            sorted_non_lexi =
                                hint::black_box(unsorted_items).into_vec().into_vec_cami();
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
