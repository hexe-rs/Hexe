//! A structure for zobrist hashing.

use std::{fmt, hash, mem, ptr};

use core::castle::Rights;
use core::misc::Extract;
use core::piece::Role;
use core::square::{File, Square};
use zero::Zero;

mod tables;

const NUM_PIECES:  usize = 6;
const NUM_SQUARES: usize = 64;
const PIECE_TOTAL: usize = NUM_SQUARES * NUM_PIECES;
const NUM_CASTLE:  usize = 0b1111 + 1;
const NUM_COLORS:  usize = 1;
const NUM_EP:      usize = 8;

const NUM_KEYS:  usize = PIECE_TOTAL + NUM_CASTLE + NUM_EP + NUM_COLORS;
const NUM_BYTES: usize = NUM_KEYS * 8;

type Keys = [u64; NUM_KEYS];

type Bytes = [u8; NUM_BYTES];

/// Static pseudorandomly-generated [`Zobrist`](struct.Zobrist.html) keys.
///
/// # Backward Compatibility
///
/// Hashes within this instance are **not** guaranteed to remain the same across
/// different crate versions. Changing hashes is a backward compatible change.
pub static KEYS: Zobrist = tables::STATIC;

/// Keys for [Zobrist hashing][wiki].
///
/// [wiki]: https://en.wikipedia.org/wiki/Zobrist_hashing
#[repr(C)]
pub struct Zobrist {
    /// Keys for each piece at each square.
    pub pieces: [[u64; NUM_SQUARES]; NUM_PIECES],
    /// Keys for each possible set of castle rights.
    pub castle: [u64; NUM_CASTLE],
    /// Keys for each en passant file.
    pub en_passant: [u64; NUM_EP],
    /// Key for the playing color.
    pub color: u64,
}

unsafe impl Zero for Zobrist {}

// Copy is not implemented due to large structure size
impl Clone for Zobrist {
    #[inline]
    fn clone(&self) -> Zobrist {
        // Should be efficient memcpy
        unsafe { ptr::read(self) }
    }
}

impl fmt::Debug for Zobrist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `[u64; 64]` does not implement `fmt::Debug`
        let pieces: [&[u64]; 6] = [
            &self.pieces[0], &self.pieces[1], &self.pieces[2],
            &self.pieces[3], &self.pieces[4], &self.pieces[5],
        ];
        f.debug_struct("Zobrist")
            .field("pieces",     &pieces)
            .field("castle",     &self.castle)
            .field("en_passant", &self.en_passant)
            .field("color",      &self.color)
            .finish()
    }
}

impl hash::Hash for Zobrist {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write(self.as_bytes());
    }
}

impl Default for Zobrist {
    /// Returns an instance with all keys zeroed out.
    #[inline]
    fn default() -> Zobrist {
        unsafe { mem::zeroed() }
    }
}

impl PartialEq for Zobrist {
    #[inline]
    fn eq(&self, other: &Zobrist) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for Zobrist {}

impl AsRef<[u64]> for Zobrist {
    #[inline]
    fn as_ref(&self) -> &[u64] { self.as_slice() }
}

impl AsMut<[u64]> for Zobrist {
    #[inline]
    fn as_mut(&mut self) -> &mut [u64] { self.as_mut_slice() }
}

impl AsRef<[u8]> for Zobrist {
    #[inline]
    fn as_ref(&self) -> &[u8] { self.as_bytes() }
}

impl AsMut<[u8]> for Zobrist {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] { self.as_bytes_mut() }
}

#[cfg(any(test, feature = "rand"))]
impl ::rand::Rand for Zobrist {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Zobrist {
        let mut zobrist = Zobrist::default();
        rng.fill_bytes(zobrist.as_bytes_mut());
        zobrist
    }
}

impl Zobrist {
    /// Returns the key for the piece role at a square.
    #[inline]
    pub fn piece(&self, role: Role, square: Square) -> u64 {
        self.pieces[role as usize][square as usize]
    }

    /// Returns the key for the castle rights.
    #[inline]
    pub fn castle(&self, rights: Rights) -> u64 {
        *rights.extract(&self.castle)
    }

    /// Returns the en passant key for the file.
    #[inline]
    pub fn en_passant(&self, file: File) -> u64 {
        self.en_passant[file as usize]
    }

    /// Clear all hashes by setting them to zero.
    #[inline]
    pub fn clear(&mut self) {
        self.zero();
    }

    /// Returns the zobrist keys as a contiguous slice.
    #[inline]
    pub fn as_slice(&self) -> &[u64] {
        let ptr = self as *const Zobrist as *const Keys;
        unsafe { &*ptr }
    }

    /// Returns the zobrist keys as a contiguous mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u64] {
        let ptr = self as *mut Zobrist as *mut Keys;
        unsafe { &mut *ptr }
    }

    /// Returns the zobrist keys as a contiguous slice of bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        let ptr = self as *const Zobrist as *const Bytes;
        unsafe { &*ptr }
    }

    /// Returns the zobrist keys as a contiguous mutable slice of bytes.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self as *mut Zobrist as *mut Bytes;
        unsafe { &mut *ptr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng, ChaChaRng};

    assert_eq_size!(type_size; Zobrist, Keys, Bytes);

    #[test]
    fn keys_init() {
        const SEED: u32 = 0xDEAD_BEEF;

        let mut rng = ChaChaRng::from_seed(&[SEED]);
        let zobrist = rng.gen::<Zobrist>();
        assert_eq!(zobrist, KEYS);
    }
}
