//! Castling rights for two players of a chess game.
//!
//! # What is Castling?
//!
//! In chess, [castling] is a special move performed by a king and a rook.
//! Castling can only be done under certain conditions.
//!
//! For example, a piece can't be moved in a castle if it has been moved
//! previously. You can use the [`CastleRights`] type to keep track of this
//! case:
//!
//! - If a king has moved, both castle rights for its color must be cleared
//! - If a rook has moved, the castle right for its color and board side must be
//! cleared
//!
//! ```txt
//! Before:         | After:
//! r . + . k . + r | . . k r . . . r
//! . . . . . . . . | . . . . . . . .
//! . . . . . . . . | . . . . . . . .
//! . . . . . . . . | . . . . . . . .
//! . . . . . . . . | . . . . . . . .
//! . . . . . . . . | . . . . . . . .
//! . . . . . . . . | . . . . . . . .
//! R . + . K . + R | R . . . . R K .
//! ```
//!
//! In the **before** frame, kings and rooks are in their initial positions.
//! Kings may be moved to the indicated (+) squares. In the **after** frame,
//! White has castled kingside and Black has castled queenside.
//!
//! Notice that the king can only move a maximum of two squares when castling,
//! regardless of which board side.
//!
//! [`CastleRights`]: struct.CastleRights.html
//! [castling]: https://en.wikipedia.org/wiki/Castling

use core::{fmt, ops, str};
use prelude::*;

#[cfg(feature = "serde")]
use serde::*;

/// Castle rights for a chess game.
///
/// # Examples
///
/// Similar to with [`Bitboard`], castle rights can be composed with set
/// operations.
///
/// ```
/// # use hexe_core::prelude::*;
/// assert_eq!(
///     CastleRight::WhiteKingside   | CastleRight::WhiteQueenside,
///     CastleRights::WHITE_KINGSIDE | CastleRights::WHITE_QUEENSIDE
/// );
/// ```
///
/// [`Bitboard`]: ../bitboard/struct.Bitboard.html
#[derive(PartialEq, Eq, Clone, Copy, Hash, FromUnchecked)]
pub struct CastleRights(u8);

impl From<u8> for CastleRights {
    #[inline]
    fn from(inner: u8) -> CastleRights {
        Self::FULL & CastleRights(inner)
    }
}

impl From<Color> for CastleRights {
    #[inline]
    fn from(color: Color) -> CastleRights {
        match color {
            Color::White => Self::WHITE_KINGSIDE | Self::WHITE_QUEENSIDE,
            Color::Black => Self::BLACK_KINGSIDE | Self::BLACK_QUEENSIDE,
        }
    }
}

impl Default for CastleRights {
    #[inline]
    fn default() -> CastleRights { CastleRights::FULL }
}

impl fmt::Debug for CastleRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            fmt::Display::fmt("(empty)", f)
        } else {
            self.map_str(|s| fmt::Display::fmt(s, f))
        }
    }
}

impl fmt::Display for CastleRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.map_str(|s| s.fmt(f))
    }
}

define_from_str_error! { CastleRights;
    /// The error returned when `CastleRights::from_str` fails.
    "failed to parse a string as castling rights"
}

#[cfg(feature = "serde")]
impl Serialize for CastleRights {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        self.map_str(|s| ser.serialize_str(s))
    }
}

impl str::FromStr for CastleRights {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<CastleRights, FromStrError> {
        let bytes = s.as_bytes();
        let mut result = CastleRights::EMPTY;

        if bytes.len() == 1 && bytes[0] == b'-' {
            Ok(result)
        } else {
            for &byte in bytes {
                result |= match byte {
                    b'K' => CastleRights::WHITE_KINGSIDE,
                    b'k' => CastleRights::BLACK_KINGSIDE,
                    b'Q' => CastleRights::WHITE_QUEENSIDE,
                    b'q' => CastleRights::BLACK_QUEENSIDE,
                    _ => return Err(FromStrError(())),
                };
            }
            Ok(result)
        }
    }
}

impl CastleRights {
    /// White kingside.
    pub const WHITE_KINGSIDE: CastleRights = CastleRights(0b0001);

    /// White queenside.
    pub const WHITE_QUEENSIDE: CastleRights = CastleRights(0b0010);

    /// Black kingside.
    pub const BLACK_KINGSIDE: CastleRights = CastleRights(0b0100);

    /// Black queenside.
    pub const BLACK_QUEENSIDE: CastleRights = CastleRights(0b1000);

