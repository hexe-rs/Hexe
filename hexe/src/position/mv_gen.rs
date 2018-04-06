//! A move generator and options.

use core::mv::kind::*;
use core::mv::MoveVec;
use super::Position;

/// A type that can be used to generate a series of moves.
pub struct MoveGen<'pos, 'buf> {
    pub(super) pos: &'pos Position,
    pub(super) buf: &'buf mut MoveVec,
}

impl<'a, 'b> MoveGen<'a, 'b> {
    /// Generates all legal moves.
    pub fn legal(&mut self) -> &mut Self {
        self
    }

    /// Generates all pseudo-legal castling moves.
    pub fn castle(&mut self) -> &mut Self {
        self
    }
}
