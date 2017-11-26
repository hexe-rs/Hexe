use bitboard::Bitboard;
use square::Square;

mod tables;

// Fixed shift magic
struct Magic {
    mask: u64,
    num: u64, // Factor
    idx: u32, // Offset
}

#[inline]
fn attacks(magic: &Magic, occupied: Bitboard, shift: u8) -> u64 {
    let val = (occupied.0 & magic.mask).wrapping_mul(magic.num) >> (64 - shift);
    let idx = (val as usize).wrapping_add(magic.idx as usize);
    unsafe { *tables::ATTACKS.get_unchecked(idx) }
}

#[inline]
pub fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    attacks(&tables::MAGICS[0][square as usize], occupied, 12).into()
}

#[inline]
pub fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    attacks(&tables::MAGICS[1][square as usize], occupied, 9).into()
}
