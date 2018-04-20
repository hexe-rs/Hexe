//! The core prelude re-exports various types and traits that are used often.
//!
//! # Usage
//!
//! This module may be batch imported for convenience.
//!
//! ```
//! use hexe_core::prelude::*;
//! ```
//!
//! Specific items can also be chosen.
//!
//! ```
//! use hexe_core::prelude::{BitBoard, Square, Piece};
//! ```

// Concrete types
pub use board::BitBoard;
pub use castle::{Rights, Right};
pub use color::Color;
pub use mv::Move;
pub use piece::{Piece, Role, Promotion};
pub use square::{Square, File, Rank};

// Abstract types (traits)
pub use iter::All;
pub use misc::Extract;
