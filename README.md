# Cami = Cache Friendly

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
Hence NOT suitable for items, or their fields, of fixed size, like `sha256/other` hashes, UUID's,
fixed-length usernames, condensed dates/timestamps...

## HashMap/HashSet items

This comparison doesn't give as much benefit for `HashMap` & `HashSet` (because those use `Hash` for
determining the buckets). But it can speed up comparison of keys in the same bucket (with the same
hash). And, since `HashMap` & `HashSet` don't keep/guarantee any order, using `cami` makes
transition/backwards compatibility easier.

## BTreeMap/BTreeSet items

Transmuting those would be AGAINST their correctness, because they maintain the ordered state, and
they depend on it. TODO explain more.

Indeed, the actual items stored in a `BTreeMap/BTreeSet` can use this crate.

## Actual traits & applicability

TODO rename CPartial etc.

### CPartialEq & COrd

TODO

You could transmute (zero cost) a wrapper back to the original Vec/slice/array/String/&str, as far
as no existing code depends on `derive`'s default ordering and Vec/slice/array/String/&str's
lexicographic ordering.

### Fallback to PartialEq & Ord

TODO

Suitable if you can't change some existing (3rd party) code to use `cami`'s binary search methods.
Possible only if no existing code depends on `derive`'s default ordering and
Vec/slice/array/String/&str's lexicographic ordering.

Compared to using `CPartialEq` and `COrd`, this doesn't give any binary search benefit for
primitive/local-only types. But it can speed up binary search for types with references.

## Scope

### No derive macro(s) initially
No `#derive(...)` macro(s) for `CamiPartialEq/CamiOrd`, because no macro can
access/differentiate/interpret/"know" types being used.

### No design theories

This design is (arguably) "coupled", because it puts these features together. Guess what: Life is
too short for uncoupled designs here. They would be both difficult to implement, unergonomic to use,
and complicated to maintain.

This is complicated enough - if it weren't, it would have existed already. We're not here to please
some design theories, but to make it useful.

## Permanently below 1.0.0

### Unstable: below 0.1.0

### Stable: From 0.1.0, but below 1.0.0

## Depending on Cami and 3rd party crates

Recommending three methods:

**1.** For
- controlled/conservative development,
- if implementation of Cami traits for a 3rd party crate is a part of Cami (rather than the 3rd
party crate itself), and especially if it seems that this may not change within a few months (so the
implementation of Cami traits would stay in Cami rather than in the 3rd party crate),

use
- a flexible (major) version `"0.*"` of `cami`, with `adapt-abccc` feature(s) enabled for any (3rd
  party) crates like `abccc` for which Cami has implemented its traits, and
