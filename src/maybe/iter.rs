use super::Maybe::{self, Nothing, Just};


impl<T> Maybe<T> {
  /// Returns an iterator over the possibly contained value.
  pub fn iter(&self) -> Iter<T> {
    Iter {
      inner: self.as_ref()
    }
  }
} // impl


pub struct Iter<'a, T: 'a> {
  inner: Maybe<&'a T>
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.inner.as_ref() {
      Nothing => None,
      Just(v) => Some(v),
    }
  }
}


/*
impl<T> Maybe<T> {
  /// Returns an iterator over the possibly contained value.
  pub fn iter(&self) -> Iter<T> {
    Iter {
      inner: Item {
        opt: {
          //self.as_ref()
          match *self {
            Just(ref v) => Some(v),
            Nothing => None,
          }
        }
      }
    }
  }
} // impl

#[derive(Clone, Debug)]
struct Item<A> {
    opt: Option<A>
}

impl<A> Iterator for Item<A> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        self.opt.take()
    }
}

pub struct Iter<'a, A: 'a> {
  inner: Item<&'a A>
}

impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;

    fn next(&mut self) -> Option<&'a A> {
      self.inner.next()
    }
}
*/




/*
impl<T> IntoIterator for Option<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Returns a consuming iterator over the possibly contained value.
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { inner: Item { opt: self } }
    }
}

impl<'a, T> IntoIterator for &'a Option<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Option<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}
*/
