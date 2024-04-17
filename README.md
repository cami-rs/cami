# Camigo = Cache Friendly

Zero cost wrappers & related implementation of cache-friendly comparison. `no_std`-friendly.

## Non-vector-like items

This is about comparing values/objects other than slices/arrays/Vec/String/&str. Of course, these
values/objects can be stored in a slice/array/Vec/String/&str, and that's most likely where this
cache-friendly comparison brings benefits.

This comparison may DIFFER to the `#[derive(...)]`'s default order: [the top-to-bottom declaration
order of the struct’s members](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#derivable).

## Vec/slice/array as containers

- sequential search for primitive/local-only item types (no references) has no speed benefit
- sequential search for item types that have both local fields and references can have speed benefit
- binary search - speed benefit, for both
  - primitive/local-only item types (no references), and (even more so) - only for small types (with
    size less than half a cache line, so less than 64B on mainstream CPU's) - work in progress, it
    may turn out not to be beneficial (because of extra code branching)
  - item types that have both local fields and references (potentially much more beneficial than for
    local-only)

## Vec/slice/array/String/&str as items

This DIFFERS to their [`Ord` > Lexicographical
comparison](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison).

Beneficial only if the items being compared/stored in the same container are not of the same size.
Hence not suitable for items, or their fields, of fixed size, like `sha256/other` hashes, UUID's,
fixed-length usernames...

## HashMap/HashSet items

This comparison doesn't give as much benefit for `HashMap` & `HashSet` (because those use `Hash` for
determining the buckets). But it can speed up comparison of keys in the same bucket (with the same
hash). And, since `HashMap` & `HashSet` don't keep/guarantee any order, using `camigo` makes
transition/backwards compatibility easier.

## BTreeMap/BTreeSet items

Transmuting those would be AGAINST their correctness, because they maintain the ordered state, and
they depend on it. TODO explain more.

Indeed, the actual items stored in a `BTreeMap/BTreeSet` can use this crate.

## Actual traits & applicability

### CPartialEq & COrd

TODO

You could transmute (zero cost) a wrapper back to the original Vec/slice/array/String/&str, as far
as no existing code depends on `derive`'s default ordering and Vec/slice/array/String/&str's
lexicographic ordering.

### Fallback to PartialEq & Ord

TODO

Suitable if you can't change some existing (3rd party) code to use camigo's binary search methods.
Possible only if no existing code depends on `derive`'s default ordering and
Vec/slice/array/String/&str's lexicographic ordering.

Compared to using `CPartialEq` and `COrd`, this doesn't give any binary search benefit for
primitive/local-only types. But it can speed up binary search for types with references.

## Scope

### No derive macro(s)
We will never have `#derive(...)` macro(s) for `CPartialEq/COrd`, because no macro can
access/differentiate/interpret/"know" types being used.

## Design

This design is (arguably) "coupled", because it puts these features together. Guess what: Life is
too short for uncoupled designs here. They would be both difficult to implement, unergonomic to use,
and complicated to maintain.
