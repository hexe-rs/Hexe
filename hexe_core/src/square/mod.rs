//! A chess board square and its components.
//!
//! A chess board is comprised of sixty-four squares ranging from A1 through H8.
//! Each square has two components:
//!
//! - File: a column represented by a letter from A through F
//! - Rank: a row represented by a number from 1 through 8
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use hexe_core::square::{Square, File, Rank};
//!
//! let f = File::B;
//! let r = Rank::Seven;
//! let sq = Square::B7;
//!
//! assert_eq!(sq, Square::new(f, r));
//! ```
//!
//! [`Square`] is an `enum` so that we can safely and conveniently index into
//! tables of sixty-four elements. Because the optimizer knows that the index
//! will **never** be greater than 64, the bounds check gets removed, thus
//! making lookups fast.
//!
//! ```
//! # use hexe_core::prelude::*;
//! # type T = ();
//! static TABLE: [T; 64] = [
//!     /* ... */
//! #   (); 64
//! ];
//!
//! pub fn get_value(sq: Square) -> T {
//!     // Will never panic
//!     TABLE[sq as usize]
//! }
//! ```
//!
//! [`Square`]: enum.Square.html

use core::{fmt, ops, str};
use prelude::*;

#[cfg(feature = "serde")]
use serde::*;

mod tables;

mod squares;
pub use self::squares::*;

/// A square on a chess board.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl fmt::Debug for Square {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Square {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.map_str(|s| s.fmt(f))
    }
}

impl From<(File, Rank)> for Square {
    #[inline]
    fn from((file, rank): (File, Rank)) -> Square {
        Square::new(file, rank)
    }
}

/// The error returned when `Square::from_str` fails.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FromStrError(());

