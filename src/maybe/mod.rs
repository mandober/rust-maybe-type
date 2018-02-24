//! Maybe values: maybe just the value, maybe nothing.

/// The `Maybe` type.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Maybe<T> {
  Just(T),
  Nothing,
}


pub use self::Maybe::{Just, Nothing};
pub mod iter;
pub mod default;
pub mod cloned;

pub mod test;


impl<T> Maybe<T> {

  /// Returns `true` if the option is a [`Just`] value.
  pub fn is_just(&self) -> bool {
    match *self {
      Just(_) => true,
      Nothing => false,
    }
  }

  /// Returns `true` if the option is a [`Nothing`] value.
  pub fn is_nothing(&self) -> bool {
    !self.is_just()
  }

  /// Converts from `Maybe<T>` to `Maybe<&T>`.
  pub fn as_ref(&self) -> Maybe<&T> {
    match *self {
      Just(ref v) => Just(v), // Maybe::Just<&T>(v)
      Nothing => Nothing,     // Maybe::Nothing::<&T>
    }
  }

  /// Converts from `Maybe<T>` to `Maybe<&mut T>`.
  pub fn as_mut(&mut self) -> Maybe<&mut T> {
    match *self {
      Just(ref mut v) => Just(v), // Maybe::Just<&mut T>(v)
      Nothing => Nothing,         // Maybe::Nothing::<&mut T>
    }
  }

  /// Unwraps an option, yielding the content of a [`Just`].
  pub fn expect(self, msg: &str) -> T {
    match self {
      Just(v) => v,                 // T
      Nothing => panic!("{}", msg), // !
    }
  }

  /// Moves the value `v` out of the `Maybe<T>` if it is [`Just(v)`].
  pub fn unwrap(self) -> T {
    match self {
      Just(v) => v,
      Nothing => panic!("called `Maybe::unwrap()` on a `Nothing` value"),
    }
  }

  /// Returns the contained value or a param.
  pub fn unwrap_or(self, param: T) -> T {
    match self {
      Just(v) => v,
      Nothing => param,
    }
  }

  /// Returns the contained value or computes it from a closure.
  pub fn unwrap_or_else<F>(self, fx: F) -> T
   where F: FnOnce() -> T {
    match self {
      Just(v) => v,
      Nothing => fx(),
    }
  }

  /// Maps an `Maybe<T>` to `Maybe<U>` by applying a fn to a contained value.
  pub fn map<F, U>(self, fx: F) -> Maybe<U>
   where F: FnOnce(T) -> U {
    match self {
      Just(v) => Just(fx(v)), // Maybe::Just<U>(v)
      Nothing => Nothing,     // Maybe::Nothing::<U>
    }
  }

  /// Applies a function to the contained value (if any),
  /// or returns the provided default (if not).
  pub fn map_or<F, U>(self, alt: U, fx: F) -> U
   where F: FnOnce(T) -> U {
    match self {
      Just(v) => fx(v),
      Nothing => alt,
    }
  }

  /// Applies a function to the contained value (if any),
  /// or computes a default (if not).
  pub fn map_or_else<F, U, A>(self, gx: A, fx: F) -> U
   where F: FnOnce(T) -> U,
         A: FnOnce()  -> U {
    match self {
      Just(v) => fx(v),
      Nothing => gx(),
    }
  }

  /// Transforms the `Maybe<T>` into a [`Result<T, E>`], mapping
  /// [`Just(v)`] to [`Ok(v)`] and [`Nothing`] to [`Err(err)`].
  pub fn ok_or<E>(self, err: E) -> Result<T, E> {
    match self {
      Just(v) => Ok(v),
      Nothing => Err(err),
    }
  }

  /// Transforms the `Maybe<T>` into a `Result<T, E>`, mapping
  /// `Just(v)` to `Ok(v)` and Nothing to `Err(fx())`.
  pub fn ok_or_else<E, F>(self, fx: F) -> Result<T, E>
   where F: FnOnce() -> E {
    match self {
      Just(v) => Ok(v),
      Nothing => Err(fx()),
    }
  }

