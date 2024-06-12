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

## Future-proof

### Stable API

`cami` crate has only a few traits, with only a few functions. So its API surface is small. And, we
don't expect any major and backward-incompatible change to the traits themselves. Any functionality
that can be outside of `cami`, is so - for example,
[`cami-rs/cami-helpers`](https://github.com/cami-rs/cami-helpers).

### Permanently below 1.0.0

There will never be version `1.0.0` (or any higher). This allows dependant crates to refer to `cami`
with wildcard `0.*` AND such a wildcard **is** accepted by [crates.io](https://crates.io) (since
it's **only** bare `"*"` wildcard that is not accepted by [crates.io](https://crates.io)).

However, if we went to `1.0.0` (and over), dependencies couldn't wildcard-match Cami anymore -
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
  then we hope to move the implementation to the 3rd party crate. Such a move would be a major
  change (more below),
- will stay longterm only if the (3rd party) crate is unmaintained, or its maintainers are
  overloaded,
- counts towards [crates.io](https://crates.io)'s limit of 300 features (see below). Why? We make
  such 3rd part crates optional dependencies, hence each crate counts as one feature. And,
- if migrated out (to the 3rd party crate) later, this temporarily needs two features (defined in
  `cami`), in addition to any extra feature(s) mentioned next, hence limiting the number of crates
  being migrated at the same time to be under 150,
- if the related behavior varies depending on that crate's feature(s), then we'd need to add related
  features to `cami`, too - and those extra `cami` features would also count against the limit of 300,
- increases `cami`'s **minor** version, but if migrated to the 3rd party later, it increases
  `cami`'s **major** version.

### Implemented by 3rd party (preferred)

We prefer (3rd party) crates to implement `cami` traits (for their types, indeed), because

- they can assure correctness more, as know their types, they can adapt the Cami traits'
  implementation if they change their type's implementation,
- they can optimize based on their knowledge of the type's behavior and its consistency,
- they can optimize by using private/`pub(crate)` fields/functions,
- implementing the traits on their side doesn't need any new features added to `cami` (so it doesn't
  count towards [crates.io](https://crates.io)'s limit of 300 features (more below)).

### Smooth upgrades: Zero incompatibility period

<!--This assumes that
- all Cami dependents specify `cami` version as wildcard `0.*`, and
- `cami` implementations (whether as part of `cami` - "NOT preferred", or in the 3rd party crate itself - "preferred")-->

If the (3rd party) crate's maintainer(s) agree to absorb the implementation of Cami traits (for
their types) into their crate, then the previous implementation has to be moved out of `cami`
(because of coherence/"orphan" rule). You could expect an interim incompatibility/upgrade period
(for users who depend on `cami` traits for that crate) until both the 3rd party crate and `cami` are
updated.

Fortunately, that is not necessary. How come?

When `cami` adds an implementation of its traits for a (3rd party) crate (`abcc`), it defines only
one `adapt-***` feature related to this crate (`adapt-abcc`). (Unless implementation of `cami`
traits varies depending on feature(s) defined by that crate.)

But, once the (3rd party) crate (`abcc`) adapts the implementation of `cami` traits, it
- publishes a new **minor** version, which
- adds `cami` (with flexible version `0.*`) as an **optional** dependency, under an
  (automatically-generated) feature named `cami`, and
- it requires the "migrated" feature (`migrated-abcc`) of `cami`. That makes `cargo` and `cargo
  update` **NOT** fetch this new (**minor**) version of the crate (`abcc`), until `cami` publishes a
  new version with `migrated-abcc`, and
- it does not, can can not, require `adopt-abcc` feature (because that would make the dependency
  graph cyclic).

In parallel (even before, or soon after), `cami` publishes a new (**minor**) `0.x.y` that
- contains a new feature `migrated-abcc` (with no extra functionality at all), and
- contains the old feature `adapt-abcc`, which still implements `cami` traits for that crate, and
- this starts the first migration period.

After the first migration period is over,
1. `cami` publishes a new **major** version `0.w.0` that
  - removes feature `adapt-abcc`, and
  - it deprecates feature `migrated-abcc`, and
  - starts the second migration period.
2. Only then the (3rd party) crate (`abcc`) publishes a new **patch** version, which
  - depends on `cami` with no features specified, so removing `migrated-abcc` feature for this
    dependency.

After the second migration period is over, `cami` publishes a new **major** version `0.z.0` that
removes feature `migrated-abcc`.

## Depending on Cami

Rule #1: Always use a flexible (major) version `"0.*"` of `cami`.

Rule #2: Follow the rule #1.

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
- under `cami`'s feature `adapt-abcc` (regardless of whether together with `migrated-abcc`, that is,
  already in the first migration, or not), and
- and (the other dependency path is) **past** the second migration period (for that same crate
 `abcc`).
 
 That newer dependency path would require the updated version of that crate (`abcc`) which depends
 on `cami` with no features (it doesn't depend on `migrated-abcc` feature of `cami` anymore). But,
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
