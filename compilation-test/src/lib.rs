//! Mainly just test that these all compile

#![allow(unused)]

use impartial_ord::ImpartialOrd;
use std::hash::Hash;

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct A;

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct B {}

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct C();

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct D(u8);

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct E {
    x: u8,
}

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct Foo<T: Clone, E>(T, E)
where
    E: Hash;

#[derive(Eq, PartialEq, Ord, ImpartialOrd)]
struct G<T = u8>(T);
