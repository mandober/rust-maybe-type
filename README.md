# Maybe

- lang: Rust
- date: 2017-12-11
- type: exercise
- desc: Option's doppelganger, the Maybe type (just for exercise)

Implementation of the `Maybe` type, exactly the same thing as the `Option`, purely as get-intimate-with-impl-details sort of exercise. Heavily commented.

```rust
pub enum Maybe<T> {
    Just(T),
    Nothing
}
```

[Source code][opt] of `Option` in `libcore`.


[opt]: https://doc.rust-lang.org/src/core/option.rs.html