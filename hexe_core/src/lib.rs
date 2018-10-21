//! This crate defines the **core** building blocks for [the Hexe chess
//! engine][hexe]. This library is designed to be used by Hexe but intended to
//! be used by _any_ chess program, library, or engine.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! hexe_core = "0.0.5"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate hexe_core;
//! # fn main() {}
//! ```
//!
//! # Configuration
//!
//! See [`CONFIGURATION.md`](https://github.com/hexe-rs/Hexe/blob/master/CONFIGURATION.md).
//!
//! [hexe]: https://docs.rs/hexe/0.0.5/hexe/
//! [crate]: https://crates.io/crates/hexe_core

#![doc(html_root_url = "https://docs.rs/hexe_core/0.0.5")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png")]

// Lints ///////////////////////////////////////////////////////////////////////

// Built-in
#![allow(unknown_lints)]
#![deny(missing_docs)]

// Clippy
#![allow(
    cast_lossless,
    cast_ptr_alignment,
    explicit_into_iter_loop,
    inline_always,
    needless_lifetimes,
    redundant_field_names,
    suspicious_arithmetic_impl, // very suspicious
    unreadable_literal,
    verbose_bit_mask,
    zero_prefixed_literal,
)]
#![deny(bool_comparison)]

// Attributes //////////////////////////////////////////////////////////////////

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(test, nightly), feature(test))]

// no_std //////////////////////////////////////////////////////////////////////
#[cfg(feature = "std")]
use std as core;

// Internal Crates /////////////////////////////////////////////////////////////

#[cfg(all(test, nightly))]
extern crate test;

// External Crates /////////////////////////////////////////////////////////////
#[cfg(any(test, feature = "rand"))]
extern crate rand;

#[cfg(feature = "simd")]
extern crate packed_simd;

#[cfg(test)]
#[macro_use]
extern crate static_assertions;

extern crate memchr;

#[macro_use]
extern crate uncon_derive;
extern crate uncon;

// Modules /////////////////////////////////////////////////////////////////////
#[macro_use]
mod macros;

pub mod prelude;

pub mod board;
pub mod castle;
pub mod color;
pub mod fen;
pub mod iter;
pub mod misc;
pub mod mv;
pub mod piece;
pub mod square;

// Modules shared with hexe that aren't meant for public use
#[doc(hidden)]
pub mod _shared {
    #[cfg(feature = "serde")]
    pub extern crate serde;
}

#[allow(unused_imports)]
use _shared::*;

mod sealed {
    pub trait Sealed {}
}

mod consts;
mod util;
