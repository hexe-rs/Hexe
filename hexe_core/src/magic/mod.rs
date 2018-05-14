use board::BitBoard;
use square::Square;

mod tables;
pub use self::tables::TABLES;

const BISHOP_SHIFT: u8 = 64 - 09;
const ROOK_SHIFT:   u8 = 64 - 12;

type Table = [Magic; 64];

// Fixed shift magic
pub struct Magic {
    pub mask: u64,
    pub num: u64, // Factor
    pub idx: usize, // Offset
}

impl Magic {
    #[inline]
    pub fn index(&self, occupied: u64, shift: u8) -> usize {
        let val = (occupied & self.mask).wrapping_mul(self.num);
        ((val >> shift) as usize).wrapping_add(self.idx)
    }
}

#[inline]
pub fn attacks(table: &Table, sq: Square, occupied: u64, shift: u8) -> u64 {
    let index = table[sq as usize].index(occupied, shift);
    unsafe { *TABLES.attack.get_unchecked(index) }
}

#[inline]
pub fn rook_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    attacks(&TABLES.rook, sq, occupied.0, ROOK_SHIFT).into()
}

#[inline]
pub fn bishop_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    attacks(&TABLES.bishop, sq, occupied.0, BISHOP_SHIFT).into()
}
