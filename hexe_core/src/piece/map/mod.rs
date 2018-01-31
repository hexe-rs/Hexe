//! A square to piece mapping for fast square lookups.

use super::*;
use core::{fmt, mem, ops, ptr};
use consts::PTR_SIZE;
use misc::Contained;
use prelude::*;
use util::*;

#[cfg(feature = "simd")]
use simd::u8x16;

const NONE: u8 = 12;

const SQUARE_NUM: usize = 64;

mod tables {
    use super::*;

    macro_rules! def_pieces {
        ($($n:ident => $p:ident),+ $(,)*) => {
            $(const $n: u8 = Piece::$p as u8;)+
        }
    }

    def_pieces! {
        WP => WhitePawn,   BP => BlackPawn,
        WN => WhiteKnight, BN => BlackKnight,
        WB => WhiteBishop, BB => BlackBishop,
        WR => WhiteRook,   BR => BlackRook,
        WK => WhiteKing,   BK => BlackKing,
        WQ => WhiteQueen,  BQ => BlackQueen,
    }

    /// The piece map for standard chess.
    pub const STANDARD: [u8; SQUARE_NUM] = [
        WR,   WN,   WB,   WQ,   WK,   WB,   WN,   WR,
        WP,   WP,   WP,   WP,   WP,   WP,   WP,   WP,
        NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
        NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
        NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
        NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
        BP,   BP,   BP,   BP,   BP,   BP,   BP,   BP,
        BR,   BN,   BB,   BQ,   BK,   BB,   BN,   BR,
    ];
}

mod entry;
pub use self::entry::*;

mod iter;
pub use self::iter::*;

/// A mapping of sixty-four squares to pieces.
///
/// This allows for faster lookups than possible with bitboards.
///
/// **Note:** `PieceMap::default()` returns an empty piece map. Use
/// [`PieceMap::STANDARD`](#associatedconstant.STANDARD) to get a mapping for
/// standard chess.
pub struct PieceMap([u8; SQUARE_NUM]);

impl Clone for PieceMap {
    #[inline]
    fn clone(&self) -> PieceMap { PieceMap(self.0) }
}

impl PartialEq for PieceMap {
    #[inline]
    fn eq(&self, other: &PieceMap) -> bool {
        #[cfg(feature = "simd")]
        {
            if self as *const _ == other as *const _ {
                return true;
            }
            for i in (0..4).map(|i| i * 16) {
                let this = u8x16::load(&self.0, i);
                let that = u8x16::load(&other.0, i);
                if !this.eq(that).all() {
                    return false;
                }
            }
            true
        }
        #[cfg(not(feature = "simd"))]
        {
            self.0[..] == other.0[..]
        }
    }
}

impl Eq for PieceMap {}

impl Default for PieceMap {
    #[inline]
    fn default() -> PieceMap {
        PieceMap::EMPTY
    }
}

static INDEX_ERR: &'static str = "no piece found for square";

impl ops::Index<Square> for PieceMap {
    type Output = Piece;

    #[inline]
    fn index(&self, sq: Square) -> &Piece {
        self.get(sq).expect(INDEX_ERR)
    }
}

impl ops::IndexMut<Square> for PieceMap {
    #[inline]
    fn index_mut(&mut self, sq: Square) -> &mut Piece {
        self.get_mut(sq).expect(INDEX_ERR)
    }
}

impl fmt::Debug for PieceMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl fmt::Display for PieceMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.map_str(|s| s.fmt(f))
    }
}

impl ::core::iter::FromIterator<(Square, Piece)> for PieceMap {
    #[inline]
    fn from_iter<T: IntoIterator<Item=(Square, Piece)>>(iter: T) -> PieceMap {
        let mut map = PieceMap::new();
        map.extend(iter);
        map
    }
}

