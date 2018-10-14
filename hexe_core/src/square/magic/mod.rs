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
    // Factor
    pub num: u64,
    // Pointer into attacks table
    pub ptr: &'static u64,
}

impl Magic {
    #[inline]
    unsafe fn get(&self, occupied: u64, shift: u8) -> u64 {
        let val = (occupied & self.mask).wrapping_mul(self.num);
        *(self.ptr as *const u64).offset((val >> shift) as isize)
    }
}

#[inline]
fn attacks(table: &Table, sq: Square, occupied: u64, shift: u8) -> u64 {
    unsafe { table[sq as usize].get(occupied, shift) }
}

#[inline]
pub fn rook_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    attacks(&TABLES.rook, sq, occupied.0, ROOK_SHIFT).into()
}

#[inline]
pub fn bishop_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    attacks(&TABLES.bishop, sq, occupied.0, BISHOP_SHIFT).into()
}