    /// Returns the result of applying a function to a mutable string
    /// representation of `self`.
    #[inline]
    pub fn map_str<T, F: FnOnce(&mut str) -> T>(&self, f: F) -> T {
        let mut buf = [0u8; 4];
        let slice: &mut [u8] = if self.is_empty() {
            buf[0] = b'-';
            &mut buf[..1]
        } else {
            let mut idx = 0;
            for right in *self {
                unsafe {
                    *buf.get_unchecked_mut(idx) = char::from(right) as u8;
                }
                idx += 1;
            }
            unsafe { buf.get_unchecked_mut(..idx) }
        };
        unsafe { f(str::from_utf8_unchecked_mut(slice)) }
    }
}

impl_bit_set! { CastleRights 0b1111 => CastleRight }

impl_composition_ops! { CastleRights => CastleRight }

impl From<CastleRight> for CastleRights {
    #[inline]
    fn from(right: CastleRight) -> Self {
        CastleRights(1 << right as usize)
    }
}

/// An individual castle right for a chess game.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum CastleRight {
    /// White kingside: E1 to G1.
    WhiteKingside,
    /// White queenside: E1 to C1.
    WhiteQueenside,
    /// Black kingside: E8 to G8.
    BlackKingside,
    /// Black queenside: E8 to C8.
    BlackQueenside,
}

impl ops::Not for CastleSide {
    type Output = CastleSide;

    #[inline]
    fn not(self) -> CastleSide {
        (1 - self as u8).into()
    }
}

impl From<CastleRight> for char {
    #[inline]
    fn from(right: CastleRight) -> char {
        b"KQkq"[right as usize] as char
    }
}

impl CastleRight {
    /// Creates a new castle right for `color` and `side`.
    #[inline]
    pub fn new(color: Color, side: CastleSide) -> CastleRight {
        (((color as u8) << 1) | side as u8).into()
    }

    /// Returns a castle right from the parsed character.
    #[inline]
    pub fn from_char(ch: char) -> Option<CastleRight> {
        match ch {
            'K' => Some(CastleRight::WhiteKingside),
            'Q' => Some(CastleRight::WhiteQueenside),
            'k' => Some(CastleRight::BlackKingside),
            'q' => Some(CastleRight::BlackQueenside),
            _ => None,
        }
    }

    /// Returns the path between the rook and king for this right.
    #[inline]
    pub fn path(self) -> Bitboard {
        path::ALL[self as usize]
    }

    /// Returns the color for `self`.
    #[inline]
    pub fn color(self) -> Color {
        ((self as u8) >> 1).into()
    }

    /// Returns the castle side for `self`.
    #[inline]
    pub fn side(self) -> CastleSide {
        (1 & self as u8).into()
    }
}

pub mod path {
    //! The paths between the rook and king for each castling right.
    use super::*;

    /// White kingside path.
    pub const WHITE_KINGSIDE: Bitboard = Bitboard(0x60);

    /// Black kingside path.
    pub const BLACK_KINGSIDE: Bitboard = Bitboard(WHITE_KINGSIDE.0 << 56);

    /// White queenside path.
    pub const WHITE_QUEENSIDE: Bitboard = Bitboard(0x0E);

    /// Black queenside path.
    pub const BLACK_QUEENSIDE: Bitboard = Bitboard(WHITE_QUEENSIDE.0 << 56);

    /// All paths.
    pub static ALL: [Bitboard; 4] = [
        WHITE_KINGSIDE,
        BLACK_KINGSIDE,
        WHITE_QUEENSIDE,
        BLACK_QUEENSIDE,
    ];
}

/// A side used to castle.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum CastleSide {
    /// King castling side (O-O).
    King,
    /// Queen castling side (O-O-O).
    Queen,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn castle_right_new() {
        for &side in &[CastleSide::King, CastleSide::Queen] {
            for &color in &[Color::White, Color::Black] {
                let right = CastleRight::new(color, side);
                assert_eq!(right.side(),  side);
                assert_eq!(right.color(), color);
            }
        }
    }

    #[test]
    fn castle_right_char() {
        for right in CastleRights::FULL {
            let ch = char::from(right);
            assert_eq!(Some(right), CastleRight::from_char(ch));
        }
    }

    #[test]
    fn castle_rights_string() {
        use self::CastleRight::*;

        let pairs = [
            (CastleRights::FULL, "KQkq"),
            (CastleRights::EMPTY, "-"),
            (BlackKingside.into(), "k"),
            (BlackKingside | WhiteQueenside, "Qk"),
        ];

        for &(rights, exp) in &pairs {
            rights.map_str(|s| {
                assert_eq!(s, exp);
                assert_eq!(rights, s.parse().unwrap());
            });
        }
    }
}
