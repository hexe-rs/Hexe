//! # Hexe
//!
//! This crate serves as a way to use the Hexe chess engine from within any Rust
//! project.

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
