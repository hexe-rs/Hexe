use color::Color;
use core::ops;
use uncon::IntoUnchecked;

impl_rand!(u8 => Direction);

/// A relative direction that can be used for shifts or fills.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum Direction {
    /// Up only.
    Up,
    /// Right only.
    Right,
    /// Up and right.
    UpRight,
    /// Down and right.
    DownRight,
    /// Up and left.
    UpLeft,
    /// Down and left.
    DownLeft,
    /// Left only.
    Left,
    /// Down only.
    Down,
}

impl ops::Not for Direction {
    type Output = Direction;

    #[inline]
    fn not(self) -> Direction {
        unsafe { (7 - self as u8).into_unchecked() }
    }
}

impl Direction {
    /// Returns `Up` for `White` and `Down` for `Black`.
    #[inline]
    pub fn forward(color: Color) -> Direction {
        Direction::Up.swap(color)
    }

    /// Returns `Down` for `White` and `Up` for `Black`.
    #[inline]
    pub fn backward(color: Color) -> Direction {
        Direction::forward(!color)
    }

    /// Swaps the direction on `Color::Black`.
    #[inline]
    pub fn swap(self, color: Color) -> Direction {
        match color { Color::Black => !self, _ => self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not() {
        use self::Direction::*;
        static NOT: [(Direction, Direction); 4] = [
            (Up,      Down),
            (Right,   Left),
            (UpRight, DownLeft),
            (UpLeft,  DownRight),
        ];

        for &(a, b) in &NOT {
            assert_eq!(a, !b);
            assert_eq!(!a, b);
        }
    }
}
