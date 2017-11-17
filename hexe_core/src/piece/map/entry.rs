use super::*;

/// A view into an occupied entry in a [`PieceMap`]. It is part of the [`Entry`] enum.
///
/// [`PieceMap`]: struct.PieceMap.html
/// [`Entry`]: enum.Entry.html
pub struct OccupiedEntry<'a> {
    map: &'a mut PieceMap,
    key: Square,
}

/// A view into a vacant entry in a [`PieceMap`]. It is part of the [`Entry`] enum.
///
/// [`PieceMap`]: struct.PieceMap.html
/// [`Entry`]: enum.Entry.html
pub struct VacantEntry<'a> {
    map: &'a mut PieceMap,
    key: Square,
}

/// A view into a single entry in a map, which may either be vacant or occupied.
///
/// This enum is constructed from the [`entry`] method on [`PieceMap`].
///
/// [`PieceMap`]: struct.PieceMap.html
/// [`entry`]: struct.PieceMap.html#method.entry
pub enum Entry<'a> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a>),
    /// A vacant entry.
    Vacant(VacantEntry<'a>),
}

impl<'a> Entry<'a> {
    #[inline]
    pub(crate) fn from_map(map: &'a mut PieceMap, sq: Square) -> Entry<'a> {
        if map.contains(sq) {
            Entry::Occupied(OccupiedEntry { map: map, key: sq })
        } else {
            Entry::Vacant(VacantEntry { map: map, key: sq, })
        }
    }
}
