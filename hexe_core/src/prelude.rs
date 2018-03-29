//! The Hexe core prelude imports various primitives and traits that may be
//! used often when interacting with this crate.

// Concrete types
pub use board::Bitboard;
pub use castle::{Rights, Right};
pub use color::Color;
pub use mv::Move;
pub use piece::{Piece, PieceKind, Promotion};
pub use square::{Square, File, Rank};

// Abstract types (traits)
pub use iter::AllIterable;
