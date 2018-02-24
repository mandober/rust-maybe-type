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

Default trait
=============
pub trait Default: Sized {
    fn default() -> Self;
}

Default trait requires impl 1 method, default(), that takes nothing
and returns some meaningful default value for specific type.

In case of Maybe<T> that can be `Nothing` variant.

To get default value for particulat type T, annotate variable:
    let def: T = Default::default();

or use turbofish syntax:
    let def = Default::default::<T>();

*/

// import Maybe, Maybe::Just and Maybe::Nothing
use super::Maybe::{self, Nothing, Just};


impl<T> Default for Maybe<T> {
  fn default() -> Self {
    Nothing
  }
}

impl<T: Default> Maybe<T> {
  /// Returns the contained value or default value for that type.
  pub fn unwrap_or_default(self) -> T {
    match self {
      Just(v) => v,
      Nothing => Default::default(),
    }
  }
}
