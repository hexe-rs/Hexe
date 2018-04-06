use std::mem;

const CLUSTER_SIZE: usize = mem::size_of::<Cluster>();
const ENTRY_COUNT:  usize = 1;
const MB_SIZE:      usize = 1024 * 1024;

#[cfg(test)]
const CACHE_LINE: usize = 64;

#[cfg(test)]
const_assert!(cluster_size; 64 % CACHE_LINE == 0);

/// A transposition table.
#[derive(Debug)]
pub struct Table {
    clusters: Vec<Cluster>
}

impl Table {
    /// Creates a new table with a capacity and size that matches `size_mb`
    /// number of megabytes.
    pub fn new(size_mb: usize) -> Table {
        let mut table = Table {
            clusters: Default::default()
        };
        table.resize(size_mb);
        table
    }

    /// Returns the number of entries in the table.
    pub fn size(&self) -> usize {
        self.clusters.len() * ENTRY_COUNT
    }

    /// Returns the size of the table in megabytes.
    pub fn size_mb(&self) -> usize {
        self.clusters.len() * CLUSTER_SIZE / MB_SIZE
    }

    /// Resizes the table to the next power of two number of megabytes.
    pub fn resize(&mut self, size_mb: usize) {
        self.resize_exact(size_mb.next_power_of_two());
    }

    /// Resizes the table to exactly `size_mb` number of megabytes.
    pub fn resize_exact(&mut self, size_mb: usize) {
        let new = size_mb * MB_SIZE / CLUSTER_SIZE;
        let old = self.clusters.len();
        if new == old {
            return;
        }

        if new > old {
            self.clusters.reserve_exact(new - old);
            unsafe {
                let slice = self.clusters.get_unchecked_mut(old..new);
                ::util::zero(slice);
            }
        }

        unsafe { self.clusters.set_len(new) };
    }

    /// Zeroes out the entire table.
    pub fn clear(&mut self) {
        unsafe { ::util::zero(&mut self.clusters[..]) };
    }
}

#[derive(Debug)]
#[repr(C)]
struct Cluster([Entry; ENTRY_COUNT]);

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct Entry {
    mv:    u16,
    value: i16,
}
