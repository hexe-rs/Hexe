//! # Hexe
//!
//! This crate serves as a way to use the Hexe chess engine from within any Rust
//! project.

#![doc(html_logo_url = "https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png")]

extern crate hexe_core;
extern crate uncon;

#[cfg(feature = "simd")]
use hexe_core::_simd as simd;

#[cfg(test)]
#[macro_use]
extern crate static_assertions;

pub use hexe_core::{bitboard, castle_rights, color, piece, square};

pub mod position;
pub mod prelude;
