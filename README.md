# Cami = Cache Friendly

Zero cost wrappers & related implementation of cache-friendly comparison & ordering.
`no_std`-friendly.

## Use cases & Architectures

- "yes" to common-and-bigger data thoughputs, where most parts of data are handled/visited multiple
  times.
- "no" to datasets so tiny that most of the data (or, the hot part of it) fits into CPU/RAM
  cache(s).
- "no" to single-run processing, especially if the data is not (re)sorted and not looked up based on
  its comparison/order.
- "yes" to architectures/executions with (transparent) CPU/RAM cache(s).
- "no" to architectures/executions with no cache.

## Consumer types

### Compounds, or variable-length slices/&str/String/Vec...

`cami` is primarily for comparing, equality, ordering & binary search of
- custom compound types, that is, other than primitives or slices/arrays/`&str`/`String`/`Vec`. Or
- slices/`&str`/`String`/`Vec` themselves (where items are of any type) - however, **not** of
  fixed/pre-determined/batch size, but of **variable** length. This is where the slices/`Vec`...
  themselves are being compared/ordered/searched for.

Then, instead of storing/using the consumer ("classic") item type, transmute it into `Cami` (zero
  cost) wrapper around that "classic" item type.

If the consumer ("classic") item type itself is a slice/`&str`/`String`/`Vec`, it can contain either
"classic" deeper items (not `Cami` zero cost wrappers), or `Cami` (zero cost) wrappers around
"classic" deeper items. It brings benefit either way.

Comparison and ordering of `Cami` (wrapper) over
- a compound type **may** DIFFER to the `#[derive(...)]`'s default order (for that compound type):
[the top-to-bottom declaration order of the struct’s
members](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#derivable) (for primitives or
custom types).
- a slice/`&str`/`String`/`Vec` **does** differ to their [`Ord` > Lexicographical
comparison](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#lexicographical-comparison).

#### slice/&str/String/Vec as items

If the items being compared/ordered/searched for (themselves) are slices/`&str`/`String`/`Vec`),
`cami` is beneficial only if they are **not** of the same size.

Hence **not** suitable/beneficial for items, or their fields, that (themselves) are
- arrays or
- slices/`&str`/`String`/`Vec` of fixed size, or of

like `sha256/other` hashes, UUID's, fixed-length usernames, condensed dates/timestamp, matrices...

### Items in HashSet, keys in HashMap

This comparison doesn't give as much benefit for keys/items in `HashMap` & `HashSet` (because those
use `Hash` for determining the buckets). But it can speed up comparison of keys in the same bucket
(with the same hash). And, since `HashMap` & `HashSet` don't keep/don't guarantee any order, using
`cami` makes transition/backwards compatibility easier.

### Items in BTreeSet, keys in BTreeMap

Transmuting those collections themselves would be AGAINST their correctness, because they maintain
the ordered state, and they depend on it.

Indeed, the actual items stored in a `BTreeMap/BTreeSet` can use this `cami`. So, instead of storing
values `V` in `BTreeSet`, or instead of mapping keys `K` in `BTreeMap`, you store values `Cami<V>`
in `BTreeSet`, or `Cami<K>` as keys in `BTreeMap`>. Here `cami` brings benefit, and is easy to use.

## Actual traits & applicability

TODO rename CPartial etc.

### CPartialEq & COrd

TODO

You could transmute (zero cost) a wrapper back to the original Vec/slice/array/String/&str, as far
as no existing code depends on `derive`'s default ordering and Vec/slice/array/String/&str's
lexicographic ordering.

## Scope

### No derive macro(s) initially
No `#derive(...)` macro(s) for `CamiPartialEq/CamiOrd`, because no macro can
access/differentiate/interpret/"know" types being used.

## Future-proof

### Stable API

