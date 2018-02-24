//import
use maybe::Maybe::{self, Nothing, Just};

//clone 1
impl<'a, T> Maybe<&'a T> where T: Clone {
  /// Maps Maybe<&T> to Maybe<T> by cloning the contents
  pub fn cloned(self) -> Maybe<T> {
    match self {
      Just(v) => Just(v.clone()),
      Nothing => Nothing
    }
  }
}

//clone 2
impl<'a, T> Maybe<&'a mut T> where T: Clone {
  /// Maps Maybe<&mut T> to Maybe<T> by cloning the contents
  pub fn cloned(self) -> Maybe<T> {
    match self {
      Just(v) => Just(v.clone()),
      Nothing => Nothing
    }
  }
}
