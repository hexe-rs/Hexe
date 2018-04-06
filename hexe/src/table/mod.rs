use std::cmp::Ordering;
use std::mem;

#[cfg(all(test, nightly))]
mod benches;

const CACHE_LINE:   usize = 64;
const CLUSTER_SIZE: usize = mem::size_of::<Cluster>();
const ENTRY_COUNT:  usize = CACHE_LINE / mem::size_of::<Entry>();
const MB_SIZE:      usize = 1024 * 1024;

#[cfg(test)]
assert_eq_size! { cluster_size;
    Cluster,
    [u8; mem::align_of::<Cluster>()], // Same size and alignment
    [u8; CACHE_LINE],                 // as the cache line size
}

/// A transposition table.
pub struct Table {
    clusters: Vec<Cluster>
}

impl Table {
    /// Creates a new table with a capacity and size that matches `size_mb`
    /// number of megabytes.
    pub fn new(size_mb: usize, exact: bool) -> Table {
        let mut table = Table {
            clusters: Default::default()
        };
        if exact {
            table.resize_exact(size_mb);
        } else {
            table.resize(size_mb);
        }
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

        match new.cmp(&old) {
            Ordering::Equal => return,
            Ordering::Greater => unsafe {
                self.clusters.reserve_exact(new - old);
                self.clusters.set_len(new);
                let slice = self.clusters.get_unchecked_mut(old..new);
                ::util::zero(slice);
            },
            Ordering::Less => {
                self.clusters.truncate(new);
                self.clusters.shrink_to_fit();
            },
        }
    }

    /// Zeroes out the entire table.
    pub fn clear(&mut self) {
        unsafe { ::util::zero(&mut self.clusters[..]) };
    }
}

#[repr(C, align(64))]
union Cluster {
    entries: [Entry; ENTRY_COUNT],
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
struct Entry {
    mv:  u16,
    val: i16,
}
