# Benches (of public API only)

## How to run

You **must** specify any optional features used by the appropriate bench. Relevant features (ones that affect any benches; this list may change):

- only `std` is required, and
- `transmute` is optional.

Invoke `cargo bench` or `cargo check --benches` like:

- `cargo check --benches --bench vec_u8 --features std`
- `cargo check --benches --bench vec_u8 --features std,transmute`
- `cargo check --benches                --features std`
- `cargo check --benches                --features std,transmute`
- `cargo bench --bench vec_u8           --features std`
- `cargo bench --bench vec_u8           --features std,transmute`
- `cargo bench                          --features std`
- `cargo bench                          --features std,transmute`

`std` is required by the benches. But, because `std` is not a default feature in `camigo`, those
benches won't be run until you specify it.