## Names & Naming Conventions

### Macros: no prefix/postfix

cpartial_eq ? partial_eq eq! ???

c_ord! -> cord -> impl COrd ord = impl core::cmp::Ord

### Traits/Types: "C" prefix
CPartialEq, CPartialOrd, COrd

### Functions: postfix
Names like in core/std, with a "local" or other postfix.
- eq_local, eq_non_local, eq_full
- cmp_local

- binary_search_cami
- sort_cami
- is_sorted_cami

## Benchmarking - Criterion or otherwise

Purging cache, to eliminate & measure impact on cache.

## Crates for randomness

Not just bytes/usize, but any fast random data generators. Ideally with options like for String:
ASCII-only, major languages only, lowercase/uppercase...?
