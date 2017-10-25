//! A chess board square and its components.

#[cfg(feature = "try-from")]
use core::convert::TryFrom;
use core::fmt;
use core::ops::Range;
use core::str;

#[cfg(feature = "serde")]
use serde::*;

use prelude::*;

mod tables;

/// A square on a chess board.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
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
        self.map_str(|s| fmt::Display::fmt(s, f))
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
        use serde::de;

        <&str>::deserialize(de)?.parse().map_err(|_| {
            de::Error::custom("failed to parse square")
        })
    }
}

impl Square {
    /// An efficient iterator over all squares.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use hexe_core::square::Square;
    /// // Print from A1 through H8
    /// for square in Square::all() {
    ///     println!("{:?}", square);
    /// }
    /// ```
    #[inline]
    pub fn all() -> Squares {
        Squares { iter: 0..64 }
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
        (((rank as u8) << 3) | (file as u8)).into()
    }

    /// Returns the `File` for `self`.
    #[inline]
    pub fn file(&self) -> File {
        ((*self as u8) & 7).into()
    }

    /// Returns the `Rank` for `self`.
    #[inline]
    pub fn rank(&self) -> Rank {
        ((*self as u8) >> 3).into()
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
    pub fn color(&self) -> Color {
        (Bitboard::BLACK >> *self as u64).0.into()
    }

    /// Returns the result of applying a function to a mutable string
    /// representation of `self`.
    #[inline]
    pub fn map_str<T, F>(&self, f: F) -> T
        where F: for<'a> FnOnce(&'a mut str) -> T
    {
        let mut buf = [0; 2];
        buf[0] = char::from(self.file()) as u8;
        buf[1] = char::from(self.rank()) as u8;
        unsafe { f(str::from_utf8_unchecked_mut(&mut buf)) }
    }

    /// Returns the pawn attacks for `self` and `color`.
    #[inline]
    pub fn pawn_attacks(&self, color: Color) -> Bitboard {
        Bitboard(self::tables::PAWN_ATTACKS[color as usize][*self as usize])
    }

    /// Returns the knight attacks for `self`.
    #[inline]
    pub fn knight_attacks(&self) -> Bitboard {
        Bitboard(self::tables::KNIGHT_ATTACKS[*self as usize])
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
    pub fn rook_attacks(&self, occupied: Bitboard) -> Bitboard {
        ::magic::rook_attacks(*self, occupied)
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
    pub fn bishop_attacks(&self, occupied: Bitboard) -> Bitboard {
        ::magic::bishop_attacks(*self, occupied)
    }

    /// Returns the king attacks for `self`.
    #[inline]
    pub fn king_attacks(&self) -> Bitboard {
        Bitboard(self::tables::KING_ATTACKS[*self as usize])
    }

    /// Returns the queen attacks for `self` and `occupied`.
    ///
    /// This works the same as combining the results of `rook_attacks` and
    /// `bishop_attacks`.
    #[inline]
    pub fn queen_attacks(&self, occupied: Bitboard) -> Bitboard {
        self.rook_attacks(occupied) | self.bishop_attacks(occupied)
    }
}

/// An iterator over all squares.
#[derive(Clone, PartialEq, Eq)]
pub struct Squares {
    // Range for iterating in reverse
    // Invariant: always within 0..64
    iter: Range<u8>
}

impl Iterator for Squares {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        use uncon::IntoUnchecked;
        if let Some(n) = self.iter.next() {
            unsafe { Some(n.into_unchecked()) }
        } else {
            None
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn last(mut self) -> Option<Square> {
        self.next_back()
    }
}

impl DoubleEndedIterator for Squares {
    #[inline]
    fn next_back(&mut self) -> Option<Square> {
        use uncon::IntoUnchecked;
        if let Some(n) = self.iter.next_back() {
            unsafe { Some(n.into_unchecked()) }
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Squares {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl Squares {
    /// Returns whether `self` contains `square`.
    #[inline]
    pub fn contains(&self, square: Square) -> bool {
        let value = square as u8;
        (self.iter.start <= value) && (value < self.iter.end)
    }
}

/// A file (or column) for a chess board.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
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
pub enum Rank { One, Two, Three, Four, Five, Six, Seven, Eight }

impl Rank {
    /// Returns a file from the parsed character.
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
}

/// The error returned when `try_from` fails for `File` or `Rank`.
#[cfg(feature = "try-from")]
pub struct TryFromCharError(());

macro_rules! impl_components {
    ($($t:ty, $c:expr, $m:expr;)+) => {
        $(impl From<$t> for char {
            #[inline]
            fn from(val: $t) -> char {
                ($c + val as u8) as char
            }
        }

        #[cfg(feature = "try-from")]
        impl TryFrom<char> for $t {
            type Error = TryFromCharError;

            #[inline]
            fn try_from(ch: char) -> Result<Self, TryFromCharError> {
                Self::from_char(ch).ok_or(TryFromCharError(()))
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
        })+
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
                for occupied in (0..10_000).map(|_| Bitboard(rng.gen())) {
                    for square in Square::all() {
                        assert_eq!(
                            square.$fn(occupied),
                            Bitboard::from(square).$fn(!occupied)
                        );
                    }
                }
            })*
        }
    }

    sliding_attacks! { rook_attacks bishop_attacks queen_attacks }

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
}
