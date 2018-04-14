use board::Bitboard;
use square::Square;

mod tables;

type Table = [Magic; 64];

// Fixed shift magic
struct Magic {
    mask: u64,
    num: u64, // Factor
    idx: usize, // Offset
}

impl Magic {
    #[inline]
    fn index(&self, occupied: u64, shift: u8) -> usize {
        let val = (occupied & self.mask).wrapping_mul(self.num);
        ((val >> (64 - shift)) as usize).wrapping_add(self.idx)
    }
}

#[inline]
fn attacks(table: &Table, sq: Square, occupied: u64, shift: u8) -> u64 {
    let index = table[sq as usize].index(occupied, shift);
    unsafe { *tables::ATTACKS.get_unchecked(index) }
}

#[inline]
pub fn rook_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    attacks(&tables::MAGICS.rook, sq, occupied.0, 12).into()
}

#[inline]
pub fn bishop_attacks(sq: Square, occupied: Bitboard) -> Bitboard {
    attacks(&tables::MAGICS.bishop, sq, occupied.0, 9).into()
}
