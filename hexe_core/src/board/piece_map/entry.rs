use super::*;

/// A view into an occupied entry in a [`PieceMap`]. It is part of the [`Entry`] enum.
///
/// [`PieceMap`]: struct.PieceMap.html
/// [`Entry`]: enum.Entry.html
pub struct OccupiedEntry<'a> {
    map: &'a mut PieceMap,
    key: Square,
}

impl<'a> fmt::Debug for OccupiedEntry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

impl<'a> OccupiedEntry<'a> {
    /// Gets a reference to the square in the entry.
    #[inline]
    pub fn key(&self) -> &Square { &self.key }

    /// Take ownership of the piece and square from the map.
    #[inline]
    pub fn remove_entry(self) -> (Square, Piece) {
        let key = self.key;
        let buf = unsafe { self.map.as_bytes_mut() };
        let val = mem::replace(key.extract_mut(buf), NONE);
        unsafe { (key, val.into_unchecked()) }
    }

    /// Gets a reference to the piece in the entry.
    #[inline]
    pub fn get(&self) -> &Piece {
        unsafe { self.map.get_unchecked(self.key) }
    }

    /// Gets a mutable reference to the piece in the entry.
    #[inline]
    pub fn get_mut(&mut self) -> &mut Piece {
        unsafe { self.map.get_unchecked_mut(self.key) }
    }

    /// Converts the entry into a mutable reference to its value.
    #[inline]
    pub fn into_mut(self) -> &'a mut Piece {
        unsafe { self.map.get_unchecked_mut(self.key) }
    }

    /// Sets the piece of the entry with the `OccupiedEntry`'s square, and
    /// returns the entry's old value.
    #[inline]
    pub fn insert(&mut self, piece: Piece) -> Piece {
        let buf = unsafe { self.map.as_bytes_mut() };
        let pc = mem::replace(self.key.extract_mut(buf), piece as u8);
        unsafe { pc.into_unchecked() }
    }

    /// Takes the piece of the entry out of the map, and returns it.
    #[inline]
    pub fn remove(self) -> Piece {
        self.remove_entry().1
    }
}

/// A view into a vacant entry in a [`PieceMap`]. It is part of the [`Entry`] enum.
///
/// [`PieceMap`]: struct.PieceMap.html
/// [`Entry`]: enum.Entry.html
pub struct VacantEntry<'a> {
    map: &'a mut PieceMap,
    key: Square,
}

impl<'a> fmt::Debug for VacantEntry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("VacantEntry")
            .field(self.key())
            .finish()
    }
}

impl<'a> VacantEntry<'a> {
    /// Gets a reference to the square that would be used when inserting a value
    /// through the vacant entry.
    #[inline]
    pub fn key(&self) -> &Square { &self.key }

    /// Take ownership of the square.
    #[inline]
    pub fn into_key(self) -> Square { self.key }

    /// Sets the piece of the entry and returns a mutable reference to it.
    #[inline]
    pub fn insert(self, piece: Piece) -> &'a mut Piece {
        unsafe {
            let slot = self.key.extract_mut(&mut self.map.0.array);
            *slot = piece as u8;
            slot.into_unchecked()
        }
    }
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

impl<'a> fmt::Debug for Entry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let field: &fmt::Debug = match *self {
            Entry::Vacant(ref v) => v,
            Entry::Occupied(ref o) => o,
        };
        f.debug_tuple("Entry").field(field).finish()
    }
}

impl<'a> Entry<'a> {
    #[inline]
    pub(super) fn from_map(map: &'a mut PieceMap, sq: Square) -> Entry<'a> {
        if map.contains(sq) {
            Entry::Occupied(OccupiedEntry { map: map, key: sq })
        } else {
            Entry::Vacant(VacantEntry { map: map, key: sq })
        }
    }

    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    #[inline]
    pub fn or_insert(self, default: Piece) -> &'a mut Piece {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty, and returns a mutable reference to the value in the
    /// entry.
    #[inline]
    pub fn or_insert_with<F>(self, default: F) -> &'a mut Piece
        where F: FnOnce() -> Piece
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Returns a reference to this entry's square.
    #[inline]
    pub fn key(&self) -> &Square {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }
}
