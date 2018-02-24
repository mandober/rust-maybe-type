#[cfg(test)]
use super::Maybe::{self, Nothing, Just};

#[test]
fn fuzzy() {
  // &mut &&mut T
  let ref mut juju: Maybe<&str> = Just("Alpha");
  let ref mut ini = &juju;
  assert_eq!(ini, &mut &&mut Maybe::Just::<&'static str>("Alpha"));
  // match ***ini {
  //     Just(v) => println!("{:?}", v),
  //     Nothing => println!("Nothing"),
  // }

  let ref mut nick: u8 = 122;
  let bob = &nick;
  assert_eq!(bob, &nick);

  let ref mut nick2: &u8 = &42;
  let bob2: &&mut &u8 = &nick2;
  assert_eq!(bob2, &nick2);

  let ref mut ant: &'static str = "42";
  let ref mut iny: &&mut &str = &ant;
  assert_eq!(&iny, &&mut &&mut "42");
  // let mut num = 323;
  // let mref = &&mut &&mut &&num;
}

#[test]
fn iter() {
  let x = Just(4);
  assert_eq!(x.iter().next(), Some(&4));
}

#[test]
fn cloned() {
  // cloned
  let x = 12;
  let opt_x = Just(&x);
  assert_eq!(opt_x, Just(&12));
  let cloned = opt_x.cloned();
  assert_eq!(cloned, Just(12));

  // cloned (mut)
  let mut x = 12;
  let opt_x = Just(&mut x);
  assert_eq!(opt_x, Just(&mut 12));
  let cloned = opt_x.cloned();
  assert_eq!(cloned, Just(12));
}

#[test]
fn unwrap_or_default() {
  // unwrap_or_default
  let nil: Maybe<u8> = Nothing;
  let def: u8 = Default::default();
  assert_eq!(nil.unwrap_or_default(), def);

  let opt: Maybe<u8> = Just(6);
  assert_eq!(opt.unwrap_or_default(), 6_u8);
}

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
