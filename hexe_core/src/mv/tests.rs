use super::*;

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
