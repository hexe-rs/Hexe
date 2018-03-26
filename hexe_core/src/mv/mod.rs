//! A chess move.

use castle::Right;
use piece::Promotion as PromotionPiece;
use square::{Rank, Square};

mod vec;
pub use self::vec::*;

const SRC_SHIFT:    usize =  0;
const DST_SHIFT:    usize =  6;
const RANK_SHIFT:   usize =  3;
const PROM_SHIFT:   usize = 12;
const KIND_SHIFT:   usize = 14;
const CASTLE_SHIFT: usize = KIND_SHIFT - 2;

const SRC_MASK:    u16 = 0b111111;
const DST_MASK:    u16 = SRC_MASK;
const PROM_MASK:   u16 = 0b11;
const FILE_MASK:   u16 = 0b000111000111;
const RANK_MASK:   u16 = FILE_MASK << RANK_SHIFT;
const KIND_MASK:   u16 = PROM_MASK;
const CASTLE_MASK: u16 = 0b11;

macro_rules! base_bits {
    ($s1:expr, $s2:expr) => {
        (($s1 as u16) << SRC_SHIFT) | (($s2 as u16) << DST_SHIFT)
    }
}

/// A chess piece move that can either be [`Normal`], [`Promotion`], [`Castle`],
/// or [`EnPassant`].
///
/// [`Normal`]:    ./kind/struct.Normal.html
/// [`Promotion`]: ./kind/struct.Promotion.html
/// [`Castle`]:    ./kind/struct.Castle.html
/// [`EnPassant`]: ./kind/struct.EnPassant.html
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Move(pub(crate) u16);

impl From<Move> for u16 {
    #[inline]
    fn from(mv: Move) -> u16 { mv.0 }
}

impl Move {
    /// Creates a new `Move` from one square to another.
    #[inline]
    pub fn normal(src: Square, dst: Square) -> Move {
        kind::Normal::new(src, dst).into()
    }

    /// Creates a new `Move` from one square to another with a promotion.
    #[inline]
    pub fn promotion(src: Square, dst: Square, piece: PromotionPiece) -> Move {
        kind::Promotion::new(src, dst, piece).into()
    }

    /// Creates a new castle move for `right`.
    #[inline]
    pub fn castle(right: Right) -> Move {
        kind::Castle::new(right).into()
    }

    /// Creates an en passant move from one square to another.
    #[inline]
    pub fn en_passant(src: Square, dst: Square) -> Option<Move> {
        kind::EnPassant::new(src, dst).map(Into::into)
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
    ///     |c| println!("{:?}", c.side()),
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

    macro_rules! impl_from_move {
        ($($t:ty),+) => { $(
            impl From<$t> for Move {
                #[inline]
                fn from(mv: $t) -> Move { mv.0 }
            }

            impl ops::Deref for $t {
                type Target = Move;

                #[inline]
                fn deref(&self) -> &Move { &self.0 }
            }

            impl ops::DerefMut for $t {
                #[inline]
                fn deref_mut(&mut self) -> &mut Move { &mut self.0 }
            }

            impl AsRef<Move> for $t {
                #[inline]
                fn as_ref(&self) -> &Move { self }
            }

            impl AsMut<Move> for $t {
                #[inline]
                fn as_mut(&mut self) -> &mut Move { self }
            }
        )+}
    }

    impl_from_move! { Normal, Castle, Promotion, EnPassant }

    /// A normal, non-special move.
    #[derive(PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Normal(pub(crate) Move);

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

    impl Castle {
        /// Creates a new castle move for `side`.
        #[inline]
        pub fn new(right: Right) -> Castle {
            static SRC_DST: [u16; 4] = [
                base_bits!(Square::E1, Square::G1),
                base_bits!(Square::E1, Square::C1),
                base_bits!(Square::E8, Square::G8),
                base_bits!(Square::E8, Square::C8),
            ];
            let base  = SRC_DST[right as usize];
            let kind  = (MoveKind::Castle as u16) << KIND_SHIFT;
            let right = (right as u16) << CASTLE_SHIFT;
            Castle(Move(base | right | kind))
        }

        /// Returns the castle right for `self`.
        #[inline]
        pub fn right(self) -> Right {
            (((self.0).0 >> CASTLE_SHIFT) & CASTLE_MASK).into()
        }
    }

    /// A promotion move.
    #[derive(PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Promotion(pub(crate) Move);

    impl Promotion {
        /// Creates a new promotion move.
        #[inline]
        pub fn new(src: Square, dst: Square, piece: PromotionPiece) -> Promotion {
            let kind = MoveKind::Promotion;
            Promotion(Move(
                base_bits!(src, dst) |
                (piece as u16) << PROM_SHIFT |
                (kind  as u16) << KIND_SHIFT
            ))
        }

        /// Returns the promotion piece.
        #[inline]
        pub fn piece(self) -> PromotionPiece {
            (((self.0).0 >> PROM_SHIFT) & PROM_MASK).into()
        }
    }

    /// An en passant move.
    #[derive(PartialEq, Eq, Clone, Copy, Hash)]
    pub struct EnPassant(pub(crate) Move);

    impl EnPassant {
        /// Creates a new en passant move.
        #[inline]
        pub fn new(src: Square, dst: Square) -> Option<EnPassant> {
            let val = unsafe { EnPassant::new_unchecked(src, dst) };
            if val.is_legal() { Some(val) } else { None }
        }

        /// Creates a new en passant move without checking whether it is legal.
        #[inline]
        pub unsafe fn new_unchecked(src: Square, dst: Square) -> EnPassant {
            let kind = (MoveKind::EnPassant as u16) << KIND_SHIFT;
            EnPassant(Move(base_bits!(src, dst) | kind))
        }

        /// Returns whether the en passant is legal and is acting on the correct
        /// squares.
        #[inline]
        fn is_legal(self) -> bool {
            let src = self.src();
            let dst = self.dst();

            match (src.file() as i8) - (dst.file() as i8) {
                1 | -1 => (),
                _ => return false,
            }

            const W: u16 = base_bits!(Rank::Five, Rank::Six)   << RANK_SHIFT;
            const B: u16 = base_bits!(Rank::Four, Rank::Three) << RANK_SHIFT;

            let ranks = u16::from(*self) & RANK_MASK;
            ranks == W || ranks == B
        }
    }
}

#[cfg(all(test, nightly))]
mod benches {
    use super::*;
    use test::{Bencher, black_box};

    #[bench]
    fn en_passant_new_1000(b: &mut Bencher) {
        use super::kind::EnPassant;

        let mut squares = [(Square::A1, Square::A1); 1000];
        for s in squares.iter_mut() {
            s.0 = ::rand::random();
            s.1 = ::rand::random();
        }

        b.iter(|| {
            for &(s1, s2) in squares.iter().map(black_box) {
                if let Some(ep) = EnPassant::new(s1, s2) {
                    black_box(ep);
                }
            }
        });
    }
}
