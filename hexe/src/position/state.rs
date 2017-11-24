use super::*;
use std::sync::Arc;

/// A partial game state representation. Responsible for tracking [`Position`]
/// history.
///
/// [`Position`]: struct.Position.html
pub struct State {
    /// The previous state. Acts as a linked list of move history.
    pub(super) prev: Option<Arc<State>>,

    /// The square used in an en passant capture, if any.
    ///
    /// Uses a value of `NO_SQUARE` when empty. This is because `Option<Square>`
    /// currently uses two bytes instead of one. Should be made `Option<Square>`
    /// once this PR is in stable: https://github.com/rust-lang/rust/pull/45225.
    pub(super) en_passant: u8,

    /// The castle rights for both players.
    pub(super) castle_rights: CastleRights,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        // Updated with previous states
        let mut this = self;
        let mut that = other;

        loop {
            let eq = this.castle_rights == that.castle_rights &&
                     this.en_passant    == that.en_passant;
            if eq {
                match (&this.prev, &that.prev) {
                    (&Some(ref a), &Some(ref b)) => {
                        // Short circuit if same history
                        if Arc::ptr_eq(a, b) {
                            return true;
                        }
                        this = a;
                        that = b;
                    },
                    (&None, &None) => return true,
                    _ => return false,
                }
            } else {
                return false;
            }
        }
    }
}

impl Eq for State {}

impl Default for State {
    #[inline]
    fn default() -> State {
        State {
            prev: None,
            en_passant: NO_SQUARE,
            castle_rights: CastleRights::FULL,
        }
    }
}
