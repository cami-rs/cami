// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use camigo::prelude::*;
use core::mem;
use core::{hint, time::Duration};
use criterion::{BenchmarkId, Criterion};
use fastrand::Rng;
use std::{marker::PhantomData, ops::RangeBounds};

pub fn criterion_config() -> Criterion {
    Criterion::default().warm_up_time(Duration::from_millis(200))
}

// On heap.
pub const MIN_ITEMS: usize = 4; //10;
pub const MAX_ITEMS: usize = 10; //100_000;

#[allow(unused)]
/// On heap. For example, for String, this is the maximum number of
/// `char` - so the actual UTF-8 size may be a few times higher.
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

pub trait InCollection<InItem> {
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, item: InItem);
}
impl<InItem> InCollection<InItem> for Vec<InItem> {
    fn with_capacity(capacity: usize) -> Self {
        //<Self as Vec<InItem>>::with_capacity(capacity)
        Vec::<InItem>::with_capacity(capacity)
    }
    fn push(&mut self, item: InItem) {
        Vec::push(self, item)
    }
}

pub trait OutCollection<OutItem> {
    fn it<'a>(&'a self) -> impl Iterator<Item = &'a OutItem>
    where
        OutItem: 'a;
    fn into_it(self) -> impl Iterator<Item = OutItem>; // where OutItem: 'a;
                                                       //fn as_slice(&self) -> &[OutItem];
}

impl<OutItem> OutCollection<OutItem> for Vec<OutItem> {
    fn it<'a>(&'a self) -> impl Iterator<Item = &'a OutItem>
    where
        OutItem: 'a,
    {
        //Vec::iter(self as &Vec<OutItem>)
        <[_]>::iter(self)
        //Vec::<OutItem>::iter(self)
    }
    fn into_it(self) -> impl Iterator<Item = OutItem> {
        //IntoIterator::into_iter(self)
        <Vec<OutItem>>::into_iter(self)
    }
    /*fn as_slice(&self) -> &[OutItem] {
        Vec::as_slice(self)
    }*/
}

impl<'o, OutItem> OutCollection<OutItem> for Vec<&'o OutItem> {
    fn it<'a>(&'a self) -> impl Iterator<Item = &'a OutItem>
    where
        OutItem: 'a,
        'o: 'a,
    {
        <[_]>::iter(self).map(|ref_ref| *ref_ref)
    }
    fn into_it(self) -> impl Iterator<Item = OutItem> {
        #[cfg(off)]
        if true {
            unimplemented!("Unsupported. We could implement it for Clone only. But instead, use a seed that is Clone, clone the seed, and get a new collection.");
        }
        // without this, rustc complained that "() is not an interator" - even though
        // `unimplemented!(...)` does panic!, so it should return never type `!`
        let empty_vec = Vec::<OutItem>::new();
        empty_vec.into_iter()
    }
}

pub trait TransRef<InItem, OutItem> {
    type In: InCollection<InItem>;
    // NOT needed: where T: 'own
    type Own<'own>;

    type OutSeed;
    // Out<> DOES need: where T: 'out
    type Out<'out>: OutCollection<OutItem> + 'out
    where
        OutItem: 'out;
    //where        Self::Out: 'out;

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

