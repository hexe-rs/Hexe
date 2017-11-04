//! # Hexe Core
//!
//! This crate defines the building blocks for the Hexe chess engine.

#![doc(html_logo_url = "https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png")]

// Unstable Features ///////////////////////////////////////////////////////////
#![cfg_attr(feature = "try-from", feature(try_from))]

// Lints ///////////////////////////////////////////////////////////////////////

// Built-in
#![allow(unknown_lints)]
#![deny(missing_docs)]

// Clippy
#![allow(unreadable_literal)]

// no_std //////////////////////////////////////////////////////////////////////
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]
use std as core;

// External Crates /////////////////////////////////////////////////////////////
#[cfg(test)]
extern crate rand;
#[cfg(test)]
#[macro_use]
extern crate static_assertions;

extern crate memchr;

#[cfg(feature = "serde")]
extern crate serde;

// Re-exported to use within hexe
#[cfg(feature = "simd")]
#[doc(hidden)]
pub extern crate simd as _simd;

#[macro_use]
extern crate uncon_derive;
extern crate uncon;

// Modules /////////////////////////////////////////////////////////////////////
#[macro_use]
mod macros;

pub mod prelude;

pub mod bitboard;
pub mod castle_rights;
pub mod color;
pub mod misc;
pub mod piece;
pub mod square;

mod magic;
