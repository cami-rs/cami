# Benches (of public API only)

## How to run

You **must** specify features used by the appropriate bench. At the moment it's only `std` or
`std,transmute`:

- `cargo bench --features std`
- `cargo bench --features std,transmute`

`std` is required by the benches. But, because `std` is not a default feature in `camigo`, those
benches won't be run until you specify it.