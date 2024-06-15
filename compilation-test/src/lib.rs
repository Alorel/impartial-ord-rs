//! Mainly just test that these all compile

#![allow(unused)]

use impartial_ord::ImpartialOrd;
use static_assertions::{assert_impl_any, assert_not_impl_any};
use std::hash::Hash;
use std::sync::atomic::AtomicU8;

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct A;
assert_impl_any!(A: PartialOrd);

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct B {}
assert_impl_any!(B: PartialOrd);

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct C();
assert_impl_any!(C: PartialOrd);

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct D(u8);
assert_impl_any!(D: PartialOrd);

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct E {
    x: u8,
}
assert_impl_any!(E: PartialOrd);

#[derive(ImpartialOrd, Eq, PartialEq, Ord)]
struct Foo<T: Clone, E>(T, E)
where
    E: Hash;
assert_impl_any!(Foo::<u8, u8>: PartialOrd);

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct G<T = u8>(T);
assert_impl_any!(G::<u8>: PartialOrd);
assert_not_impl_any!(G::<AtomicU8>: PartialOrd);

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct H<E, const T: usize = 0>(E, [u8; T]);
assert_impl_any!(H::<u8>: PartialOrd);
assert_not_impl_any!(H::<AtomicU8>: PartialOrd);
