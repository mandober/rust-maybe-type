use maybe::Maybe::{self, Nothing, Just};

impl<'a, T> Maybe<&'a T> where T: Clone {
  /// Maps Maybe<&T> to Maybe<T> by cloning the contents
  pub fn cloned(self) -> Maybe<T> {
    match self {
      Just(v) => Just(v.clone()),
      Nothing => Nothing,
    }
  }
}

impl<'a, T> Maybe<&'a mut T> where T: Clone {
  /// Maps Maybe<&mut T> to Maybe<T> by cloning the contents
  pub fn cloned(self) -> Maybe<T> {
    match self {
      Just(v) => Just(v.clone()),
      Nothing => Nothing,
    }
  }
}
