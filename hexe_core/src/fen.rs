//! [Forsyth–Edwards Notation][fen] parsing.
//!
//! [fen]: https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation

use core::fmt::{self, Write};
use core::str;

use prelude::*;
use board::PieceMap;

/// A type that can be used to parse [Forsyth–Edwards Notation (FEN)][fen].
///
/// [fen]: https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
#[derive(Clone, PartialEq, Eq)]
pub struct Fen {
    /// The pieces on the board.
    pub pieces: PieceMap,
    /// The active color.
    pub color: Color,
    /// The castling rights.
    pub castling: Rights,
    /// The en passant target square.
    pub en_passant: Option<Square>,
    /// The number of halfmoves since the last capture or pawn advance.
    pub halfmoves: u32,
    /// The fullmove number.
    pub fullmoves: u32,
}

impl fmt::Display for Fen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pieces.map_fen(|s| f.write_str(s))?;

        {
            let mut buf: [u8; 4] = *b"    ";
            buf[1] = match self.color {
                Color::White => b'w',
                Color::Black => b'b',
            };
            let string = unsafe { str::from_utf8_unchecked(&buf[..3]) };
            f.write_str(string)?;
        }

        self.castling.map_str(|s| f.write_str(s))?;

        if let Some(sq) = self.en_passant {
            let mut buf: [u8; 4] = *b"    ";
            buf[1] = char::from(sq.file()) as u8;
            buf[2] = char::from(sq.rank()) as u8;
            let string = unsafe { str::from_utf8_unchecked(&buf) };
            f.write_str(string)?;
        } else {
            f.write_str(" - ")?;
        }

        self.halfmoves.fmt(f)?;
        f.write_char(' ')?;

        self.fullmoves.fmt(f)
    }
}

impl Fen {
    /// FEN for the starting position in standard chess. It is equivalent to:
    ///
    /// ```txt
    /// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    /// ```
    pub const STANDARD: Fen = Fen {
        pieces: PieceMap::STANDARD,
        color: Color::White,
        castling: Rights::FULL,
        en_passant: None,
        halfmoves: 0,
        fullmoves: 1,
    };

    /// FEN for the empty position. It is equivalent to:
    ///
    /// ```txt
    /// 8/8/8/8/8/8/8/8 w - - 0 1
    /// ```
    pub const EMPTY: Fen = Fen {
        pieces: PieceMap::EMPTY,
        color: Color::White,
        castling: Rights::EMPTY,
        en_passant: None,
        halfmoves: 0,
        fullmoves: 1,
    };
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let fens = [
            (Fen::STANDARD, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            (Fen::EMPTY,    "8/8/8/8/8/8/8/8 w - - 0 1"),
        ];

        for &(ref fen, exp) in fens.iter() {
            let string = format!("{}", fen);
            assert_eq!(string, exp);
        }
    }
}
