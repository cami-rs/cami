//#![allow(warnings, unused)]
use crate::*;
use camigo::prelude::*;
use core::{hint, iter, ops::RangeBounds, time::Duration};
use criterion::{criterion_group, BenchmarkId, Criterion};
use fastrand::Rng;

// On heap.
const MIN_ITEMS: usize = 4; //10;
const MAX_ITEMS: usize = 10; //100_000;
                             // On heap. For example, for String, this is the maximum number of `char` - so the actual UTF-8
                             // size may be a few times higher.
const MAX_ITEM_LEN: usize = 4; //1_000;

pub fn bench_target(c: &mut Criterion) {
    let mut rng = Rng::new();

    bench_range(c, &mut rng, MIN_ITEMS..MAX_ITEMS);
}

pub fn bench_range(c: &mut Criterion, mut rng: &mut Rng, num_items: impl RangeBounds<usize>) {
    let num_items = rng.usize(num_items);
    let mut unsorted_items = Vec::<String>::with_capacity(num_items);
    let mut total_length = 0usize;

    for _ in 0..num_items {
        let item_len = rng.usize(..MAX_ITEM_LEN);
        let mut item = Vec::<char>::with_capacity(item_len);
        item.extend(iter::repeat_with(|| rng.char(..)).take(item_len));

        let mut string = String::with_capacity(4 * item_len);
        string.extend(item.into_iter());
        total_length += string.len();
        unsorted_items.push(string);
    }

    let mut group = c.benchmark_group("strings");

    //for size in [K, 2 * K, 4 * K, 8 * K, 16 * K].iter() {
    let id_string =
        format!("{num_items} items, each len max {MAX_ITEM_LEN}. Sum len: {total_length}.");
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
        purge_cache(&mut rng);
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
        purge_cache(&mut rng);
        #[cfg(not(feature = "transmute"))]
        let unsorted_items = {
            let mut unsorted_items_cami = Vec::with_capacity(unsorted_items.len());
            unsorted_items_cami.extend(unsorted_items.iter().map(|v| StringCami::new(v.clone())));
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
        purge_cache(&mut rng);
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

criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_millis(200));
    targets = bench_target
}
// Based on expansion of `criterion_main!(benches);`
fn main() {
    benches();

    Criterion::default().configure_from_args().final_summary();
}
