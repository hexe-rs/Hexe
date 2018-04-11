use std::cell::UnsafeCell;
use std::mem;

use uncon::*;

use zero::{Zero, ZeroBuffer};

#[cfg(all(test, nightly))]
mod benches;

#[cfg(test)]
mod tests;

const CACHE_LINE:    usize = 64;
const CLUSTER_ALIGN: usize = mem::align_of::<Cluster>();
const CLUSTER_SIZE:  usize = mem::size_of::<Cluster>();
const ENTRY_COUNT:   usize = CACHE_LINE / mem::size_of::<Entry>();
const MB_SIZE:       usize = 1024 * 1024;

#[cfg(test)]
assert_eq_size! { cluster_size;
    Cluster,
    [u8; CLUSTER_ALIGN], // Same size and alignment
    [u8; CACHE_LINE],    // as the cache line size
}

/// A transposition table.
#[derive(Default)]
pub struct Table(ZeroBuffer<UnsafeCell<Cluster>>);

impl Table {
    /// Creates a table with its capacity and size set to the smallest power of
    /// two greater than or equal to `size_mb` number of megabytes.
    pub fn new(size_mb: usize) -> Table {
        let mut table = Table::default();
        table.resize(size_mb);
        table
    }

    /// Returns the number of entries in the table.
    pub fn size(&self) -> usize {
        self.clusters().len() * ENTRY_COUNT
    }

    /// Returns the size of the table in megabytes.
    pub fn size_mb(&self) -> usize {
        mem::size_of_val(self.clusters()) / MB_SIZE
    }

    /// Resizes the table to the next power of two number of megabytes.
    pub fn resize(&mut self, size_mb: usize) {
        unsafe { self.resize_exact(size_mb.next_power_of_two()) };
    }

    /// Resizes the table to exactly `size_mb` number of megabytes.
    ///
    /// # Safety
    ///
    /// This type's internals assume that the buffer has a power of two size.
    unsafe fn resize_exact(&mut self, size_mb: usize) {
        debug!("Setting table size to {} megabytes", size_mb);
        debug_assert!(size_mb.is_power_of_two());
        self.0.resize_exact(size_mb * MB_SIZE / CLUSTER_SIZE);
    }

    /// Returns `self` as a slice of clusters.
    pub fn clusters(&self) -> &[Cluster] {
        Cluster::slice(&self.0)
    }

    /// Returns `self` as a mutable slice of clusters.
    pub fn clusters_mut(&mut self) -> &mut [Cluster] {
        Cluster::slice_mut(&mut self.0)
    }

    /// Zeroes out the entire table.
    pub fn clear(&mut self) {
        self.clusters_mut().zero();
    }
}

/// A cluster of table entries aligned to the cache line size.
#[derive(Debug)]
#[repr(C, align(64))]
pub struct Cluster {
    entries: [Entry; ENTRY_COUNT],
}

unsafe impl Zero for Cluster {}

impl Cluster {
    fn slice(s: &[UnsafeCell<Self>]) -> &[Self] {
        unsafe { s.into_unchecked() }
    }

    fn slice_mut(s: &mut [UnsafeCell<Self>]) -> &mut [Self] {
        unsafe { s.into_unchecked() }
    }
}

impl Cluster {
    fn entries(&self) -> &[Entry; ENTRY_COUNT] {
        &self.entries
    }

    fn entries_mut(&mut self) -> &mut [Entry; ENTRY_COUNT] {
        &mut self.entries
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct Entry {
    mv:  u16,
    val: i16,
}

unsafe impl Zero for Entry {}
