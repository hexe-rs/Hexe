use std::cmp::Ordering;
use std::mem;
use std::ptr::{self, NonNull};
use std::slice;

use libc;

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
pub struct Table {
    /// The start of the `calloc`ed buffer.
    start: *mut libc::c_void,
    /// A pointer offset to the correct alignment of `Cluster`.
    align: NonNull<Cluster>,
    /// The size of the table by number of clusters.
    len: usize,
}

unsafe impl Send for Table {}
unsafe impl Sync for Table {}

impl Default for Table {
    #[inline]
    fn default() -> Table {
        Table {
            start: ptr::null_mut(),
            align: NonNull::dangling(),
            len: 0,
        }
    }
}

impl Drop for Table {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.dealloc() };
    }
}

impl Table {
    /// Creates a new table with a capacity and size that matches `size_mb`
    /// number of megabytes.
    pub fn new(size_mb: usize, exact: bool) -> Table {
        let mut table = Table::default();
        if exact {
            table.resize_exact(size_mb);
        } else {
            table.resize(size_mb);
        }
        table
    }

    #[inline]
    unsafe fn dealloc(&mut self) {
        if !self.start.is_null() {
            libc::free(self.start);
        }
    }

    #[cfg(test)]
    fn is_aligned(&self) -> bool {
        self.align.as_ptr() as usize % CLUSTER_ALIGN == 0
    }

    /// Returns the number of entries in the table.
    pub fn size(&self) -> usize {
        self.len * ENTRY_COUNT
    }

    /// Returns the size of the table in megabytes.
    pub fn size_mb(&self) -> usize {
        mem::size_of_val(self.clusters()) / MB_SIZE
    }

    /// Resizes the table to the next power of two number of megabytes.
    pub fn resize(&mut self, size_mb: usize) {
        self.resize_exact(size_mb.next_power_of_two());
    }

    /// Resizes the table to exactly `size_mb` number of megabytes.
    pub fn resize_exact(&mut self, size_mb: usize) {
        let len = size_mb * MB_SIZE / CLUSTER_SIZE;
        if len == self.len {
            return;
        }

        unsafe { self.dealloc() };

        let calloc = unsafe { libc::calloc(len + 1, CLUSTER_SIZE) };
        self.start = calloc;
        self.len   = len;

        self.align = unsafe {
            const MASK: usize = !(CLUSTER_SIZE - 1);
            let val = calloc.offset(CLUSTER_SIZE as _) as usize;
            NonNull::new_unchecked((val & MASK) as *mut Cluster)
        };
    }

    /// Returns `self` as a slice of clusters.
    pub fn clusters(&self) -> &[Cluster] {
        let ptr = self.align.as_ptr();
        unsafe { slice::from_raw_parts(ptr, self.len) }
    }

    /// Returns `self` as a mutable slice of clusters.
    pub fn clusters_mut(&mut self) -> &mut [Cluster] {
        let ptr = self.align.as_ptr();
        unsafe { slice::from_raw_parts_mut(ptr, self.len) }
    }

    /// Zeroes out the entire table.
    pub fn clear(&mut self) {
        unsafe { ::util::zero(self.clusters_mut()) };
    }
}

/// A cluster of table entries aligned to the cache line size.
#[derive(Debug)]
#[repr(C, align(64))]
pub struct Cluster {
    entries: [Entry; ENTRY_COUNT],
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