- a fixed **minor** version of the (3rd party) crate. For example, `"1.0.*"` of
  [`smartstring`](https://crates.io/crates/smartstring): `smartstring = "1.0.*"`.

Once/if implementation of Cami traits (for such a 3rd party crate) is moved out of Cami and into the
crate itself, that crate should have a minor version increase. Soon after (or, hopefully, at near
the same time), Cami will have a new (major) version for it. Then you can migrate to one of the last
two of the following three methods (**#3** or **#4**).

**2.** For
- fairly conservative development,
- accepting that there may be a need for a small manual change, but not earlier than in six months,
- if implementation of Cami traits for a 3rd party crate is (**still**) a part of Cami (rather than
the 3rd party crate itself), especially if there is an indication/conversations about moving it (the
implementation of Cami traits for the crate) out of Cami and to the (3rd party) crate itself,
- but where you are available to manually update `Cargo.toml` - if need be and no earlier than in
  six months,

use
- a flexible (major) version `"0.*"` of `cami`, with `adapt-abccc` feature(s) enabled for any (3rd
  party) crates like `abc` for which Cami has implemented its traits, and
- a fixed **major** version of the (3rd party) crate. For example, `"1.*"` of `abccc`: `abccc =
  "1.0.*"`.

**3.** For
- fairly conservative development, but
- only once an implementation of Cami traits (for a 3rd party crate being used) is a part of that
  crate itself,

use
- a flexible (major) `"0.*"` version of `cami` - but with NO `adapt-abccc` feature, because that
  becomes deprecated right then, and it will be removed six months later (or it has been removed
  already if six months have passed); and
- a fixed **minor** version of the 3rd party crate upgraded. For example, `"1.1.*"` of `abccc`:
  `abccc = "1.1.*"`.
- beware that
  - once `adapt-abccc` feature(s) are removed (six months later),
  - if you use any other 
  - your usage of Cami may become incompatible with any out-of-date 4th party crates that depend on
  old `adapt-***` features, because such old crates would then pull a respective old version of
  Cami. (`cargo` fetches the newest version which still has all the requested `adapt-***` features.)
  Then ask maintainer(s) of such 4th party crate(s) to update them.

**4.** For
- fluid/adaptable development, and
- only once an implementation of Cami traits (for a 3rd party crate being used) is a part of that
  crate itself,

use
- a flexible (major) `"0.*"` version of `cami` - but with NO `adapt-abccc` feature, because that
  becomes deprecated right then, and it will be removed six months later (or it has been removed
  already if six months have passed); and
- a minimum **minor** (or, less likely, **patch**) version of the 3rd party crate. For example,
  `"1.1.0"` of `abccc`: `abccc = "1.1.0"`. In [The Cargo Book > Version requirement
  syntax](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#version-requirement-syntax)
  that is also known as [Caret
  requirements](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#caret-requirements).
- beware out-of-date 4th party crates - the same as for **#3**
- beware potentially more out-of-date maintenance caused by those 3rd party crates and their
  dependencies - but not Cami-related.

See also
- [The Cargo Book > Dependency Resolution > SemVer
compatibility](https://doc.rust-lang.org/nightly/cargo/reference/resolver.html#semver-compatibility)
&gt; "Equals" and "Compound"
- [The Cargo Book > Dependency Resolution >
  Features](https://doc.rust-lang.org/nightly/cargo/reference/resolver.html#features)
  - "The resolver will skip over versions of packages that are missing required features", which
    determines  the oldest version it can select.
  - "Similarly, if a feature is removed from a new release, then packages that require that feature
    will be stuck on the older releases that contain that feature."
  - "It is discouraged to remove features in a SemVer-compatible release." - in our case, this is
    what we need to do.
- [The Cargo Book > Specifying
  Dependencies](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html)
  - [> Wildcard
    requirements](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#wildcard-requirements)
  - [Comparison
  requirements](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#comparison-requirements)
  &gt; `= 1.2.3`.

## Trait impls for 3rd party types

The following two methods, and limitations, are due to [Rust's
trait](https://doc.rust-lang.org/book/ch10-02-traits.html) coherence ("orphan rule").

### Implemented by 3rd party (preferred)

We prefer (3rd party) crates to implement Cami traits for their types, because

- they can assure correctness more, as know their types, they can adapt the Cami traits'
  implementation if they change their type's implementation,
- they can optimize based on their knowledge of the type's behavior,
- they can optimize by using private/`pub(crate)` fields/functions,
- implementing the traits on their side doesn't count towards [crates.io](https://crates.io)'s limit
  of 300 features (more below).

### Implemented by Cami (not preferred)

This

- is initial (for a given 3rd party crate),
- is deemed temporary/unstable, especially if the (3rd party) crate is actively maintained, because
  then we hope to move the implementation to the 3rd party crate. Such a move would be a major
  change (more below),
- can stay longterm only if the (3rd party) crate is unmaintained, or its maintainers are
  overloaded,
- counts towards [crates.io](https://crates.io)'s limit of 300 features, because we make such 3rd
  part crates optional dependencies, hence each crate counts as one feature. And, if the related
  behavior depends on that crate's feature(s), then we'd need to add related features to Cami, too.

#### Short incompatibility period

If the (3rd party) crate's maintainer(s) agree to move the implementation of Cami traits to their
crate, then there will be an interim incompatibility/upgrade period (for users who depend on Cami
traits for that crate). Hopefully that would be only for a few hours (if the crate's maintainers
synchronize it with Cami maintainers).

### Potentially exceeding crates.io limit of 300 features

This may hit the [crates.io](https://crates.io)'s limit of 300 features [The Cargo Book >
Features](https://doc.rust-lang.org/nightly/cargo/reference/features.html) &gt; "New crates or
versions published on crates.io are now limited to a maximum of 300 features. See this [blog
post](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html) for details..."

Hopefully there will never be anywhere close to 300 implementations for (3rd party) crates at the
same time (including a grace period before a feature name is removed).