`cami` crate has only a few traits, with only a few functions. So its API surface is small. And, we
don't expect any major and backward-incompatible change to the traits themselves. Any functionality
that can be outside of `cami`, is so - for example,
[`cami-rs/cami-helpers`](https://github.com/cami-rs/cami-helpers).

### Permanently below 1.0.0

There will never be version `1.0.0` (or any higher). This allows dependant crates to refer to `cami`
with wildcard `0.*` AND such a wildcard **is** accepted by [crates.io](https://crates.io) (since
it's **only** bare `"*"` wildcard that is not accepted by [crates.io](https://crates.io) - see
next).

However, if we went to `1.0.0` (and over), dependencies couldn't wildcard-match `cami` anymore -
because as per [The Cargo Book > Specifying Dependencies > Wildcard
requirements](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html):
"crates.io does not allow bare `*` versions."

(And, if there ever is a need for a backwards-incompatible change to the traits, someone could start
a new crate instead.)

## Trait impls for 3rd party types

The following two methods, and limitations, are due to [Rust's
trait](https://doc.rust-lang.org/book/ch10-02-traits.html) coherence ("orphan rule").

### Implemented by Cami (NOT preferred)

This

- is initial (for a given 3rd party crate),
- is deemed temporary/unstable, especially if the (3rd party) crate is actively maintained, because
  then we hope to move the implementation to the 3rd party crate. Such a move would be a **major**
  change (more below),
- will stay longterm only if the (3rd party) crate is unmaintained, or its maintainers are
  overloaded, but:
- counts towards [crates.io](https://crates.io)'s limit of 300 features (see below). Why? We make
  such (3rd party) crates optional dependencies in `cami`, hence each crate counts as one feature.
- adds a new feature `adopt-***` for the given crate, like `adopt-abcc` for crate `abcc`.
- if migrated out (to the involved 3rd party crate, let's say, `abcc`) later, this temporarily needs
  two features each (defined in `cami`):
  - `adapt-abcc` (during the first migration period only) and
  - `migrate-abcc` (during both the first and the second migration period)
  
  (in addition to any extra features mentioned below). That limits the number of crates being
  supported in `cami` itself, or being migrated out, at any time, to be under 150.
  
  And, if a crate doesn't get migrated completely (for example, if it becomes unmaintained), it
  keeps depending on
  - both `adapt-abcc` and `migrate-abcc` features of `cami` (if it becomes unmaintained during the
    first migration period), or
  - "just" `migrate-abcc` feature (if it becomes unmaintained during the second migration period).
  Such "occupied" features count against the limit of 300 features - possibly longterm.
- if the related behavior (ordering and equality) varies depending on that crate's feature(s), then
  we'd need to add related features to `cami`, too, with names like `a And those extra `cami`
  features would also count against the limit of 300,
- increases `cami`'s **minor** version, but if later migrated out (to that crate itself), it then
  increases `cami`'s **major** version.

### Implemented by 3rd party (preferred)

We prefer (3rd party) crates to implement `cami` traits (for their types, indeed), because

- they can assure correctness more, as they know their types
- they can update `cami` traits' implementation if they change their type's implementation,
- they can optimize based on their knowledge of the type's behavior and its consistency,
- they can optimize by using private/`pub(crate)` fields/functions,
- implementing the traits on their side doesn't need any new features added to `cami` (so it doesn't
  count towards [crates.io](https://crates.io)'s limit of 300 features (more below)).

### Smooth upgrades: Zero incompatibility period

If the (3rd party) crate's maintainer(s) agree to absorb the implementation of Cami traits (for
their types) into their crate, then the previous implementation has to be moved out of `cami`
(because of coherence/"orphan" rule). You could expect an interim incompatibility/freeze period (for
users who depend on `cami` traits being implemented for that crate) until both the 3rd party crate
and `cami` are updated.

Fortunately, that is not necessary. How come?

This assumes that all `cami`-enabled crates (which implement `cami` traits), and all their
consumers, follow simple rules (see later). Then the following migration is possible.

### 3rd party crates: Migrations

This is not a one-off "global" migration of `cami` itself. Instead, it's a one-on-one migration of
`cami` support out of `cami` to the 3rd party crate (which defines the type). There may be many such
migration in progress at the same time, in various stages.

If `cami` adds an implementation of its traits for a (3rd party) crate (`abcc`), it defines only one
feature related to this crate: `adapt-***` (`adapt-abcc`). (Unless implementation of `cami` traits
varies depending on feature(s) defined by that crate.) Any consumer depends on `cami` with feature
`adapt-abcc`.

When the (3rd party) crate (`abcc`) adapts the implementation of `cami` traits, that (3rd party)
crate
- publishes a new **minor** version, which
- adds `cami` (with flexible version `0.*`) as an **optional** dependency, under an
  (automatically-generated) feature named `cami`. This change (of adding) of dependency feature is a
  minor change (as per [The Cargo Book > SemVer Compatibility > Minor: adding dependencies
  features](https://doc.rust-lang.org/nightly/cargo/reference/semver.html#cargo-dep-add)). And
- requires a new "migrated" feature (`migrate-abcc`) of `cami`. That makes `cargo update` **NOT**
  fetch this new (**minor**) version of the crate (`abcc`), until `cami` publishes a new version
  with a new `migrate-abcc` feature (below). And
- it does not, and cannot, require `adopt-abcc` feature (because that would make the dependency
  graph cyclic).

Soon after, `cami` publishes a new (**minor**) `0.x.y` that
- adds a new feature `migrate-abcc` (with no extra functionality at all - it's a "marking feature"
  only), and
- deprecates the existing feature `adapt-abcc`, which still implements `cami` traits for that crate,
  and
- consumers remove feature `adapt-abcc` from their dependency on `cami`, and they add `cami` feature
  for their dependency on the (3rd party) crate (`abcc`). And
- this starts the **first migration period**.

Consumers don't need to use feature `migrate-abcc` (of `cami`). If they do, that fails to build (as
early detection). Yes, that makes features `adapt-abcc` and `migrate-abcc` mutually exclusive. As
per [The Cargo Book > Features > Mutually exclusive
features](https://doc.rust-lang.org/nightly/cargo/reference/features.html#mutually-exclusive-features),
that is "rare" - and it's needed in this case.

After the first migration period is over,
1. `cami` publishes a new **major** version `0.w.0` that
  - removes feature `adapt-abcc`, and
  - deprecates feature `migrate-abcc`.
2. Only then, the (3rd party) crate (`abcc`) publishes a new **patch** version, which
  - depends on `cami`, but with no features specified, so removing `migrate-abcc` feature for this
    dependency. This change (removal) of dependency feature is a **minor** change (as per [The Cargo
    Book > SemVer Compatibility > Minor: changing dependency
    features](https://doc.rust-lang.org/nightly/cargo/reference/semver.html#cargo-change-dep-feature)).
    And
  - consumers don't change `Cargo.toml`, but they run `cargo update`.
  - starts the **second migration period**.

After the **second migration period** is **over**,
- `cami` publishes a new **major** version `0.z.0` that removes feature `migrate-abcc`.
- consumers don't change `Cargo.toml`, but they run `cargo update`.

## Depending on Cami

Rule #1: Always use a flexible (major) version `"0.*"` of `cami`.

Rule #2: If you use any `adapt-***` feature of `cami`,
- subscribe to the respective tracking issue (with `adapt-` in the title). All such issue are
  **quiet** (collaborators-only). And/or
- check the tracking issue's status regularly.

## Depending on Cami and 3rd party crates

Out of the following four methods, use
- the first two (`#1.` and `#2.`) if, and only if, implementation of Cami traits for a 3rd party
crate is a part of Cami (rather than in the 3rd party crate itself) - the "NOT preferred" way. This
is only for
 - initial/temporary/unstable support, or
 - unmaintained crates (or with overloaded maintainers).

### 1. Controlled, with in-Cami 3rd party support

This applies especially if the implementation of Cami traits for a 3rd party crate is not likely to
move out of `cami` and into that crate anytime soon.

Use
- an applicable `cami` feature like `adapt-***` enabled for the (3rd party) crate (that you depend
  on). (A hypothetical example: Cami's feature `adapt-abccc` for a 3rd party crate `abccc`.) Check
  if that feature doesn't raise a `deprecated` warning (otherwise apply one of the rules `#3` or
  `#4`). And
- more controlled: a fixed **minor** version of the (3rd party) crate. For example, `"1.0.*"` of
  [`smartstring`](https://crates.io/crates/smartstring): `smartstring = "1.0.*"`, or
- less controlled: a fixed **major** version of the (3rd party) crate. For example, `"1.*"` of
  [`smartstring`](https://crates.io/crates/smartstring): `smartstring = "1.*"`.
  - Surprisingly, this will continue working (temporarily) even once/if the (3rd party) crate moves
    the implementation of Cami traits into itself (because the 3rd party crate can inactivate)

Once/if implementation of Cami traits (for such a 3rd party crate) is moved out of Cami and into the
crate itself, that crate should have a minor version increase. Soon after (or, hopefully, at near
the same time), Cami will have a new (major) version for it. Then you can migrate to one of the last
two of the following three methods (**#3** or **#4**).

### 2. Conservative, with in-Cami 3rd party support
- accepting that there may be a need for a small manual change, but not earlier than in six months,
- if implementation of Cami traits for a 3rd party crate is (**still**) a part of Cami (rather than
the 3rd party crate itself), and especially if there is an indication/conversations about moving it
(the implementation of Cami traits for the crate) out of Cami and to the (3rd party) crate itself,
- but where you are available to manually update `Cargo.toml` - if need be and no earlier than in
  six months,

use
- a flexible (major) version `"0.*"` of `cami`, with `adapt-abccc` feature(s) enabled for any (3rd
  party) crates like `abc` for which Cami has implemented its traits, and
- a fixed **major** version of the (3rd party) crate. For example, `"1.*"` of `abccc`: `abccc =
  "1.0.*"`.

**3.** For
- fairly conservative development, but/and
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
- fluid/adaptable development, but/and
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

### What about the SemVer trick?

[David Tolnay](https://github.com/dtolnay)'s [SemVer
Trick](https://github.com/dtolnay/semver-trick/blob/master/README.md) is good. But, no real reason
for it here, if everyone follows simple rules (above).

SemVer trick can't help here, in general, (for example) if
- multiple dependencies are left old, and even worse if they are at various out-of-date "age",
  and/or
- even just one dependency is at a fixed version.

Specifically, if there are two (or more) dependency paths on `cami` support for the same crate
(`abcc`), both
- under `cami`'s feature `adapt-abcc` (regardless of whether together with `migrate-abcc`, that is,
  already in the first migration, or not), and
- and (the other dependency path is) **past** the second migration period (for that same crate
 `abcc`).
 
 That newer dependency path would require the updated version of that crate (`abcc`) which depends
 on `cami` with no features (it doesn't depend on `migrate-abcc` feature of `cami` anymore). But,
 this updated version is only a **patch** (unless there was an unrelated minor release during the
 migration - but even then, the updated version would have **major** the same). So Cargo resolver
 can't load both the pre-patch and post-patch versions (or two **minor** versions under the same
 **major** version) at the same time, but it has to load the newer - and it can't load them as two
 separate major versions, because their **major** version is the same. it the same crate is depended
 on with can't help if any (4th party) crate depends on an obsolete `adopt-

See also
- [The Cargo Book > The Manifest Format > The version
field](https://doc.rust-lang.org/nightly/cargo/reference/manifest.html#the-version-field)
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

### Potentially exceeding crates.io limit of 300 features

Implementing Cami traits for 3rdf party crates in `cami` itself (the "NOT preferred" way) may hit
the [crates.io](https://crates.io)'s limit of 300 features [The Cargo Book >
Features](https://doc.rust-lang.org/nightly/cargo/reference/features.html) &gt; "New crates or
versions published on crates.io are now limited to a maximum of 300 features. See this [blog
post](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html) for details..."

Hopefully there will never be anywhere close to 300 implementations for (3rd party) crates at the
same time (including a grace period before a relevant `adapt-***` feature name is removed).

## MSRV

Currently, the minimum supported Rust version is `1.62.1`.

## Benchmarking

Benchmarks are separate, in [cami-rs/cami-benches](https://github.com/cami-rs/cami-benches), because
- they are many and complex, with their own scaffolding,
- they require `nightly` Rust version (regardless of whether using `Criterion` or the standard
  harness) - even if the benchmarked feature(s) of `cami` don't need `nightly`.
- Having benchmarks separate allows us to switch `nightly` "on" for them (with
  `rust-toolchain.toml`). However, that would not be possible if benches were part of `cami` itself.
- they have their own dependencies, which could cause assumptions/confusion when maintaining `cami`
  itself (since dev dependencies are not separated between benchmarks and tests).
