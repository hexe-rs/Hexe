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

macro_rules! base_bits {
    ($s1:expr, $s2:expr) => {
        (($s1 as u16) << SRC_SHIFT) | (($s2 as u16) << DST_SHIFT)
    }
}

const W_EP: u16 = base_bits!(Rank::Five, Rank::Six)   << RANK_SHIFT;
const B_EP: u16 = base_bits!(Rank::Four, Rank::Three) << RANK_SHIFT;

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
        match self.kind() {
            MoveKind::Normal    => kind::Normal(*self).fmt(f),
            MoveKind::Castle    => kind::Castle(*self).fmt(f),
            MoveKind::Promotion => kind::Promotion(*self).fmt(f),
            MoveKind::EnPassant => kind::EnPassant(*self).fmt(f),
        }
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

    /// Returns the result of the match against `self` over each inner type.
    /// Because of how `Move` is represented, this is the best way to safely
    /// match against each variant.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hexe_core::mv::Move;
    /// use hexe_core::square::Square;
    ///
    /// let mv = Move::normal(Square::A1, Square::A7);
    /// mv.matches(
    ///     |n| println!("{:?}", n.src()),
    ///     |c| println!("{:?}", c.right()),
    ///     |p| println!("{:?}", p.piece()),
    ///     |e| println!("{:?}", e.dst()),
    /// );
    /// ```
    #[inline]
    pub fn matches<T, A, B, C, D>(self, a: A, b: B, c: C, d: D) -> T
        where
            A: FnOnce(kind::Normal) -> T,
            B: FnOnce(kind::Castle) -> T,
            C: FnOnce(kind::Promotion) -> T,
            D: FnOnce(kind::EnPassant) -> T,
    {
        match self.kind() {
            MoveKind::Normal    => a(kind::Normal(self)),
            MoveKind::Castle    => b(kind::Castle(self)),
            MoveKind::Promotion => c(kind::Promotion(self)),
            MoveKind::EnPassant => d(kind::EnPassant(self)),
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
    /// [Castling][wiki] move.
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Castling
    Castle,
    /// [Promotion][wiki] move.
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Promotion_(chess)
    Promotion,
    /// [En passant][wiki] move.
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/En_passant
    EnPassant,
}

/// The different underlying kinds of moves.
pub mod kind {
    use super::*;
    use core::ops;

    mod mask {
        use super::*;

        pub const WHITE_KING:  u16 = base_bits!(Square::E1, Square::G1);
        pub const WHITE_QUEEN: u16 = base_bits!(Square::E1, Square::C1);
        pub const BLACK_KING:  u16 = base_bits!(Square::E8, Square::G8);
        pub const BLACK_QUEEN: u16 = base_bits!(Square::E8, Square::C8);

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
            let kind = (MoveKind::Normal as u16) << KIND_SHIFT;
            Normal(Move(base_bits!(src, dst) | kind))
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
            let base  = mask::ALL_RIGHTS[right as usize];
            let kind  = (MoveKind::Castle as u16) << KIND_SHIFT;
            let right = (right as u16) << META_SHIFT;
            Castle(Move(base | right | kind))
        }
    }

    impl Castle {
        /// Creates a new instance for the castle right.
        #[inline]
        pub fn new(right: Right) -> Castle { right.into() }

        /// Attempts to create a new castle move for the given squares.
        #[inline]
        pub fn try_new(src: Square, dst: Square) -> Option<Castle> {
            let base = base_bits!(src, dst);
            let kind = (MoveKind::Castle as u16) << KIND_SHIFT;

            let right = match base {
                mask::WHITE_KING  => Right::WhiteKing,
                mask::WHITE_QUEEN => Right::WhiteQueen,
                mask::BLACK_KING  => Right::BlackKing,
                mask::BLACK_QUEEN => Right::BlackQueen,
                _ => return None,
            };
            let right = (right as u16) << META_SHIFT;

            Some(Castle(Move(base | right | kind)))
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
            const WHITE: u16 = base_bits!(Rank::Seven, Rank::Eight) << RANK_SHIFT;
            const BLACK: u16 = base_bits!(Rank::Two,   Rank::One)   << RANK_SHIFT;
            const KIND:  u16 = (MoveKind::Promotion as u16) << KIND_SHIFT;

            let file = FILE_LO * file as u16;
            let rank = match color {
                Color::White => WHITE,
                Color::Black => BLACK,
            };

            Promotion(Move(file | rank | KIND | (piece as u16) << META_SHIFT))
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
            EnPassant(Move(base_bits!(src, dst) | kind))
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
