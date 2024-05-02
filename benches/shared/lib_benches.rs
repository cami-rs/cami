// This file is used from various benches, and not all of them use all functionality from here. So,
// some items have `#[allow(unused)]`.
use camigo::prelude::*;
use core::{hint, time::Duration};
use criterion::{BenchmarkId, Criterion};
use fastrand::Rng;
use std::ops::RangeBounds;

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

pub fn bench_vec_sort_bin_search<T: CamiOrd + Ord + Clone, RND: Random, ID>(
    c: &mut Criterion,
    rng: &mut RND,
    group_name: impl Into<String>,
    id: &mut ID,
    id_postfix: fn(&ID) -> String,
    generate_item: fn(&mut RND, &mut ID) -> T,
)
//GEN: FnMut() -> T,
//ID: Fn() -> String,
{
    let num_items = rng.usize(MIN_ITEMS..MAX_ITEMS);
    let mut unsorted_items = Vec::<T>::with_capacity(num_items);

    for _ in 0..num_items {
        let item = generate_item(rng, id);
        unsorted_items.push(item);
    }

    let mut group = c.benchmark_group(group_name);

    //for size in [K, 2 * K, 4 * K, 8 * K, 16 * K].iter() {
    let id_string = format!(
        "{num_items} items, each len max {MAX_ITEM_LEN}.{}",
        id_postfix(id)
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
        purge_cache(rng);
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
        purge_cache(rng);
        #[cfg(not(feature = "transmute"))]
        let unsorted_items = {
            let mut unsorted_items_cami = Vec::with_capacity(unsorted_items.len());
            unsorted_items_cami.extend(unsorted_items.iter().map(|v| Cami::<T>::new(v.clone())));
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
        purge_cache(rng);
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
