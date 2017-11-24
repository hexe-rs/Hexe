//! A chess game state position.

use core::piece::map::PieceMap;
use core::misc::Contained;
use prelude::*;
use uncon::*;

mod state;
pub use self::state::*;

// The raw value used to represent no square for a space-optimized square.
//
// This should be unnecessary once `Option<Square>` is optimized to be a single
// byte as per https://github.com/rust-lang/rust/pull/45225.
const NO_SQUARE: u8 = 1 + Square::H8 as u8;

#[cfg(test)]
const_assert_eq!(no_sq; NO_SQUARE, 64);

/// A representation of the current game state.
#[derive(Clone)]
pub struct Position {
    /// The current state.
    state: State,

    /// A piece map board representation for fast lookups.
    piece_map: PieceMap,

    /// Bitboards for each piece kind. Uses `u64` for convenience.
    pieces: [u64; 6],

    /// Bitboards for each color. Uses `u64` for convenience.
    colors: [u64; 2],

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
        const PAWN:   u64 = 0x00FF00000000FF00;
        const KNIGHT: u64 = 0x4200000000000042;
        const BISHOP: u64 = 0x2400000000000024;
        const ROOK:   u64 = 0x8100000000000081;
        const QUEEN:  u64 = 0x0800000000000008;
        const KING:   u64 = 0x1000000000000010;
        const WHITE:  u64 = 0x000000000000FFFF;
        const BLACK:  u64 = 0xFFFF000000000000;

        Position {
            state: State::default(),
            piece_map: PieceMap::STANDARD,
            pieces: [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING],
            colors: [WHITE, BLACK],
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
        self.state.en_passant()
    }

    /// Returns the castle rights for both players.
    #[inline]
    pub fn castle_rights(&self) -> CastleRights {
        self.state.castle_rights()
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

/// A type whose instances serve to retrieve a `Bitboard` from a `Position`.
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
