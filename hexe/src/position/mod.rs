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
    pieces: PieceMap,

    /// Bitboards for each color and piece kind.
    board: MultiBoard,

    /// The color for the player whose turn it is.
    player: Color,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        // Skip checking `board`; it represents the same data as `pieces`.
        self.pieces == other.pieces &&
        self.player == other.player &&
        self.state  == other.state
    }
}

impl Eq for Position {}

impl Default for Position {
    fn default() -> Position {
        Position {
            state: State::default(),
            pieces: PieceMap::STANDARD,
            board: MultiBoard::STANDARD,
            player: Color::White,
        }
    }
}

impl Position {
    /// Returns the inner piece map.
    #[inline]
    pub fn pieces(&self) -> &PieceMap {
        &self.pieces
    }

    /// Returns the inner board.
    #[inline]
    pub fn board(&self) -> &MultiBoard {
        &self.board
    }

    /// Returns whether `self` contains the value.
    #[inline]
    #[allow(needless_lifetimes)]
    pub fn contains<'a, T: Contained<&'a Self>>(&'a self, value: T) -> bool {
        value.contained_in(self)
    }

    /// Returns the current player's color.
    #[inline]
    pub fn player(&self) -> Color {
        self.player
    }

    /// Returns the bitboard corresponding to the current player.
    #[inline]
    pub fn player_bitboard(&self) -> Bitboard {
        self.board().bitboard(self.player())
    }

    /// Returns the opponent player's color.
    #[inline]
    pub fn opponent(&self) -> Color {
        !self.player()
    }

    /// Returns the bitboard corresponding to the opponent player.
    #[inline]
    pub fn opponent_bitboard(&self) -> Bitboard {
        self.board().bitboard(self.opponent())
    }

    /// Returns the en passant square.
    #[inline]
    pub fn en_passant(&self) -> Option<Square> {
        self.state.en_passant()
    }

    /// Returns the castle rights for both players.
    #[inline]
    pub fn castle_rights(&self) -> Rights {
        self.state.castle_rights()
    }

    /// Returns the square where the color's king lies on.
    #[inline]
    pub fn king_square(&self, color: Color) -> Square {
        let piece = Piece::new(PieceKind::King, color);
        let board = self.board().bitboard(piece);

        // Both colors should *always* have a king
        debug_assert!(!board.is_empty(), "{:?} not found", piece);

        unsafe { board.lsb_unchecked() }
    }
}

impl<'a> Contained<&'a Position> for Square {
    #[inline]
    fn contained_in(self, pos: &Position) -> bool {
        pos.pieces().contains(self)
    }
}

macro_rules! impl_contained {
    ($($t:ty),+) => {
        $(impl<'a> Contained<&'a Position> for $t {
            #[inline]
            fn contained_in(self, pos: &Position) -> bool {
                !pos.board().bitboard(self).is_empty()
            }
        })+
    }
}

impl_contained! { Piece, PieceKind, Color }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_pieces() {
        let pos = Position::default();
        let all = pos.board().all_bits();

        for square in Square::ALL {
            if let Some(&piece) = pos.pieces().get(square) {
                assert!(all.contains(square));

                let board = pos.board();
                assert!(board.contains(square, piece));
                assert!(board.contains(square, piece.kind()));
                assert!(board.contains(square, piece.color()));
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
