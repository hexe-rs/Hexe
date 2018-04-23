use board::BitBoard;
use square::Square;

mod tables;

#[cfg(feature = "simd")]
pub mod simd {
    use super::*;
    use core::simd::u64x4;

    #[inline]
    fn extract_4(table: &Table, sq: [Square; 4]) -> [&Magic; 4] {
        use misc::Extract;
        [
            sq[0].extract(table),
            sq[1].extract(table),
            sq[2].extract(table),
            sq[3].extract(table),
        ]
    }

    #[inline]
    fn attacks_4([a, b, c, d]: [&Magic; 4], occupied: u64x4, shift: u8) -> u64x4 {
        let mask = u64x4::new(a.mask, b.mask, c.mask, d.mask);
        let num  = u64x4::new(a.num,  b.num,  c.num,  d.num);

        let idx = u64x4::new(
            a.idx as _,
            b.idx as _,
            c.idx as _,
            d.idx as _,
        ) + (((occupied & mask) * num) >> (64 - shift));

        unsafe { u64x4::new(
            *tables::ATTACKS.get_unchecked(idx.extract(0) as usize),
            *tables::ATTACKS.get_unchecked(idx.extract(1) as usize),
            *tables::ATTACKS.get_unchecked(idx.extract(2) as usize),
            *tables::ATTACKS.get_unchecked(idx.extract(3) as usize),
        ) }
    }

    #[inline]
    pub fn rook_attacks_4(sq: [Square; 4], occupied: u64x4) -> u64x4 {
        attacks_4(extract_4(&tables::MAGICS.rook, sq), occupied, 12)
    }

    #[inline]
    pub fn bishop_attacks_4(sq: [Square; 4], occupied: u64x4) -> u64x4 {
        attacks_4(extract_4(&tables::MAGICS.bishop, sq), occupied, 9)
    }
}

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
pub fn rook_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    attacks(&tables::MAGICS.rook, sq, occupied.0, 12).into()
}

#[inline]
pub fn bishop_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    attacks(&tables::MAGICS.bishop, sq, occupied.0, 9).into()
}
