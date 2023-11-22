<!-- cargo-rdme start -->

A macro for deriving `Index` & `IndexMut` implementations.

[![MASTER CI status](https://github.com/Alorel/impl_index-rs/actions/workflows/core.yml/badge.svg)](https://github.com/Alorel/impl_index-rs/actions/workflows/core.yml?query=branch%3Amaster)
[![crates.io badge](https://img.shields.io/crates/v/impl_index)](https://crates.io/crates/impl_index)
[![docs.rs badge](https://img.shields.io/docsrs/impl_index?label=docs.rs)](https://docs.rs/impl_index)
[![dependencies badge](https://img.shields.io/librariesio/release/cargo/impl_index)](https://libraries.io/cargo/impl_index)

# Basic usage

```rust
// Derive only Index
index!(Struct by Enum => Output:
  Variant1 => field1,
  Variant2 => field2,
);

// Derive Index and IndexMut
index!(Struct by Enum => mut Output:
  Variant1 => field1,
  Variant2 => field2,
);

// Match/get by pattern
index!(Struct by Enum => Output:
  pat Enum::Variant(Foo::Bar) => field1,
  pat Enum::Index(idx) => pat field2[idx],
);
````

# Example

```rust
use impl_index::index;

#[derive(Default)]
struct Struct {
    a: u8,
    arr: [u8; 10],
    thing_1: u8,
    thing_2: u8,
}

enum Enum {
    A,
    Arr(usize),
    Thing(Thing),
}

enum Thing {
    One,
    Two,
}

index!(Struct by Enum => mut u8:
    A => a,
    pat Enum::Arr(idx) if idx < 10 => pat arr[idx],
    pat Enum::Thing(Thing::One) => thing_1,
    pat _ => thing_2,
);

let mut s = Struct::default();

s[Enum::A] = 1;

for idx in 0u8..10 {
    s[Enum::Arr(idx.into())] = idx * 10;
}

s[Enum::Thing(Thing::One)] = 200;
s[Enum::Thing(Thing::Two)] = 201;

assert_eq!(s[Enum::A], 1, "A");
for idx in 0u8..10 {
    assert_eq!(s[Enum::Arr(idx.into())], idx * 10, "Arr({})", idx);
}
assert_eq!(s[Enum::Thing(Thing::One)], 200, "Thing(One)");
assert_eq!(s[Enum::Thing(Thing::Two)], 201, "Thing(Two)");
```

## Generated output

```rust
#[automatically_derived]
impl ::core::ops::Index<Enum> for Struct {
    type Output = u8;
    fn index(&self, index_macro_derived_index_input: Enum) -> &Self::Output {
        match index_macro_derived_index_input {
            Enum::A => &self.a,
            Enum::Arr(idx) if idx < 10 => &self.arr[idx],
            Enum::Thing(Thing::One) => &self.thing_1,
            _ => &self.thing_2,
        }
    }
}
#[automatically_derived]
impl ::core::ops::IndexMut<Enum> for Struct {
    fn index_mut(&mut self, index_macro_derived_index_input: Enum) -> &mut Self::Output {
        match index_macro_derived_index_input {
            Enum::A => &mut self.a,
            Enum::Arr(idx) if idx < 10 => &mut self.arr[idx],
            Enum::Thing(Thing::One) => &mut self.thing_1,
            _ => &mut self.thing_2,
        }
    }
}
````

# Example without generating `IndexMut`

This will fail to compile on `instance[Idx::Foo] = 1;`

```rust
struct Struct {
  foo: usize,
}

enum Idx {
  Foo,
}

index!(Struct by Idx => usize: Foo => foo);

let mut instance = Struct { foo: 0 };
instance[Idx::Foo] = 1;
```

<!-- cargo-rdme end -->
