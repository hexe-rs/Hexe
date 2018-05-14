use board::BitBoard;
use square::Square;

mod tables;
pub use self::tables::*;

const BISHOP_SHIFT: u8 = 64 - 09;
const ROOK_SHIFT:   u8 = 64 - 12;

#[cfg(feature = "simd")]
pub mod simd {
    macro_rules! sliding {
        ($l:ident, $n:expr, $s:ident, $($tmp:ident),+) => {
            #[allow(non_snake_case)]
            pub mod $l {
                use super::super::{Magic, tables, Table, ROOK_SHIFT, BISHOP_SHIFT};
                use simd::{Level, $l};
                use core::simd::$s;

                pub type Square = <$l as Level>::Square;

                #[inline]
                fn extract(table: &Table, [$($tmp),+]: Square) -> [&Magic; $n] {
                    use misc::Extract;
                    [$($tmp.extract(table)),+]
                }

                #[inline]
                fn attacks([$($tmp),+]: [&Magic; $n], occupied: $s, shift: u8) -> $s {
                    use core::mem;

                    let mask = $s::new($($tmp.mask),+);
                    let num  = $s::new($($tmp.num),+);
                    let idx  = $s::new($($tmp.idx as _),+)
                             + (((occupied & mask) * num) >> shift);

                    let [$($tmp),+] = unsafe {
                        mem::transmute::<_, [u64; $n]>(idx)
                    };

                    unsafe { $s::new(
                        $(*tables::ATTACKS.get_unchecked($tmp as usize)),+
                    ) }
                }

                #[inline]
                pub fn rook_attacks(sq: Square, occupied: $s) -> $s {
                    attacks(extract(&tables::MAGICS.rook, sq), occupied, ROOK_SHIFT)
                }

                #[inline]
                pub fn bishop_attacks(sq: Square, occupied: $s) -> $s {
                    attacks(extract(&tables::MAGICS.bishop, sq), occupied, BISHOP_SHIFT)
                }
            }
        };
    }

    sliding! { L2, 2, u64x2, a, b }
    sliding! { L4, 4, u64x4, a, b, c, d }
    sliding! { L8, 8, u64x8, a, b, c, d, e, f, g, h }
}

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
    unsafe { *tables::ATTACKS.get_unchecked(index) }
}

#[inline]
pub fn rook_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    attacks(&tables::MAGICS.rook, sq, occupied.0, ROOK_SHIFT).into()
}

#[inline]
pub fn bishop_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    attacks(&tables::MAGICS.bishop, sq, occupied.0, BISHOP_SHIFT).into()
}
