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
//! hexe = "0.0.0"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate hexe;
//! # fn main() {}
//! ```
//!
//!
//! # Using `hexe_core`
//!
//! This crate reexports the `hexe_core` crate as `hexe::core`. Its most
//! commonly used parts are available in the [`prelude`].
//!
//! If you wish to use `hexe_core` with its original name, you may do:
//!
//! ```
//! use hexe::core as hexe_core;
//! ```
//!
//! [crate]: https://crates.io/crates/hexe
//! [`prelude`]: prelude/index.html

#![doc(html_logo_url = "https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png")]

// Lints ///////////////////////////////////////////////////////////////////////

// Built-in
#![allow(unknown_lints)]
#![deny(missing_docs)]

// Clippy
#![allow(unreadable_literal)]

// External Crates /////////////////////////////////////////////////////////////

pub extern crate hexe_core as core;

#[allow(unused_imports)]
use core::_shared::*;

#[macro_use]
extern crate uncon_derive;
extern crate uncon;

#[cfg(test)]
#[macro_use]
extern crate static_assertions;

// Modules /////////////////////////////////////////////////////////////////////

pub mod mv;
pub mod position;
pub mod prelude;
