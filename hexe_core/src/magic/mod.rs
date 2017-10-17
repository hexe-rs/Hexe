use bitboard::Bitboard;
use square::Square;

mod tables;

pub struct Magic { pub num: u64, pub mask: u64, pub index: u32, pub shift: u8 }

#[inline]
fn attacks(table: &[u64], m: &Magic, occ: Bitboard) -> Bitboard {
    let value = (occ.0 & m.mask).wrapping_mul(m.num) >> m.shift;
    let index = (m.index as usize).wrapping_add(value as usize);
    let board = if cfg!(debug_assertions) {
        table[index]
    } else {
        unsafe { *table.get_unchecked(index) }
    };
    board.into()
}

#[inline]
pub fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    attacks(
        self::tables::rook_attacks(),
        self::tables::rook_magic(square),
        occupied
    )
}

#[inline]
pub fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    attacks(
        self::tables::bishop_attacks(),
        self::tables::bishop_magic(square),
        occupied
    )
}
