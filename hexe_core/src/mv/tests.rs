use super::*;

#[test]
fn castle() {
    use prelude::*;

    for right in Right::ALL {
        let (src, dst) = match right {
            Right::WhiteKing  => (Square::E1, Square::G1),
            Right::WhiteQueen => (Square::E1, Square::C1),
            Right::BlackKing  => (Square::E8, Square::G8),
            Right::BlackQueen => (Square::E8, Square::C8),
        };
        let mv = kind::Castle::new(right);
        assert_eq!(mv.right(), right, "{:?}", mv);
        assert_eq!(mv.src(),   src,   "{:?}", mv);
        assert_eq!(mv.dst(),   dst,   "{:?}", mv);
    }
}

#[test]
fn promotion() {
    use prelude::*;

    for file in File::ALL {
        for color in Color::ALL {
            for piece in piece::Promotion::ALL {
                let mv = kind::Promotion::new(file, color, piece);
                assert_eq!(file, mv.src().file());
                assert_eq!(file, mv.dst().file());
                assert_eq!(piece, mv.piece());
                match color {
                    Color::White => {
                        assert_eq!(Rank::Seven, mv.src().rank());
                        assert_eq!(Rank::Eight, mv.dst().rank());
                    },
                    Color::Black => {
                        assert_eq!(Rank::Two, mv.src().rank());
                        assert_eq!(Rank::One, mv.dst().rank());
                    },
                }
            }
        }
    }
}
