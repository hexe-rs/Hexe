use board::Bitboard;
use square::Square;

mod tables;

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
fn attacks(magic: &Magic, occupied: u64, shift: u8) -> u64 {
    unsafe { *tables::ATTACKS.get_unchecked(magic.index(occupied, shift)) }
}

#[inline]
pub fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    attacks(&tables::MAGICS.rook[square as usize], occupied.0, 12).into()
}

#[inline]
pub fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    attacks(&tables::MAGICS.bishop[square as usize], occupied.0, 9).into()
}
