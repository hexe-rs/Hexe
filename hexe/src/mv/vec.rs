//! An inline vector of moves.

use super::*;
use uncon::*;
use std::mem;
use std::ops;
use std::u8;

const VEC_CAP: usize = u8::MAX as usize;

/// An inline vector of moves generated by a `Position`.
///
/// There is no known case where there have been more than 255 moves for a legal
/// position. Because of this, performing an allocation for a list of generated
/// moves is an avoidable waste of time.
pub struct MoveVec {
    /// The internal inline buffer. Uses u16 for convenience.
    buf: [u16; VEC_CAP],
    /// The vector's length.
    len: u8,
}

impl PartialEq for MoveVec {
    fn eq(&self, other: &MoveVec) -> bool {
        self.len     == other.len &&
        self.buf[..] == other.buf[..]
    }
}

impl Eq for MoveVec {}

impl Clone for MoveVec {
    #[inline]
    fn clone(&self) -> MoveVec {
        MoveVec { buf: self.buf, len: self.len }
    }
}

impl Default for MoveVec {
    #[inline]
    fn default() -> Self {
        MoveVec {
            buf: unsafe { mem::uninitialized() },
            len: 0,
        }
    }
}

impl AsRef<[Move]> for MoveVec {
    #[inline]
    fn as_ref(&self) -> &[Move] { self }
}

impl AsMut<[Move]> for MoveVec {
    #[inline]
    fn as_mut(&mut self) -> &mut [Move] { self }
}

impl ops::Deref for MoveVec {
    type Target = [Move];

    #[inline]
    fn deref(&self) -> &[Move] {
        let slice = &self.buf[..(self.len as usize)];
        unsafe { slice.into_unchecked() }
    }
}

impl ops::DerefMut for MoveVec {
    #[inline]
    fn deref_mut(&mut self) -> &mut [Move] {
        let slice = &mut self.buf[..(self.len as usize)];
        unsafe { slice.into_unchecked() }
    }
}

impl MoveVec {
    /// Creates a new empty vector.
    #[inline]
    pub fn new() -> MoveVec {
        MoveVec::default()
    }

    /// Creates a new `MoveVec` by instantiating each slot with the provided
    /// initializer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexe::mv::*;
    /// # use hexe::mv::vec::MoveVec;
    /// # use hexe::prelude::*;
    /// # use hexe::core::piece::*;
    /// fn random() -> Move {
    ///     # Move::new(Square::A1, Square::A2, Promotion::Queen, MoveKind::Normal)
    ///     /* ... */
    /// }
    ///
    /// // Generate 50 random moves
    /// let vec = MoveVec::from_init(50, |_| random());
    /// ```
    #[inline]
    pub fn from_init<F: FnMut(usize) -> Move>(len: u8, mut init: F) -> MoveVec {
        let mut vec = MoveVec::new();
        vec.len = len;
        vec.iter_mut().enumerate().for_each(|(i, m)| {
            *m = init(i);
        });
        vec
    }
}
