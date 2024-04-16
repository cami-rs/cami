use camigo::{ca_wrap, Slice};
use core::hint;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// On heap.
const MAX_ITEMS: usize = 100_000;
// On heap. For example, for String, this is the maximum number of `char` - so the actual UTF-8
// size may be a few times higer.
const MAX_ITEM_LEN: usize = 1_000;

// For purging the L1, L2..., in bytes.
const MAX_CACHE_SIZE: usize = 2_080_000;

const USIZE_MAX_HALF: usize = usize::MAX / 2;

fn unfrequent_rnd<T>(recently_allocated: *const T) -> usize {
    #[cfg(feature = "fastrand")]
    return {
        let _ = recently_allocated; // to avoid "unused" warning
    };
    #[cfg(not(feature = "fastrand"))]
    return recently_allocated as usize;
}

fn frequent_rnd<T>(recently_allocated: *const T) -> usize {
    #[cfg(feature = "rnd_frequent")]
    return {
        let _ = recently_allocated; // to avoid "unused" warning

        #[cfg(feature = "fastrand")]
        let _ = {};
        // @TODO
    };
    #[cfg(not(feature = "rand_frequent"))]
    return recently_allocated as usize;
}

fn purge_cache() {
    let mut vec = Vec::<u8>::with_capacity(MAX_CACHE_SIZE);

    // Entropy doesn't matter here, the source/stream is only to fill up the cache, and then it goes
    // to black_box.
    let rnd_start = frequent_rnd(&vec) % USIZE_MAX_HALF;
    let mut rnd = rnd_start;

    for _ in [0..MAX_CACHE_SIZE] {
        vec.push((rnd % 256) as u8);
        rnd += rnd_start;
        rnd %= USIZE_MAX_HALF;
    }
    hint::black_box(vec);
}

pub fn bench_strings(c: &mut Criterion) {
    let num_items = unfrequent_rnd(&c) % MAX_ITEMS;
    let mut unsorted_items = Vec::<String>::with_capacity(num_items);
    let mut total_length = 0usize;

    for _ in [0..num_items] {
        let item_len = frequent_rnd(&unsorted_items) % MAX_ITEM_LEN;
        let mut item = Vec::<char>::with_capacity(item_len);

        let rnd_start = frequent_rnd(&item) % USIZE_MAX_HALF;
        let mut rnd = rnd_start;

        for _ in 0..item_len {
            item.push(((rnd % 10) as u8 + b'0').into());
            rnd += rnd_start;
            rnd %= USIZE_MAX_HALF;
        }
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
    group.bench_with_input(
        BenchmarkId::new("std bin search (non-lexi)", id_string.clone()),
        hint::black_box(&unsorted_items),
        |b, unsorted_items| {
            b.iter(|| {
                let sorted = hint::black_box(&sorted_lexi);
                for item in hint::black_box(unsorted_items.into_iter()) {
                    hint::black_box(sorted_non_lexi.binary_search(item)).unwrap();
                }
            })
        },
    );
    group.bench_with_input(
        BenchmarkId::new("ca  bin search      ", id_string.clone()),
        hint::black_box(&unsorted_items),
        |b, unsorted_items| {
            b.iter(|| {
                let sorted = hint::black_box(&sorted_lexi); // @TODO sorted_non_lexi?
                for item in hint::black_box(unsorted_items.into_iter()) {
                    // !! TODO
                    //
                    // Check: Should we FIRST sort the items as per COrd (on-lexi)?
                    //
                    // If so, transmute unsorted_items, clone, .sort().
                    hint::black_box(sorted.binary_search_ca(item)).unwrap();
                }
            })
        },
    );

    group.finish();
}

criterion_group!(benches, bench_strings);
criterion_main!(benches);
