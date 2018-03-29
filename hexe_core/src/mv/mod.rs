//! A chess move.

use core::fmt;

use uncon::FromUnchecked;

use color::Color;
use castle::Right;
use piece::Promotion as PromotionPiece;
use square::{File, Rank, Square};

mod vec;
pub use self::vec::*;

const SRC_SHIFT:  usize =  0;
const DST_SHIFT:  usize =  6;
const RANK_SHIFT: usize =  3;
const META_SHIFT: usize = 12;
const KIND_SHIFT: usize = 14;

const SRC_MASK:  u16 = 0b111111;
const DST_MASK:  u16 = SRC_MASK;
const META_MASK: u16 = 0b11;
const KIND_MASK: u16 = META_MASK;

const FILE_MASK: u16 = 0b000111000111;
const RANK_MASK: u16 = FILE_MASK << RANK_SHIFT;

const LO_MASK: u16 = 0b111;
const FILE_LO: u16 = FILE_MASK / LO_MASK;

macro_rules! base {
    ($s1:expr, $s2:expr) => {
        (($s1 as u16) << SRC_SHIFT) | (($s2 as u16) << DST_SHIFT)
    }
}

macro_rules! kind {
    ($k:ident) => { (MoveKind::$k as u16) << KIND_SHIFT };
}

macro_rules! meta {
    ($m:expr) => { ($m as u16) << META_SHIFT };
}

const W_EP: u16 = base!(Rank::Five, Rank::Six)   << RANK_SHIFT;
const B_EP: u16 = base!(Rank::Four, Rank::Three) << RANK_SHIFT;

/// A chess piece move from one square to another.
///
/// Each instance has the following memory layout:
///
/// - Source **[6 bits]**
///
/// - Destination **[6 bits]**
///
/// - Meta **[2 bits]** (optional)
///
/// - Kind **[2 bits]** ([`Normal`], [`Promotion`], [`Castle`], [`EnPassant`])
///
/// [`Normal`]:    ./kind/struct.Normal.html
/// [`Promotion`]: ./kind/struct.Promotion.html
/// [`Castle`]:    ./kind/struct.Castle.html
/// [`EnPassant`]: ./kind/struct.EnPassant.html
#[derive(PartialEq, Eq, Clone, Copy, Hash, FromUnchecked)]
pub struct Move(pub(crate) u16);

impl From<Move> for u16 {
    #[inline]
    fn from(mv: Move) -> u16 { mv.0 }
}

impl fmt::Debug for Move {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.matches().fmt(f)
    }
}

impl Move {
    /// Creates a new `Move` from one square to another.
    #[inline]
    pub fn normal(src: Square, dst: Square) -> Move {
        kind::Normal::new(src, dst).into()
    }

    /// Creates a new promotion move for `color` at `file`.
    #[inline]
    pub fn promotion(file: File, color: Color, piece: PromotionPiece) -> Move {
        kind::Promotion::new(file, color, piece).into()
    }

    /// Creates a new castle move for `right`.
    #[inline]
    pub fn castle(right: Right) -> Move {
        kind::Castle::from(right).into()
    }

    /// Creates an en passant move from one square to another.
    #[inline]
    pub fn en_passant(src: Square, dst: Square) -> Option<Move> {
        kind::EnPassant::try_new(src, dst).map(Into::into)
    }

    /// Returns the source square for `self`.
    #[inline]
    pub fn src(self) -> Square {
        ((self.0 >> SRC_SHIFT) & SRC_MASK).into()
    }

    /// Returns the destination square for `self`.
    #[inline]
    pub fn dst(self) -> Square {
        ((self.0 >> DST_SHIFT) & DST_MASK).into()
    }

    /// Returns the kind for `self`.
    #[inline]
    pub fn kind(self) -> MoveKind {
        ((self.0 >> KIND_SHIFT) & KIND_MASK).into()
    }

