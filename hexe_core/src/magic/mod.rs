use board::BitBoard;
use square::Square;

mod tables;

#[cfg(feature = "simd")]
pub mod simd {
    use super::*;
    use core::simd::{u64x2, u64x4, u64x8};

    macro_rules! sliding {
        ($m:ident, $n:expr, $s:ident, $($tmp:ident),+) => {
            pub mod $m {
                use super::*;

                pub const FACTOR: usize = $n;

                pub type Squares = [Square; FACTOR];

                #[inline]
                fn extract(table: &Table, [$($tmp),+]: Squares) -> [&Magic; $n] {
                    use misc::Extract;
                    [$($tmp.extract(table)),+]
                }

                #[inline]
                fn attacks([$($tmp),+]: [&Magic; $n], occupied: $s, shift: u8) -> $s {
                    use core::mem;

                    let mask = $s::new($($tmp.mask),+);
                    let num  = $s::new($($tmp.num),+);
                    let idx  = $s::new($($tmp.idx as _),+)
                             + (((occupied & mask) * num) >> (64 - shift));

                    let [$($tmp),+] = unsafe {
                        mem::transmute::<_, [u64; $n]>(idx)
                    };

                    unsafe { $s::new(
                        $(*tables::ATTACKS.get_unchecked($tmp as usize)),+
                    ) }
                }

                #[inline]
                pub fn rook_attacks(sq: Squares, occupied: $s) -> $s {
                    attacks(extract(&tables::MAGICS.rook, sq), occupied, 12)
                }

                #[inline]
                pub fn bishop_attacks(sq: Squares, occupied: $s) -> $s {
                    attacks(extract(&tables::MAGICS.bishop, sq), occupied, 9)
                }
            }
        };
    }

    sliding! { x2, 2, u64x2, a, b }
    sliding! { x4, 4, u64x4, a, b, c, d }
    sliding! { x8, 8, u64x8, a, b, c, d, e, f, g, h }
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
