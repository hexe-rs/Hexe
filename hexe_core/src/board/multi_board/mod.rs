//! A bitboard-segmented chess board representations.

use core::{hash, ops, mem};

use board::Bitboard;
use castle::CastleRight;
use color::Color;
use piece::{Piece, PieceKind};
use uncon::*;

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
const NUM_BYTES:  usize = NUM_BOARDS * 8;

/// A full chess board, represented as multiple bitboard segments.
#[repr(C)]
#[derive(Clone, Eq)]
pub struct MultiBoard {
    pieces: [u64; NUM_PIECES],
    colors: [u64; NUM_COLORS],
}

impl PartialEq for MultiBoard {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        #[cfg(feature = "simd")]
        {
            use simd::u8x16;
            const NUM_SIMD: usize = 16;

            if self as *const _ == other as *const _ {
                return true;
            }

            let this = self.bytes();
            let that = other.bytes();

            for i in (0..(NUM_BYTES / NUM_SIMD)).map(|i| i * NUM_SIMD) {
                let a = u8x16::load(this, i);
                let b = u8x16::load(that, i);
                if !a.eq(b).all() {
                    return false;
                }
            }
            true
        }
        #[cfg(not(feature = "simd"))]
        {
            self.bytes()[..] == other.bytes()[..]
        }
    }
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

impl hash::Hash for MultiBoard {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(self.bytes());
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

    #[inline]
    fn bytes(&self) -> &[u8; NUM_BYTES] {
        unsafe { self.into_unchecked() }
    }

