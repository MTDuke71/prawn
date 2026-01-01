//! Transposition Table
//! 
//! Fixed-size hash table for caching search results to avoid re-searching
//! identical positions reached by different move orders.

use prawn::Move;

/// Bound type for transposition table entries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound {
    /// Exact score (PV node)
    Exact,
    /// Lower bound (failed high / beta cutoff)
    Lower,
    /// Upper bound (failed low / all node)
    Upper,
}

/// Entry in the transposition table
#[derive(Debug, Clone, Copy)]
pub struct TTEntry {
    /// Zobrist hash key (or portion for verification)
    pub key: u64,
    /// Evaluation score
    pub score: i32,
    /// Search depth
    pub depth: u8,
    /// Type of bound
    pub bound: Bound,
    /// Best move found (if any)
    pub best_move: Option<Move>,
    /// Age for replacement
    pub age: u8,
}

impl Default for TTEntry {
    fn default() -> Self {
        Self {
            key: 0,
            score: 0,
            depth: 0,
            bound: Bound::Upper,
            best_move: None,
            age: 0,
        }
    }
}

/// Transposition table using Zobrist hashing
pub struct TranspositionTable {
    entries: Vec<TTEntry>,
    size: usize,
    age: u8,
    hits: u64,
    stores: u64,
}

impl TranspositionTable {
    /// Create a new transposition table with the given size in MB
    pub fn new(size_mb: usize) -> Self {
        let entry_size = std::mem::size_of::<TTEntry>();
        let num_entries = (size_mb * 1024 * 1024) / entry_size;
        // Round down to power of 2 for fast indexing
        let size = num_entries.next_power_of_two() / 2;
        
        Self {
            entries: vec![TTEntry::default(); size],
            size,
            age: 0,
            hits: 0,
            stores: 0,
        }
    }
    
    /// Get the index for a hash key
    #[inline(always)]
    fn index(&self, key: u64) -> usize {
        (key as usize) & (self.size - 1)
    }
    
    /// Store an entry in the table
    #[inline]
    pub fn store(&mut self, key: u64, score: i32, depth: u8, bound: Bound, best_move: Option<Move>) {
        let idx = self.index(key);
        let existing = &self.entries[idx];
        
        // Replacement strategy: always replace if:
        // - Empty entry
        // - Same position (update)
        // - New entry is deeper
        // - Old entry is from a previous search
        let should_replace = existing.key == 0 
            || existing.key == key
            || depth >= existing.depth
            || existing.age != self.age;
        
        if should_replace {
            self.entries[idx] = TTEntry {
                key,
                score,
                depth,
                bound,
                best_move,
                age: self.age,
            };
            self.stores += 1;
        }
    }
    
    /// Probe the table for an entry
    #[inline]
    pub fn probe(&mut self, key: u64) -> Option<&TTEntry> {
        let idx = self.index(key);
        let entry = &self.entries[idx];
        
        if entry.key == key {
            self.hits += 1;
            Some(entry)
        } else {
            None
        }
    }
    
    /// Probe and get a copy (for concurrent-safe access pattern)
    #[inline]
    pub fn probe_copy(&self, key: u64) -> Option<TTEntry> {
        let idx = self.index(key);
        let entry = self.entries[idx];
        
        if entry.key == key {
            Some(entry)
        } else {
            None
        }
    }
    
    /// Increment the age (call at start of each search)
    pub fn new_search(&mut self) {
        self.age = self.age.wrapping_add(1);
    }
    
    /// Clear the table
    pub fn clear(&mut self) {
        self.entries.fill(TTEntry::default());
        self.age = 0;
        self.hits = 0;
        self.stores = 0;
    }
    
    /// Get statistics
    pub fn stats(&self) -> (u64, u64) {
        (self.hits, self.stores)
    }
    
    /// Get table fill percentage
    pub fn hashfull(&self) -> usize {
        let sample_size = 1000.min(self.size);
        let filled = self.entries[..sample_size]
            .iter()
            .filter(|e| e.key != 0)
            .count();
        (filled * 1000) / sample_size
    }
}
