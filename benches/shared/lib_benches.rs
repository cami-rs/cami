// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use camigo::prelude::*;
use core::{hint, time::Duration};
use criterion::{BenchmarkId, Criterion};
use fastrand::Rng;
//use std::{marker::PhantomData, ops::RangeBounds};
use core::mem;
use core::ops::RangeBounds;

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

pub trait TransRef<T>: Sized {
    type In;
    // NOT needed: where T: 'own
    type Own<'own>;

    type OutSeed;
    // Out<> DOES need: where T: 'out
    type Out<'out>
    where
        T: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed);
    fn reserve_own<'own>() -> Self::Own<'own> {
        todo!()
    }

    fn reserve_out<'out>() -> Self::Out<'out>
    where
        T: 'out;

    fn ini_out_move_seed<'out>(out: &mut Self::Out<'out>, mut out_seed: Self::OutSeed)
    where
        T: 'out,
    {
        Self::ini_out_mut_seed(out, &mut out_seed);
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        T: 'out;

    //fn set_out<'own: 'out, 'out>(out: &mut Self::OUT<'out>, own: &Self::OWN<'own>)
    //fn set_out<'out>(out: &mut Self::OUT<'out>, own: & Self::OWN)
    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        out: &mut Self::Out<'out>,
        own: &'ownref Self::Own<'own>,
    ) where
        T: 'out;
}

pub trait TransRefInnerHolder<'out, 'own: 'out, #[allow(non_camel_case_types)] InItems, T>
where
    T: 'out,
{
    #[allow(non_camel_case_types)]
    type TransRefImpl: TransRef<T, In = Vec<InItems>, Own<'own> = Vec<InItems>, Out<'out> = Vec<T>>
    where
        T: 'out,
        <Self as TransRefInnerHolder<'out, 'own, InItems, T>>::TransRefImpl: 'out;
}
pub trait TransRefOuterHolder<#[allow(non_camel_case_types)] InItems, T> {
    #[allow(non_camel_case_types)]
    type TransRefInnerHolder<'out, 'own: 'out>: TransRefInnerHolder<'out, 'own, InItems, T>
    where
        T: 'out;
}

//pub struct VecVecToVecSlice<T>(PhantomData<T>);
//pub struct VecVecToVecSlice<'t>(PhantomData<&'t ()>);

// @TODO how avbout removing <T> from VecVecToVecSlice:
//
pub struct VecVecToVecSlice();
//
impl<T> TransRef<T> for VecVecToVecSlice
//impl<'t, T: 't> TransRef<'t, T> for VecVecToVecSlice<'t>
// where Self: 't,
{
    //impl<T> TransRef<T> for VecVecToVecSlice<T> {
    type In = Vec<Vec<T>>;
    type Own<'own> = Vec<Vec<T>>;
    type OutSeed = usize;
    type Out<'out> = Vec<&'out [T]> where T: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        let len = input.len();
        (input, len)
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        T: 'out,
    {
        Vec::new()
    }

    // @TODO Delay reservation?
    fn ini_out_move_seed<'out>(out: &mut Self::Out<'out>, out_seed: Self::OutSeed)
    where
        T: 'out,
    {
        out.reserve(out_seed);
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        T: 'out,
    {
        out.reserve(*out_seed);
    }

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        out: &mut Self::Out<'out>,
        own: &'ownref Self::Own<'own>,
    ) where
        T: 'out,
    {
        out.extend(own.iter().map(|v| &v[..]));
    }
}

//pub struct VecToVecCloned<T>(PhantomData<T>);
pub struct VecToVecCloned();

//impl<'t, T: 't + Clone /*'t, T: 't + Clone*/> TransRef<'t, T> for VecToVecCloned<T> where Self : 't{
impl<T: Clone> TransRef<T> for VecToVecCloned {
    type In = Vec<T>;
    type Own<'own> = Vec<T>;
    type OutSeed = ();
    type Out<'out> = Vec<T> where T: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        (input, ())
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        T: 'out,
    {
        Vec::new()
    }

    /// Delay allocation a.s.a.p. - lazy.
    fn ini_out_move_seed<'out>(_out: &mut Self::Out<'out>, _out_seed: Self::OutSeed)
    where
        T: 'out,
    {
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        T: 'out,
    {
    }

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        out: &mut Self::Out<'out>,
        own: &'ownref Self::Own<'own>,
    ) where
        T: 'out,
    {
        out.reserve(own.len());
        out.extend_from_slice(&own[..]);
    }
}

//pub struct VecToVecMoved<T>(PhantomData<T>);
pub struct VecToVecMoved();

//impl<'t, T: 't> TransRef<'t, T> for VecToVecMoved<T> where Self: 't{
impl<T> TransRef<T> for VecToVecMoved {
    type In = Vec<T>;
    type Own<'own> = Vec<T>;
    type OutSeed = Vec<T>;
    type Out<'out> = Vec<T> where T: 'out;

