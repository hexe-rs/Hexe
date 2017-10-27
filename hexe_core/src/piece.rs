//! A piece used to play chess.

use core::fmt;
use core::str;

use color::Color;
use uncon::*;

impl_try_from_char! {
    /// The error returned when `try_from` fails for `Piece` or `PieceKind`.
    message = "failed to parse a character as piece or piece kind";
    impl for { Piece PieceKind }
}

/// A chess piece with a kind and color.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
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

static PIECE_CHARS_ASCII: &[u8; 12] = b"PpNnBbRrQqKk";

impl From<Piece> for char {
    #[inline]
    fn from(p: Piece) -> char {
        PIECE_CHARS_ASCII[p as usize] as char
    }
}

impl Piece {
    /// Creates a new `Piece` with a `PieceKind` and `Color`.
    #[inline]
    pub fn new(kind: PieceKind, color: Color) -> Piece {
        unsafe { Piece::from_unchecked((kind as u8) << 1 | color as u8) }
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

    /// Returns the `PieceKind` for the `Piece`.
    #[inline]
    pub fn kind(&self) -> PieceKind {
        unsafe { PieceKind::from_unchecked((*self as u8) >> 1) }
    }

    /// Returns the `Color` for the `Piece`.
    #[inline]
    pub fn color(&self) -> Color {
        ((*self as u8) & 1).into()
    }

    /// Converts `self` into a character.
    #[inline]
    pub fn into_char(self) -> char {
        self.into()
    }
}

/// A chess piece kind.
#[derive(Copy, Clone, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

static KINDS: [&str; 6] = ["Pawn", "Knight", "Bishop", "Rook", "Queen", "King"];

impl fmt::Debug for PieceKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for PieceKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.into_str(), f)
    }
}

impl From<PieceKind> for char {
    #[inline]
    fn from(pk: PieceKind) -> char {
        PIECE_CHARS_ASCII[(pk as usize) << 1] as char
    }
}

impl From<Promotion> for PieceKind {
    #[inline]
    fn from(promotion: Promotion) -> PieceKind {
        unsafe { PieceKind::from_unchecked((promotion as u8) + 1) }
    }
}

/// The error returned when `PieceKind::from_str` fails.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FromStrError(());

static FROM_STR_ERROR: &str = "failed to parse a string as a piece";

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

impl str::FromStr for PieceKind {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<PieceKind, FromStrError> {
        const ERR: FromStrError = FromStrError(());
        const LOW: u8 = 32;
        let bytes = s.as_bytes();

        let (kind, exp, rem): (_, &[_], _) = match bytes.len() {
            1 => return PieceKind::from_char(bytes[0] as char).ok_or(ERR),
            4 => {
                let kind = match bytes[0] | LOW {
                    b'p' => PieceKind::Pawn,
                    b'r' => PieceKind::Rook,
                    b'k' => PieceKind::King,
                    _ => return Err(ERR),
                };
                (kind, &kind.into_str().as_bytes()[1..], &bytes[1..])
            },
            5 => (PieceKind::Queen, b"queen", bytes),
            6 => {
                let kind = match bytes[0] | LOW {
                    b'k' => PieceKind::Knight,
                    b'b' => PieceKind::Bishop,
                    _ => return Err(ERR),
                };
                (kind, &kind.into_str().as_bytes()[1..], &bytes[1..])
            },
            _ => return Err(ERR),
        };

        for (&a, &b) in rem.iter().zip(exp.iter()) {
            if a | LOW != b {
                return Err(ERR);
            }
        }
        Ok(kind)
    }
}

impl PieceKind {
    /// Returns a piece kind from the parsed character.
    pub fn from_char(ch: char) -> Option<PieceKind> {
        use self::PieceKind::*;
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
        KINDS[self as usize]
    }

    /// Converts `self` into a character.
    #[inline]
    pub fn into_char(self) -> char {
        self.into()
    }

    /// The kind is a promotion.
    #[inline]
    pub fn is_promotion(&self) -> bool {
        use self::PieceKind::*;
        *self == Knight || *self == Bishop || *self == Rook || *self == Queen
    }
}

/// A promotion piece kind.
#[derive(Copy, Clone, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum Promotion { Knight, Bishop, Rook, Queen }

impl FromUnchecked<PieceKind> for Promotion {
    #[inline]
    unsafe fn from_unchecked(pk: PieceKind) -> Promotion {
        Promotion::from_unchecked((pk as u8) - 1)
    }
}

impl Promotion {
    /// Returns a promotion for the piece kind, if possible.
    #[inline]
    pub fn from_kind(pk: PieceKind) -> Option<Promotion> {
        if pk.is_promotion() {
            unsafe { Some(pk.into_unchecked()) }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static CHARS: [char; 6] = ['P', 'N', 'B', 'R', 'Q', 'K'];

    #[test]
    fn piece_kind_char() {
        for i in 0..6 {
            let ch = CHARS[i];
            let pk = PieceKind::from(i);
            assert_eq!(pk.into_char(), ch);
        }
    }

    #[test]
    fn piece_kind_from_str() {
        for pk in (0..6u8).map(PieceKind::from) {
            assert_eq!(
                Some(pk),
                pk.into_str().parse().ok()
            );
        }

        for (i, ch) in CHARS.iter().enumerate() {
            assert_eq!(
                Some(PieceKind::from(i)),
                ch.encode_utf8(&mut [0; 1]).parse().ok()
            );
        }
    }
}
