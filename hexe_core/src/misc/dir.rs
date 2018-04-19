use color::Color;
use core::{ops, mem};

impl_rand!(u8 => Direction);

/// A cardinal direction that can be used to shifts or fills.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum Direction {
    /// North (up).
    North,
    /// East (right).
    East,
    /// Northeast (up + right).
    Northeast,
    /// Southeast (down + right).
    Southeast,
    /// Northwest (up + left).
    Northwest,
    /// Southwest (down + left).
    Southwest,
    /// West (left).
    West,
    /// South (down).
    South,
}

impl ops::Not for Direction {
    type Output = Direction;

    #[inline]
    fn not(self) -> Direction {
        unsafe { mem::transmute(7 - self as u8) }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not() {
        use self::Direction::*;
        static NOT: [(Direction, Direction); 4] = [
            (North,     South),
            (East,      West),
            (Northeast, Southwest),
            (Northwest, Southeast),
        ];

        for &(a, b) in &NOT {
            assert_eq!(a, !b);
            assert_eq!(!a, b);
        }
    }
}
