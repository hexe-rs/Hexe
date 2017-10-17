//! # Hexe Core
//!
//! This crate defines the building blocks for the Hexe chess engine.

#![doc(html_logo_url = "https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png")]

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
