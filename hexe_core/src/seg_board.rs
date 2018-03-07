//! A bitboard-segmented chess board representations.

use core::ops;

use bitboard::Bitboard;
use color::Color;
use piece::PieceKind;

const NUM_PIECES: usize = 6;
const NUM_COLORS: usize = 2;
const NUM_BOARDS: usize = NUM_PIECES * NUM_COLORS;

/// A full chess board, represented as multiple bitboard segments.
#[derive(Clone)]
pub struct SegBoard {
    boards: [u64; NUM_BOARDS],
}

impl ops::Index<PieceKind> for SegBoard {
    type Output = Bitboard;

    #[inline]
    fn index(&self, kind: PieceKind) -> &Bitboard {
        &self.split().1[kind as usize]
    }
}

impl ops::IndexMut<PieceKind> for SegBoard {
    #[inline]
    fn index_mut(&mut self, kind: PieceKind) -> &mut Bitboard {
        &mut self.split_mut().1[kind as usize]
    }
}

impl ops::Index<Color> for SegBoard {
    type Output = Bitboard;

    #[inline]
    fn index(&self, color: Color) -> &Bitboard {
        &self.split().0[color as usize]
    }
}

impl ops::IndexMut<Color> for SegBoard {
    #[inline]
    fn index_mut(&mut self, color: Color) -> &mut Bitboard {
        &mut self.split_mut().0[color as usize]
    }
}

impl SegBoard {
    /// Returns references to the underlying bitboards for `Color` and
    /// `PieceKind`, respectively.
    #[inline]
    pub fn split(&self) -> (&[Bitboard; NUM_COLORS], &[Bitboard; NUM_PIECES]) {
        let colors = &self.boards[0]          as *const u64 as *const _;
        let pieces = &self.boards[NUM_COLORS] as *const u64 as *const _;
        unsafe { (&*colors, &*pieces) }
    }

    /// Returns mutable references to the underlying bitboards for `Color` and
    /// `PieceKind`, respectively.
    #[inline]
    pub fn split_mut(&mut self) -> (&mut [Bitboard; NUM_COLORS], &mut [Bitboard; NUM_PIECES]) {
        let colors = &mut self.boards[0]          as *mut u64 as *mut _;
        let pieces = &mut self.boards[NUM_COLORS] as *mut u64 as *mut _;
        unsafe { (&mut *colors, &mut *pieces) }
    }
}
