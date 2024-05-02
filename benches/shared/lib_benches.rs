// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use camigo::prelude::*;
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

pub trait TransRef<T> {
    type IN;
    type OWN;
    type OUT<'t>
    where
        T: 't;

    /// This initializes `REFS`, for example, if it's a
    /// vector (of references/slices) this initializes it with [Vec::with_capacity] based on
    /// capacity of given `own`.
    fn ini_own_and_out<'t>(input: Self::OWN) -> (Self::OWN, Self::OUT<'t>);

    fn set_out<'t>(own: &'t Self::OWN, refs: &'t mut Self::OUT<'t>);
}

pub struct VecVecToVecSlice<T>(PhantomData<T>);

impl<T> TransRef<T> for VecVecToVecSlice<T> {
    type IN = Vec<Vec<T>>;
    type OWN = Vec<Vec<T>>;
    type OUT<'t> = Vec<&'t [T]> where T: 't;

    fn ini_own_and_out<'t>(input: Self::OWN) -> (Self::OWN, Self::OUT<'t>) {
        let out = Vec::with_capacity(input.len());
        (input, out)
    }
    fn set_out<'t>(own: &'t Self::OWN, refs: &'t mut Self::OUT<'t>) {
        refs.extend(own.iter().map(|v| &v[..]));
    }
}

pub struct VecToVecCloned<T>(PhantomData<T>);

impl<T: Clone> TransRef<T> for VecToVecCloned<T> {
    type IN = Vec<T>;
    type OWN = Vec<T>;
    type OUT<'t> = Vec<T> where T: 't;

    fn ini_own_and_out<'t>(input: Self::OWN) -> (Self::OWN, Self::OUT<'t>)
    where
        T: 't,
    {
        let out = Vec::with_capacity(input.len());
        (input, out)
    }
    fn set_out<'t>(own: &'t Self::OWN, refs: &'t mut Self::OUT<'t>) {
        refs.extend(own.iter().cloned());
    }
}

pub struct VecToVecMoved<T>(PhantomData<T>);

impl<T> TransRef<T> for VecToVecMoved<T> {
    type IN = Vec<T>;
    type OWN = Vec<T>;
    type OUT<'t> = Vec<T> where T: 't;

    fn ini_own_and_out<'t>(input: Self::OWN) -> (Self::OWN, Self::OUT<'t>)
    where
        T: 't,
    {
        (Vec::new(), input)
    }
    fn set_out<'t>(_own: &'t Self::OWN, _refs: &'t mut Self::OUT<'t>) {}
}

fn _apply_transref() {
    let own = vec![vec![1i32, 2], vec![1, 2, 3], vec![4], vec![5, 6]];
    let (own, mut refs) = VecVecToVecSlice::<i32>::ini_own_and_out(own);
    VecVecToVecSlice::<i32>::set_out(&own, &mut refs);
}

pub fn bench_vec_sort_bin_search<GEN, TO, TRANSF, RND, IDSTATE>(
    c: &mut Criterion,
    rnd: &mut RND,
    group_name: impl Into<String>,
    id_state: &mut IDSTATE,
    id_postfix: fn(&IDSTATE) -> String,
    generate: fn(&mut RND, &mut IDSTATE) -> TO,
    _transform: TRANSF,
) where
    TO: CamiOrd + Ord + Clone,
    TRANSF: Fn(GEN) -> TO,
    RND: Random,
{
    let num_items = rnd.usize(MIN_ITEMS..MAX_ITEMS);
    let mut unsorted_items = Vec::<TO>::with_capacity(num_items);
    for _ in 0..num_items {
        let item = generate(rnd, id_state);
        unsorted_items.push(item);
    }

    let mut group = c.benchmark_group(group_name);

    //for size in [K, 2 * K, 4 * K, 8 * K, 16 * K].iter() {
    let id_string = format!(
        "{num_items} items, each len max {MAX_ITEM_LEN}.{}",
        id_postfix(id_state)
    );
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
    {
        purge_cache(rnd);
        #[cfg(not(feature = "transmute"))]
        let unsorted_items = {
            let mut unsorted_items_cami = Vec::with_capacity(unsorted_items.len());
            unsorted_items_cami.extend(unsorted_items.iter().map(|v| Cami::<TO>::new(v.clone())));
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
                        sorted_non_lexi = hint::black_box(unsorted_items.clone()).into_vec_cami();
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
                            hint::black_box(sorted.binary_search(item.into_ref_cami())).unwrap();
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
    group.finish();
}