impl Extend<(Square, Piece)> for PieceMap {
    #[inline]
    fn extend<T: IntoIterator<Item=(Square, Piece)>>(&mut self, iter: T) {
        for (s, p) in iter.into_iter() {
            self.insert(s, p);
        }
    }
}

impl PieceMap {
    /// An empty piece map.
    pub const EMPTY: PieceMap = PieceMap([NONE; SQUARE_NUM]);

    /// The piece map for standard chess.
    pub const STANDARD: PieceMap = PieceMap(tables::STANDARD);

    /// Creates an empty piece map.
    #[inline]
    pub fn new() -> PieceMap {
        PieceMap::default()
    }

    /// Attempts to create a piece map from the fen string.
    pub fn from_fen(fen: &str) -> Option<PieceMap> {
        let mut map = PieceMap::EMPTY;
        let bytes = fen.as_bytes();

        let mut rank: usize = 7;
        let mut file: usize = 0;

        for &byte in bytes {
            match byte {
                b'/' => {
                    if file != 8 || rank == 0 {
                        return None;
                    }
                    file = 0;
                    rank -= 1;
                },
                b'1'...b'8' => {
                    file += (byte - b'0') as usize;
                    if file > 8 {
                        return None;
                    }
                },
                _ => if let Some(pc) = Piece::from_char(byte as char) {
                    let sq = Square::new(File::from(file),
                                         Rank::from(rank));
                    map.insert(sq, pc);
                    file += 1;
                } else {
                    return None;
                },
            }
        }

        if rank == 0 && file == 8 {
            Some(map)
        } else {
            None
        }
    }

    /// Creates a map with _all_ squares populated by `piece`.
    #[inline]
    pub fn filled(piece: Piece) -> PieceMap {
        PieceMap([piece as u8; SQUARE_NUM])
    }

    /// Creates a new piece map by instantiating each slot with the provided
    /// initializer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::piece::map::*;
    /// let piece_map = PieceMap::from_init(|sq| {
    ///     # None
    ///     /* ... */
    /// });
    /// ```
    #[inline]
    pub fn from_init<F>(mut init: F) -> PieceMap
        where F: FnMut(Square) -> Option<Piece>
    {
        let mut map: PieceMap = unsafe { mem::uninitialized() };
        for i in 0..SQUARE_NUM {
            let val = init(i.into()).map(|p| p as u8).unwrap_or(NONE);
            unsafe { ptr::write(&mut map.0[i], val) };
        }
        map
    }

    /// Reverses the square mapping.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::piece::map::*;
    /// # use hexe_core::prelude::*;
    /// let mut map = PieceMap::new();
    /// let piece = Piece::WhitePawn;
    ///
    /// map.insert(Square::A1, piece);
    /// map.reverse();
    ///
    /// assert_eq!(map[Square::H8], piece);
    /// ```
    #[inline]
    pub fn reverse(&mut self) {
        self.0.reverse()
    }

    /// Shuffles the map in-place.
    #[cfg(any(test, feature = "rand"))]
    pub fn shuffle<R: ::rand::Rng>(&mut self, rng: &mut R) {
        rng.shuffle(&mut self.0)
    }

    #[inline]
    #[cfg_attr(feature = "simd", allow(dead_code))]
    fn inner_ptr_sized(&self) -> &[usize; SQUARE_NUM / PTR_SIZE] {
        unsafe { (&self.0).into_unchecked() }
    }

    #[inline]
    fn inner_u64(&self) -> &[u64; 8] {
        unsafe { (&self.0).into_unchecked() }
    }

    #[inline]
    fn inner_2d_mut(&mut self) -> &mut [[u8; 8]; 8] {
        unsafe { (&mut self.0).into_unchecked() }
    }

    /// Mirrors the map across the horizontal axis of a chess board.
    pub fn mirror_horizontal(&mut self) {
        self.inner_2d_mut().reverse();
    }

