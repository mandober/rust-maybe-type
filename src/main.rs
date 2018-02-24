// #![allow(unused_variables,dead_code,unused_mut)]
pub mod maybe;
use maybe::Maybe::{self, Nothing, Just};

fn main() {

  // iter
  let x: Maybe<u8> = Just(4);
  assert_eq!(x.iter().next(), Some(&4));

  // get_or_insert
  let six = "sixer";
  let mut opt: Maybe<&str> = Nothing;
  {
    let res = opt.get_or_insert(six);
    assert_eq!(res, &mut "sixer");
    *res = "niner";
  }
  println!("{:?}", opt);


  // or
  let just: maybe::Maybe<&str> = maybe::Maybe::Just("Alpha");
  showme(just);
  let null: maybe::Maybe<&str> = maybe::Maybe::Nothing;
  let opt = null.unwrap_or_default();
  showme(opt);


  let unit: () = ();
  showme(unit);

  fn showme<T: ::std::fmt::Debug>(p: T) {
    println!("showme: {:?}", p);
  }


  let _opt: Option<&str> = None;




/*
    // Option::iter
    use std::u16;
    let v = vec![1, 2];

    //  it: std::slice::Iter<'_, u16>
    let it = v.iter();

    let it = it.map(|&x|
                if x == u16::MAX { None }
                else { Some(x + 1) });

    let res = it.collect::<Option<Vec<u16>>>();


    // let res = v.iter()
    //            .map(|&x: &u16|
    //                if x == u16::MAX { None }
    //                else { Some(x + 1) })
    //            .collect::<Option<Vec<u16>>>();

    assert!(res == Some(vec![2, 3]));
*/

} // main
