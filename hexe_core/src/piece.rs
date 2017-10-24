/// A chess piece.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

/// A chess piece kind.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    /// Returns a piece kind from the parsed character.
    pub fn from_char(ch: char) -> Option<PieceKind> {
        use self::PieceKind::*;
        match 32 | ch as u8 {
            b'p' => Some(Pawn),
            b'n' => Some(Knight),
            b'b' => Some(Bishop),
            b'r' => Some(Rook),
            b'q' => Some(Queen),
            b'k' => Some(King),
            _ => None,
        }
    }
}