    /// Mirrors the map across the vertical axis of a chess board.
    pub fn mirror_vertical(&mut self) {
        self.reverse();
        self.mirror_horizontal();
    }

    /// Returns the first square and piece pair in the map.
    #[inline]
    pub fn first(&self) -> Option<(Square, &Piece)> {
        self.iter().next()
    }

    /// Returns the first square and mutable piece pair in the map.
    #[inline]
    pub fn first_mut(&mut self) -> Option<(Square, &mut Piece)> {
        self.iter_mut().next()
    }

    /// Returns the last square and piece pair in the map.
    #[inline]
    pub fn last(&self) -> Option<(Square, &Piece)> {
        self.iter().next_back()
    }

    /// Returns the last square and mutable piece pair in the map.
    #[inline]
    pub fn last_mut(&mut self) -> Option<(Square, &mut Piece)> {
        self.iter_mut().next_back()
    }

    /// Efficiently fills the rank entirely with the given piece.
    #[inline]
    pub fn fill_rank(&mut self, r: Rank, pc: Piece) {
        self.inner_2d_mut()[r as usize] = [pc as u8; 8];
    }

    /// Performs a raw replacement.
    #[inline]
    unsafe fn replace(&mut self, sq: Square, pc: u8) -> Option<Piece> {
        match mem::replace(&mut self.0[sq as usize], pc) {
            NONE => None,
            p => Some(p.into_unchecked())
        }
    }

    /// Inserts the piece at a square, returning the previous one if any.
    #[inline]
    pub fn insert(&mut self, sq: Square, pc: Piece) -> Option<Piece> {
        unsafe { self.replace(sq, pc as u8) }
    }

    /// Removes the piece at a square.
    #[inline]
    pub fn remove(&mut self, sq: Square) -> Option<Piece> {
        unsafe { self.replace(sq, NONE) }
    }

    /// Swaps two values in the map.
    #[inline]
    pub fn swap<T: Swap>(&mut self, i: T, j: T) {
        T::swap(i, j, self);
    }

    /// Takes the piece at the square and moves it.
    #[inline]
    pub fn relocate(&mut self, from: Square, to: Square) {
        self.0[to as usize] = mem::replace(&mut self.0[from as usize], NONE);
    }

    /// Performs a capture of the piece at `to` via the piece at `from`.
    ///
    /// If the squares are the same, then this will simply perform a removal.
    #[inline]
    pub fn capture(&mut self, from: Square, to: Square) -> Option<Piece> {
        let pc = self.remove(to);
        self.swap(from, to);
        pc
    }

    /// Performs an en passant capture of the piece on the same file as `to` and
    /// the same rank as `from`, via the piece at `from`.
    ///
    /// There are no checks made regarding whether `from` and `to` are legal en
    /// passant squares. The capture is performed with no assumptions. This also
    /// does not check whether the destination square contains a piece. If it
    /// does, it will be replaced by whichever value is at `from`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::piece::map::*;
    /// # use hexe_core::prelude::Square::*;
    /// # use hexe_core::prelude::Piece::*;
    /// let mut map = PieceMap::STANDARD;
    /// map.relocate(D2, D5);
    /// map.relocate(C7, C5);
    ///
    /// assert_eq!(map[D5], WhitePawn);
    /// assert_eq!(map[C5], BlackPawn);
    ///
    /// let pc = map.en_passant(D5, C6);
    /// assert_eq!(pc, Some(BlackPawn));
    /// assert_eq!(map.get(C5), None);
    /// ```
    #[inline]
    pub fn en_passant(&mut self, from: Square, to: Square) -> Option<Piece> {
        let ep = to.combine(from);
        let pc = self.remove(ep);
        self.relocate(from, to);
        pc
    }

