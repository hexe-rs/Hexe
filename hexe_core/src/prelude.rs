//! The Hexe core prelude imports various primitives and traits that may be
//! used often when interacting with this crate.

// Concrete types
pub use bitboard::Bitboard;
pub use castle_rights::{CastleRights, CastleRight};
pub use color::Color;
pub use piece::{Piece, PieceKind};
pub use square::{Square, File, Rank};

// Abstract types (traits)
pub use iter::AllIterable;