impl str::FromStr for Square {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Square, FromStrError> {
        use uncon::IntoUnchecked;
        let bytes = s.as_bytes();
        if bytes.len() != 2 { Err(FromStrError(())) } else {
            // Gets better optimized as a macro for some strange reason
            macro_rules! convert {
                ($b:expr, $lo:expr, $hi:expr) => {
                    if $b >= $lo && $b <= $hi {
                        unsafe { ($b - $lo).into_unchecked() }
                    } else {
                        return Err(FromStrError(()))
                    }
                }
            }
            Ok(Square::new(convert!(bytes[0] | 32, b'a', b'h'),
                           convert!(bytes[1], b'1', b'8')))
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Square {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.map_str(|s| ser.serialize_str(s))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Square {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        <&str>::deserialize(de)?.parse().map_err(|_| {
            de::Error::custom("failed to parse square")
        })
    }
}

const FILE_BITS: u8 = 7;
const RANK_BITS: u8 = FILE_BITS << RANK_SHIFT;
const RANK_SHIFT: usize = 3;

const TRIANGLE_LEN: usize = 64 * 65 / 2;

impl Square {
    /// An efficient iterator over all squares.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::square::Square;
    /// // Perform operation on A1 through H8
    /// for square in Square::all() {
    ///     # break;
    ///     /* ... */
    /// }
    /// ```
    #[inline]
    pub fn all() -> Squares {
        Squares::default()
    }

    /// Initializes a `Square` from a `File` and `Rank`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::square::*;
    /// let s = Square::new(File::B, Rank::Five);
    ///
    /// assert_eq!(s.file(), File::B);
    /// assert_eq!(s.rank(), Rank::Five);
    /// ```
    #[inline]
    pub fn new(file: File, rank: Rank) -> Square {
        (((rank as u8) << RANK_SHIFT) | (file as u8)).into()
    }

    #[inline]
    pub(crate) fn between(self, other: Square) -> Bitboard {
        use self::tables::*;
        Bitboard(TABLES[BETWEEN_START..][self as usize][other as usize])
    }

    #[inline]
    pub(crate) fn line(self, other: Square) -> Bitboard {
        use self::tables::*;
        Bitboard(TABLES[LINE_START..][self as usize][other as usize])
    }

    /// Returns the `File` for `self`.
    #[inline]
    pub fn file(self) -> File {
        ((self as u8) & FILE_BITS).into()
    }

    /// Returns the `Rank` for `self`.
    #[inline]
    pub fn rank(self) -> Rank {
        ((self as u8) >> RANK_SHIFT).into()
    }

    /// Reverses the file of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// assert_eq!(Square::B2.rev_file(), Square::G2);
    /// ```
    #[inline]
    pub fn rev_file(self) -> Square {
        (FILE_BITS ^ self as u8).into()
    }

    /// Reverses the rank of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// assert_eq!(Square::B2.rev_rank(), Square::B7);
    /// ```
    #[inline]
    pub fn rev_rank(self) -> Square {
        (RANK_BITS ^ self as u8).into()
    }

    /// Combines the file of `self` with the rank of `other`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let s1 = Square::B5;
    /// let s2 = Square::C7;
    ///
    /// assert_eq!(s1.combine(s2), Square::B7);
    /// assert_eq!(s2.combine(s1), Square::C5);
    /// ```
    #[inline]
    pub fn combine(self, other: Square) -> Square {
        ((FILE_BITS & self as u8) | (RANK_BITS & other as u8)).into()
    }

    /// Returns the `Color` for `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let a = Square::A1;
    /// assert_eq!(a.color(), Color::Black);
    ///
    /// let b = Square::B5;
    /// assert_eq!(b.color(), Color::White);
    /// ```
    #[inline]
    pub fn color(self) -> Color {
        (Bitboard::BLACK >> self as u64).0.into()
    }

    /// Returns whether `self` and `other` are equal in color.
    #[inline]
    pub fn color_eq(self, other: Square) -> bool {
        (self.file() as usize ^ other.file() as usize) & 1 ==
        (self.rank() as usize ^ other.rank() as usize) & 1
    }

    /// Returns whether `self` is aligned with two other squares along a file,
    /// rank, or diagonal.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let sq = Square::A3;
    ///
    /// assert!(sq.is_aligned(Square::C5, Square::F8));
    /// ```
    #[inline]
    pub fn is_aligned(self, a: Square, b: Square) -> bool {
        a.line(b).contains(self)
    }

    /// Returns whether `self` is between two other squares along a file, rank,
    /// or diagonal.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let sq = Square::D4;
    ///
    /// assert!(sq.is_between(Square::B2, Square::G7));
    /// ```
    #[inline]
    pub fn is_between(self, a: Square, b: Square) -> bool {
        a.between(b).contains(self)
    }

    /// Calculates the [Chebyshev distance][wiki] between `self` and `other`.
    ///
    /// The result is the number of steps required to move a king from one
    /// square to the other.
    ///
    /// # Examples
    ///
    /// It takes a king two moves to travel the same distance as a knight:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// for s1 in Square::all() {
    ///     for s2 in s1.knight_attacks() {
    ///         assert_eq!(s1.distance(s2), 2);
    ///     }
    /// }
    /// ```
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Chebyshev_distance
    #[inline]
    pub fn distance(self, other: Square) -> usize {
        self::tables::DISTANCE[self as usize][other as usize] as usize
    }

    /// Calculates the [Manhattan distance][wiki] between `self` and `other`.
    ///
    /// The result is the distance when strictly taking a horizontal/vertical
    /// path from one square to the other.
    ///
    /// # Examples
    ///
    /// The knight's path always traverses three squares:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// for s1 in Square::all() {
    ///     for s2 in s1.knight_attacks() {
    ///         assert_eq!(s1.man_distance(s2), 3);
    ///     }
    /// }
    /// ```
    ///
    /// [wiki]: https://en.wiktionary.org/wiki/Manhattan_distance
    #[inline]
    pub fn man_distance(self, other: Square) -> usize {
        self.file().distance(other.file()) + self.rank().distance(other.rank())
    }

    /// Calculates the [Chebyshev distance][wiki] between `self` and the center
    /// of the board.
    ///
    /// # Examples
    ///
    /// It takes a king three moves to travel to the center of the board from
    /// any edge:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let edges = File::A | File::H | Rank::One | Rank::Eight;
    ///
    /// for sq in edges {
    ///     assert_eq!(sq.center_distance(), 3);
    /// }
    /// ```
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Chebyshev_distance
    #[inline]
    pub fn center_distance(self) -> usize {
        self::tables::CENTER_DISTANCE[0][self as usize] as usize
    }

    /// Calculates the [Manhattan distance][wiki] between `self` and the center
    /// of the board.
    ///
    /// [wiki]: https://en.wiktionary.org/wiki/Manhattan_distance
    #[inline]
    pub fn center_man_distance(self) -> usize {
        self::tables::CENTER_DISTANCE[1][self as usize] as usize
    }

    /// Returns the [triangular index][wiki] for `self` and `other`.
    ///
    /// This allows indexing into tables of size 2080, which is slightly greater
    /// than half of 64 by 64.
    ///
    /// # Safety
    /// The result index has been thoroughly tested to **always** return a value
    /// less than 2080. Because of this, it is safe to call [`get_unchecked`] on
    /// arrays/slices of that size or greater.
    ///
    /// [wiki]: https://chessprogramming.wikispaces.com/Square+Attacked+By#Legality%20Test-In%20Between-Triangular%20Lookup
    /// [`get_unchecked`]: https://doc.rust-lang.org/std/primitive.slice.html#method.get_unchecked
    #[inline]
    pub fn tri_index(self, other: Square) -> usize {
        let mut a = self  as isize;
        let mut b = other as isize;
        let mut d = a - b;
        d &= d >> 31;
        b += d;
        a -= d;
        b *= b ^ 127;
        ((b >> 1) + a) as usize
    }

    /// Returns a shared reference to an element from the table via triangular
    /// index.
    #[inline]
    pub fn tri<T>(self, other: Square, table: &[T; TRIANGLE_LEN]) -> &T {
        // tri index < TRIANGLE_LEN
        unsafe { table.get_unchecked(self.tri_index(other)) }
    }

    /// Returns a mutable reference to an element from the table via triangular
    /// index.
    #[inline]
    pub fn tri_mut<T>(self, other: Square, table: &mut [T; TRIANGLE_LEN]) -> &mut T {
        unsafe { table.get_unchecked_mut(self.tri_index(other)) }
    }

    /// Returns the result of applying a function to a mutable string
    /// representation of `self`.
    ///
    /// This is a _much_ preferred way of getting the string representation of
    /// a square, especially in when using `#[no_std]`. The alternative would be
    /// to use `format!`, which performs a heap allocation whereas this uses a
    /// stack-allocated string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let sq = Square::A5;
    /// sq.map_str(|s| {
    ///     assert_eq!(s, "A5");
    /// });
    /// ```
    #[inline]
    pub fn map_str<T, F>(self, f: F) -> T
        where F: for<'a> FnOnce(&'a mut str) -> T
    {
        let mut buf = [0; 2];
        buf[0] = char::from(self.file()) as u8;
        buf[1] = char::from(self.rank()) as u8;
        unsafe { f(str::from_utf8_unchecked_mut(&mut buf)) }
    }

    /// Returns the pawn attacks for `self` and `color`.
    #[inline]
    pub fn pawn_attacks(self, color: Color) -> Bitboard {
        Bitboard(self::tables::TABLES[color as usize][self as usize])
    }

    /// Returns the knight attacks for `self`.
    #[inline]
    pub fn knight_attacks(self) -> Bitboard {
        Bitboard(self::tables::TABLES[2][self as usize])
    }

    /// Returns the rook attacks for `self` and `occupied`.
    ///
    /// Whether or not `occupied` contains `self` does not matter.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let start = Square::A1;
    ///
    /// let occ = Square::A3 | Square::C1;
    /// let exp = Square::A2 | Square::B1 | occ;
    ///
    /// assert_eq!(start.rook_attacks(occ), exp);
    /// ```
    #[inline]
    pub fn rook_attacks(self, occupied: Bitboard) -> Bitboard {
        ::magic::rook_attacks(self, occupied)
    }

    /// Returns the bishop attacks for `self` and `occupied`.
    ///
    /// Whether or not `occupied` contains `self` does not matter.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let start = Square::A1;
    ///
    /// let occ = Square::C3;
    /// let exp = Square::B2 | occ;
    ///
    /// assert_eq!(start.bishop_attacks(occ.into()), exp);
    /// ```
    #[inline]
    pub fn bishop_attacks(self, occupied: Bitboard) -> Bitboard {
        ::magic::bishop_attacks(self, occupied)
    }

    /// Returns the king attacks for `self`.
    #[inline]
    pub fn king_attacks(self) -> Bitboard {
        Bitboard(self::tables::TABLES[3][self as usize])
    }

    /// Returns the queen attacks for `self` and `occupied`.
    ///
    /// This works the same as combining the results of `rook_attacks` and
    /// `bishop_attacks`.
    #[inline]
    pub fn queen_attacks(self, occupied: Bitboard) -> Bitboard {
        self.rook_attacks(occupied) | self.bishop_attacks(occupied)
    }
}

/// A file (or column) for a chess board.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum File { A, B, C, D, E, F, G, H }

impl File {
    /// Returns a file from the parsed character.
    #[inline]
    pub fn from_char(ch: char) -> Option<File> {
        use uncon::IntoUnchecked;
        match 32 | ch as u8 {
            b @ b'a' ... b'f' => unsafe {
                Some((b - b'a').into_unchecked())
            },
            _ => None,
        }
    }

