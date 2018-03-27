use super::*;
use std::fmt;
use std::sync::Arc;

/// A partial game state representation. Responsible for tracking [`Position`]
/// history.
///
/// [`Position`]: struct.Position.html
#[derive(Clone)]
pub struct State {
    /// The previous state. Acts as a linked list of move history.
    pub(super) prev: Option<Arc<State>>,

    /// The square used in an en passant capture, if any.
    pub(super) en_passant: Option<Square>,

    /// The castle rights for both players.
    pub(super) castle_rights: Rights,
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        // Updated with previous states
        let mut this = self;
        let mut that = other;

        loop {
            if this.castle_rights == that.castle_rights
            && this.en_passant    == that.en_passant {
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
    fn default() -> State { State::STANDARD }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("State")
            .field("prev",          &self.prev())
            .field("en_passant",    &self.en_passant())
            .field("castle_rights", &self.castle_rights())
            .finish()
    }
}

impl State {
    pub(crate) const STANDARD: State = State {
        prev: None,
        en_passant: None,
        castle_rights: Rights::FULL,
    };

    /// Returns the previous state.
    #[inline]
    pub fn prev(&self) -> Option<&State> {
        self.prev.as_ref().map(AsRef::as_ref)
    }

    /// Returns the en passant square.
    #[inline]
    pub fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    /// Returns the castle rights for both players.
    #[inline]
    pub fn castle_rights(&self) -> Rights {
        self.castle_rights
    }
}
