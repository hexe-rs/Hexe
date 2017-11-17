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
#![allow(explicit_into_iter_loop)]
#![allow(inline_always)]
#![allow(unreadable_literal)]
#![allow(verbose_bit_mask)]

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

// Re-exported to use within hexe
#[cfg(feature = "serde")]
#[doc(hidden)]
pub extern crate serde as _serde;

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

mod ext;
mod magic;
