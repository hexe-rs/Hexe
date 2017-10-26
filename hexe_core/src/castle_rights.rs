//! Castling rights for two players of a chess game.

use core::fmt;

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
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct CastleRights(u8);

impl Default for CastleRights {
    #[inline]
    fn default() -> CastleRights {
        CastleRights(0b1111)
    }
}

impl fmt::Debug for CastleRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 2 for "0b" + 4 for number
        write!(f, "CastleRights({:#06b})", self.0)
    }
}

impl CastleRights {
    /// White kingside.
    pub const WHITE_KINGSIDE: CastleRights = CastleRights(0b0001);

    /// Black kingside.
    pub const BLACK_KINGSIDE: CastleRights = CastleRights(0b0010);

    /// White queenside.
    pub const WHITE_QUEENSIDE: CastleRights = CastleRights(0b0100);

    /// Black queenside.
    pub const BLACK_QUEENSIDE: CastleRights = CastleRights(0b1000);
}

impl_bit_set! { CastleRights => CastleRight }

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
    WhiteKingside,
    BlackKingside,
    WhiteQueenside,
    BlackQueenside,
}
