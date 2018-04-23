//! This crate serves as a way to use the Hexe chess engine from within any Rust
//! project.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! hexe = "0.0.5"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate hexe;
//! # fn main() {}
//! ```
//!
//! # Configuration
//!
//! See [`CONFIGURATION.md`](https://github.com/hexe-rs/Hexe/blob/master/CONFIGURATION.md).
//!
//! # Core
//!
//! The [`hexe_core`] crate serves as the building blocks for Hexe. It is
//! designed to be a general-purpose chess library that can be easily used by
//! any chess program or library. Its most commonly used parts can be found in
//! the [`prelude`].
//!
//! What it provides:
//!
//! - [Various board representations](https://docs.rs/hexe_core/0.0.5/hexe_core/board/)
//!
//! - [Dead.](https://docs.rs/hexe_core/0.0.5/hexe_core/castle/struct.Rights.html)
//!   [Simple.](https://docs.rs/hexe_core/0.0.5/hexe_core/square/enum.Square.html)
//!   [Primitives.](https://docs.rs/hexe_core/0.0.5/hexe_core/piece/enum.Piece.html)
//!
//! [crate]: https://crates.io/crates/hexe
//! [`hexe_core`]: https://docs.rs/hexe_core
//! [`prelude`]: prelude/index.html

#![doc(html_root_url = "https://docs.rs/hexe/0.0.5")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png")]

// Lints ///////////////////////////////////////////////////////////////////////

// Built-in
#![allow(unknown_lints)]
#![deny(missing_docs)]

// Clippy
#![allow(
    needless_lifetimes,
    redundant_field_names,
    unreadable_literal,
    while_immutable_condition,
    zero_prefixed_literal,
)]

// Attributes //////////////////////////////////////////////////////////////////

#![cfg_attr(all(test, nightly), feature(test))]

// Standard Crates /////////////////////////////////////////////////////////////

#[cfg(all(test, nightly))]
extern crate test;

// crates.io ///////////////////////////////////////////////////////////////////

extern crate hexe_core as core;

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

#[cfg(not(feature = "log"))]
#[macro_use]
mod log {
    macro_rules! trace { ($($t:tt)*) => {} }
    macro_rules! debug { ($($t:tt)*) => {} }
    macro_rules! info  { ($($t:tt)*) => {} }
    macro_rules! warn  { ($($t:tt)*) => {} }
    macro_rules! error { ($($t:tt)*) => {} }
}

extern crate crossbeam_deque;
extern crate libc;
extern crate num_cpus;
extern crate parking_lot;
extern crate uncon;

#[cfg(any(test, feature = "rand"))]
extern crate rand;

#[cfg(test)]
#[macro_use]
extern crate static_assertions;

// Macros //////////////////////////////////////////////////////////////////////

/// A compile-time string containing the authors of this project.
#[macro_export]
macro_rules! authors {
    () => { "Nikolai Vazquez" }
}

// Modules /////////////////////////////////////////////////////////////////////

#[doc(inline)]
pub use core::{board, castle, color, fen, iter, misc, mv, piece, simd, square};

#[allow(unused_imports)]
use core::_shared::*;

#[macro_use]
mod macros;
mod table;
mod util;
mod zero;

pub mod engine;
pub mod position;
pub mod prelude;
pub mod zobrist;

#[doc(inline)] pub use self::engine::Engine;
#[doc(inline)] pub use self::position::Position;
