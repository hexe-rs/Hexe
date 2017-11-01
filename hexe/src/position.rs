//! A chess game state position.

use hexe_core::piece::map::PieceMap;
use prelude::*;
use uncon::*;

/// The raw value used to represent no square for a space-optimized square.
const NO_SQUARE: u8 = 1 + Square::H8 as u8;

#[cfg(test)]
const_assert_eq!(no_sq; NO_SQUARE, 64);

/// A representation of the current game state.
#[derive(PartialEq, Eq)]
pub struct Position {
    /// A piece map board representation for fast lookups.
    piece_map: PieceMap,

    /// Bitboards for each piece kind.
    pieces: [u64; 6],

    /// Bitboards for each color.
    colors: [u64; 2],

    /// The color for the player whose turn it is.
    player: Color,

    /// The square used in an en passant capture, if any.
    ///
    /// Uses a value of `NO_SQUARE` when empty. This is because `Option<Square>`
    /// currently uses two bytes instead of one.
    en_passant: u8,
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
            player: Color::White,
            en_passant: NO_SQUARE,
        }
    }
}

impl Position {
    /// Returns the inner piece map.
    #[inline]
    pub fn piece_map(&self) -> &PieceMap {
        &self.piece_map
    }

    /// Returns the piece at the square, if any.
    #[inline]
    pub fn piece_at(&self, sq: Square) -> Option<&Piece> {
        self.piece_map.get(sq)
    }

    /// Returns a bitboard containing squares for where all pieces reside.
    #[inline]
    pub fn all_pieces(&self) -> Bitboard {
        self.bitboard(Color::White) | self.bitboard(Color::Black)
    }

    /// Returns the number of pieces on the board.
    #[inline]
    pub fn count(&self) -> usize {
        self.all_pieces().len()
    }

    /// Returns the number of pieces for the retriever.
    #[inline]
    pub fn count_of<T: BitboardRetriever>(&self, retr: T) -> usize {
        self.bitboard(retr).len()
    }

    /// Returns the current player's color.
    #[inline]
    pub fn player(&self) -> Color {
        self.player
    }

    /// Returns the bitboard corresponding to the current player.
    #[inline]
    pub fn player_bitboard(&self) -> Bitboard {
        self.bitboard(self.player())
    }

    /// Returns the opponent player's color.
    #[inline]
    pub fn opponent(&self) -> Color {
        !self.player()
    }

    /// Returns the bitboard corresponding to the opponent player.
    #[inline]
    pub fn opponent_bitboard(&self) -> Bitboard {
        self.bitboard(self.opponent())
    }

    /// Returns the en passant square.
    #[inline]
    pub fn en_passant(&self) -> Option<&Square> {
        match self.en_passant {
            NO_SQUARE => None,
            ref ep => unsafe { Some(ep.into_unchecked()) }
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_pieces() {
        let pos = Position::default();

        for kind in (0..6u8).map(PieceKind::from) {
            for &color in &[Color::White, Color::Black] {
                let piece = Piece::new(kind, color);

                for square in pos.bitboard(kind) {
                    assert_eq!(pos.piece_map.kind_at(square), Some(kind));
                }

                for square in pos.bitboard(color) {
                    assert_eq!(pos.piece_map.color_at(square), Some(color));
                }

                for square in pos.bitboard(piece) {
                    assert_eq!(pos.piece_at(square), Some(&piece));
                }
            }
        }
    }
}
