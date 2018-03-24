//! A chess game state position.

use core::board::{MultiBoard, PieceMap};
use core::misc::Contained;
use prelude::*;

mod state;
pub use self::state::*;

#[cfg(all(test, nightly))]
mod benches;

/// A representation of the current game state.
#[derive(Clone)]
pub struct Position {
    /// The current state.
    state: State,

    /// A piece map board representation for fast lookups.
    piece_map: PieceMap,

    /// Bitboards for each color and piece kind.
    board: MultiBoard,

    /// The color for the player whose turn it is.
    player: Color,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        // We can skip checking `pieces` and `colors` because they represent the
        // same data as `piece_map`.
        self.piece_map == other.piece_map &&
        self.player    == other.player    &&
        self.state     == other.state
    }
}

impl Eq for Position {}

impl Default for Position {
    fn default() -> Position {
        Position {
            state: State::default(),
            piece_map: PieceMap::STANDARD,
            board: MultiBoard::STANDARD,
            player: Color::White,
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

    /// Returns whether `self` contains the value.
    #[inline]
    #[allow(needless_lifetimes)]
    pub fn contains<'a, T: Contained<&'a Self>>(&'a self, value: T) -> bool {
        value.contained_in(self)
    }

    /// Returns the total number of pieces on the board.
    #[inline]
    pub fn total_count(&self) -> usize {
        self.all_bitboard().len()
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
    /// assert_eq!(pos.count(PieceKind::Knight), 4);
    /// assert_eq!(pos.count(Color::White), 16);
    /// assert_eq!(pos.count(Piece::BlackPawn), 8);
    /// ```
    ///
    /// [`Piece`]:     ../piece/enum.Piece.html
    /// [`PieceKind`]: ../piece/enum.PieceKind.html
    /// [`Color`]:     ../color/enum.Color.html
    #[inline]
    pub fn count<T: BitboardRetriever>(&self, retr: T) -> usize {
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
    pub fn en_passant(&self) -> Option<Square> {
        self.state.en_passant()
    }

    /// Returns the castle rights for both players.
    #[inline]
    pub fn castle_rights(&self) -> CastleRights {
        self.state.castle_rights()
    }

    /// Returns a bitboard containing squares for where all pieces reside.
    #[inline]
    pub fn all_bitboard(&self) -> Bitboard {
        self.bitboard(Color::White) | self.bitboard(Color::Black)
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

    /// Returns the square where the color's king lies on.
    #[inline]
    pub fn king_square(&self, color: Color) -> Square {
        // Both colors should *always* have a king
        debug_assert!(
            self.contains(Piece::new(PieceKind::King, color)),
            "No king found for {}", color
        );
        unsafe { self.bitboard(color).lsb_unchecked() }
    }
}

impl<'a> Contained<&'a Position> for Square {
    #[inline]
    fn contained_in(self, pos: &Position) -> bool {
        pos.piece_map.contains(self)
    }
}

macro_rules! impl_contained {
    ($($t:ty),+) => {
        $(impl<'a> Contained<&'a Position> for $t {
            #[inline]
            fn contained_in(self, pos: &Position) -> bool {
                !pos.bitboard(self).is_empty()
            }
        })+
    }
}

impl_contained! { Piece, PieceKind, Color }

/// A type whose instances serve to retrieve a `Bitboard` from a `Position`.
pub trait BitboardRetriever {
    /// Retrieves the corresponding `Bitboard` for `self` from a `Position`.
    fn bitboard(self, pos: &Position) -> Bitboard;
}

impl BitboardRetriever for PieceKind {
    #[inline]
    fn bitboard(self, pos: &Position) -> Bitboard {
        pos.board[self]
    }
}

impl BitboardRetriever for Color {
    #[inline]
    fn bitboard(self, pos: &Position) -> Bitboard {
        pos.board[self]
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
        let all = pos.all_bitboard();

        for square in Square::ALL {
            if let Some(&piece) = pos.piece_at(square) {
                assert!(all.contains(square));
                assert!(pos.bitboard(piece).contains(square));
                assert!(pos.bitboard(piece.kind()).contains(square));
                assert!(pos.bitboard(piece.color()).contains(square));
            } else {
                let (a, b) = pos.board.split();
                for &slice in &[&a[..], &b[..]] {
                    for &bitboard in slice {
                        assert!(!bitboard.contains(square));
                    }
                }
            }
        }
    }
}
