
## Searches (in a slice/array/Vec)
- sequential search for primitive/local-only item types (no references) - no speed benefit
- binary search - speed benefit, for both
  - primitive/local-only item types (no references), and
  - item types that have both local fields and references (much more beneficial than for local-only
    item types)

# Design

This design is (arguably) "coupled", because both declaring and implementingf these features goes
together.