//#![allow(warnings, unused)]

use camigo::ca_wrap_struct;
use core::{hint, iter, ops::RangeBounds, time::Duration};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fastrand::Rng;

// On heap.
const MIN_ITEMS: usize = 10;
const MAX_ITEMS: usize = 100_000;
// On heap. For example, for String, this is the maximum number of `char` - so the actual UTF-8
// size may be a few times higher.
const MAX_ITEM_LEN: usize = 1_000;

// For purging the L1, L2..., in bytes.
const MAX_CACHE_SIZE: usize = 2_080_000;

const USIZE_MAX_HALF: usize = usize::MAX / 2;

fn purge_cache(rng: &mut Rng) {
    let mut vec = Vec::<u8>::with_capacity(MAX_CACHE_SIZE);

    for _ in [0..MAX_CACHE_SIZE] {
        vec.push(rng.u8(..));
    }
    hint::black_box(vec);
}

pub fn bench_strings(c: &mut Criterion) {
    let mut rng = Rng::new();

    bench_strings_range(c, &mut rng, MIN_ITEMS..MAX_ITEMS);
}

pub fn bench_strings_range(
    c: &mut Criterion,
    mut rng: &mut Rng,
    num_items: impl RangeBounds<usize>,
) {
    let num_items = rng.usize(num_items);
    let mut unsorted_items = Vec::<String>::with_capacity(num_items);
    let mut total_length = 0usize;

    for _ in [0..num_items] {
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

    purge_cache(&mut rng);
    let mut sorted_non_lexi = Vec::new();
    group.bench_with_input(
        BenchmarkId::new("std sort non-lexi.      ", id_string.clone()),
        hint::black_box(&unsorted_items),
        |b, unsorted_items| {
            b.iter(|| {
                sorted_non_lexi = hint::black_box(unsorted_items.clone()); // @TODO Transmute
                sorted_non_lexi.sort();
            })
        },
    );
    purge_cache(&mut rng);
    group.bench_with_input(
        BenchmarkId::new("std bin search (non-lexi)", id_string.clone()),
        hint::black_box(&unsorted_items),
        |b, unsorted_items| {
            b.iter(|| {
                let sorted = hint::black_box(&sorted_non_lexi);
                for item in hint::black_box(unsorted_items.into_iter()) {
                    //@TODO wrap/transmute item
                    hint::black_box(sorted.binary_search(item)).unwrap();
                }
            })
        },
    );
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_millis(200));
    targets = bench_strings
}
criterion_main!(benches);