    /// Returns the adjacent mask for `self`, containing all squares on the
    /// files directly to the left and right of `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let val = File::C;
    /// let adj = File::B | File::D;
    ///
    /// assert_eq!(val.adjacent_mask(), adj);
    /// ```
    #[inline]
    pub fn adjacent_mask(&self) -> Bitboard {
        use bitboard::masks::*;
        static ADJACENT: [u64; 8] = [
            FILE_B.0, FILE_A.0 | FILE_C.0, FILE_B.0 | FILE_D.0, FILE_C.0 | FILE_E.0,
            FILE_D.0 | FILE_F.0, FILE_E.0 | FILE_G.0, FILE_F.0 | FILE_H.0, FILE_G.0,
        ];
        Bitboard(ADJACENT[*self as usize])
    }
}

/// A rank (or row) for a chess board.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Rank { One, Two, Three, Four, Five, Six, Seven, Eight }

impl Rank {
    /// Returns a rank from the parsed character.
    #[inline]
    pub fn from_char(ch: char) -> Option<Rank> {
        use uncon::IntoUnchecked;
        match ch as u8 {
            b @ b'1' ... b'8' => unsafe {
                Some((b - b'1').into_unchecked())
            },
            _ => None,
        }
    }

    /// Returns the adjacent mask for `self`, containing all squares on the
    /// ranks directly ahead and behind `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let val = Rank::Five;
    /// let adj = Rank::Four | Rank::Six;
    ///
    /// assert_eq!(val.adjacent_mask(), adj);
    /// ```
    #[inline]
    pub fn adjacent_mask(&self) -> Bitboard {
        use bitboard::masks::*;
        static ADJACENT: [u64; 8] = [
            RANK_2.0, RANK_1.0 | RANK_3.0, RANK_2.0 | RANK_4.0, RANK_3.0 | RANK_5.0,
            RANK_4.0 | RANK_6.0, RANK_5.0 | RANK_7.0, RANK_6.0 | RANK_8.0, RANK_7.0,
        ];
        Bitboard(ADJACENT[*self as usize])
    }

