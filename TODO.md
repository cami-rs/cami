Then core/alloc/std + for 3rd party:
`type StringCami = Cami<String>;`
- 3rd party can define their `type` aliases easily

`Cami<[xyz; N]>` !== `[Cami<xyz>; N]`
`Cami<Vec<T>>`   !== `Vec<Cami<T>>`
`Cami<&[T]>`     !== `&[Cami<T>]`
- Usually we want the second one, so we can apply `Cami` ordering to the items.


#[cfg_attr(doc_cfg, doc(cfg(any(feature = "full", feature = "derive"))))]