    //fn set_out<'own: 'out, 'out>(out: &mut Self::OUT<'out>, own: &Self::OWN<'own>)
    //fn set_out<'out>(out: &mut Self::OUT<'out>, own: & Self::OWN)
    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        out: &mut Self::Out<'out>,
        own: &'ownref Self::Own<'own>,
    ) where
        Self::Out<'out>: 'out;
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
impl<'slice, Item> TransRef<Vec<Item>, &'slice [Item]> for VecVecToVecSlice
//impl<'t, T: 't> TransRef<'t, T> for VecVecToVecSlice<'t>
// where Self: 't,
{
    //impl<T> TransRef<T> for VecVecToVecSlice<T> {
    type In = Vec<Vec<Item>>;
    type Own<'own> = Vec<Vec<Item>>;
    type OutSeed = usize;
    // Surprisingly, no lifetime here - no Vec<&'out
    //
    type Out<'out> = Vec<&'slice [Item]> where Self::Out<'out>: 'out, 'slice: 'out, Item: 'out;
    //type Out<'out> = Vec<&[Item]> where Item: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        let len = input.len();
        (input, len)
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out,
        'slice: 'out,
        Item: 'out,
    {
        Vec::new()
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
    {
        out.extend(own.it().map(|v| &v[..]));
    }
}

//pub struct VecToVecCloned<T>(PhantomData<T>);
pub struct VecToVecCloned();

//impl<'t, T: 't + Clone /*'t, T: 't + Clone*/> TransRef<'t, T> for VecToVecCloned<T> where Self : 't{
impl<Item: Clone> TransRef<Item, Item> for VecToVecCloned {
    type In = Vec<Item>;
    type Own<'own> = Vec<Item>;
    type OutSeed = ();
    type Out<'out> = Vec<Item> where Self::Out<'out>: 'out, Item: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        (input, ())
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        Vec::new()
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
    {
        out.reserve(own.len());
        out.extend_from_slice(&own[..]);
    }
}

//pub struct VecToVecMoved<T>(PhantomData<T>);
//pub struct VecToVecMoved<'out, 'own: 'out>(PhantomData<(&'out (), &'own ())>);
pub struct VecToVecMoved();
//impl<'t, T: 't> TransRef<'t, T> for VecToVecMoved<T> where Self: 't{
//impl<'out, 'own: 'out, T> TransRef<T> for VecToVecMoved<'out, 'own> {
impl<Item> TransRef<Item, Item> for VecToVecMoved {
    type In = Vec<Item>;
    type Own<'own> = Vec<Item>;
    type OutSeed = Vec<Item>;
    type Out<'out> = Vec<Item> where Self::Out<'out>: 'out, Item: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        (Vec::new(), input)
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        Vec::new()
    }

    fn ini_out_move_seed<'out>(out: &mut Self::Out<'out>, mut out_seed: Self::OutSeed)
    where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        mem::swap(out, &mut out_seed);
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
        mem::swap(out, out_seed);
    }

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        _out: &mut Self::Out<'out>,
        _own: &'ownref Self::Own<'own>,
    ) where
        Self::Out<'out>: 'out,
        Item: 'out,
    {
    }
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

pub fn bench_vec_sort_bin_search<
    InItem,
    T,
    //#[allow(non_camel_case_types)] TRANS_REF_OUTER_HOLDER,
    #[allow(non_camel_case_types)] TRANS_REF,
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
    T: CamiOrd + Ord + Clone,
    TRANS_REF: TransRef<InItem, T>,
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
    let mut in_items = TRANS_REF::In::with_capacity(num_items);
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
    >*/TRANS_REF::ini_own_and_seed(in_items);

    //#[cfg(off)]
    let own_items = /*<<<TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>>::TransRefInnerHolder<
        '_, '_,
    > as TransRefVecInnerHolder<'_, '_, IN_ITEM, T>>::TransRefImpl as TransRef<T>>*/TRANS_REF::reserve_own(
    );

    {
        let mut unsorted_items = /*<
       <
        <
            TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>
        >
        ::TransRefInnerHolder<'_, '_> as TransRefVecInnerHolder<'_, '_,IN_ITEM, T>
       >::TransRefImpl as TransRef<T>
    >*/TRANS_REF::reserve_out();
        /*<
           <
            <
                TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>
            >
            ::TransRefInnerHolder<'_, '_> as TransRefVecInnerHolder<'_, '_,IN_ITEM, T>
           >::TransRefImpl as TransRef<T>
        >*/
        TRANS_REF::ini_out_mut_seed(&mut unsorted_items, &mut out_seed);

        /*<
           <
            <
                TRANS_REF_OUTER_HOLDER as TransRefVecOuterHolder<IN_ITEM, T>
            >
            ::TransRefInnerHolder<'_, '_> as TransRefVecInnerHolder<'_, '_,IN_ITEM, T>
           >::TransRefImpl as TransRef<T>
        >*/
        TRANS_REF::set_out(&mut unsorted_items, &own_items);
        // CANNOT: let unsorted_items = unsorted_items; // Prevent mutation by mistake.

        //for size in [K, 2 * K, 4 * K, 8 * K, 16 * K].iter() {
        let id_string = format!(
            "{num_items} items, each len max {MAX_ITEM_LEN}.{}",
            id_postfix(id_state)
        );
        //#[cfg(do_later)]
        if false {
            let mut sorted_lexi = Vec::new();
            group.bench_with_input(
                BenchmarkId::new("std sort lexi.          ", id_string.clone()),
                hint::black_box(&unsorted_items),
                |b, unsorted_items| {
                    b.iter(|| {
                        sorted_lexi = hint::black_box(unsorted_items.clone());
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
                        for item in hint::black_box(unsorted_items.into_it()) {
                            hint::black_box(sorted.binary_search(item)).unwrap();
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
                unsorted_items_cami.extend(unsorted_items.it().map(|v| Cami::<TO>::new(v.clone())));
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
                            sorted_non_lexi =
                                hint::black_box(unsorted_items.clone()).into_vec_cami();
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
                        for item in hint::black_box(unsorted_items.into_iter()) {
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
