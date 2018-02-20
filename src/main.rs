pub mod maybe;
use maybe::Maybe::{self, Nothing, Just};

pub mod iter;
pub mod test;


fn main() {

// <Type as Trait>::method(args);
let def = <Maybe<u8> as Default>::default();
println!("def: {:?}", def); // Nothing
let def2: Maybe<u8> = Default::default();
println!("def2: {:?}", def2); // Nothing


let a: Maybe<String> = Just(String::from("Alpha"));
let _b: Maybe<String> = Nothing;

let _res = match a {
  Just(v) => Just(v),
  Nothing => Nothing,
};



} // main
