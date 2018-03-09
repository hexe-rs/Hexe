//! A bitboard-segmented chess board representations.

use core::{ops, mem};

use board::Bitboard;
use castle::CastleRight;
use color::Color;
use piece::PieceKind;
use square::Square;

#[cfg(all(test, nightly))]
mod benches;

mod values {
    use super::*;

    const PAWN:   u64 = 0x00FF00000000FF00;
    const KNIGHT: u64 = squares!(B1, B8, G1, G8);
    const BISHOP: u64 = squares!(C1, C8, F1, F8);
    const ROOK:   u64 = squares!(A1, A8, H1, H8);
    const QUEEN:  u64 = squares!(D1, D8);
    const KING:   u64 = squares!(E1, E8);
    const WHITE:  u64 = 0x000000000000FFFF;
    const BLACK:  u64 = 0xFFFF000000000000;

    pub const STANDARD: MultiBoard = MultiBoard {
        pieces: [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING],
        colors: [WHITE, BLACK],
    };
}

const NUM_PIECES: usize = 6;
const NUM_COLORS: usize = 2;
const NUM_BOARDS: usize = NUM_PIECES + NUM_COLORS;

/// A full chess board, represented as multiple bitboard segments.
#[repr(C)]
#[derive(Clone)]
pub struct MultiBoard {
    pieces: [u64; NUM_PIECES],
    colors: [u64; NUM_COLORS],
}

impl Default for MultiBoard {
    #[inline]
    fn default() -> MultiBoard {
        unsafe { mem::zeroed() }
    }
}

impl AsRef<[u64]> for MultiBoard {
    #[inline]
    fn as_ref(&self) -> &[u64] {
        let array = self as *const _ as *const [_; NUM_BOARDS];
        unsafe { &*array }
    }
}

impl AsMut<[u64]> for MultiBoard {
    #[inline]
    fn as_mut(&mut self) -> &mut [u64] {
        let array = self as *mut _ as *mut [_; NUM_BOARDS];
        unsafe { &mut *array }
    }
}

impl AsRef<[Bitboard]> for MultiBoard {
    #[inline]
    fn as_ref(&self) -> &[Bitboard] {
        let array = self as *const _ as *const [_; NUM_BOARDS];
        unsafe { &*array }
    }
}

impl AsMut<[Bitboard]> for MultiBoard {
    #[inline]
    fn as_mut(&mut self) -> &mut [Bitboard] {
        let array = self as *mut _ as *mut [_; NUM_BOARDS];
        unsafe { &mut *array }
    }
}

impl ops::Index<PieceKind> for MultiBoard {
    type Output = Bitboard;

    #[inline]
    fn index(&self, kind: PieceKind) -> &Bitboard {
        Bitboard::convert_ref(&self.pieces[kind as usize])
    }
}

impl ops::IndexMut<PieceKind> for MultiBoard {
    #[inline]
    fn index_mut(&mut self, kind: PieceKind) -> &mut Bitboard {
        Bitboard::convert_mut(&mut self.pieces[kind as usize])
    }
}

impl ops::Index<Color> for MultiBoard {
    type Output = Bitboard;

    #[inline]
    fn index(&self, color: Color) -> &Bitboard {
        Bitboard::convert_ref(&self.colors[color as usize])
    }
}

impl ops::IndexMut<Color> for MultiBoard {
    #[inline]
    fn index_mut(&mut self, color: Color) -> &mut Bitboard {
        Bitboard::convert_mut(&mut self.colors[color as usize])
    }
}

impl MultiBoard {
    /// The board for standard chess.
    pub const STANDARD: MultiBoard = values::STANDARD;

    /// Clears the board of all pieces.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { ::util::zero(self) }
    }

    /// Removes the pieces at `squares`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::board::MultiBoard;
    /// use hexe_core::color::Color;
    /// use hexe_core::square::Square;
    ///
    /// let mut board = MultiBoard::STANDARD;
    /// let squares = [
    ///     Square::A1,
    ///     Square::C1,
    ///     Square::F2,
    /// ];
    ///
    /// board.remove_squares(squares.iter().cloned());
    ///
    /// for &square in squares.iter() {
    ///     assert!(!board[Color::White].contains(square))
    /// }
    /// ```
    #[inline]
    pub fn remove_squares<S>(&mut self, squares: S)
        where S: IntoIterator<Item=Square>
    {
        for s in squares {
            let square = !Bitboard::from(s).0;
            let boards: &mut [u64] = self.as_mut();
            for board in boards {
                *board &= square;
            }
        }
    }

    /// Returns references to the underlying bitboards for `Color` and
    /// `PieceKind`, respectively.
    #[inline]
    pub fn split(&self) -> (&[Bitboard; NUM_COLORS], &[Bitboard; NUM_PIECES]) {
        let colors = &self.colors as *const _ as *const _;
        let pieces = &self.pieces as *const _ as *const _;
        unsafe { (&*colors, &*pieces) }
    }

    /// Returns mutable references to the underlying bitboards for `Color` and
    /// `PieceKind`, respectively.
    #[inline]
    pub fn split_mut(&mut self) -> (&mut [Bitboard; NUM_COLORS], &mut [Bitboard; NUM_PIECES]) {
        let colors = &mut self.colors as *mut _ as *mut _;
        let pieces = &mut self.pieces as *mut _ as *mut _;
        unsafe { (&mut *colors, &mut *pieces) }
    }

    /// Performs a **blind** castle of the pieces for the castling right.
    ///
    /// # Invariants
    ///
    /// Under legal castling circumstances, this method makes it so that squares
    /// involved with castling using `right` are in a correct state post-castle.
    ///
    /// There are some cases where the board state may be invalidated if the
    /// above invariant isn't correctly met:
    ///
    /// - If he king is not in its initial position, then a king will spawn
    ///   both where it was expected to be, as well as where it would move to.
    ///   The same will happen when the rook is not at its corner of the board.
    ///
    /// - If another rook is located where the castling rook is being moved to
    ///   then both rooks will be removed.
    ///
    /// - If any other pieces are located at the involved squares, then other
    ///   strange things will happen.
    ///
    /// The above are all the result of properly defined behavior. They are just
    /// side effects of how the board is represented and this use of [XOR].
    ///
    /// # Undo-Redo
    ///
    /// Because this method internally uses [XOR], it is its own inverse. If the
    /// involved king and rook sit at their destination squares, they will be
    /// moved back to their initial squares.
    ///
    /// [XOR]: https://en.wikipedia.org/wiki/Exclusive_or
    #[inline]
    pub fn castle(&mut self, right: CastleRight) {
        // (King, Rook)
        static MASKS: [(u64, u64); 4] = [
            (squares!(E1, G1), squares!(H1, F1)),
            (squares!(E1, C1), squares!(A1, D1)),
            (squares!(E8, G8), squares!(H8, F8)),
            (squares!(E8, C8), squares!(A8, D8)),
        ];

        let (king, rook) = MASKS[right as usize];
        self[right.color()]   ^= king | rook;
        self[PieceKind::King] ^= king;
        self[PieceKind::Rook] ^= rook;
    }
}