    /// Returns the remaining distance for `color` to reach the end of the board
    /// from `self`.
    ///
    /// This is useful for finding the number of moves a pawn must make to be
    /// promoted.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexe_core::prelude::*;
    /// let rank = Rank::Three;
    ///
    /// assert_eq!(rank.rem_distance(Color::White), 5);
    /// assert_eq!(rank.rem_distance(Color::Black), 2);
    /// ```
    #[inline]
    pub fn rem_distance(self, color: Color) -> usize {
        (0b111 * color as usize) ^ !self as usize
    }
}

macro_rules! impl_components {
    ($($t:ty, $c:expr, $m:expr;)+) => {
        impl_try_from_char! {
            /// The error returned when `try_from` fails for `File` or `Rank`.
            message = "failed to parse a character as a board component";
            impl for { $($t)+ }
        }

        $(
            impl From<$t> for char {
                #[inline]
                fn from(val: $t) -> char {
                    ($c + val as u8) as char
                }
            }

            impl ops::Not for $t {
                type Output = Self;

                #[inline]
                fn not(self) -> Self {
                    (7 - self as u8).into()
                }
            }

            #[cfg(feature = "serde")]
            impl Serialize for $t {
                fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                    ser.serialize_char((*self).into())
                }
            }

            #[cfg(feature = "serde")]
            impl<'de> Deserialize<'de> for $t {
                fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                    Self::from_char(char::deserialize(de)?).ok_or_else(|| {
                        de::Error::custom($m)
                    })
                }
            }

            impl $t {
                /// Returns the distance between `self` and `other`.
                #[inline]
                pub fn distance(self, other: Self) -> usize {
                    (self as isize - other as isize).abs() as usize
                }
            }
        )+
    }
}

