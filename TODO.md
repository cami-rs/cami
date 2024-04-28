cami_wrap_struct -> 
cami_impl
- we do NOT need a wrapper
- we ONLY need impl CamiPartialEq + CamiOrd

core_wrap_tuple ->
cami_tuple
cami_struct
-- may be NOT needed at all - see `Cami`

`Cami`
- a universal (generic) wrapper (tuple)?
- `impl Copy for Cami<T> where T: Copy {};`

`type StringCami = Cami<String>;`

trait `CamiCast`
- a trait that adds `.to_cami()` or `.cami()` to `core/alloc/std`

`.from_cami()` ??


feature??: casts

#[cfg_attr(doc_cfg, doc(cfg(any(feature = "full", feature = "derive"))))]
