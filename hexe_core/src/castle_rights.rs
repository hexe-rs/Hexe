//! Castling rights for two players of a chess game.

/// Castle rights for a chess game.
///
/// # Examples
///
/// Similar to with [`Bitboard`], castle rights can be composed with set
/// operations.
///
/// ```
/// # use hexe_core::prelude::*;
/// assert!(
///     CastleRight::WhiteKingside   | CastleRight::WhiteQueenside ==
///     CastleRights::WHITE_KINGSIDE | CastleRights::WHITE_QUEENSIDE
/// );
/// ```
///
/// [`Bitboard`]: ../bitboard/struct.Bitboard.html
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct CastleRights(u8);

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

impl_set_ops! { CastleRights }

impl_composition_ops! { CastleRights => CastleRight }

impl From<CastleRight> for CastleRights {
    #[inline]
    fn from(right: CastleRight) -> Self {
        CastleRights(1 << right as usize)
    }
}

/// An individual castle right for a chess game.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum CastleRight {
    WhiteKingside,
    BlackKingside,
    WhiteQueenside,
    BlackQueenside,
}
