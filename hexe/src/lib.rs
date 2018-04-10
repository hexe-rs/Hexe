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
//! # Using `hexe_core`
//!
//! This crate reexports the [`hexe_core`] crate as `hexe::core`. Its most
//! commonly used parts are available in the [`prelude`].
//!
//! If you wish to use `hexe_core` with its original name, you may do:
//!
//! ```
//! use hexe::core as hexe_core;
//! ```
//!
//! ## What `hexe_core` Provides
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
#![allow(needless_lifetimes)]
#![allow(redundant_field_names)]
#![allow(unreadable_literal)]
#![allow(while_immutable_condition)]
#![allow(zero_prefixed_literal)]

// Attributes //////////////////////////////////////////////////////////////////

#![cfg_attr(all(test, nightly), feature(test))]

// Internal Crates /////////////////////////////////////////////////////////////

#[cfg(all(test, nightly))]
extern crate test;

// External Crates /////////////////////////////////////////////////////////////

pub extern crate hexe_core as core;

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

// Modules /////////////////////////////////////////////////////////////////////

#[allow(unused_imports)]
use core::_shared::*;

mod table;
mod util;
mod zero;

pub mod engine;
pub mod position;
pub mod prelude;
pub mod zobrist;
