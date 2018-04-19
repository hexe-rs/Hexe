//! A bitboard-segmented chess board representation.

use core::{hash, ops, mem};
#[cfg(feature = "simd")]
use core::simd::u8x64;

use uncon::*;

use prelude::*;
use board::PieceMap;
use castle;

#[cfg(all(test, nightly))]
mod benches;
#[cfg(test)]
mod tests;

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
        { self as *const _ == other as *const _ || self.simd() == other.simd() }

        #[cfg(not(feature = "simd"))]
        { self.bytes()[..] == other.bytes()[..] }
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

impl<'a> From<&'a PieceMap> for MultiBoard {
    #[inline]
    fn from(map: &PieceMap) -> MultiBoard {
        let mut board = MultiBoard::default();
        for (square, &piece) in map {
            board.insert_unchecked(square, piece);
        }
        board
    }
}

impl hash::Hash for MultiBoard {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(self.bytes());
    }
}

impl ops::Index<Role> for MultiBoard {
    type Output = Bitboard;

    #[inline]
    fn index(&self, role: Role) -> &Bitboard {
        role.extract(&self.pieces).as_ref()
    }
}

impl ops::IndexMut<Role> for MultiBoard {
    #[inline]
    fn index_mut(&mut self, role: Role) -> &mut Bitboard {
        role.extract_mut(&mut self.pieces).as_mut()
    }
}

impl ops::Index<Color> for MultiBoard {
    type Output = Bitboard;

    #[inline]
    fn index(&self, color: Color) -> &Bitboard {
        color.extract(&self.colors).as_ref()
    }
}

impl ops::IndexMut<Color> for MultiBoard {
    #[inline]
    fn index_mut(&mut self, color: Color) -> &mut Bitboard {
        color.extract_mut(&mut self.colors).as_mut()
    }
}

impl MultiBoard {
    /// The board for standard chess.
    pub const STANDARD: MultiBoard = values::STANDARD;

