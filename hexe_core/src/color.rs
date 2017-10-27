//! A color to represent pieces or board squares.

use core::fmt;
use core::str;

#[cfg(feature = "serde")]
use serde::*;

/// A black or white color.
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum Color {
    /// White color.
    White,
    /// Black color.
    Black,
}

static COLORS: [[u8; 5]; 2] = [*b"White", *b"Black"];

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.into_str(), f)
    }
}

/// The error returned when `Color::from_str` fails.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FromStrError(());

static FROM_STR_ERROR: &str = "failed to parse a string as a color";

impl fmt::Display for FromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(FROM_STR_ERROR, f)
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for FromStrError {
    fn description(&self) -> &str {
        FROM_STR_ERROR
    }
}

impl str::FromStr for Color {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Color, FromStrError> {
        const ERR: Result<Color, FromStrError> = Err(FromStrError(()));
        const LOW: u8 = 32;
        if s.len() == 0 { ERR } else {
            let bytes = s.as_bytes();
            // Compare against ASCII lowercase
            let (color, exp) = match bytes[0] | LOW {
                b'w' => (Color::White, &COLORS[0][1..]),
                b'b' => (Color::Black, &COLORS[1][1..]),
                _ => return ERR,
            };
            let rem = &bytes[1..];
            if rem.len() == exp.len() {
                for (&a, &b) in rem.iter().zip(exp.iter()) {
                    // Lowercase comparison
                    if a | LOW != b {
                        return ERR;
                    }
                }
            } else if rem.len() != 0 {
                return ERR;
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

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        <&str>::deserialize(de)?.parse().map_err(|_| {
            de::Error::custom(FROM_STR_ERROR)
        })
    }
}

impl_try_from_char! {
    /// The error returned when `try_from` fails for `Color`.
    message = "failed to parse a character as a color";
    impl for { Color }
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
}
