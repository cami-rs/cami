//#![allow(warnings, unused)]
use camigo::prelude::*;
use criterion::{criterion_group, Criterion};
use fastrand::Rng;
use lib_benches::*;

#[path = "shared/lib_benches.rs"]
mod lib_benches;

pub fn bench_target(c: &mut Criterion) {
    let mut rng = Rng::new();

    type IdState = ();

    fn generate_item(rng: &mut Rng, _: &mut IdState) -> u8 {
        rng.u8(..)
    }

    fn id_postfix(_: &IdState) -> String {
        String::new()
    }

    let mut id_state: IdState = ();

    bench_vec_sort_bin_search(c, &mut rng, "u8", &mut id_state, id_postfix, generate_item);
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = bench_target
}
// Based on expansion of `criterion_main!(benches);`
fn main() {
    benches();

    Criterion::default().configure_from_args().final_summary();
}
