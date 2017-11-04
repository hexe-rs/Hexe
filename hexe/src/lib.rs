//! # Hexe
//!
//! This crate serves as a way to use the Hexe chess engine from within any Rust
//! project.

#![doc(html_logo_url = "https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png")]

pub extern crate hexe_core as core;

#[macro_use]
extern crate uncon_derive;
extern crate uncon;

#[cfg(feature = "simd")]
use core::_simd as simd;

#[cfg(test)]
#[macro_use]
extern crate static_assertions;

pub mod mv;
pub mod position;
pub mod prelude;
