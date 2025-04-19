//! A macro for deriving [`Index`](::core::ops::Index) & [`IndexMut`](::core::ops::IndexMut) implementations.
//!
//! [![MASTER CI status](https://github.com/Alorel/impl_index-rs/actions/workflows/core.yml/badge.svg)](https://github.com/Alorel/impl_index-rs/actions/workflows/core.yml?query=branch%3Amaster)
//! [![crates.io badge](https://img.shields.io/crates/v/impl_index)](https://crates.io/crates/impl_index)
//! [![docs.rs badge](https://img.shields.io/docsrs/impl_index?label=docs.rs)](https://docs.rs/impl_index)
//! [![dependencies badge](https://img.shields.io/librariesio/release/cargo/impl_index)](https://libraries.io/cargo/impl_index)
//!
//! # Basic usage
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! // Derive only Index
//! index!(Struct by Enum => Output:
//!   Variant1 => field1,
//!   Variant2 => field2,
//! );
//!
//! // Derive Index and IndexMut
//! index!(Struct by Enum => mut Output:
//!   Variant1 => field1,
//!   Variant2 => field2,
//! );
//!
//! // Match/get by pattern
//! index!(Struct by Enum => Output:
//!   pat Enum::Variant(Foo::Bar) => field1,
//!   pat Enum::Index(idx) => pat field2[idx],
//! );
//! ````
//!
//! # Example
//!
//! ```
//! use impl_index::index;
//!
//! #[derive(Default)]
//! struct Struct {
//!     a: u8,
//!     arr: [u8; 10],
//!     thing_1: u8,
//!     thing_2: u8,
//! }
//!
//! enum Enum {
//!     A,
//!     Arr(usize),
//!     Thing(Thing),
//! }
//!
//! enum Thing {
//!     One,
//!     Two,
//! }
//!
//! index!(Struct by Enum => mut u8:
//!     A => a,
//!     pat Enum::Arr(idx) if idx < 10 => pat arr[idx],
//!     pat Enum::Thing(Thing::One) => thing_1,
//!     pat _ => thing_2,
//! );
//!
//! let mut s = Struct::default();
//!
//! s[Enum::A] = 1;
//!
//! for idx in 0u8..10 {
//!     s[Enum::Arr(idx.into())] = idx * 10;
//! }
//!
//! s[Enum::Thing(Thing::One)] = 200;
//! s[Enum::Thing(Thing::Two)] = 201;
//!
//! assert_eq!(s[Enum::A], 1, "A");
//! for idx in 0u8..10 {
//!     assert_eq!(s[Enum::Arr(idx.into())], idx * 10, "Arr({})", idx);
//! }
//! assert_eq!(s[Enum::Thing(Thing::One)], 200, "Thing(One)");
//! assert_eq!(s[Enum::Thing(Thing::Two)], 201, "Thing(Two)");
//! ```
//!
//! ## Generated output
//!
#![cfg_attr(doctest, doc = " ````no_test")]
//! ```
//! #[automatically_derived]
//! impl ::core::ops::Index<Enum> for Struct {
//!     type Output = u8;
//!     fn index(&self, index_macro_derived_index_input: Enum) -> &Self::Output {
//!         match index_macro_derived_index_input {
//!             Enum::A => &self.a,
//!             Enum::Arr(idx) if idx < 10 => &self.arr[idx],
//!             Enum::Thing(Thing::One) => &self.thing_1,
//!             _ => &self.thing_2,
//!         }
//!     }
//! }
//! #[automatically_derived]
//! impl ::core::ops::IndexMut<Enum> for Struct {
//!     fn index_mut(&mut self, index_macro_derived_index_input: Enum) -> &mut Self::Output {
//!         match index_macro_derived_index_input {
//!             Enum::A => &mut self.a,
//!             Enum::Arr(idx) if idx < 10 => &mut self.arr[idx],
//!             Enum::Thing(Thing::One) => &mut self.thing_1,
//!             _ => &mut self.thing_2,
//!         }
//!     }
//! }
//! ````
//!
//! # Example without generating `IndexMut`
//!
//! This will fail to compile on `instance[Idx::Foo] = 1;`
//!
//! ```compile_fail
//! # use impl_index::index;
//! struct Struct {
//!   foo: usize,
//! }
//!
//! enum Idx {
//!   Foo,
//! }
//!
//! index!(Struct by Idx => usize: Foo => foo);
//!
//! let mut instance = Struct { foo: 0 };
//! instance[Idx::Foo] = 1;
//! ```

#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::complexity, clippy::perf, clippy::style, clippy::pedantic)]
#![warn(missing_docs)]
#![cfg_attr(doc_cfg, feature(doc_cfg, doc_auto_cfg))]

mod parse;
mod tokenise;

use proc_macro::TokenStream as BaseTokenStream;
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Pat, Token};

/// Derive [`Index`](::core::ops::Index) & [`IndexMut`](::core::ops::IndexMut) implementations.
///
#[cfg_attr(doctest, doc = " ````no_test")]
/// ```
/// index!(Struct by Enum => mut u8:
///     UnitVariant => field_1,
///     pat Enum::Array(idx) if idx < 10 => pat array_field[idx],
///     pat Enum::Thing(Thing::One) => thing_1,
///     pat _ => thing_2,
/// );
/// ````
///
/// See [crate-level docs](crate) for a full example.
#[proc_macro]
pub fn index(input: BaseTokenStream) -> BaseTokenStream {
    parse_macro_input!(input as IndexMacro)
        .into_token_stream()
        .into()
}

struct IndexMacro {
    impl_mut: bool,
    generics: Option<syn::Generics>,
    idx_by: syn::Type,
    impl_for: syn::Type,
    output: syn::Type,
    pairings: Punctuated<Pairing, Token![,]>,
}

struct Pairing {
    enum_variant: MaybeIdent<Pattern>,
    struct_field: MaybeIdent<Expr>,
}

struct Pattern {
    pat: Pat,
    guard: Option<(Token![if], Expr)>,
}

enum MaybeIdent<Other> {
    Ident(Ident),
    Other(Other),
}

struct IndexedPairing<P, T> {
    pairing: P,
    idx_by: T,
    mut_ref: Option<Token![mut]>,
}

impl AsRef<Pairing> for Pairing {
    #[inline]
    fn as_ref(&self) -> &Pairing {
        self
    }
}
