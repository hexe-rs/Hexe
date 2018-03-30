use super::*;
use prelude::*;

/// Asserts at compile-time that the piece is less than NONE.
macro_rules! assert_valid_none {
    ($($p:ident)+) => {
        const_assert!(valid_none; $((Piece::$p as u8) < NONE),+);
    }
}

assert_valid_none! {
    WhitePawn   BlackPawn
    WhiteKnight BlackKnight
    WhiteBishop BlackBishop
    WhiteRook   BlackRook
    WhiteQueen  BlackQueen
    WhiteKing   BlackKing
}

#[test]
fn len() {
    let mut map = PieceMap::new();

    macro_rules! assert_len {
        ($l:expr) => {
            assert_eq!(map.len(), $l);
            assert_eq!(map.iter().len(), $l);
            assert_eq!(map.iter_mut().len(), $l);
        }
    }

    assert_len!(0);

    map.insert(Square::A1, Piece::WhitePawn);
    assert_len!(1);

    map = PieceMap::STANDARD;
    assert_len!(32);

    map = PieceMap::filled(Piece::BlackBishop);
    assert_len!(64);

    let mut iter = map.iter();
    for _ in iter.by_ref().take(16) {}

    assert_eq!(iter.len(), 48);
}

#[test]
fn rank_contains() {
    let map = PieceMap::STANDARD;

    let pairs = [
        // White
        (Rank::Two, Piece::WhitePawn),
        (Rank::One, Piece::WhiteKnight),
        (Rank::One, Piece::WhiteBishop),
        (Rank::One, Piece::WhiteRook),
        (Rank::One, Piece::WhiteQueen),
        (Rank::One, Piece::WhiteKing),
        // Black
        (Rank::Seven, Piece::BlackPawn),
        (Rank::Eight, Piece::BlackKnight),
        (Rank::Eight, Piece::BlackBishop),
        (Rank::Eight, Piece::BlackRook),
        (Rank::Eight, Piece::BlackQueen),
        (Rank::Eight, Piece::BlackKing),
    ];

    for &(rank, piece) in &pairs {
        assert!(
            map.rank_contains(rank, piece),
            "Rank::{:?} does not contain {:?} in\n{}",
            rank, piece, map
        );
        for rank in (0..8u8).map(Rank::from).filter(|&r| r != rank) {
            assert!(!map.rank_contains(rank, piece));
        }
    }
}

#[test]
fn is_empty() {
    let mut map = PieceMap::new();
    assert!(map.is_empty());

    map.insert(Square::H8, Piece::WhitePawn);
    assert!(!map.is_empty());

    map = PieceMap::filled(Piece::BlackBishop);
    assert!(!map.is_empty());
}

#[test]
fn role_at() {
    let map = PieceMap::STANDARD;
    for sq in Square::ALL {
        let role = map.get(sq).map(|p| p.role());
        assert_eq!(map.role_at(sq), role);
    }
}

#[test]
fn fen() {
    let odd = {
        let mut map = PieceMap::STANDARD;
        map.swap(Square::D7, Square::E1);
        map.remove(Square::B1);
        map.remove(Square::C1);
        map.remove(Square::G8);
        (map, "rnbqkb1r/pppKpppp/8/8/8/8/PPPPPPPP/R2QpBNR")
    };

    let maps = [
        (PieceMap::STANDARD, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
        (PieceMap::EMPTY,    "8/8/8/8/8/8/8/8"),
        odd,
    ];

    for &(ref map, exp) in &maps {
        assert_eq!(
            Some(map),
            PieceMap::from_fen(exp).as_ref()
        );

        map.map_fen(|s| assert_eq!(s, exp));
    }

    let fails = [
        "",
        "8/8/8/8/8/8/8",
        "/8/8/8/8/8/8/8",
        "8/8/8/8//8/8/8",
        "8/8/8/8/8/8/8/",
        "8/8/8/8/8/8/8/7",
        "8/8/8/8/8/8/8/9",
        "//////",
        "///////",
        "////////",
    ];

    for &fail in &fails {
        assert_eq!(None, PieceMap::from_fen(fail));
    }
}

#[test]
fn castle() {
    fn affected_range(right: Right) -> ops::Range<usize> {
        match right {
            Right::WhiteKing  => 4..8,
            Right::WhiteQueen => 0..5,
            Right::BlackKing  => 60..64,
            Right::BlackQueen => 56..61,
        }
    }

    let original = PieceMap::STANDARD;

    macro_rules! test {
        (
            right: $right:ident;
            clear: $($cs:ident),+;
            empty: $($es:ident),+;
            king:  $king:ident;
            rook:  $rook:ident;
        ) => { {
            let mut map = original.clone();
            let right   = Right::$right;
            let color   = right.color();
            let king    = Piece::new(Role::King, color);
            let rook    = Piece::new(Role::Rook, color);
            $(map.remove(Square::$cs);)+
            map.castle(right);
            $(assert!(!map.contains(Square::$es));)+
            assert_eq!(map[Square::$king], king);
            assert_eq!(map[Square::$rook], rook);

            let range = affected_range(right);
            let start = ..range.start;
            let end   = range.end..SQUARE_NUM;

            assert_eq!(&map.as_bytes()[start.clone()],
                       &original.as_bytes()[start]);

            assert_eq!(&map.as_bytes()[end.clone()],
                       &original.as_bytes()[end]);
        } }
    }
    test! {
        right: WhiteQueen;
        clear: B1, C1, D1;
        empty: A1, B1, E1;
        king:  C1;
        rook:  D1;
    }
    test! {
        right: WhiteKing;
        clear: F1, G1;
        empty: E1, H1;
        king:  G1;
        rook:  F1;
    }
    test! {
        right: BlackQueen;
        clear: B8, C8, D8;
        empty: A8, B8, E8;
        king:  C8;
        rook:  D8;
    }
    test! {
        right: BlackKing;
        clear: F8, G8;
        empty: E8, H8;
        king:  G8;
        rook:  F8;
    }
}