    /// Clears the board of all pieces.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { ::util::zero(self) }
    }

    /// Returns whether `self` is empty.
    ///
    /// For much better performance and readability, is recommended to use this
    /// method over checking whether `board.len() == 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::board::MultiBoard;
    ///
    /// assert!(!MultiBoard::STANDARD.is_empty());
    /// assert!(MultiBoard::default().is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.all_bits().is_empty()
    }

    /// Returns the total number of pieces in `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::board::MultiBoard;
    ///
    /// let board = MultiBoard::STANDARD;
    /// assert_eq!(board.len(), 32);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.all_bits().len()
    }

    /// Returns all bits of the pieces contained in `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::board::MultiBoard;
    ///
    /// let board = MultiBoard::STANDARD;
    /// let value = 0xFFFF00000000FFFFu64;
    ///
    /// assert_eq!(board.all_bits(), value.into());
    /// ```
    #[inline]
    pub fn all_bits(&self) -> Bitboard {
        Bitboard(self.colors[0] | self.colors[1])
    }

    /// Returns whether the `bits` of `value` are contained in `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::board::MultiBoard;
    /// use hexe_core::prelude::*;
    ///
    /// let board = MultiBoard::STANDARD;
    ///
    /// assert!(board.contains(Square::C7, Color::Black));
    /// assert!(board.contains(Square::H1, Piece::WhiteRook));
    /// assert!(board.contains(Square::B8, PieceKind::Knight));
    ///
    /// assert!(!board.contains(Square::C2, Color::Black));
    /// assert!(!board.contains(Square::H8, Piece::BlackPawn));
    /// assert!(!board.contains(Square::B1, PieceKind::Bishop));
    /// ```
    #[inline]
    pub fn contains<B, V: Index<B>>(&self, bits: B, value: V) -> bool {
        value.contained(bits, self)
    }

    /// Performs a **blind** insertion of `piece` at a each square in `bits`.
    /// It _does not_ check whether pieces are located at `bits`.
    ///
    /// If the board may contain pieces at `bits`, then
    /// [`remove_all`](#method.remove_all) should be called first.
    #[inline]
    pub fn insert<T: Into<Bitboard>>(&mut self, bits: T, piece: Piece) {
        let value = bits.into().0;
        self[piece.color()] |= value;
        self[piece.kind() ] |= value;
    }

    /// Removes each piece at `bits` for `value`.
    #[inline]
    pub fn remove<B, V: Index<B>>(&mut self, bits: B, value: V) {
        value.remove(bits, self)
    }

    /// Performs a **blind** removal of `value` at `bits`.
    ///
    /// It _does not_ check whether other pieces that `value` does not represent
    /// are located at `bits`.
    #[inline]
    pub fn remove_unchecked<B, V: Index<B>>(&mut self, bits: B, value: V) {
        value.remove_unchecked(bits, self)
    }

    /// Removes all pieces at `bits`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::board::MultiBoard;
    /// use hexe_core::prelude::*;
    ///
    /// let mut board = MultiBoard::STANDARD;
    /// let squares = [
    ///     Square::A1,
    ///     Square::C1,
    ///     Square::F2,
    /// ];
    ///
    /// for &square in squares.iter() {
    ///     assert!(board[Color::White].contains(square));
    ///     board.remove_all(square);
    ///     assert!(!board[Color::White].contains(square));
    /// }
    /// ```
    #[inline]
    pub fn remove_all<T: Into<Bitboard>>(&mut self, bits: T) {
        let value = !bits.into().0;
        for board in AsMut::<[u64]>::as_mut(self) {
            *board &= value;
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
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::board::MultiBoard;
    /// use hexe_core::prelude::*;
    ///
    /// let mut board: MultiBoard = {
    ///     /* create board */
    ///     # let mut board = MultiBoard::STANDARD;
    ///     # board.remove_all(Square::B1 | Square::C1 | Square::D1);
    ///     # board
    /// };
    ///
    /// board.castle(CastleRight::WhiteQueenside);
    /// board.contains(Square::C1, Piece::WhiteKing);
    /// board.contains(Square::D1, Piece::WhiteRook);
    /// ```
    ///
    /// ## Undo-Redo
    ///
    /// Because this method internally uses [XOR], it is its own inverse. If the
    /// involved king and rook sit at their destination squares, they will be
    /// moved back to their initial squares.
    ///
    /// ```
    /// use hexe_core::board::MultiBoard;
    /// use hexe_core::castle::CastleRight;
    ///
    /// let mut board: MultiBoard = {
    ///     /* create board */
    ///     # MultiBoard::STANDARD
    /// };
    ///
    /// let right = CastleRight::WhiteQueenside;
    /// let clone = board.clone();
    ///
    /// board.castle(right);
    /// board.castle(right);
    ///
    /// assert!(board == clone);
    /// ```
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

/// A type that can be used for [`MultiBoard`](struct.MultiBoard.html) indexing
/// operations.
pub trait Index<T> {
    /// Returns whether the `bits` of `self` are contained in `board`.
    fn contained(self, bits: T, board: &MultiBoard) -> bool;

    /// Removes the `bits` of `self` from `board`.
    fn remove(self, bits: T, board: &mut MultiBoard);

    /// Performs a **blind** removal of `self` at `bits` in `board`.
    fn remove_unchecked(self, bits: T, board: &mut MultiBoard);
}

impl<T: Into<Bitboard>> Index<T> for Color {
    #[inline]
    fn contained(self, bits: T, board: &MultiBoard) -> bool {
        board[self].contains(bits)
    }

    #[inline]
    fn remove(self, bits: T, board: &mut MultiBoard) {
        self.remove_unchecked(board[self].0 & bits.into().0, board);
    }

    #[inline]
    fn remove_unchecked(self, bits: T, board: &mut MultiBoard) {
        let value = !bits.into().0;
        board[self] &= value;
        for piece in &mut board.pieces {
            *piece &= value;
        }
    }
}

impl<T: Into<Bitboard>> Index<T> for Piece {
    #[inline]
    fn contained(self, bits: T, board: &MultiBoard) -> bool {
        let value = bits.into().0;
        board[self.color()].contains(value) &&
        board[self.kind() ].contains(value)
    }

    #[inline]
    fn remove(self, bits: T, board: &mut MultiBoard) {
        let value = board[self.color()].0 | board[self.kind()].0;
        self.remove_unchecked(value & bits.into().0, board);
    }

    #[inline]
    fn remove_unchecked(self, bits: T, board: &mut MultiBoard) {
        let value = !bits.into().0;
        board[self.color()] &= value;
        board[self.kind() ] &= value;
    }
}

impl<T: Into<Bitboard>> Index<T> for PieceKind {
    #[inline]
    fn contained(self, bits: T, board: &MultiBoard) -> bool {
        board[self].contains(bits)
    }

    #[inline]
    fn remove(self, bits: T, board: &mut MultiBoard) {
        self.remove_unchecked(board[self].0 & bits.into().0, board);
    }

    #[inline]
    fn remove_unchecked(self, bits: T, board: &mut MultiBoard) {
        let value = !bits.into().0;
        board[self] &= value;
        for color in &mut board.colors {
            *color &= value;
        }
    }
}