    /// Returns `self` a castle move if it can be converted into one.
    #[inline]
    pub fn to_castle(self) -> Option<kind::Castle> {
        match self.kind() {
            MoveKind::Castle => Some(kind::Castle(self)),
            _ => kind::Castle::try_new(self.src(), self.dst()),
        }
    }

    /// Returns `self` as an en passant move if it can be converted into one.
    #[inline]
    pub fn to_en_passant(self) -> Option<kind::EnPassant> {
        match self.kind() {
            MoveKind::EnPassant => Some(kind::EnPassant(self)),
            _ => kind::EnPassant::try_new(self.src(), self.dst()),
        }
    }

    /// Returns a `match`-able type that represents the inner variant of `self`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::mv::{Move, Matches};
    /// use hexe_core::square::Square;
    ///
    /// let mv = Move::normal(Square::A1, Square::A7);
    ///
    /// match mv.matches() {
    ///     Matches::Normal(mv)    => println!("{:?}", mv.src()),
    ///     Matches::Castle(mv)    => println!("{:?}", mv.right()),
    ///     Matches::Promotion(mv) => println!("{:?}", mv.piece()),
    ///     Matches::EnPassant(mv) => println!("{:?}", mv.capture()),
    /// }
    /// ```
    #[inline]
    pub fn matches(self) -> Matches {
        match self.kind() {
            MoveKind::Normal    => kind::Normal(self).into(),
            MoveKind::Castle    => kind::Castle(self).into(),
            MoveKind::Promotion => kind::Promotion(self).into(),
            MoveKind::EnPassant => kind::EnPassant(self).into(),
        }
    }
}

/// A chess piece move kind.
#[derive(PartialEq, Eq, Clone, Copy, Hash, FromUnchecked)]
#[uncon(impl_from, other(u16, u32, u64, usize))]
#[repr(u8)]
pub enum MoveKind {
    /// Normal move.
    Normal,
    /// [Castling](https://en.wikipedia.org/wiki/Castling) move.
    Castle,
    /// [Promotion](https://en.wikipedia.org/wiki/Promotion_(chess)) move.
    Promotion,
    /// [En passant](https://en.wikipedia.org/wiki/En_passant) move.
    EnPassant,
}

/// A `match`-able inner representation `Move`.
#[derive(Copy, Clone)]
pub enum Matches {
    /// Normal move.
    Normal(kind::Normal),
    /// [Castling](https://en.wikipedia.org/wiki/Castling) move.
    Castle(kind::Castle),
    /// [Promotion](https://en.wikipedia.org/wiki/Promotion_(chess)) move.
    Promotion(kind::Promotion),
    /// [En passant](https://en.wikipedia.org/wiki/En_passant) move.
    EnPassant(kind::EnPassant),
}

impl From<Move> for Matches {
    #[inline]
    fn from(mv: Move) -> Matches { mv.matches() }
}

macro_rules! impl_matches {
    ($($k:ident, $m:ident, $d:expr;)+) => {
        impl Matches {
            $(
                #[doc = $d]
                pub fn $m(self) -> Option<kind::$k> {
                    if let Matches::$k(mv) = self { Some(mv) } else { None }
                }
            )+
        }
    };
    ($($k:ident, $m:ident;)+) => {
        $(
            impl From<kind::$k> for Matches {
                #[inline]
                fn from(mv: kind::$k) -> Matches { Matches::$k(mv) }
            }
        )+

        impl_matches! { $(
            $k, $m, concat!("Returns the inner `", stringify!($k), "` match.");
        )+ }
    };
}

impl_matches! {
    Normal,    normal;
    Castle,    castle;
    Promotion, promotion;
    EnPassant, en_passant;
}

impl fmt::Debug for Matches {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Matches::Normal(mv)    => mv.fmt(f),
            Matches::Castle(mv)    => mv.fmt(f),
            Matches::Promotion(mv) => mv.fmt(f),
            Matches::EnPassant(mv) => mv.fmt(f),
        }
    }
}