    /// Performs a **blind** castle of the pieces for the castling right.
    #[inline]
    pub fn castle(&mut self, castling: CastleRight) {
        static SQUARES: [[(Square, Square); 2]; 4] = [
            // King, Rook
            [(Square::E1, Square::G1), (Square::H1, Square::F1)],
            [(Square::E8, Square::G8), (Square::H8, Square::F8)],
            [(Square::E1, Square::C1), (Square::A1, Square::D1)],
            [(Square::E8, Square::C8), (Square::A8, Square::D8)],
        ];
        let squares = &SQUARES[castling as usize];
        let (k1, k2) = squares[0];
        let (r1, r2) = squares[1];
        self.relocate(k1, k2);
        self.relocate(r1, r2);
    }

    /// Inserts all pieces for which the function returns `Some`.
    #[inline]
    pub fn extend_from<F>(&mut self, mut f: F)
        where F: FnMut(Square) -> Option<Piece>
    {
        self.extend(Square::ALL.filter_map(|s| f(s).map(|p| (s, p))));
    }

    /// Retains only the elements specified by the predicate.
    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
        where F: FnMut(Square, &mut Piece) -> bool
    {
        for (i, slot) in self.0.iter_mut().enumerate() {
            if *slot != NONE && !f(i.into(), unsafe { slot.into_unchecked() }) {
                *slot = NONE;
            }
        }
    }

    /// Clears the map, removing all pieces.
    #[inline]
    pub fn clear(&mut self) {
        self.0 = [NONE; SQUARE_NUM];
    }

    /// Efficiently removes all pieces from the given rank.
    #[inline]
    pub fn clear_rank(&mut self, rank: Rank) {
        self.inner_2d_mut()[rank as usize] = [NONE; 8];
    }

