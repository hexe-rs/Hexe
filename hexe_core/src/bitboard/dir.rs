use color::Color;
use core::ops;

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

impl ops::Not for Direction {
    type Output = Direction;

    #[inline]
    fn not(self) -> Direction {
        use self::Direction::*;
        static NOT: [Direction; 8] = [
            South, North, West, East, Southwest, Northwest, Southeast, Northeast
        ];
        NOT[self as usize]
    }
}

impl Direction {
    /// Returns the forward direction for `color`.
    ///
    /// - `White` becomes `North`
    /// - `Black` becomes `South`
    #[inline]
    pub fn forward(color: Color) -> Direction {
        match color {
            Color::White => Direction::North,
            Color::Black => Direction::South,
        }
    }

    /// Returns the backward direction for `color`.
    ///
    /// - `White` becomes `South`
    /// - `Black` becomes `North`
    #[inline]
    pub fn backward(color: Color) -> Direction {
        Direction::forward(!color)
    }
}