impl_components! {
    File, b'A', "failed to parse board file";
    Rank, b'1', "failed to parse board rank";
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, thread_rng};

    macro_rules! sliding_attacks {
        ($($fn:ident)*) => {
            $(#[test]
            fn $fn() {
                let mut rng = thread_rng();
                for occupied in (0..20_000).map(|_| Bitboard(rng.gen())) {
                    for square in Square::all() {
                        let exp = Bitboard::from(square).$fn(!occupied);
                        let res = square.$fn(occupied);
                        if exp != res {
                            panic!(
                                "Square: {}\n\
                                 Occupied: {1:?}\n{1}\n\
                                 Expected: {2:?}\n{2}\n\
                                 Generated: {3:?}\n{3}",
                                square,
                                occupied,
                                exp,
                                res,
                            );
                        }
                    }
                }
            })*
        }
    }

    macro_rules! jump_attacks {
        ($($fn:ident)*) => {
            $(#[test]
            fn $fn() {
                for square in Square::all() {
                    let exp = Bitboard::from(square).$fn();
                    let res = square.$fn();
                    assert_eq!(exp, res);
                }
            })*
        }
    }

    sliding_attacks! { rook_attacks bishop_attacks queen_attacks }

    jump_attacks! { knight_attacks king_attacks }

    #[test]
    fn distance() {
        fn square(a: Square, b: Square) -> usize {
            use core::cmp::max;
            max(a.file().distance(b.file()), a.rank().distance(b.rank()))
        }

        for s1 in Square::all() {
            for s2 in Square::all() {
                assert_eq!(square(s1, s2), s1.distance(s2));
            }
        }
    }

    #[test]
    fn tri_index() {
        for s1 in Square::all() {
            for s2 in Square::all() {
                let idx = s1.tri_index(s2);
                assert_eq!(idx, s2.tri_index(s1));
                assert!(idx < TRIANGLE_LEN);
            }
        }
    }

    #[test]
    fn pawn_attacks() {
        for &color in &[Color::White, Color::Black] {
            for square in Square::all() {
                let exp = Bitboard::from(square).pawn_attacks(color);
                let res = square.pawn_attacks(color);
                assert_eq!(exp, res);
            }
        }
    }

    #[test]
    fn file_from_char() {
        for ch in b'A'..(b'F' + 1) {
            for &ch in &[ch, ch | 32] {
                assert!(File::from_char(ch as _).is_some());
            }
        }
    }

    #[test]
    fn rank_from_char() {
        for ch in b'1'..(b'8' + 1) {
            assert!(Rank::from_char(ch as _).is_some());
        }
    }

    #[test]
    fn square_color() {
        for s1 in Square::all() {
            for s2 in Square::all() {
                assert_eq!(s1.color() == s2.color(), s1.color_eq(s2));
            }
        }
    }
}