    /// Returns whether `self` is empty.
    ///
    /// For much better performance and readability, is recommended to use this
    /// method over checking whether `self.len() == 0`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        #[cfg(feature = "simd")]
        {
            let empty = u8x16::splat(NONE);
            for i in 0..4 {
                let vec = u8x16::load(&self.0, i * 16);
                if !vec.eq(empty).all() {
                    return false;
                }
            }
        }
        #[cfg(not(feature = "simd"))]
        {
            let empty = usize::splat(NONE);
            for &slot in self.inner_ptr_sized() {
                if slot != empty {
                    return false;
                }
            }
        }
        true
    }

    /// Returns the total number of pieces in `self`.
    ///
    /// This operation is performed in O(n) time. It is recommended to store
    /// the result if it is used repeatedly.
    #[inline]
    pub fn len(&self) -> usize {
        SQUARE_NUM - self.0.count_of(NONE)
    }

    /// Returns the number of occurrences of `piece` in `self`.
    ///
    /// This operation is performed in O(n) time. It is recommended to store
    /// the result if it is used repeatedly.
    #[inline]
    pub fn count_of(&self, piece: Piece) -> usize {
        self.0.count_of(piece as u8)
    }

    /// Returns whether the map contains the value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::piece::map::*;
    /// # use hexe_core::prelude::*;
    /// let sq = Square::B7;
    /// let pc = Piece::WhiteRook;
    ///
    /// let mut map = PieceMap::new();
    /// map.insert(sq, pc);
    ///
    /// assert!(map.contains(sq));
    /// assert!(map.contains(pc));
    /// ```
    #[inline]
    #[allow(needless_lifetimes)]
    pub fn contains<'a, T: Contained<&'a Self>>(&'a self, value: T) -> bool {
        value.contained_in(self)
    }

    /// Returns whether the rank contains the piece.
    #[inline]
    pub fn rank_contains(&self, rank: Rank, pc: Piece) -> bool {
        self.inner_u64()[rank as usize].contains_byte(pc as u8)
    }

    /// Returns the first square for the piece.
    #[inline]
    pub fn find(&self, pc: Piece) -> Option<Square> {
        if let Some(index) = ::memchr::memchr(pc as u8, &self.0) {
            unsafe { Some(index.into_unchecked()) }
        } else {
            None
        }
    }

    /// Returns the last square for the piece.
    #[inline]
    pub fn rfind(&self, pc: Piece) -> Option<Square> {
        if let Some(index) = ::memchr::memrchr(pc as u8, &self.0) {
            unsafe { Some(index.into_unchecked()) }
        } else {
            None
        }
    }

    /// Gets the given square's corresponding entry in the map for in-place
    /// manipulation.
    #[inline]
    pub fn entry(&mut self, sq: Square) -> Entry {
        Entry::from_map(self, sq)
    }

    /// Returns a reference to the piece at a square, if any.
    #[inline]
    pub fn get(&self, sq: Square) -> Option<&Piece> {
        match self.0[sq as usize] {
            NONE => None,
            ref p => unsafe { Some(p.into_unchecked()) }
        }
    }

    /// Returns a mutable reference to the piece at a square, if any.
    #[inline]
    pub fn get_mut(&mut self, sq: Square) -> Option<&mut Piece> {
        match self.0[sq as usize] {
            NONE => None,
            ref mut p => unsafe { Some(p.into_unchecked()) }
        }
    }

    /// Returns a reference to the piece at a square without checking.
    ///
    /// # Safety
    ///
    /// Calling this method when there's no piece at the given square will
    /// produce [undefined behavior][ub]. Use with caution.
    ///
    /// [ub]: https://en.wikipedia.org/wiki/Undefined_behavior
    #[inline]
    pub unsafe fn get_unchecked(&self, sq: Square) -> &Piece {
        (&self.0[sq as usize]).into_unchecked()
    }

    /// Returns a mutable reference to the piece at a square without checking.
    ///
    /// # Safety
    ///
    /// Calling this method when there's no piece at the given square will
    /// produce [undefined behavior][ub]. Use with caution.
    ///
    /// [ub]: https://en.wikipedia.org/wiki/Undefined_behavior
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, sq: Square) -> &mut Piece {
        (&mut self.0[sq as usize]).into_unchecked()
    }

    /// Returns the color of the piece at the given square, if any.
    #[inline]
    pub fn color_at(&self, sq: Square) -> Option<Color> {
        self.get(sq).map(Piece::color)
    }

    /// Returns the kind of the piece at the given square, if any.
    #[inline]
    pub fn kind_at(&self, sq: Square) -> Option<PieceKind> {
        self.get(sq).map(Piece::kind)
    }

    /// Returns the result of applying a function to a mutable string
    /// representation of `self`.
    ///
    /// This method has the same benefits as [`Square::map_str`]
    ///
    /// # Examples
    ///
    /// **Note:** `PieceMap` implements [`Display`], thus it can be printed
    /// directly without using this method.
    ///
    /// ```
    /// # use hexe_core::piece::map::*;
    /// let map = PieceMap::STANDARD;
    /// let exp = "r n b q k b n r\n\
    ///            p p p p p p p p\n\
    ///            . . . . . . . .\n\
    ///            . . . . . . . .\n\
    ///            . . . . . . . .\n\
    ///            . . . . . . . .\n\
    ///            P P P P P P P P\n\
    ///            R N B Q K B N R";
    ///
    /// map.map_str(|s| assert_eq!(s, exp));
    /// ```
    ///
    /// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
    /// [`Square::map_str`]: ../../square/enum.Square.html#method.map_str
    #[inline]
    pub fn map_str<T, F: FnOnce(&mut str) -> T>(&self, f: F) -> T {
        let mut buf = *::consts::BOARD_DOTS;
        for square in Square::ALL {
            if let Some(&piece) = self.get(square) {
                let ch = char::from(piece) as u8;
                let idx = (0b111000 ^ square as usize) << 1;
                unsafe { *buf.get_unchecked_mut(idx) = ch };
            }
        }
        unsafe { f(str::from_utf8_unchecked_mut(&mut buf)) }
    }

    /// Returns the result of applying a function to a [FEN] string
    /// representation of `self`.
    ///
    /// [FEN]: https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    #[inline]
    pub fn map_fen<T, F: FnOnce(&mut str) -> T>(&self, f: F) -> T {
        const NUM: usize = 8;
        const MAX: usize = NUM * NUM + 7;
        let mut len: usize = 0;

        unsafe {
            let mut buf: [u8; MAX] = mem::uninitialized();
            macro_rules! write_buf {
                ($val:expr) => {
                    ptr::write(buf.get_unchecked_mut(len), $val);
                    len += 1;
                }
            }

            for rank in (0..NUM).rev().map(Rank::from) {
                let mut n: u8 = 0;
                for file in (0..NUM).map(File::from) {
                    let square = Square::new(file, rank);
                    if let Some(&pc) = self.get(square) {
                        if n != 0 {
                            write_buf!(b'0' + n);
                            n = 0;
                        }
                        write_buf!(char::from(pc) as u8);
                    } else {
                        n += 1;
                    }
                }
                if n != 0 {
                    write_buf!(b'0' + n);
                }
                if rank != Rank::One {
                    write_buf!(b'/');
                }
            }

            f(str::from_utf8_unchecked_mut(buf.get_unchecked_mut(..len)))
        }
    }

    /// Returns an owned [FEN] string representation of `self`.
    ///
    /// [FEN]: https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    #[cfg(feature = "std")]
    pub fn to_fen(&self) -> String {
        self.map_fen(|s| String::from(s as &str))
    }

    /// Returns an iterator visiting all square-piece pairs in order.
    #[inline]
    pub fn iter(&self) -> Iter { self.into_iter() }

    /// Returns an iterator visiting all square-piece pairs mutably in order.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut { self.into_iter() }

    /// Returns a view into the bytes of the map.
    ///
    /// # Values
    ///
    /// - Bytes with values less than 12 refer to a valid [`Piece`] instance.
    /// - Empty slots have a value of 12.
    ///
    /// You may safely assume that that no values greater than 12 exist.
    ///
    /// [`Piece`]: ../enum.Piece.html
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }

    /// Returns a mutable view into the bytes of the map.
    ///
    /// For more information, see [`as_bytes`](#method.as_bytes).
    ///
    /// # Safety
    ///
    /// Internal operations rely on certain assumptions about the contents of
    /// this buffer. Mutating these bytes such that piece values become invalid
    /// will cause [undefined behavior][ub].
    ///
    /// [ub]: https://en.wikipedia.org/wiki/Undefined_behavior
    #[inline]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8; 64] {
        &mut self.0
    }
}

