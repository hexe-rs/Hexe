use core::fmt;

use color::Color;
use uncon::*;

/// A chess piece.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
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

static PIECE_CHARS_ASCII: &[u8; 12] = b"PpNnBbRrQqKk";

impl From<Piece> for char {
    #[inline]
    fn from(p: Piece) -> char {
        PIECE_CHARS_ASCII[p as usize] as char
    }
}

impl Piece {
    /// Creates a new `Piece` with a `PieceKind` and `Color`.
    #[inline]
    pub fn new(kind: PieceKind, color: Color) -> Piece {
        unsafe { Piece::from_unchecked((kind as u8) << 1 | color as u8) }
    }

    /// Converts `self` into a character.
    #[inline]
    pub fn into_char(self) -> char {
        self.into()
    }
}

/// A chess piece kind.
#[derive(Copy, Clone, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

static KINDS: [&str; 6] = ["Pawn", "Knight", "Bishop", "Rook", "Queen", "King"];

impl fmt::Debug for PieceKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for PieceKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.into_str(), f)
    }
}

impl From<PieceKind> for char {
    #[inline]
    fn from(pk: PieceKind) -> char {
        PIECE_CHARS_ASCII[(self as usize) << 1] as char
    }
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

    /// Converts `self` into a static string.
    #[inline]
    pub fn into_str(self) -> &'static str {
        KINDS[self as usize]
    }

    /// Converts `self` into a character.
    #[inline]
    pub fn into_char(self) -> char {
        self.into()
    }
}
