/*
derivable traits
- `Clone` to create `T` from `&T` via a copy.
- `Copy` to give a type copy instead of move semantics
- `Debug` to format a value using the `{:?}` formatter.
- `Default` to create an empty instance of a data type.
- `Hash` to compute a hash from `&T`.
- `Eq` equality comparison
- `PartialEq` partial comparison
- `Ord` ordering
- `PartialOrd` partial ordering

Maybe enum auto-implements all derivable traits, except Default. It will be
impl manually to return `Nothing`. It could be auto impl, if all values in
Maybe enum impl Default, which means if Maybe is used with integers, default
would be `Just(0)`.
*/
