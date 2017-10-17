/// Castle rights for a chess game.
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
