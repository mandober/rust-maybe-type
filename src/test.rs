#[cfg(test)]
use maybe::Maybe::{self, Nothing, Just};

#[test]
fn inner() {
  // map inner values
  let opt_a: Maybe<String> = Just(String::from("Alpha"));
  let opt_b: Maybe<String> = Nothing;
  let closure = |s: String| s.len();
  // in case the Maybe is Just
  let mapped: Maybe<usize> = opt_a.map(&closure);
  let expected: Maybe<usize> = Maybe::Just::<usize>(5_usize);
  assert_eq!(mapped, expected);
  // in case the Maybe is Nothing
  let mapped: Maybe<usize> = opt_b.map(&closure);
  assert_eq!(mapped, Maybe::Nothing::<usize>);
}

#[test]
fn iter() {
  // maybeiter
  let x = Just(4);
  assert_eq!(x.iter().next(), Some(&4));
}

#[test]
fn and() {
  // and
  let opta: Maybe<u8> = Just(2);
  let optb: Maybe<&str> = Nothing::<&str>;
  // opta is Just, so return optb
  let opt: Maybe<&str> = opta.and(optb);
  assert_eq!(opt, Nothing::<&str>);

  let opta: Maybe<u8> = Nothing::<u8>;
  let optb: Maybe<&str> = Just("foo");
  // opta is Nothing, so return Nothing that has the type of optb
  let opt: Maybe<&str> = opta.and(optb);
  assert_eq!(opt, Nothing::<&str>);
}

#[test]
fn get_or_insert() {
  // get_or_insert
  let mut my = Nothing;
  {
    let yours = my.get_or_insert(5);
    assert_eq!(yours, &mut 5);
    *yours = 33;
  }
  assert_eq!(my, Just(33));
}

#[test]
fn taken() {
  // take
  let mut x = Just(2);
  x.take();
  assert_eq!(x, Nothing);

  let mut x: Maybe<u32> = Nothing;
  x.take();
  assert_eq!(x, Nothing);
}

#[test]
fn and_then() {
  // and_then
  fn ff(x: u8) -> Maybe<u8> { Just(x * x) }
  let num: Maybe<u8> = Just(2_u8);
  let res: Maybe<u8> = num.and_then(ff).and_then(ff);
  assert_eq!(res, Just(16));
  //
  fn lee2(s: &str) -> Maybe<usize> { Just(s.len()) }
  let opz2: Maybe<&str> = Just("foo");
  let rez2: Maybe<usize> = opz2.and_then(lee2);
  assert_eq!(rez2, Just(3));
}

#[test]
fn maps() {
  // map
  let more: Maybe<String> = Just(String::from("Tango"));
  let len = more.as_ref().map(|v| v.len());
  assert_eq!(len, Just(5_usize));
  assert_eq!(more, Just(String::from("Tango")));
}

#[test]
fn unwrap_or_else() {
  // unwrap_or_else
  let more: Maybe<String> = Just(String::from("Tango"));
  let null: Maybe<String> = Nothing;
  assert_eq!(more.unwrap_or_else(|| String::from("Zebra")), String::from("Tango"));
  assert_eq!(null.unwrap_or_else(|| String::from("Zebra")), String::from("Zebra"));

}

#[test]
fn unwrap_or() {
  // unwrap_or
  let more: Maybe<String> = Just(String::from("Tango"));
  let null: Maybe<String> = Nothing;
  assert_eq!(more.unwrap_or(String::from("Zebra")), String::from("Tango"));
  assert_eq!(null.unwrap_or(String::from("Zebra")), String::from("Zebra"));
}

#[test]
fn as_mut() {
  // as_mut
  let mut more: Maybe<String> = Just(String::from("Tango"));
  let mut null: Maybe<String> = Nothing;
  assert_eq!(more.as_mut(), Just(&mut String::from("Tango")));
  assert_eq!(null.as_mut(), Nothing);
}

#[test]
fn as_ref() {
  let just: Maybe<&str> = Just("Tango");
  let null: Maybe<&str> = Nothing;
  // as_ref
  //
  let result = just.as_ref();
  assert_eq!(result, Just(&"Tango"));
  assert_eq!(null.as_ref(), Maybe::Nothing::<&&str>);
}

#[test]
#[ignore]
#[should_panic(expected = "failure")]
fn unwrap_fails() {
  let nada: Maybe<&str> = Nothing;
  let r = nada.expect("failure");
  assert_eq!(r, "the thing");
}

#[test]
fn is_just_or_nothing() {
  let just: Maybe<String> = Just(String::from("Tango"));
  let nada: Maybe<String> = Nothing;
  // is_just
  //
  assert_eq!(just.is_just(), true);
  assert_eq!(nada.is_just(), false);
  // is_nothing
  //
  assert_eq!(just.is_nothing(), false);
  assert_eq!(nada.is_nothing(), true);
}
