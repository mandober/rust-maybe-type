# Maybe type

- language: Rust
- date: 2018-02-19
- type: exercise
- desc: rewriting Option as Maybe type, for learning purposes
- [Option source][opt] in std.

Ignoring that Option already exists, this is heavily commented and explained 
implementation of Maybe type and its methods, doubling the utility of Option 
type. For learning purposes only.

```rust
pub enum Maybe<T> {
    Just(T),
    Nothing
}
```

## Enums
- Maybe<T>

## Structs
- Iter

## Implemented traits
- Iterator




[opt]: https://doc.rust-lang.org/nightly/src/core/option.rs.html