//! A color to represent pieces or board squares.
//!
//! Chess deals primarily with two colors: white and black. They're used to
//! categorize players, pieces, and board squares.
//!
//! Because [`Color`] is an `enum`, it can be used to safely index into tables
//! of size 2 or greater. The optimizer knows that a color never results in an
//! index greater than 2, and thus it can safely remove bounds checks where
//! applicable.
//!
//! # Examples
//!
//! We can use color to determine various aspects of a game. If two bishops lie
//! on squares of opposing color, we know that one can **never** capture the
//! other. For example, bishops that start on opposite sides of the board cannot
//! capture each other:
//!
//! ```
//! # use hexe_core::prelude::*;
//! // Bishop queenside start squares
//! let bis1 = Square::C1;
//! let bis2 = Square::C8;
//!
//! assert!(!bis1.color_eq(bis2));
//! ```
//!
//! [`Color`]: enum.Color.html

use core::{fmt, ops, str};

#[cfg(feature = "serde")]
use serde::*;

/// A black or white color.
#[derive(Copy, Clone, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum Color {
    /// White color.
    White,
    /// Black color.
    Black,
}

impl_ord!(Color);

static COLORS: [[u8; 5]; 2] = [*b"White", *b"Black"];

#[cfg(any(test, feature = "rand"))]
impl ::rand::Rand for Color {
    #[inline]
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
        if bool::rand(rng) {
            Color::White
        } else {
            Color::Black
        }
    }
}

impl fmt::Debug for Color {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Color {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.into_str().fmt(f)
    }
}

define_from_str_error! { Color;
    /// The error returned when `Color::from_str` fails.
    "failed to parse a string as a color"
}

impl str::FromStr for Color {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Color, FromStrError> {
        const ERR: FromStrError = FromStrError(());
        const LOW: u8 = 32;
        if s.is_empty() { Err(ERR) } else {
            let bytes = s.as_bytes();
            // Compare against ASCII lowercase
            let (color, exp) = match bytes[0] | LOW {
                b'w' => (Color::White, &COLORS[0][1..]),
                b'b' => (Color::Black, &COLORS[1][1..]),
                _ => return Err(ERR),
            };
            let rem = &bytes[1..];
            if rem.len() == exp.len() {
                for (&a, &b) in rem.iter().zip(exp.iter()) {
                    // Lowercase comparison
                    if a | LOW != b {
                        return Err(ERR);
                    }
                }
            } else if !rem.is_empty() {
                return Err(ERR);
            }
            Ok(color)
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Color {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(self.into_str())
    }
}

impl ops::Not for Color {
    type Output = Color;

    #[inline]
    fn not(self) -> Color {
        (1 - self as u8).into()
    }
}

impl Color {
    /// Returns a color from the parsed character.
    #[inline]
    pub fn from_char(ch: char) -> Option<Color> {
        match 32 | ch as u8 {
            b'w' => Some(Color::White),
            b'b' => Some(Color::Black),
            _ => None,
        }
    }

    /// Converts `self` into a static string.
    #[inline]
    pub fn into_str(self) -> &'static str {
        unsafe { str::from_utf8_unchecked(&COLORS[self as usize]) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        use self::Color::*;

        static STRINGS: &[(&str, Color)] = &[
            ("white", White), ("black", Black),
            ("WHITE", White), ("BLACK", Black),
            ("wHiTe", White), ("BlAcK", Black),
            ("w", White),     ("b", Black),
            ("W", White),     ("B", Black),
        ];

        static FAILS: &[&str] = &[
            "whit",  "blac",
            "whits", "block",
            "a", "c", "d"
        ];

        for &(s, c) in STRINGS {
            assert_eq!(s.parse().ok(), Some(c));
        }

        for &f in FAILS {
            assert_eq!(f.parse::<Color>().ok(), None);
        }
    }

    #[test]
    fn from_char() {
        use self::Color::*;

        static CHARS: [(char, Color); 4] = [
            ('w', White), ('W', White),
            ('b', Black), ('B', Black),
        ];

        for &(ch, color) in &CHARS {
            assert_eq!(Color::from_char(ch), Some(color));
        }
    }
}

#[cfg(all(test, nightly))]
mod benches {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn from_str(b: &mut Bencher) {
        static STRINGS: &[&str] = &[
            "white", "whitE", "whiTE", "whITE", "wHITE", "WHITE",
            "black", "blacK", "blaCK", "blACK", "bLACK", "BLACK",
        ];
        b.iter(|| {
            for &s in STRINGS {
                let _: Result<Color, _> = black_box(black_box(s).parse());
            }
        });
    }
}
