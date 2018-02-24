//  MOD: maybe
// FILE: maybe/mod.rs
// although in subfolder, this mod is in the same level as root mod (main.rs)
// so it is declared there as: `pub mod maybe`


// declare maybe (sub)module
pub mod maybe_enum;
// inside maybe/maybe_enum.rs
// use self::Maybe::{Just, Nothing};


// re-export
//
// ::(root) = main.rs
// super    = main.rs
// self     = maybe/mod.rs = maybe


// use uses abs path: it implicitly starts with ::(root)
// abs path to Maybe type from here:
//pub use maybe::maybe_enum::Maybe;

// rel path to Maybe type from here
// must start with self/super to force rel path:
pub use self::maybe_enum::Maybe;
// pub use super::maybe::maybe_enum::Maybe;

// super = main.rs
//
// pub use self::maybe_enum::Maybe;
// re-export Maybe, Maybe::Just and Maybe::Nothing
// pub use self::maybe_enum::Maybe::{self, Just, Nothing};

// pub use super::maybe::maybe_enum::Maybe;


pub mod iter;
// inside iter.rs:
// use super::Maybe::{self, Nothing, Just};



pub mod test;
