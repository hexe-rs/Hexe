//! The Hexe prelude re-exports various types and traits that are used often.
//!
//! Notice that the contents of the core prelude are also exported, removing the
//! need to it separately.
//!
//! # Usage
//!
//! This module may be batch imported for convenience.
//!
//! ```
//! use hexe::prelude::*;
//! ```
//!
//! Specific items can also be chosen.
//!
//! ```
//! use hexe::prelude::{BitBoard, Square, Position};
//! ```

pub use core::prelude::*;

pub use engine::Engine;
pub use position::Position;
