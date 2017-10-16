//! A bitmap chess board representation.

/// A mapping of sixty-four bits to squares of a chess board.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bitboard(pub u64);
