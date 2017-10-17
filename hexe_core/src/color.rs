//! A color to represent pieces or board squares.

use core::fmt;
use core::str::FromStr;

/// A black or white color.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum Color {
    /// White color.
    White,
    /// Black color.
    Black,
}

/// The error returned when `Color::from_str` fails.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FromStrError(());

static FROM_STR_ERROR: &str = "failed to parse a string as a color";

impl fmt::Display for FromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", FROM_STR_ERROR)
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for FromStrError {
    fn description(&self) -> &str {
        FROM_STR_ERROR
    }
}

impl FromStr for Color {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Color, FromStrError> {
        if s.len() > 1 {
            let (color, exp) = match s.as_bytes()[0] | 32 {
                b'w' => (Color::White, "hite"),
                b'b' => (Color::Black, "lack"),
                _ => return Err(FromStrError(())),
            };
            // We know that the first character is either "w" or "b"
            let rem = unsafe { s.get_unchecked(1..) };
            if rem.len() == 4 {
                for (&a, &b) in rem.as_bytes().iter().zip(exp.as_bytes().iter()) {
                    if a | 32 != b {
                        return Err(FromStrError(()));
                    }
                }
            } else if rem.len() != 0 {
                return Err(FromStrError(()));
            }
            Ok(color)
        } else {
            Err(FromStrError(()))
        }
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
        ];

        static FAILS: &[&str] = &[
            "whit",  "blac",
            "whits", "block",
        ];

        for &(s, c) in STRINGS {
            assert_eq!(s.parse().ok(), Some(c));
        }

        for &f in FAILS {
            assert_eq!(Color::from_str(f).ok(), None);
        }
    }
}
