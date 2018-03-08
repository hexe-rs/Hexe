//! This crate defines the **core** building blocks for the Hexe chess engine.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! hexe_core = "0.0.2"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! extern crate hexe_core;
//! # fn main() {}
//! ```
//!
//! ## Board Representation
//!
//! There are two primary chess board representations provided. Both have
//! various advantages and disadvantages, which are outlined below:
//!
//! ### [`Bitboard`](bitboard/struct.Bitboard.html)
//!
//! **Mapping:** bit-to-square
//!
//! **Advantages:**
//!
//! - Throughput—excellent for performing parallel operations on the board
//!
//!     - Checking whether a file is empty
//!
//!     - Generating moves for all pieces
//!
//! **Disadvantages:**
//!
//! - Size—larger overall memory cost
//!
//!     - A common compact way of representing all pieces with bitboards is to
//!       have 6 × [`PieceKind`](piece/enum.PieceKind.html) bitboards and 2 ×
//!       [`Color`](color/enum.Color.html) bitboards. This results in
//!       (2 + 6) × 8 = 64 bytes used to represent all pieces.
//!
//!     - Using 12 × [`Piece`](hexe_core/piece/enum.Piece.html) bitboards is
//!       another representation of the entire chess board. This results in
//!       12 × 8 = 96 bytes used to represent all pieces.
//!
//!     - Operations are often done using 64-bit (8 byte) integers
//!
//! ### [`PieceMap`](piece/map/struct.PieceMap.html)
//!
//! **Mapping:** byte-to-square
//!
//! **Advantages:**
//!
//! - Latency—great for performing instantaneous operations on the board
//!
//!     - Finding whether a square is empty or otherwise what piece sits on it
//!
//! - Size—lower overall memory cost
//!
//!     - Operations usually done with a few bytes
//!
//! **Disadvantages:**
//!
//! - Size—larger upfront memory cost
//!
//!     - Uses exactly 64 bytes for each square on the board and its piece
//!
//! [crate]: https://crates.io/crates/hexe_core

#![doc(html_logo_url = "https://raw.githubusercontent.com/hexe-rs/Hexe/assets/Icon.png")]

// Lints ///////////////////////////////////////////////////////////////////////

// Built-in
#![allow(unknown_lints)]
#![deny(missing_docs)]

// Clippy
#![allow(cast_lossless)]
#![allow(explicit_into_iter_loop)]
#![allow(inline_always)]
#![allow(redundant_field_names)]
#![allow(suspicious_arithmetic_impl)]
#![allow(unreadable_literal)]
#![allow(verbose_bit_mask)]
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

pub mod bitboard;
pub mod castle_rights;
pub mod color;
pub mod fen;
pub mod iter;
pub mod misc;
pub mod multi_board;
pub mod piece;
pub mod square;

// Modules shared with hexe that aren't meant for public use
#[doc(hidden)]
pub mod _shared {
    #[cfg(feature = "serde")]
    pub extern crate serde;

    #[cfg(feature = "simd")]
    pub extern crate simd;
}

#[allow(unused_imports)]
use _shared::*;

mod consts;
mod util;
mod magic;
