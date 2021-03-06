//! An inline vector of moves.

use super::*;
use uncon::*;
use core::{cmp, mem, ops, ptr, u8};
use core::borrow::{Borrow, BorrowMut};

const VEC_CAP: usize = MoveVec::MAX_LEN;

/// An inline vector of moves ideal for move generation.
///
/// There is no known case where there have been more than 255 moves for a legal
/// position. Because of this, performing an allocation for a list of generated
/// moves is an avoidable waste of time.
///
/// # Notes
///
/// - When comparing equality of a `MoveVec` to some `[Move]`, place the vector
///   before the slice. This should emit a `memcmp` call which is _much_ faster
///   than `[Move] == [Move]`, which will check each move individually.
#[repr(C)]
pub struct MoveVec {
    /// The internal inline buffer. Uses u16 for convenience.
    buf: [u16; VEC_CAP],
    /// The vector's length.
    len: u8,
}

impl<T: ?Sized + AsRef<[Move]>> PartialEq<T> for MoveVec {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        let this: &[u16] = &self.buf[..self.len as usize];
        let that: &[u16] = unsafe { other.as_ref().into_unchecked() };
        this == that
    }
}

impl Eq for MoveVec {}

impl Clone for MoveVec {
    #[inline]
    fn clone(&self) -> MoveVec {
        unsafe { ptr::read(self) }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        unsafe { ptr::copy_nonoverlapping(source, self, 1) };
    }
}

impl Default for MoveVec {
    #[inline]
    fn default() -> Self {
        MoveVec { buf: unsafe { mem::uninitialized() }, len: 0 }
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

impl Borrow<[Move]> for MoveVec {
    #[inline]
    fn borrow(&self) -> &[Move] { self }
}

impl BorrowMut<[Move]> for MoveVec {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [Move] { self }
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
    /// The maximum length of a vector.
    pub const MAX_LEN: usize = u8::MAX as usize;

    /// Creates a new empty vector.
    #[inline]
    pub fn new() -> MoveVec {
        MoveVec::default()
    }

    /// Creates a new vector with a move repeated `len` times.
    ///
    /// If `len` is greater than the max possible length, the max length will be
    /// used.
    ///
    /// This is analogous to `vec![mv; len]` but for `MoveVec`.
    #[inline]
    pub fn from_elem(mv: Move, len: usize) -> MoveVec {
        MoveVec::from_init(len, |_| mv)
    }

    /// Creates a new `MoveVec` by instantiating each slot with the provided
    /// initializer.
    ///
    /// If `len` is greater than the max possible length, the max length will be
    /// used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexe_core::mv::*;
    /// # use hexe_core::prelude::*;
    /// # use hexe_core::piece::*;
    /// fn random() -> Move {
    ///     # Move::normal(Square::A1, Square::A2)
    ///     /* ... */
    /// }
    ///
    /// // Generate 50 random moves
    /// let vec = MoveVec::from_init(50, |_| random());
    /// ```
    #[inline]
    pub fn from_init<F: FnMut(usize) -> Move>(len: usize, mut init: F) -> MoveVec {
        let mut vec = MoveVec::new();
        vec.len = cmp::min(len, VEC_CAP) as u8;
        for (i, m) in vec.iter_mut().enumerate() {
            unsafe { ptr::write(m, init(i)) };
        }
        vec
    }

    /// Returns the number of moves within the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns whether the vector is empty.
    #[inline]
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// Returns the internal fixed capacity of the vector.
    ///
    /// This is the same value as
    /// [`MoveVec::MAX_LEN`](#associatedconstant.MAX_LEN).
    #[inline]
    pub fn capacity(&self) -> usize { VEC_CAP }

    /// Removes all values from the vector.
    #[inline]
    pub fn clear(&mut self) { self.len = 0 }

    /// Pushes a new move onto the end of the vector, or returns it if full.
    #[inline]
    pub fn push(&mut self, mv: Move) -> Option<Move> {
        if self.len == u8::MAX {
            Some(mv)
        } else {
            unsafe { ptr::write(&mut self.buf[self.len as usize], mv.0) };
            self.len += 1;
            None
        }
    }

    /// Pushes a new move onto the end of the vector. Swaps out the last move
    /// and returns it if full.
    #[inline]
    pub fn push_swap(&mut self, mv: Move) -> Option<Move> {
        self.push(mv).map(|mv| {
            Move(mem::replace(&mut self.buf[VEC_CAP - 1], mv.0))
        })
    }

    /// Pushes a new move onto the end of the vector without checking whether
    /// it is full.
    #[inline]
    pub unsafe fn push_unchecked(&mut self, mv: Move) {
        ptr::write(self.buf.get_unchecked_mut(self.len as usize), mv.0);
        self.len = self.len.wrapping_add(1);
    }

    /// Pops the last move from the end of the vector and returns it.
    #[inline]
    pub fn pop(&mut self) -> Option<Move> {
        if self.len == 0 { None } else {
            self.len -= 1;
            Some(Move(self.buf[self.len as usize]))
        }
    }

    /// Removes the last `n` moves from the vector.
    #[inline]
    pub fn remove_last(&mut self, n: usize) {
        if n < self.len as usize {
            self.len -= n as u8;
        } else {
            self.len = 0;
        }
    }

    /// Shortens the vector, keeping the first `len` moves.
    ///
    /// If `len` is greater than the current length, this has no effect.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        if len < (self.len as usize) {
            self.len = len as u8;
        }
    }

    /// Sets the length of the vector.
    ///
    /// If `len` is greater than the max possible length, the max length will be
    /// used.
    ///
    /// # Safety
    ///
    /// Although it is perfectly safe to shrink the vector this way, one should
    /// use [`truncate`](#method.truncate) instead.
    ///
    /// If used to grow the vector, moves past the previous length must be
    /// initialized via `ptr::write`. Otherwise, [undefined behavior][ub] will
    /// occur.
    ///
    /// [ub]: https://en.wikipedia.org/wiki/Undefined_behavior
    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = cmp::min(len, VEC_CAP) as u8;
    }

    /// Extracts a slice containing the entire vector.
    ///
    /// Equivalent to `&vec[..]`.
    #[inline]
    pub fn as_slice(&self) -> &[Move] { self }

    /// Extracts a mutable slice of the entire vector.
    ///
    /// Equivalent to `&mut vec[..]`.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Move] { self }
}
