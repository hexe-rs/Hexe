//! A structure for zobrist hashing.

use std::fmt;

type Keys = [u64; 409];

#[cfg(test)]
assert_eq_size!(zobrist_keys_size; Zobrist, Keys);

/// Zobrist keys for hashing.
pub struct Zobrist {
    /// Keys for each piece at each square.
    pub pieces: [[u64; 64]; 6],
    /// Keys for each possible set of castle rights.
    pub castle: [u64; 16],
    /// Keys for each en passant file.
    pub en_passant: [u64; 8],
    /// Key for the playing color.
    pub color: u64,
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

impl Default for Zobrist {
    #[inline]
    fn default() -> Zobrist {
        Zobrist::ZERO
    }
}

impl AsRef<[u64]> for Zobrist {
    #[inline]
    fn as_ref(&self) -> &[u64] {
        self.as_slice()
    }
}

impl AsMut<[u64]> for Zobrist {
    #[inline]
    fn as_mut(&mut self) -> &mut [u64] {
        self.as_mut_slice()
    }
}

impl Zobrist {
    /// An instance with all hashes set to zero.
    pub const ZERO: Zobrist = Zobrist {
        pieces: [[0; 64]; 6],
        castle: [0; 16],
        en_passant: [0; 8],
        color: 0,
    };

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
}
