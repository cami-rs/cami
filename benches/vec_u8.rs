//#![allow(warnings, unused)]
use camigo::prelude::*;
use core::{hint, ops::RangeBounds, time::Duration};
use criterion::{criterion_group, BenchmarkId, Criterion};
use fastrand::Rng;

// On heap.
const MIN_ITEMS: usize = 4; //10;
const MAX_ITEMS: usize = 10; //100_000;

// For purging the L1, L2..., in bytes.
const MAX_CACHE_SIZE: usize = 2_080_000;

//const USIZE_MAX_HALF: usize = usize::MAX / 2;

fn purge_cache(rng: &mut Rng) {
    let mut vec = Vec::<u8>::with_capacity(MAX_CACHE_SIZE);

    for _ in [0..MAX_CACHE_SIZE] {
        vec.push(rng.u8(..));
    }
    hint::black_box(vec);
}

pub fn bench_target(c: &mut Criterion) {
    let mut rng = Rng::new();

    bench_range(c, &mut rng, MIN_ITEMS..MAX_ITEMS);
}

pub fn bench_range(c: &mut Criterion, mut rng: &mut Rng, num_items: impl RangeBounds<usize>) {
    let num_items = rng.usize(num_items);
    let mut unsorted_items = Vec::<u8>::with_capacity(num_items);

    for _ in 0..num_items {
        let item = rng.u8(..);
        unsorted_items.push(item);
    }

    let mut group = c.benchmark_group("u8's");

    //for size in [K, 2 * K, 4 * K, 8 * K, 16 * K].iter() {
    let id_string = format!("{num_items} items.");
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
        let mut sorted_non_lexi = Vec::new();
        group.bench_with_input(
            BenchmarkId::new("std sort non-lexi.      ", id_string.clone()),
            hint::black_box(&unsorted_items),
            |b, unsorted_items| {
                b.iter(|| {
                    sorted_non_lexi = hint::black_box(unsorted_items.clone()).into_vec_cami();
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
                        hint::black_box(sorted.binary_search(item.into_ref_cami())).unwrap();
                        //hint::black_box(sorted.binary_search(item)).unwrap();
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
