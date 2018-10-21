//! A square to piece mapping for fast square lookups.

use core::{fmt, hash, mem, ops, ptr, str};

#[cfg(feature = "simd")]
use packed_simd::u8x64;

use castle;
use misc::Contained;
use piece::Piece;
use prelude::*;
use uncon::*;
use util::{Bytes as UtilBytes, Count, Usize64};

mod entry;
pub use self::entry::*;

mod iter;
pub use self::iter::*;

#[cfg(all(test, nightly))]
mod benches;

#[cfg(test)]
mod tests;

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
    pub const STANDARD: Bytes = [
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

pub(crate) const NONE: u8 = 12;

#[cfg(feature = "simd")]
pub(crate) const NONE_SIMD: u8x64 = u8x64::splat(NONE);

const NUM_SQUARES: usize = NUM_FILES * NUM_RANKS;
const NUM_FILES: usize = NUM_RANKS;
const NUM_RANKS: usize = 1 + Rank::Eight as usize;

/// An array of `Option<Piece>` as a view into
/// [`PieceMap`](struct.PieceMap.html)'s storage.
pub type Array = [Option<Piece>; NUM_SQUARES];

/// A two-dimensional array of `Option<Piece>` as a view into
/// [`PieceMap`](struct.PieceMap.html)'s storage.
pub type Array2d = [Slice; NUM_RANKS];

/// A one-dimensional fixed-size slice into [`PieceMap`](struct.PieceMap.html).
///
/// Usually used for when performing operations with
/// [`File`](../../square/enum.File.html) or
/// [`Rank`](../../square/enum.Rank.html).
pub type Slice = [Option<Piece>; NUM_FILES];

/// An array of bytes as a view into [`PieceMap`](struct.PieceMap.html)'s
/// storage.
pub type Bytes = [u8; NUM_SQUARES];

/// A mapping of sixty-four squares to pieces.
///
/// This allows for faster lookups than possible with bit boards.
///
/// **Note:** `PieceMap::default()` returns an empty piece map. Use
/// [`PieceMap::STANDARD`](#associatedconstant.STANDARD) to get a mapping for
/// standard chess.
#[repr(C)]
pub struct PieceMap(Inner);

#[derive(Copy, Clone)]
#[repr(C, align(64))]
union Inner {
    #[cfg(feature = "simd")]
    simd: u8x64,
    bytes: Bytes,
    // Safe if `tests::none_value` passes
    array: Array,
    array_2d: Array2d,
}

#[cfg(test)]
assert_eq_size!(inner_size; Inner, Bytes, Array, Usize64);

impl FromUnchecked<Bytes> for PieceMap {
    #[inline]
    unsafe fn from_unchecked(bytes: Bytes) -> PieceMap {
        PieceMap(Inner { bytes })
    }
}

#[cfg(feature = "simd")]
impl FromUnchecked<u8x64> for PieceMap {
    #[inline]
    unsafe fn from_unchecked(simd: u8x64) -> PieceMap {
        PieceMap(Inner { simd })
    }
}

impl From<Array> for PieceMap {
    #[inline]
    fn from(array: Array) -> PieceMap {
        PieceMap(Inner { array })
    }
}

impl From<Array2d> for PieceMap {
    #[inline]
    fn from(array_2d: Array2d) -> PieceMap {
        PieceMap(Inner { array_2d })
    }
}

impl AsRef<Array> for PieceMap {
    #[inline]
    fn as_ref(&self) -> &Array { self.as_array() }
}

impl AsMut<Array> for PieceMap {
    #[inline]
    fn as_mut(&mut self) -> &mut Array { self.as_array_mut() }
}

impl AsRef<Array2d> for PieceMap {
    #[inline]
    fn as_ref(&self) -> &Array2d { self.as_2d() }
}

impl AsMut<Array2d> for PieceMap {
    #[inline]
    fn as_mut(&mut self) -> &mut Array2d { self.as_2d_mut() }
}

impl Clone for PieceMap {
    #[inline]
    fn clone(&self) -> PieceMap { PieceMap(self.0) }
}

impl PartialEq for PieceMap {
    #[inline]
    fn eq(&self, other: &PieceMap) -> bool {
        #[cfg(feature = "simd")]
        {
            self as *const _ == other as *const _ ||
            self.as_vector() == other.as_vector()
        }

        #[cfg(not(feature = "simd"))]
        { self.as_bytes()[..] == other.as_bytes()[..] }
    }
}

impl Eq for PieceMap {}

impl Default for PieceMap {
    #[inline]
    fn default() -> PieceMap {
        PieceMap::EMPTY
    }
}

impl hash::Hash for PieceMap {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(self.as_bytes());
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

impl ops::Index<Rank> for PieceMap {
    type Output = Slice;

    #[inline]
    fn index(&self, r: Rank) -> &Self::Output {
        &self.as_2d()[r as usize]
    }
}

impl ops::IndexMut<Rank> for PieceMap {
    #[inline]
    fn index_mut(&mut self, r: Rank) -> &mut Self::Output {
        &mut self.as_2d_mut()[r as usize]
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
    pub const EMPTY: PieceMap = PieceMap(Inner { bytes: [NONE; NUM_SQUARES] });

    /// The piece map for standard chess.
    pub const STANDARD: PieceMap = PieceMap(Inner { bytes: tables::STANDARD });

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
        PieceMap(Inner { array: [Some(piece); NUM_SQUARES] })
    }

    /// Creates a new piece map by instantiating each slot with the provided
    /// initializer.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::board::piece_map::*;
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
        for i in 0..NUM_SQUARES {
            let val = init(i.into()).map(|p| p as u8).unwrap_or(NONE);
            unsafe { ptr::write(&mut map.0.bytes[i], val) };
        }
        map
    }

    #[cfg(feature = "simd")]
    #[inline]
    fn inner(&self) -> &u8x64 { self.as_vector() }

    #[cfg(not(feature = "simd"))]
    #[inline]
    fn inner(&self) -> &Bytes { self.as_bytes() }

    /// Reverses the square mapping.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::board::piece_map::*;
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
        self.as_array_mut().reverse();
    }

    #[inline]
    #[cfg_attr(feature = "simd", allow(dead_code))]
    fn inner_ptr_sized(&self) -> &Usize64 {
        unsafe { (&self.0).into_unchecked() }
    }

    #[inline]
    fn inner_u64(&self) -> &bytes64!(u64) {
        unsafe { (&self.0).into_unchecked() }
    }

    /// Mirrors the map across the horizontal axis of a chess board.
    pub fn mirror_horizontal(&mut self) {
        self.as_2d_mut().reverse();
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

    /// Replaces all pieces at `loc`, returning any previous pieces.
    #[inline]
    pub fn replace<T: Replace>(&mut self, loc: T, pc: Option<Piece>) -> T::Output {
        loc.replace(self, pc)
    }

    /// Inserts the piece at `loc`, returning any previous pieces.
    #[inline]
    pub fn insert<T: Replace>(&mut self, loc: T, pc: Piece) -> T::Output {
        self.replace(loc, Some(pc))
    }

    /// Removes all pieces at `loc` and returns them, if any.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::board::PieceMap;
    /// # use hexe_core::prelude::*;
    /// let mut map = PieceMap::STANDARD;
    ///
    /// assert_eq!(map.remove(Rank::Two), [Some(Piece::WhitePawn); 8]);
    /// assert_eq!(map.remove(Square::H8), Some(Piece::BlackRook));
    /// ```
    #[inline]
    pub fn remove<T: Replace>(&mut self, loc: T) -> T::Output {
        loc.replace(self, None)
    }

    /// Swaps two values in the map.
    #[inline]
    pub fn swap<T: Swap>(&mut self, i: T, j: T) {
        T::swap(i, j, self);
    }

    /// Takes the piece at the square and moves it.
    #[inline]
    pub fn relocate(&mut self, from: Square, to: Square) {
        let rem = self.remove(from);
        self.replace(to, rem);
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
    /// # use hexe_core::board::piece_map::*;
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
    ///
    /// Under legal castling circumstances, this method makes it so that squares
    /// involved with castling using `right` are in a correct state post-castle.
    #[inline]
    pub fn castle(&mut self, right: Right) {
        let (king_sq, start_sq) = castle::TABLES.pm_pairs[right as usize];
        self.remove(king_sq);

        let buf = unsafe { self.as_bytes_mut() };
        let ptr = &mut buf[start_sq as usize] as *mut u8 as *mut u32;
        unsafe { *ptr = castle::TABLES.pm_value[right as usize] };
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
        let iter = self.as_array_mut().iter_mut();
        for (i, slot) in iter.enumerate() {
            if let Some(ref mut s) = *slot {
                if f(i.into(), s) {
                   continue;
                }
            }
            *slot = None;
        }
    }

    /// Clears the map, removing all pieces.
    #[inline]
    pub fn clear(&mut self) {
        *self = PieceMap::EMPTY;
    }

    /// Returns whether `self` is empty.
    ///
    /// For much better performance and readability, is recommended to use this
    /// method over checking whether `self.len() == 0`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        #[cfg(feature = "simd")]
        { *self.as_vector() == NONE_SIMD }

        #[cfg(not(feature = "simd"))]
        {
            let empty = usize::splat(NONE);
            for &slot in self.inner_ptr_sized() {
                if slot != empty {
                    return false;
                }
            }
            true
        }
    }

    /// Returns the total number of pieces in `self`.
    ///
    /// This operation is performed in O(n) time. It is recommended to store
    /// the result if it is used repeatedly.
    #[inline]
    pub fn len(&self) -> usize {
        NUM_SQUARES - self.inner().count(NONE)
    }

    /// Returns the number of occurrences of `piece` in `self`.
    ///
    /// This operation is performed in O(n) time. It is recommended to store
    /// the result if it is used repeatedly.
    #[inline]
    pub fn count(&self, piece: Piece) -> usize {
        self.inner().count(piece as u8)
    }

    /// Returns whether the map contains the value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::board::piece_map::*;
    /// # use hexe_core::prelude::*;
    /// let sq = Square::B7;
    /// let pc = Piece::WhiteRook;
    ///
    /// let mut map = PieceMap::new();
    /// map.insert(sq, pc);
    ///
    /// assert!(map.contains(sq));
    /// assert!(map.contains(pc));
    ///
    /// assert!(map.contains(sq.file()));
    /// assert!(map.contains(sq.rank()));
    /// ```
    #[inline]
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
        ::memchr::memchr(pc as u8, self.as_bytes()).map(|index| unsafe {
            index.into_unchecked()
        })
    }

    /// Returns the last square for the piece.
    #[inline]
    pub fn rfind(&self, pc: Piece) -> Option<Square> {
        ::memchr::memrchr(pc as u8, self.as_bytes()).map(|index| unsafe {
            index.into_unchecked()
        })
    }

    /// Gets the given square's corresponding entry in the map for in-place
    /// manipulation.
    #[inline]
    pub fn entry(&mut self, sq: Square) -> Entry {
        Entry::from_map(self, sq)
    }

    /// Returns a reference to the piece at `square`, if any.
    #[inline]
    pub fn get(&self, square: Square) -> Option<&Piece> {
        self.as_array()[square as usize].as_ref()
    }

    /// Returns a mutable reference to the piece at `square`, if any.
    #[inline]
    pub fn get_mut(&mut self, square: Square) -> Option<&mut Piece> {
        self.as_array_mut()[square as usize].as_mut()
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
        (&self.as_bytes()[sq as usize]).into_unchecked()
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
        (&mut self.as_bytes_mut()[sq as usize]).into_unchecked()
    }

    /// Returns the color of the piece at the given square, if any.
    #[inline]
    pub fn color_at(&self, sq: Square) -> Option<Color> {
        self.get(sq).map(|p| p.color())
    }

    /// Returns the color of the piece at the square without checking whether a
    /// valid piece exists there.
    ///
    /// Because of how `Color` is encoded in `Piece`, this is not an `unsafe`
    /// operation. If the square is empty, the returned color is `White`.
    /// However, errors and bugs may arise if this is used on an empty square.
    #[inline]
    pub fn color_at_unchecked(&self, sq: Square) -> Color {
        (self.as_bytes()[sq as usize] & 1).into()
    }

    /// Returns the role of the piece at the given square, if any.
    #[inline]
    pub fn role_at(&self, sq: Square) -> Option<Role> {
        self.get(sq).map(|pc| pc.role())
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
    /// # use hexe_core::board::piece_map::*;
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
        let mut buf = ::consts::BOARD_DOTS;
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

    /// Returns a view into the map as an array of `Option<Piece>`.
    #[inline]
    pub fn as_array(&self) -> &Array {
        unsafe { &self.0.array }
    }

    /// Returns a mutable view into the map as an array of `Option<Piece>`.
    ///
    /// See also: [`as_2d_mut`](#method.as_2d_mut)
    ///
    /// # Safety
    ///
    /// This method is not marked as `unsafe` because it does not allow for the
    /// same value violations that
    /// [`PieceMap::as_bytes_mut`](#method.as_bytes_mut) allows.
    #[inline]
    pub fn as_array_mut(&mut self) -> &mut Array {
        unsafe { &mut self.0.array }
    }

    /// Returns a view into the map as a two-dimensional array of
    /// `Option<Piece>`.
    #[inline]
    pub fn as_2d(&self) -> &Array2d {
        unsafe { &self.0.array_2d }
    }

    /// Returns a mutable view into the map as a two-dimensional array of
    /// `Option<Piece>`.
    ///
    /// See also: [`as_array_mut`](#method.as_array_mut)
    ///
    /// # Safety
    ///
    /// This method is not marked as `unsafe` because it does not allow for the
    /// same value violations that
    /// [`PieceMap::as_bytes_mut`](#method.as_bytes_mut) allows.
    #[inline]
    pub fn as_2d_mut(&mut self) -> &mut Array2d {
        unsafe { &mut self.0.array_2d }
    }

    /// Returns a view into the raw bytes of the map.
    ///
    /// # Values
    ///
    /// - Bytes with values less than 12 refer to a valid `Piece` instance.
    /// - Empty slots have a value of 12.
    ///
    /// You may safely assume that that no values greater than 12 exist.
    #[inline]
    pub fn as_bytes(&self) -> &Bytes {
        unsafe { &self.0.bytes }
    }

    /// Returns a mutable view into the raw bytes of the map.
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
    pub unsafe fn as_bytes_mut(&mut self) -> &mut Bytes {
        &mut self.0.bytes
    }

    /// A reference to the inner SIMD vector for `self`.
    ///
    /// Requires enabling the `simd` feature.
    #[cfg(feature = "simd")]
    #[inline]
    pub fn as_vector(&self) -> &u8x64 {
        unsafe { &self.0.simd }
    }

    /// A mutable reference to the inner SIMD vector for `self`.
    ///
    /// Requires enabling the `simd` feature.
    ///
    /// # Safety
    ///
    /// See [`PieceMap::as_bytes_mut`](#method.as_bytes_mut) for how to handle
    /// safely writing to the vector.
    #[cfg(feature = "simd")]
    #[inline]
    pub unsafe fn as_vector_mut(&mut self) -> &mut u8x64 {
        &mut self.0.simd
    }
}

impl<'a> Contained<&'a PieceMap> for Square {
    #[inline]
    fn contained_in(self, map: &PieceMap) -> bool {
        map.as_array()[self as usize].is_some()
    }
}

impl<'a> Contained<&'a PieceMap> for File {
    #[inline]
    fn contained_in(self, map: &PieceMap) -> bool {
        for rank in map.as_2d() {
            if rank[self as usize].is_some() {
                return true;
            }
        }
        false
    }
}

impl<'a> Contained<&'a PieceMap> for Rank {
    #[inline]
    fn contained_in(self, map: &PieceMap) -> bool {
        map.inner_u64()[self as usize] != u64::splat(NONE)
    }
}

impl<'a> Contained<&'a PieceMap> for Piece {
    #[inline]
    fn contained_in(self, map: &PieceMap) -> bool {
        #[cfg(feature = "simd")]
        { (*map.as_vector()).eq(u8x64::splat(self as u8)).any() }

        #[cfg(not(feature = "simd"))]
        { map.find(self).is_some() }
    }
}

/// A type whose instances may be used to replace values in a
/// [`PieceMap`](struct.PieceMap.html).
pub trait Replace {
    /// The resulting type after replacement.
    type Output;

    /// Replaces all pieces in `map` at `self` with `piece`, returning any
    /// previous pieces.
    fn replace(self, map: &mut PieceMap, piece: Option<Piece>) -> Self::Output;
}

impl Replace for Square {
    type Output = Option<Piece>;

    #[inline]
    fn replace(self, map: &mut PieceMap, piece: Option<Piece>) -> Self::Output {
        mem::replace(self.extract_mut(map.as_array_mut()), piece)
    }
}

impl Replace for File {
    type Output = Slice;

    #[inline]
    fn replace(self, map: &mut PieceMap, piece: Option<Piece>) -> Self::Output {
        let mut out = [None; 8];

        for rank in Rank::ALL {
            let slot = &mut map[rank][self as usize];
            out[rank as usize] = mem::replace(slot, piece);
        }

        out
    }
}

impl Replace for Rank {
    type Output = Slice;

    #[inline]
    fn replace(self, map: &mut PieceMap, piece: Option<Piece>) -> Self::Output {
        mem::replace(&mut map[self], [piece; 8])
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
        map.as_array_mut().swap(i as usize, j as usize);
    }
}

impl Swap for File {
    #[inline]
    fn swap(i: File, j: File, map: &mut PieceMap) {
        for rank in map.as_2d_mut() {
            rank.swap(i as usize, j as usize);
        }
    }
}

impl Swap for Rank {
    #[inline]
    fn swap(i: Rank, j: Rank, map: &mut PieceMap) {
        map.as_2d_mut().swap(i as usize, j as usize);
    }
}
