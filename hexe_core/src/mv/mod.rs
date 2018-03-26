//! A chess move.

use castle::Side;
use piece::Promotion as PromotionPiece;
use square::Square;

mod vec;
pub use self::vec::*;

const SRC_SHIFT:    usize =  0;
const DST_SHIFT:    usize =  6;
const PROM_SHIFT:   usize = 12;
const KIND_SHIFT:   usize = 14;
const CASTLE_SHIFT: usize =  0;

const SRC_MASK:    u16 = 0b111111;
const DST_MASK:    u16 = SRC_MASK;
const PROM_MASK:   u16 = 0b11;
const KIND_MASK:   u16 = PROM_MASK;
const CASTLE_MASK: u16 = 0b1;

macro_rules! base_bits {
    ($s1:expr, $s2:expr) => {
        (($s1 as u16) << SRC_SHIFT) | (($s2 as u16) << DST_SHIFT)
    }
}

/// A chess piece move that can either be normal, a promotion, a king-rook
/// castle, or an en passant.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Move(pub(crate) u16);

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

    /// Creates a new castle move for `side`.
    #[inline]
    pub fn castle(side: Side) -> Move {
        kind::Castle::new(side).into()
    }

    /// Creates an en passant move from one square to another.
    #[inline]
    pub fn en_passant(src: Square, dst: Square) -> Move {
        kind::EnPassant::new(src, dst).into()
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

#[inline]
fn src(bits: u16) -> Square {
    ((bits >> SRC_SHIFT) & SRC_MASK).into()
}

#[inline]
fn dst(bits: u16) -> Square {
    ((bits >> DST_SHIFT) & DST_MASK).into()
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

        /// Returns the source square for `self`.
        #[inline]
        pub fn src(self) -> Square { src((self.0).0) }

        /// Returns the destination square for `self`.
        #[inline]
        pub fn dst(self) -> Square { dst((self.0).0) }
    }

    /// A castling move.
    #[derive(PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Castle(pub(crate) Move);

    impl Castle {
        /// Creates a new castle move for `side`.
        #[inline]
        pub fn new(side: Side) -> Castle {
            let kind = (MoveKind::Castle as u16) << KIND_SHIFT;
            let side = (side as u16) << CASTLE_SHIFT;
            Castle(Move(side | kind))
        }

        /// Returns the castle side for `self`.
        #[inline]
        pub fn side(self) -> Side {
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

        /// Returns the source square for `self`.
        #[inline]
        pub fn src(self) -> Square { src((self.0).0) }

        /// Returns the destination square for `self`.
        #[inline]
        pub fn dst(self) -> Square { dst((self.0).0) }

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
        pub fn new(src: Square, dst: Square) -> EnPassant {
            let kind = (MoveKind::EnPassant as u16) << KIND_SHIFT;
            EnPassant(Move(base_bits!(src, dst) | kind))
        }

        /// Returns the source square for `self`.
        #[inline]
        pub fn src(self) -> Square { src((self.0).0) }

        /// Returns the destination square for `self`.
        #[inline]
        pub fn dst(self) -> Square { dst((self.0).0) }
    }
}
