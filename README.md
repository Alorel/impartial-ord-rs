[![master CI badge](https://img.shields.io/github/actions/workflow/status/Alorel/impartial-ord-rs/ci.yml?label=master%20CI)](https://github.com/Alorel/impartial-ord-rs/actions/workflows/ci.yml?query=branch%3Amaster)
[![crates.io badge](https://img.shields.io/crates/v/impartial-ord)](https://crates.io/crates/impartial-ord)
[![docs.rs badge](https://img.shields.io/docsrs/impartial-ord?label=docs.rs)](https://docs.rs/impartial-ord)
[![dependencies badge](https://img.shields.io/librariesio/release/cargo/impartial-ord)](https://libraries.io/cargo/impartial-ord)

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