/// The different underlying kinds of moves.
pub mod kind {
    use super::*;
    use core::ops;

    mod mask {
        use super::*;

        pub const WHITE_KING:  u16 = base!(Square::E1, Square::G1);
        pub const WHITE_QUEEN: u16 = base!(Square::E1, Square::C1);
        pub const BLACK_KING:  u16 = base!(Square::E8, Square::G8);
        pub const BLACK_QUEEN: u16 = base!(Square::E8, Square::C8);

        pub static ALL_RIGHTS: [u16; 4] = [
            WHITE_KING, WHITE_QUEEN,
            BLACK_KING, BLACK_QUEEN,
        ];
    }

    macro_rules! impl_from_move {
        ($($t:ident),+) => { $(
            impl From<$t> for Move {
                #[inline]
                fn from(mv: $t) -> Move { mv.0 }
            }

            impl From<$t> for u16 {
                #[inline]
                fn from(mv: $t) -> u16 { (mv.0).0 }
            }

            impl FromUnchecked<Move> for $t {
                #[inline]
                unsafe fn from_unchecked(mv: Move) -> $t { $t(mv) }
            }

            impl FromUnchecked<u16> for $t {
                #[inline]
                unsafe fn from_unchecked(bits: u16) -> $t { $t(Move(bits)) }
            }

            impl ops::Deref for $t {
                type Target = Move;

                #[inline]
                fn deref(&self) -> &Move { &self.0 }
            }

            impl AsRef<Move> for $t {
                #[inline]
                fn as_ref(&self) -> &Move { self }
            }
        )+}
    }

    impl_from_move! { Normal, Castle, Promotion, EnPassant }

    /// A normal, non-special move.
    #[derive(PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Normal(pub(crate) Move);

