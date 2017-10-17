#![no_std]

#[cfg(test)]
extern crate rand;

#[macro_use]
extern crate uncon_derive;
extern crate uncon;

pub mod prelude;

pub mod bitboard;
pub mod color;
pub mod square;

mod magic;
