Then core/alloc/std + for 3rd party:
`type StringCami = Cami<String>;`
- 3rd party can define their `type` aliases easily

trait `CamiCast` OR `IntoCami`
- a trait that adds `.into_cami()` to `core/alloc/std/custom`
- it has a `type CAMI`??

`Cami<[xyz; N]>` !== `[Cami<xyz>; N]`
`Cami<Vec<T>>`   !== `Vec<Cami<T>>`
`Cami<&[T]>`     !== `&[Cami<T>]`
- Usually we want the second one, so we can apply `Cami` ordering to the items.


feature??: casts (transmutes)

#[cfg_attr(doc_cfg, doc(cfg(any(feature = "full", feature = "derive"))))]
