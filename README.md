[![crates.io badge](https://img.shields.io/crates/v/impartial-ord)](https://crates.io/crates/impartial-ord)

Derives a quicker `PartialOrd` for types that already implement `Ord`.

```rust
// Input
#[derive(PartialEq, Eq, Ord, impartial_ord::ImpartialOrd)]
struct MyStruct;

// Output
impl PartialOrd for MyStruct {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
```