  /// Returns Nothing if the option is Nothing, otherwise returns optb.
  pub fn and<U>(self, optb: Maybe<U>) -> Maybe<U> {
    match self {
      Just(_) => optb,
      Nothing => Nothing,
    }
  }

  /// Returns `Nothing` if the option is `Nothing`, otherwise
  /// calls `fx` with the wrapped value and returns the result.
  pub fn and_then<F, U>(self, fx: F) -> Maybe<U>
   where F: FnOnce(T) -> Maybe<U> {
    match self {
      Just(v) => fx(v),   // Maybe::Just<U>(v)
      Nothing => Nothing, // Maybe::Nothing::<U>
    }
  }

  /// Returns Nothing if the option is Nothing, otherwise
  /// calls predicate with the contained value
  pub fn filter<P>(self, predicate: P) -> Self
   where P: FnOnce(&T) -> bool {
    match self {
      Nothing => Nothing,
      Just(v) => {
        if predicate(&v) { Just(v) } else { Nothing }
      }
    }
  }

  pub fn filter_std<P>(self, predicate: P) -> Self
   where P: FnOnce(&T) -> bool {
    if let Just(x) = self {
      if predicate(&x) {
        return Just(x)
      }
    }
    Nothing
  }

  /// Returns the option if it contains a value, otherwise returns optb.
  pub fn or(self, optb: Self) -> Self {
    match self {
      Just(v) => Just(v),
      Nothing => optb,
    }
  }

  /// Returns the option if it contains a value,
  /// otherwise calls fx and returns the result.
  pub fn or_else<F, U>(self, fx: F) -> Self
   where F: FnOnce() -> Self {
    match self {
      Just(v) => Just(v),
      Nothing => fx(),
    }
  }

  /// Returns a mutable reference to the contained value.
  /// If Nothing, it first inserts the parameter val into the Maybe.
  pub fn get_or_insert(&mut self, val: T) -> &mut T {
    // this fn may mutate the Maybe in place, so it takes mut ref to self
    // and a value (to insert) of the same type as the inner value.

    // We need to check wich variant the Maybe holds by matching `self`.
    // `&mut self` as parameter in a method is a shorthand for:
    // `self: &mut Maybe<T>`

    // It's either doing a match directly on `self` and then having
    // `&mut Just...` and `&mut Nothing...` patterns, or doing a match on
    // dereferenced `self` and having a pleasent looking patterns.

    // `*self` is a fn call `Deref::deref(self)` which returns a temporary of
    // type `Maybe<T>` on which we match.
    match *self {
      // is the value is present, we take a mut ref to it and return it
      Just(ref mut v) => v,
      // if not, we mutate Maybe in place
      Nothing => {
        // by using mem::replace
        //::std::mem::replace(self, Just(val));

        // by derefing self and assigning it a new value
        *self = Just(val);

        // now that Maybe definitly has a value, we can match `self` all over
        // again or just use as_mut to get mut ref and than unwrap it:
        self.as_mut().expect("get_or_insert unwrapped Nothing")
        // it can't fail, but nevertheless there's a tracking message
      }
    }
  }

  // get_or_insert method for Option as is in the std:
  pub fn get_or_insert_std(&mut self, v: T) -> &mut T {
    match *self {
        Nothing => *self = Just(v),
        _ => (),
    }

    match *self {
        Just(ref mut v) => v,
        _ => unreachable!(),
    }
  }

  /// Inserts a value computed from fx into the option if it is Nothing,
  /// then returns a mutable reference to the contained value.
  pub fn get_or_insert_with<F>(&mut self, fx: F) -> &mut T
   where F: FnOnce() -> T {
    match *self {
      Just(ref mut v) => v,
      Nothing => {
        *self = Just(fx());
        self.as_mut().expect("get_or_insert_with unwrapped Nothing")
      }
    }
  }

  /// Takes the value out of the Maybe, leaving a Nothing in its place.
  pub fn take(&mut self) -> Self {
    ::std::mem::replace(self, Nothing)
  }


} // impl