    impl fmt::Debug for Normal {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Normal").field("src", &self.src())
                                    .field("dst", &self.dst())
                                    .finish()
        }
    }

    impl Normal {
        /// Creates a new normal move from `src` to `dst`.
        #[inline]
        pub fn new(src: Square, dst: Square) -> Normal {
            Normal(Move(base!(src, dst) | kind!(Normal)))
        }
    }

    /// A castling move.
    #[derive(PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Castle(pub(crate) Move);

    impl fmt::Debug for Castle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Castle").field("src", &self.src())
                                    .field("dst", &self.dst())
                                    .field("right", &self.right())
                                    .finish()
        }
    }

    impl From<Right> for Castle {
        #[inline]
        fn from(right: Right) -> Castle {
            let base = mask::ALL_RIGHTS[right as usize];
            Castle(Move(base | meta!(right) | kind!(Castle)))
        }
    }

    impl Castle {
        /// Creates a new instance for the castle right.
        #[inline]
        pub fn new(right: Right) -> Castle { right.into() }

        /// Attempts to create a new castle move for the given squares.
        #[inline]
        pub fn try_new(src: Square, dst: Square) -> Option<Castle> {
            let base  = base!(src, dst);
            let right = match base {
                mask::WHITE_KING  => Right::WhiteKing,
                mask::WHITE_QUEEN => Right::WhiteQueen,
                mask::BLACK_KING  => Right::BlackKing,
                mask::BLACK_QUEEN => Right::BlackQueen,
                _ => return None,
            };
            Some(Castle(Move(base | meta!(right) | kind!(Castle))))
        }

        /// Returns the castle right for `self`.
        #[inline]
        pub fn right(self) -> Right {
            (((self.0).0 >> META_SHIFT) & META_MASK).into()
        }
    }

    /// A promotion move.
    #[derive(PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Promotion(pub(crate) Move);

    impl fmt::Debug for Promotion {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Promotion").field("src", &self.src())
                                       .field("dst", &self.dst())
                                       .field("piece", &self.piece())
                                       .finish()
        }
    }

    impl Promotion {
        /// Creates a new promotion move.
        #[inline]
        pub fn new(file: File, color: Color, piece: PromotionPiece) -> Promotion {
            const WHITE: u16 = base!(Rank::Seven, Rank::Eight) << RANK_SHIFT;
            const BLACK: u16 = base!(Rank::Two,   Rank::One)   << RANK_SHIFT;

            let file = FILE_LO * file as u16;
            let rank = match color {
                Color::White => WHITE,
                Color::Black => BLACK,
            };

            Promotion(Move(file | rank | kind!(Promotion) | meta!(piece)))
        }

        /// Creates a promotion move using `Queen` as its piece.
        #[inline]
        pub fn queen(file: File, color: Color) -> Promotion {
            Promotion::new(file, color, PromotionPiece::Queen)
        }

        /// Returns the color of the moving piece.
        #[inline]
        pub fn color(self) -> Color {
            // src rank is even for white and odd for black
            let inner = u16::from(self);
            ((inner >> RANK_SHIFT) & 1).into()
        }

        /// Returns the promotion piece.
        #[inline]
        pub fn piece(self) -> PromotionPiece {
            (((self.0).0 >> META_SHIFT) & META_MASK).into()
        }
    }

    /// An en passant move.
    #[derive(PartialEq, Eq, Clone, Copy, Hash)]
    pub struct EnPassant(pub(crate) Move);

    impl fmt::Debug for EnPassant {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("EnPassant").field("src", &self.src())
                                       .field("dst", &self.dst())
                                       .finish()
        }
    }

    impl EnPassant {
        /// Attempts to create a new en passant move.
        #[inline]
        pub fn try_new(src: Square, dst: Square) -> Option<EnPassant> {
            let val = unsafe { EnPassant::new_unchecked(src, dst) };
            if val.is_legal() { Some(val) } else { None }
        }

        /// Creates a new en passant move without checking whether it is legal.
        #[inline]
        pub unsafe fn new_unchecked(src: Square, dst: Square) -> EnPassant {
            let kind = (MoveKind::EnPassant as u16) << KIND_SHIFT;
            EnPassant(Move(base!(src, dst) | kind))
        }

        /// Returns the square of the captured piece.
        #[inline]
        pub fn capture(self) -> Square {
            Square::new(self.dst().file(),
                        self.src().rank())
        }

        /// Returns the color of the moving piece.
        #[inline]
        pub fn color(self) -> Color {
            // src rank is even for white and odd for black
            let inner = u16::from(self);
            ((inner >> RANK_SHIFT) & 1).into()
        }

        /// Returns whether the en passant is legal and is acting on the correct
        /// squares.
        #[inline]
        fn is_legal(self) -> bool {
            let ranks = u16::from(self) & RANK_MASK;
            let color = match ranks {
                W_EP => Color::White,
                B_EP => Color::Black,
                _ => return false,
            };
            self.src().pawn_attacks(color).contains(self.dst())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promotion() {
        use prelude::*;

        for file in File::ALL {
            for color in Color::ALL {
                for piece in PromotionPiece::ALL {
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
}

#[cfg(all(test, nightly))]
mod benches {
    use super::*;
    use test::{Bencher, black_box};

    fn gen_squares() -> [(Square, Square); 1000] {
        ::util::rand_pairs()
    }

    #[bench]
    fn en_passant_try_new_1000(b: &mut Bencher) {
        let squares = gen_squares();
        b.iter(|| {
            for &(s1, s2) in squares.iter().map(black_box) {
                if let Some(mv) = kind::EnPassant::try_new(s1, s2) {
                    black_box(mv);
                }
            }
        });
    }

    #[bench]
    fn castle_try_new_1000(b: &mut Bencher) {
        let squares = gen_squares();
        b.iter(|| {
            for &(s1, s2) in squares.iter().map(black_box) {
                if let Some(mv) = kind::Castle::try_new(s1, s2) {
                    black_box(mv);
                }
            }
        });
    }
}
