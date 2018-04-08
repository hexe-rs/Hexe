use super::*;
use rand::{Rng, thread_rng};

macro_rules! sliding_attacks {
    ($($fn:ident)*) => {
        $(#[test]
        fn $fn() {
            let mut rng = thread_rng();
            for occupied in (0..20_000).map(|_| Bitboard(rng.gen())) {
                for square in Square::ALL {
                    let exp = Bitboard::from(square).$fn(!occupied);
                    let res = square.$fn(occupied);
                    if exp != res {
                        panic!(
                            "Square: {}\n\
                             Occupied: {1:?}\n{1}\n\
                             Expected: {2:?}\n{2}\n\
                             Generated: {3:?}\n{3}",
                            square,
                            occupied,
                            exp,
                            res,
                        );
                    }
                }
            }
        })*
    }
}

macro_rules! jump_attacks {
    ($($fn:ident)*) => {
        $(#[test]
        fn $fn() {
            for square in Square::ALL {
                let exp = Bitboard::from(square).$fn();
                let res = square.$fn();
                assert_eq!(exp, res);
            }
        })*
    }
}

sliding_attacks! { rook_attacks bishop_attacks queen_attacks }

jump_attacks! { knight_attacks king_attacks }

#[test]
fn tables_alignment() {
    const ALIGN: usize = 64;

    macro_rules! test {
        ($($field:ident),+) => { $({
            let ptr   = &TABLES.$field as *const _;
            let align = ptr as usize % ALIGN;
            assert_eq!(
                align,
                0,
                concat!(
                    "TABLES.",
                    stringify!($field),
                    " at {:p} is incorrectly aligned"
                ),
                ptr,
            );
        })+ }
    }

    test!(distance, pawns, knight, king, between, line);
}

#[test]
fn distance() {
    fn square(a: Square, b: Square) -> usize {
        use core::cmp::max;
        max(a.file().distance(b.file()), a.rank().distance(b.rank()))
    }

    for s1 in Square::ALL {
        for s2 in Square::ALL {
            assert_eq!(square(s1, s2), s1.distance(s2));
        }
    }
}

#[test]
fn tri_index() {
    for s1 in Square::ALL {
        for s2 in Square::ALL {
            let idx = s1.tri_index(s2);
            assert_eq!(idx, s2.tri_index(s1));
            assert!(idx < TRIANGLE_LEN);
        }
    }
}

#[test]
fn pawn_attacks() {
    for &color in &[Color::White, Color::Black] {
        for square in Square::ALL {
            let exp = Bitboard::from(square).pawn_attacks(color);
            let res = square.pawn_attacks(color);
            assert_eq!(exp, res);
        }
    }
}

#[test]
fn file_from_char() {
    for ch in b'A'..(b'F' + 1) {
        for &ch in &[ch, ch | 32] {
            assert!(File::from_char(ch as _).is_some());
        }
    }
}

#[test]
fn rank_from_char() {
    for ch in b'1'..(b'8' + 1) {
        assert!(Rank::from_char(ch as _).is_some());
    }
}

#[test]
fn square_color() {
    for s1 in Square::ALL {
        for s2 in Square::ALL {
            assert_eq!(s1.color() == s2.color(), s1.color_eq(s2));
        }
    }
    for &(b, c) in &[(Bitboard::WHITE, Color::White),
                     (Bitboard::BLACK, Color::Black)] {
        for s in b {
            assert_eq!(s.color(), c);
        }
    }
}
