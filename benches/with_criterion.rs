use camigo::ca_struct;
// @TODO pull request to Criterion that functions should not be imported - especially because it
// "conflicts" with `core::hint::black_box`.
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
