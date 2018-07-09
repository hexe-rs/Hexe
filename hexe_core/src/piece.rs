//! A piece used to play chess.

use core::{fmt, str};

use uncon::*;
#[cfg(feature = "serde")]
use serde::*;

use prelude::{Color, Extract};

impl_rand!(u8 => Piece, Role, Promotion);

/// A chess piece with a role and color.
#[derive(Copy, Clone, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

static PIECE_CHARS_ASCII: [u8; 12] = *b"PpNnBbRrQqKk";

impl From<Piece> for char {
    #[inline]
    fn from(p: Piece) -> char {
        PIECE_CHARS_ASCII[p as usize] as char
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.color().into_str(), self.role().into_str())
    }
}

impl Piece {
    /// Creates a new `Piece` with a `Role` and `Color`.
    #[inline]
    pub fn new(role: Role, color: Color) -> Piece {
        unsafe { Piece::from_unchecked((role as u8) << 1 | color as u8) }
    }

    /// Returns a piece from the parsed character.
    #[inline]
    pub fn from_char(ch: char) -> Option<Piece> {
        use self::Piece::*;
        let pc = match ch {
            'P' => WhitePawn,   'p' => BlackPawn,
            'N' => WhiteKnight, 'n' => BlackKnight,
            'B' => WhiteBishop, 'b' => BlackBishop,
            'R' => WhiteRook,   'r' => BlackRook,
            'Q' => WhiteQueen,  'q' => BlackQueen,
            'K' => WhiteKing,   'k' => BlackKing,
            _ => return None,
        };
        Some(pc)
    }

    /// Returns the `Role` for the `Piece`.
    #[inline]
    pub fn role(self) -> Role {
        unsafe { Role::from_unchecked((self as u8) >> 1) }
    }

    /// Returns the `Color` for the `Piece`.
    #[inline]
    pub fn color(self) -> Color {
        (1 & self as u8).into()
    }

    /// Converts `self` into a character.
    #[inline]
    pub fn into_char(self) -> char {
        self.into()
    }
}

/// A chess piece role.
#[derive(Copy, Clone, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl_ord!(Role);

static ROLES: [&str; 6] = ["Pawn", "Knight", "Bishop", "Rook", "Queen", "King"];

impl fmt::Debug for Role {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Role {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.into_str().fmt(f)
    }
}

impl From<Role> for char {
    #[inline]
    fn from(role: Role) -> char {
        PIECE_CHARS_ASCII[(role as usize) << 1] as char
    }
}

impl From<Promotion> for Role {
    #[inline]
    fn from(promotion: Promotion) -> Role {
        unsafe { Role::from_unchecked((promotion as u8) + 1) }
    }
}

define_from_str_error! { Role;
    /// The error returned when `Role::from_str` fails.
    "failed to parse a string as a piece role"
}

impl str::FromStr for Role {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Role, FromStrError> {
        const ERR: FromStrError = FromStrError(());
        const LOW: u8 = 32;
        let bytes = s.as_bytes();

        let (role, exp, rem): (_, &[_], _) = match bytes.len() {
            1 => return Role::from_char(bytes[0] as char).ok_or(ERR),
            4 => {
                let role = match bytes[0] | LOW {
                    b'p' => Role::Pawn,
                    b'r' => Role::Rook,
                    b'k' => Role::King,
                    _ => return Err(ERR),
                };
                (role, &role.into_str().as_bytes()[1..], &bytes[1..])
            },
            5 => (Role::Queen, b"queen", bytes),
            6 => {
                let role = match bytes[0] | LOW {
                    b'k' => Role::Knight,
                    b'b' => Role::Bishop,
                    _ => return Err(ERR),
                };
                (role, &role.into_str().as_bytes()[1..], &bytes[1..])
            },
            _ => return Err(ERR),
        };

        for (&a, &b) in rem.iter().zip(exp.iter()) {
            if a | LOW != b {
                return Err(ERR);
            }
        }
        Ok(role)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Role {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(self.into_str())
    }
}

impl Role {
    /// Returns a piece role from the parsed character.
    pub fn from_char(ch: char) -> Option<Role> {
        use self::Role::*;
        match 32 | ch as u8 {
            b'p' => Some(Pawn),
            b'n' => Some(Knight),
            b'b' => Some(Bishop),
            b'r' => Some(Rook),
            b'q' => Some(Queen),
            b'k' => Some(King),
            _ => None,
        }
    }

    /// Converts `self` into a static string.
    #[inline]
    pub fn into_str(self) -> &'static str {
        *self.extract(&ROLES)
    }

    /// Converts `self` into a character.
    #[inline]
    pub fn into_char(self) -> char {
        self.into()
    }

    /// Returns whether `self` is a piece role that can slide across the board.
    #[inline]
    pub fn is_slider(self) -> bool {
        match self {
            Role::Rook | Role::Bishop | Role::Queen => true,
            Role::Pawn | Role::Knight | Role::King  => false,
        }
    }

    /// The role is a promotion.
    #[inline]
    pub fn is_promotion(self) -> bool {
        // Pawn wraps around to 0xFF
        (self as u8).wrapping_sub(1) < Role::Queen as u8
    }
}

/// A promotion piece role.
#[derive(Copy, Clone, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Promotion { Knight, Bishop, Rook, Queen }

impl fmt::Debug for Promotion {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.into_str(), f)
    }
}

impl Default for Promotion {
    /// Returns the default queen promotion piece.
    #[inline]
    fn default() -> Promotion { Promotion::Queen }
}

impl FromUnchecked<Role> for Promotion {
    #[inline]
    unsafe fn from_unchecked(role: Role) -> Promotion {
        Promotion::from_unchecked((role as u8) - 1)
    }
}

impl From<Promotion> for char {
    #[inline]
    fn from(prom: Promotion) -> char {
        Role::from(prom).into()
    }
}

impl Promotion {
    /// Returns a promotion for the piece role, if possible.
    #[inline]
    pub fn from_role(role: Role) -> Option<Promotion> {
        if role.is_promotion() {
            unsafe { Some(role.into_unchecked()) }
        } else {
            None
        }
    }

    /// Converts `self` into a static string.
    #[inline]
    pub fn into_str(self) -> &'static str {
        ROLES[1..][self as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static CHARS: [char; 6] = ['P', 'N', 'B', 'R', 'Q', 'K'];

    #[test]
    fn promotion_string() {
        use self::Promotion::*;

        for &prom in &[Knight, Bishop, Rook, Queen] {
            assert_eq!(prom.into_str(), Role::from(prom).into_str());
        }
    }

    #[test]
    fn piece_role_char() {
        for (i, &ch) in CHARS.iter().enumerate() {
            let role = Role::from(i);
            assert_eq!(role.into_char(), ch);
        }
    }

    #[test]
    fn piece_role_from_str() {
        for role in (0..6u8).map(Role::from) {
            assert_eq!(
                Some(role),
                role.into_str().parse().ok()
            );
        }

        for (i, ch) in CHARS.iter().enumerate() {
            assert_eq!(
                Some(Role::from(i)),
                ch.encode_utf8(&mut [0; 1]).parse().ok()
            );
        }
    }
}