impl<'a> Contained<&'a PieceMap> for Square {
    #[inline]
    fn contained_in(self, map: &PieceMap) -> bool {
        map.0[self as usize] != NONE
    }
}

impl<'a> Contained<&'a PieceMap> for Piece {
    #[inline]
    fn contained_in(self, map: &PieceMap) -> bool {
        #[cfg(feature = "simd")]
        {
            let pieces = u8x16::splat(self as u8);

            for i in 0..4 {
                let vec = u8x16::load(&map.0, i * 16);
                if vec.eq(pieces).any() {
                    return true;
                }
            }

            false
        }
        #[cfg(not(feature = "simd"))]
        {
            map.find(self).is_some()
        }
    }
}

/// A type whose instances may be used to swap values in a
/// [`PieceMap`](struct.PieceMap.html).
pub trait Swap {
    /// Swaps the values at `i` and `j` in `map`.
    fn swap(i: Self, j: Self, map: &mut PieceMap);
}

impl Swap for Square {
    #[inline]
    fn swap(i: Square, j: Square, map: &mut PieceMap) {
        map.0.swap(i as usize, j as usize);
    }
}

impl Swap for Rank {
    #[inline]
    fn swap(i: Rank, j: Rank, map: &mut PieceMap) {
        map.inner_2d_mut().swap(i as usize, j as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Asserts at compile-time that the piece is less than NONE.
    macro_rules! assert_valid_none {
        ($($p:ident)+) => {
            const_assert!(valid_none; $((Piece::$p as u8) < NONE),+);
        }
    }

    assert_valid_none! {
        WhitePawn   BlackPawn
        WhiteKnight BlackKnight
        WhiteBishop BlackBishop
        WhiteRook   BlackRook
        WhiteQueen  BlackQueen
        WhiteKing   BlackKing
    }

    #[test]
    fn len() {
        let mut map = PieceMap::new();

        macro_rules! assert_len {
            ($l:expr) => {
                assert_eq!(map.len(), $l);
                assert_eq!(map.iter().len(), $l);
                assert_eq!(map.iter_mut().len(), $l);
            }
        }

        assert_len!(0);

        map.insert(Square::A1, Piece::WhitePawn);
        assert_len!(1);

        map = PieceMap::STANDARD;
        assert_len!(32);

        map = PieceMap::filled(Piece::BlackBishop);
        assert_len!(64);

        let mut iter = map.iter();
        for _ in iter.by_ref().take(16) {}

        assert_eq!(iter.len(), 48);
    }

    #[test]
    fn rank_contains() {
        let map = PieceMap::STANDARD;

        let pairs = [
            // White
            (Rank::Two, Piece::WhitePawn),
            (Rank::One, Piece::WhiteKnight),
            (Rank::One, Piece::WhiteBishop),
            (Rank::One, Piece::WhiteRook),
            (Rank::One, Piece::WhiteQueen),
            (Rank::One, Piece::WhiteKing),
            // Black
            (Rank::Seven, Piece::BlackPawn),
            (Rank::Eight, Piece::BlackKnight),
            (Rank::Eight, Piece::BlackBishop),
            (Rank::Eight, Piece::BlackRook),
            (Rank::Eight, Piece::BlackQueen),
            (Rank::Eight, Piece::BlackKing),
        ];

        for &(rank, piece) in &pairs {
            assert!(
                map.rank_contains(rank, piece),
                "Rank::{:?} does not contain {:?} in\n{}",
                rank, piece, map
            );
            for rank in (0..8u8).map(Rank::from).filter(|&r| r != rank) {
                assert!(!map.rank_contains(rank, piece));
            }
        }
    }

    #[test]
    fn is_empty() {
        let mut map = PieceMap::new();
        assert!(map.is_empty());

        map.insert(Square::H8, Piece::WhitePawn);
        assert!(!map.is_empty());

        map = PieceMap::filled(Piece::BlackBishop);
        assert!(!map.is_empty());
    }

    #[test]
    fn fen() {
        let odd = {
            let mut map = PieceMap::STANDARD;
            map.swap(Square::D7, Square::E1);
            map.remove(Square::B1);
            map.remove(Square::C1);
            map.remove(Square::G8);
            (map, "rnbqkb1r/pppKpppp/8/8/8/8/PPPPPPPP/R2QpBNR")
        };

        let maps = [
            (PieceMap::STANDARD, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            (PieceMap::EMPTY,    "8/8/8/8/8/8/8/8"),
            odd,
        ];

        for &(ref map, exp) in &maps {
            assert_eq!(
                Some(map),
                PieceMap::from_fen(exp).as_ref()
            );

            map.map_fen(|s| assert_eq!(s, exp));
        }

        let fails = [
            "",
            "8/8/8/8/8/8/8",
            "/8/8/8/8/8/8/8",
            "8/8/8/8//8/8/8",
            "8/8/8/8/8/8/8/",
            "8/8/8/8/8/8/8/7",
            "8/8/8/8/8/8/8/9",
            "//////",
            "///////",
            "////////",
        ];

        for &fail in &fails {
            assert_eq!(None, PieceMap::from_fen(fail));
        }
    }
}
