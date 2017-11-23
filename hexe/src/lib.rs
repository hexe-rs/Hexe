//! This crate serves as a way to use the Hexe chess engine from within any Rust
//! project.
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
