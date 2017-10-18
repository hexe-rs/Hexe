//! A chess board square and its components.

use core::ops::Range;
use core::str::FromStr;

use prelude::*;

/// A square on a chess board.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromUnchecked)]
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

/// The error returned when `Square::from_str` fails.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FromStrError(());

impl FromStr for Square {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Square, FromStrError> {
        use uncon::IntoUnchecked;
        let bytes = s.as_bytes();
        if bytes.len() != 2 { Err(FromStrError(())) } else {
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
    /// let occ = Bitboard::from(Square::A3)
    ///         | Bitboard::from(Square::C1);
    /// let exp = Bitboard::from(Square::A2)
    ///         | Bitboard::from(Square::B1)
    ///         | occ;
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
    /// let occ = Bitboard::from(Square::C3);
    /// let exp = Bitboard::from(Square::B2) | occ;
    ///
    /// assert_eq!(start.bishop_attacks(occ), exp);
    /// ```
    #[inline]
    pub fn bishop_attacks(&self, occupied: Bitboard) -> Bitboard {
        ::magic::bishop_attacks(*self, occupied)
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

/// A rank (or row) for a chess board.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum Rank { One, Two, Three, Four, Five, Six, Seven, Eight }

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
}