/// A cardinal direction that can be used to shift or fill the bits of a
/// [`Bitboard`](struct.Bitboard.html).
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    /// North (up).
    North,
    /// South (down).
    South,
    /// East (right).
    East,
    /// West (left).
    West,
    /// Northeast (up + right).
    Northeast,
    /// Southeast (down + right).
    Southeast,
    /// Northwest (up + left).
    Northwest,
    /// Southwest (down + left).
    Southwest
}
