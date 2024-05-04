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
    fn into_it(self) -> impl Iterator<Item = OutItem>;
    fn len(&self) -> usize;
}

impl<OutItem> OutCollection<OutItem> for Vec<OutItem> {
    fn it<'a>(&'a self) -> impl Iterator<Item = &'a OutItem>
    where
        OutItem: 'a,
    {
        <[_]>::iter(self)
    }
    fn into_it(self) -> impl Iterator<Item = OutItem> {
        <Vec<OutItem>>::into_iter(self)
    }
    fn len(&self) -> usize {
        <[_]>::len(self)
    }
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
    fn len(&self) -> usize {
        <[_]>::len(self)
    }
}

pub trait TransRef<'slice, InItem, OutItem> {
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

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        out: &mut Self::Out<'out>,
        own: &'ownref Self::Own<'own>,
    ) where
        Self::Out<'out>: 'out, 'ownref: 'slice;
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
impl<'slice, Item> TransRef<'slice, Vec<Item>, &'slice [Item]> for VecVecToVecSlice
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
        'ownref: 'slice //////
    {
        out.extend(own.it().map(|v| &v[..]));
    }
}

//pub struct VecToVecCloned<T>(PhantomData<T>);
pub struct VecToVecCloned();

//impl<'t, T: 't + Clone /*'t, T: 't + Clone*/> TransRef<'t, T> for VecToVecCloned<T> where Self : 't{
impl<'slice, Item: Clone> TransRef<'slice, Item, Item> for VecToVecCloned {
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
        'ownref: 'slice
    {
        out.reserve(own.len());
        out.extend_from_slice(&own[..]);
    }
}

//pub struct VecToVecMoved<T>(PhantomData<T>);
pub struct VecToVecMoved<'slice, InItem, OutItem>(PhantomData<(&'slice (), InItem,  OutItem)>);
//pub struct VecToVecMoved();
//impl<'t, T: 't> TransRef<'t, T> for VecToVecMoved<T> where Self: 't{
//impl<'out, 'own: 'out, T> TransRef<T> for VecToVecMoved<'out, 'own> {
impl<'slice, InItem, OutItem> TransRef<'slice, InItem, OutItem> for VecToVecMoved<'slice, InItem, OutItem> {
    type In = Vec<InItem>;
    type Own<'own> = Vec<InItem>;
    type OutSeed = Vec<OutItem>;
    type Out<'out> = Vec<OutItem> where Self::Out<'out>: 'out, OutItem: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        todo!("transmute")
        //(Vec::new(), input)
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        Self::Out<'out>: 'out,
        OutItem: 'out,
    {
        Vec::new()
    }

    fn ini_out_move_seed<'out>(out: &mut Self::Out<'out>, mut out_seed: Self::OutSeed)
    where
        Self::Out<'out>: 'out,
        OutItem: 'out,
    {
        mem::swap(out, &mut out_seed);
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        Self::Out<'out>: 'out,
        OutItem: 'out,
    {
        mem::swap(out, out_seed);
    }

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        _out: &mut Self::Out<'out>,
        _own: &'ownref Self::Own<'own>,
    ) where
        Self::Out<'out>: 'out,
        OutItem: 'out, 'ownref: 'slice
    {
    }
}

pub struct VecToVecMovedHolder();
impl<InItem, OutItem> TransRefHolder<InItem, OutItem> for VecToVecMovedHolder {
    type TransRefImpl<'slice> = VecToVecMoved<'slice, InItem, OutItem>;
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

pub trait TransRefHolder<InItem, OutItem> {
    type TransRefImpl<'slice>: TransRef<'slice, InItem, OutItem>;
    //where OutItem: 'out;
}

pub fn bench_vec_sort_bin_search<
    InItem,
    OutItem,
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
    <
        <TRANS_REF_HOLDER as TransRefHolder<InItem, OutItem>
        >::TransRefImpl<'_>
           as TransRef<'_, InItem, OutItem>
    >::In::with_capacity(1);
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
         <
        <TRANS_REF_HOLDER as TransRefHolder<InItem, OutItem>
        >::TransRefImpl<'_>
           as TransRef<'_, InItem, OutItem>
    >::ini_out_mut_seed(&mut unsorted_items, &mut out_seed);

        /*<
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
    >::set_out(&mut unsorted_items, &own_items);
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
                        // @TODO ^^^--> .clone()  \----> change to:
                        //
                        // .sorted_lexi.extend( it().map(|it_ref| it_ref.clone()))
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
                        // @TODO We don't need .into_it() here \\\ - only .it() so we get references
                        for item in hint::black_box(unsorted_items.into_it()) {
                            // ^^^^ @TODO bin search = by reference
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
                            // @TODO replace .clone() by: Vec::with_capacity(), .iter() -> extend -> .into_vec_cami()
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
                        // The following `unsorted_items.into_iter()` is cheap (no consuming of any `Vec`), because `unsorted_items`` is a reference to a Vec.
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
