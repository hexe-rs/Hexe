//! A chess game state position.

use core::piece::map::PieceMap;
use core::misc::Contained;
use prelude::*;
use uncon::*;

/// The raw value used to represent no square for a space-optimized square.
const NO_SQUARE: u8 = 1 + Square::H8 as u8;

#[cfg(test)]
const_assert_eq!(no_sq; NO_SQUARE, 64);

/// A representation of the current game state.
pub struct Position {
    /// A piece map board representation for fast lookups.
    piece_map: PieceMap,

    /// Bitboards for each piece kind. Uses `u64` for convenience.
    pieces: [u64; 6],

    /// Bitboards for each color. Uses `u64` for convenience.
    colors: [u64; 2],

    /// The color for the player whose turn it is.
    player: Color,

    /// The square used in an en passant capture, if any.
    ///
    /// Uses a value of `NO_SQUARE` when empty. This is because `Option<Square>`
    /// currently uses two bytes instead of one.
    en_passant: u8,
}

impl PartialEq for Position {
    #[inline]
    fn eq(&self, other: &Position) -> bool {
        // We can skip checking `pieces` and `colors` because they represent the
        // same data as `piece_map`.
        self.piece_map  == other.piece_map &&
        self.player     == other.player    &&
        self.en_passant == other.en_passant
    }
}

impl Eq for Position {}

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

    /// Returns whether `self` contains the value.
    #[inline]
    pub fn contains<T: Contained<Self>>(&self, value: T) -> bool {
        value.contained_in(self)
    }

    /// Returns the number of pieces on the board.
    #[inline]
    pub fn count(&self) -> usize {
        self.all_pieces().len()
    }

    /// Returns the number of pieces for the retriever.
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
    /// assert_eq!(pos.count_of(PieceKind::Knight), 4);
    /// assert_eq!(pos.count_of(Color::White), 16);
    /// assert_eq!(pos.count_of(Piece::BlackPawn), 8);
    /// ```
    ///
    /// [`Piece`]:     ../piece/enum.Piece.html
    /// [`PieceKind`]: ../piece/enum.PieceKind.html
    /// [`Color`]:     ../color/enum.Color.html
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

impl Contained<Position> for Square {
    #[inline]
    fn contained_in(self, pos: &Position) -> bool {
        pos.piece_map.contains(self)
    }
}

macro_rules! impl_contained {
    ($($t:ty),+) => {
        $(impl Contained<Position> for $t {
            #[inline]
            fn contained_in(self, pos: &Position) -> bool {
                !pos.bitboard(self).is_empty()
            }
        })+
    }
}

impl_contained! { Piece, PieceKind, Color }

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
        let all = pos.all_pieces();

        for square in Square::all() {
            if let Some(&piece) = pos.piece_at(square) {
                assert!(all.contains(square));
                assert!(pos.bitboard(piece).contains(square));
                assert!(pos.bitboard(piece.kind()).contains(square));
                assert!(pos.bitboard(piece.color()).contains(square));
            } else {
                for &bitboard in &pos.pieces {
                    assert!(!Bitboard(bitboard).contains(square));
                }
                for &bitboard in &pos.colors {
                    assert!(!Bitboard(bitboard).contains(square));
                }
            }
        }
    }
}
