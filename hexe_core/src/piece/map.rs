use super::*;

const NONE: u8 = 1 + Piece::BlackKing as u8;

/// A mapping of sixty-four squares to pieces.
///
/// This allows for faster lookups than possible with bitboards.
#[derive(Copy, Clone)]
pub struct PieceMap([u8; 64]);

impl PartialEq for PieceMap {
    #[inline]
    fn eq(&self, other: &PieceMap) -> bool {
        self.0[..] == other.0[..]
    }
}

impl Eq for PieceMap {}

impl Default for PieceMap {
    #[inline]
    fn default() -> PieceMap {
        PieceMap::EMPTY
    }
}

impl PieceMap {
    /// An empty piece map.
    pub const EMPTY: PieceMap = PieceMap([NONE; 64]);
}
