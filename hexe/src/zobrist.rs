//! A structure for zobrist hashing.

use std::fmt;

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

impl Zobrist {
    /// An instance with all hashes set to zero.
    pub const ZERO: Zobrist = Zobrist {
        pieces: [[0; 64]; 6],
        castle: [0; 16],
        en_passant: [0; 8],
        color: 0,
    };
}
