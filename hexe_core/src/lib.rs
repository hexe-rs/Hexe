//! # Hexe Core
//!
//! This crate defines the building blocks for the Hexe chess engine.

#![doc(html_logo_url = "https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png")]

#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(feature = "try-from", feature(try_from))]

#[cfg(feature = "std")]
extern crate core;

#[cfg(test)]
extern crate rand;
#[cfg(test)]
#[macro_use]
extern crate static_assertions;

extern crate libc;

#[cfg(feature = "serde")]
extern crate serde;

#[macro_use]
extern crate uncon_derive;
extern crate uncon;

#[macro_use]
mod macros;

pub mod prelude;

pub mod bitboard;
pub mod castle_rights;
pub mod color;
pub mod piece;
pub mod square;

mod magic;