    #[cfg(feature = "simd")]
    #[inline]
    fn simd(&self) -> u8x64 {
        u8x64::load_unaligned(self.bytes())
    }

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
        let [w, b] = self.colors;
        Bitboard(w | b)
    }

    /// Returns the `Bitboard` for `value` in `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::board::MultiBoard;
    /// use hexe_core::prelude::*;
    ///
    /// let board   = MultiBoard::STANDARD;
    /// let king_sq = board.bitboard(Piece::WhiteKing).lsb();
    ///
    /// assert_eq!(king_sq, Some(Square::E1));
    /// ```
    #[inline]
    pub fn bitboard<T: Index>(&self, value: T) -> Bitboard {
        value.bitboard(self)
    }

    /// Returns the bits of the royal pieces, King and Queen.
    #[inline]
    pub fn royals(&self) -> Bitboard {
        self.bitboard(Role::Queen) | self.bitboard(Role::King)
    }

    /// Returns the first square that `value` appears at, if any.
    #[inline]
    pub fn first<T: Index>(&self, value: T) -> Option<Square> {
        self.bitboard(value).lsb()
    }

    /// Returns the first square that `value` may appear at, without checking
    /// whether it exists in `self`.
    #[inline]
    pub unsafe fn first_unchecked<T: Index>(&self, value: T) -> Square {
        self.bitboard(value).lsb_unchecked()
    }

    /// Returns the last square that `value` appears at, if any.
    #[inline]
    pub fn last<T: Index>(&self, value: T) -> Option<Square> {
        self.bitboard(value).msb()
    }

    /// Returns the last square that `value` may appear at, without checking
    /// whether it exists in `self`.
    #[inline]
    pub unsafe fn last_unchecked<T: Index>(&self, value: T) -> Square {
        self.bitboard(value).msb_unchecked()
    }

    /// Returns the total number of `value` in `self`.
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
    /// assert_eq!(board.count(Color::Black), 16);
    /// assert_eq!(board.count(Piece::WhiteRook), 2);
    /// assert_eq!(board.count(Role::Queen), 2);
    /// ```
    #[inline]
    pub fn count<T: Index>(&self, value: T) -> usize {
        self.bitboard(value).len()
    }

    /// Returns whether `value` is contained at all squares in `bits`.
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
    /// assert!(board.contains(Square::A2, Color::White));
    /// assert!(board.contains(Square::C8, Role::Bishop));
    /// assert!(board.contains(Rank::Seven, Piece::BlackPawn));
    /// ```
    #[inline]
    pub fn contains<T, U>(&self, bits: T, value: U) -> bool
        where T: Into<Bitboard>, U: Index
    {
        self.bitboard(value).contains(bits)
    }

    /// Returns whether `value` is contained at any square in `bits`.
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
    /// assert!(board.contains_any(File::B, Role::Knight));
    /// assert!(board.contains_any(Rank::One, Role::King));
    /// ```
    #[inline]
    pub fn contains_any<T, U>(&self, bits: T, value: U) -> bool
        where T: Into<Bitboard>, U: Index
    {
        !(self.bitboard(value) & bits).is_empty()
    }

    /// Inserts `piece` at each square in `bits`, removing any other pieces
    /// that may be at `bits`.
    #[inline]
    pub fn insert<T: Into<Bitboard>>(&mut self, bits: T, piece: Piece) {
        let value = bits.into();
        self.remove_all(value);
        self.insert_unchecked(value, piece);
    }

    /// Performs a **blind** insertion of `piece` at a each square in `bits`.
    ///
    /// It _does not_ check whether other pieces are located at `bits`. If the
    /// board may contain pieces at `bits`, then [`insert`](#method.insert)
    /// should be called instead.
    #[inline]
    pub fn insert_unchecked<T: Into<Bitboard>>(&mut self, bits: T, piece: Piece) {
        let value = bits.into();
        self[piece.color()] |= value;
        self[piece.role() ] |= value;
    }

    /// Removes each piece at `bits` for `value`.
    #[inline]
    pub fn remove<T, U>(&mut self, bits: T, value: U)
        where T: Into<Bitboard>, U: Index
    {
        value.remove(bits, self);
    }

    /// Performs a **blind** removal of `value` at `bits`.
    ///
    /// It _does not_ check whether other pieces that `value` does not represent
    /// are located at `bits`.
    #[inline]
    pub fn remove_unchecked<T, U>(&mut self, bits: T, value: U)
        where T: Into<Bitboard>, U: Index
    {
        value.remove_unchecked(bits, self);
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
    /// `Role`, respectively.
    #[inline]
    pub fn split(&self) -> (&[Bitboard; NUM_COLORS], &[Bitboard; NUM_PIECES]) {
        let colors = &self.colors as *const _ as *const _;
        let pieces = &self.pieces as *const _ as *const _;
        unsafe { (&*colors, &*pieces) }
    }

    /// Returns mutable references to the underlying bitboards for `Color` and
    /// `Role`, respectively.
    #[inline]
    pub fn split_mut(&mut self) -> (&mut [Bitboard; NUM_COLORS], &mut [Bitboard; NUM_PIECES]) {
        let colors = &mut self.colors as *mut _ as *mut _;
        let pieces = &mut self.pieces as *mut _ as *mut _;
        unsafe { (&mut *colors, &mut *pieces) }
    }

    /// Returns whether the square for `player` is being attacked.
    ///
    /// This method _does not_ check whether a piece for `player` actually
    /// exists at `sq`. To check for that, call `board.contains(sq, player)`.
    pub fn is_attacked(&self, sq: Square, player: Color) -> bool {
        macro_rules! check {
            ($e:expr) => { if $e { return true } };
        }

        let opp = self.bitboard(!player);
        let all = opp | self.bitboard(player);

        let pawns = opp & self.bitboard(Role::Pawn);
        check!(pawns.intersects(sq.pawn_attacks(player)));

        let knights = opp & self.bitboard(Role::Knight);
        check!(knights.intersects(sq.knight_attacks()));

        let kings = opp & (self.bitboard(Role::King));
        check!(kings.intersects(sq.king_attacks()));

        let queens = self.bitboard(Role::Queen);

        let bishops = opp & (self.bitboard(Role::Bishop) | queens);
        check!(bishops.intersects(sq.bishop_attacks(all)));

        let rooks = opp & (self.bitboard(Role::Rook) | queens);
        rooks.intersects(sq.rook_attacks(all))
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
    /// - If the king is not in its initial position, then a king will spawn
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
    /// board.castle(Right::WhiteQueen);
    /// assert!(board.contains(Square::C1, Piece::WhiteKing));
    /// assert!(board.contains(Square::D1, Piece::WhiteRook));
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
    /// use hexe_core::castle::Right;
    ///
    /// let mut board: MultiBoard = {
    ///     /* create board */
    ///     # MultiBoard::STANDARD
    /// };
    ///
    /// let right = Right::WhiteQueen;
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
    pub fn castle(&mut self, right: Right) {
        let &(king, rook) = right.extract(&castle::TABLES.mb_masks);
        self[right.color()] ^= king | rook;
        self[Role::King] ^= king;
        self[Role::Rook] ^= rook;
    }
}

/// A type that can be used for [`MultiBoard`](struct.MultiBoard.html) indexing
/// operations.
pub trait Index {
    /// Returns the bitboard for `self` in `board`.
    fn bitboard(self, board: &MultiBoard) -> Bitboard;

    /// Removes the `bits` of `self` from `board`.
    fn remove<T: Into<Bitboard>>(self, bits: T, board: &mut MultiBoard);

    /// Performs a **blind** removal of `self` at `bits` in `board`.
    fn remove_unchecked<T: Into<Bitboard>>(self, bits: T, board: &mut MultiBoard);
}

impl Index for Color {
    #[inline]
    fn bitboard(self, board: &MultiBoard) -> Bitboard {
        board[self]
    }

    #[inline]
    fn remove<T: Into<Bitboard>>(self, bits: T, board: &mut MultiBoard) {
        self.remove_unchecked(board[self] & bits.into(), board);
    }

    #[inline]
    fn remove_unchecked<T: Into<Bitboard>>(self, bits: T, board: &mut MultiBoard) {
        let value = !bits.into().0;
        board[self] &= value;
        for piece in &mut board.pieces {
            *piece &= value;
        }
    }
}

impl Index for Piece {
    #[inline]
    fn bitboard(self, board: &MultiBoard) -> Bitboard {
        self.color().bitboard(board) & self.role().bitboard(board)
    }

    #[inline]
    fn remove<T: Into<Bitboard>>(self, bits: T, board: &mut MultiBoard) {
        let value = board[self.color()] | board[self.role()];
        self.remove_unchecked(value & bits.into(), board);
    }

    #[inline]
    fn remove_unchecked<T: Into<Bitboard>>(self, bits: T, board: &mut MultiBoard) {
        let value = !bits.into().0;
        board[self.color()] &= value;
        board[self.role() ] &= value;
    }
}

impl Index for Role {
    #[inline]
    fn bitboard(self, board: &MultiBoard) -> Bitboard {
        board[self]
    }

    #[inline]
    fn remove<T: Into<Bitboard>>(self, bits: T, board: &mut MultiBoard) {
        self.remove_unchecked(board[self] & bits.into(), board);
    }

    #[inline]
    fn remove_unchecked<T: Into<Bitboard>>(self, bits: T, board: &mut MultiBoard) {
        let value = !bits.into().0;
        board[self] &= value;
        for color in &mut board.colors {
            *color &= value;
        }
    }
}