    fn ini_own_and_seed<'own>(input: Self::In) -> (Self::Own<'own>, Self::OutSeed) {
        (Vec::new(), input)
    }
    fn reserve_out<'out>() -> Self::Out<'out>
    where
        T: 'out,
    {
        Vec::new()
    }

    fn ini_out_move_seed<'out>(out: &mut Self::Out<'out>, mut out_seed: Self::OutSeed)
    where
        T: 'out,
    {
        mem::swap(out, &mut out_seed);
    }
    fn ini_out_mut_seed<'out, 'outref>(
        out: &'outref mut Self::Out<'out>,
        out_seed: &'outref mut Self::OutSeed,
    ) where
        T: 'out,
    {
        mem::swap(out, out_seed);
    }

    fn set_out<'own: 'out, 'out, 'ownref: 'out>(
        _out: &mut Self::Out<'out>,
        _own: &'ownref Self::Own<'own>,
    ) where
        T: 'out,
    {
    }
}
/*
pub struct VecToVecMovedInnerHolder();
impl<'out, #[allow(non_camel_case_types)] IN_ITEM, T> TransRefInnerHolder<'out, IN_ITEM, T>
    for VecToVecMovedInnerHolder
where
    T: 'out,
{
    type TRANS_REF = VecToVecMoved;
}
*/
/*pub struct VecToVecMovedOuterHolder();
impl<#[allow(non_camel_case_types)] IN_ITEM, T> TransRefOuterHolder<IN_ITEM, T>
    for VecToVecMovedOuterHolder
{
    type TRANS_REF_INNER_HOLDER<'out> = VecToVecMovedInnerHolder where T: 'out;
}*/

pub fn bench_vec_sort_bin_search<
    T,
    #[allow(non_camel_case_types)] IN_ITEM,
    #[allow(non_camel_case_types)] TRANS_REF_OUTER_HOLDER,
    //#[allow(non_camel_case_types)] TRANS_REF,
    RND,
    #[allow(non_camel_case_types)] ID_STATE,
>(
    c: &mut Criterion,
    rnd: &mut RND,
    group_name: impl Into<String>,
    id_state: &mut ID_STATE,
    id_postfix: fn(&ID_STATE) -> String,
    generate: fn(&mut RND, &mut ID_STATE) -> IN_ITEM,
) where
    T: CamiOrd + Ord + Clone,
    TRANS_REF_OUTER_HOLDER: for<'out, 'own> TransRefOuterHolder<
        IN_ITEM,
        T,
        TransRefInnerHolder<'out, 'own>: TransRefInnerHolder<'out, 'own, IN_ITEM, T>,
    >,

    for<'out, 'own> <TRANS_REF_OUTER_HOLDER as TransRefOuterHolder<IN_ITEM, T>>::TransRefInnerHolder<'out, 'own>:
        TransRef<T>,
    RND: Random,
{
    let mut group = c.benchmark_group(group_name);

    let num_items = rnd.usize(MIN_ITEMS..MAX_ITEMS);
    let mut in_items = Vec::<IN_ITEM>::with_capacity(num_items);
    for _ in 0..num_items {
        let item = generate(rnd, id_state);
        in_items.push(item);
    }

    let (own_items, mut out_seed) =
    <
       <
        <
            TRANS_REF_OUTER_HOLDER as TransRefOuterHolder<IN_ITEM, T>
        >
        ::TransRefInnerHolder<'_, '_> as TransRefInnerHolder<'_, '_, IN_ITEM, T>
       >::TransRefImpl as TransRef<T>
    >::ini_own_and_seed(in_items);

    //#[cfg(off)]
    let own_items = <<<TRANS_REF_OUTER_HOLDER as TransRefOuterHolder<IN_ITEM, T>>::TransRefInnerHolder<
        '_, '_,
    > as TransRefInnerHolder<'_, '_, IN_ITEM, T>>::TransRefImpl as TransRef<T>>::reserve_own(
    );

    {
        let mut unsorted_items = <
       <
        <
            TRANS_REF_OUTER_HOLDER as TransRefOuterHolder<IN_ITEM, T>
        >
        ::TransRefInnerHolder<'_, '_> as TransRefInnerHolder<'_, '_,IN_ITEM, T>
       >::TransRefImpl as TransRef<T>
    >::reserve_out();
        <
       <
        <
            TRANS_REF_OUTER_HOLDER as TransRefOuterHolder<IN_ITEM, T>
        >
        ::TransRefInnerHolder<'_, '_> as TransRefInnerHolder<'_, '_,IN_ITEM, T>
       >::TransRefImpl as TransRef<T>
    >::ini_out_mut_seed(&mut unsorted_items, &mut out_seed);

        <
       <
        <
            TRANS_REF_OUTER_HOLDER as TransRefOuterHolder<IN_ITEM, T>
        >
        ::TransRefInnerHolder<'_, '_> as TransRefInnerHolder<'_, '_,IN_ITEM, T>
       >::TransRefImpl as TransRef<T>
    >::set_out(
        &mut unsorted_items,
        &own_items
    );
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
                        for item in hint::black_box(unsorted_items.into_iter()) {
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
