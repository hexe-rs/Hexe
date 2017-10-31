//! A chess game state position.

use hexe_core::piece::map::PieceMap;
use prelude::*;

/// A representation of the current game state.
#[derive(PartialEq, Eq)]
pub struct Position {
    piece_map: PieceMap,
    pieces: [u64; 6],
    colors: [u64; 2],
}

impl Default for Position {
    fn default() -> Position {
        const PAWN:   u64 = 0x00FF00000000FF00;
        const KNIGHT: u64 = 0x4200000000000042;
        const BISHOP: u64 = 0x2400000000000024;
        const ROOK:   u64 = 0x8100000000000081;
        const QUEEN:  u64 = 0x0800000000000008;
        const KING:   u64 = 0x1000000000000010;
        const WHITE:  u64 = 0x000000000000FFFF;
        const BLACK:  u64 = 0xFFFF000000000000;

        Position {
            piece_map: PieceMap::STANDARD,
            pieces: [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING],
            colors: [WHITE, BLACK],
        }
    }
}

impl Position {
    /// Returns the corresponding bitboard for the retriever.
    ///
    /// # Examples
    ///
    /// This method can be used for [`Piece`], [`PieceKind`], and [`Color`]:
    ///
    /// ```
    /// # use hexe::position::Position;
    /// # use hexe::prelude::*;
    /// let pos = Position::default();
    ///
    /// let kind  = PieceKind::Knight;
    /// let color = Color::White;
    /// let piece = Piece::new(kind, color);
    ///
    /// assert_eq!(
    ///     pos.bitboard(kind) & pos.bitboard(color),
    ///     pos.bitboard(piece)
    /// );
    /// ```
    ///
    /// [`Piece`]:     ../piece/enum.Piece.html
    /// [`PieceKind`]: ../piece/enum.PieceKind.html
    /// [`Color`]:     ../color/enum.Color.html
    #[inline]
    pub fn bitboard<T: BitboardRetriever>(&self, retr: T) -> Bitboard {
        retr.bitboard(self)
    }
}

/// A type whose instances serve to retrieve a [`Bitboard`] from a [`Position`].
///
/// [`Bitboard`]: ../bitboard/struct.Bitboard.html
/// [`Position`]: struct.Position.html
pub trait BitboardRetriever {
    /// Retrieves the corresponding `Bitboard` for `self` from a `Position`.
    fn bitboard(self, pos: &Position) -> Bitboard;
}

impl BitboardRetriever for PieceKind {
    #[inline]
    fn bitboard(self, pos: &Position) -> Bitboard {
        Bitboard(pos.pieces[self as usize])
    }
}

impl BitboardRetriever for Color {
    #[inline]
    fn bitboard(self, pos: &Position) -> Bitboard {
        Bitboard(pos.colors[self as usize])
    }
}

impl BitboardRetriever for Piece {
    #[inline]
    fn bitboard(self, pos: &Position) -> Bitboard {
        self.kind().bitboard(pos) & self.color().bitboard(pos)
    }
}
