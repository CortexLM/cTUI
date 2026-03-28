//! String interning table for efficient symbol storage
//!
//! This module provides a string interner that maps strings to unique `SymbolId`
//! identifiers. This is used to reduce memory overhead when many cells share
//! the same symbol (e.g., spaces, common characters).
//!
//! # Example
//! ```
//! use ctui_core::symbol_table::{SymbolId, SymbolTable};
//!
//! let mut table = SymbolTable::new();
//! let id1 = table.insert("hello");
//! let id2 = table.insert("hello");
//! assert_eq!(id1, id2); // Same string → same ID
//!
//! assert_eq!(table.get(id1), Some("hello"));
//! ```

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Maximum number of unique symbols that can be interned.
/// u16::MAX = 65,535 unique symbols should be more than enough for TUI use.
pub const MAX_SYMBOLS: usize = u16::MAX as usize;

/// A unique identifier for an interned string.
///
/// This is a newtype wrapper around `u16` to provide type safety and
/// prevent accidental misuse as a raw integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SymbolId(u16);

impl SymbolId {
    /// Creates a new SymbolId from a raw u16.
    /// This is intentionally private - use SymbolTable::insert() instead.
    fn new(id: u16) -> Self {
        Self(id)
    }

    /// Returns the raw u16 value.
    /// Useful for serialization or FFI, but prefer using SymbolId directly.
    pub fn as_u16(self) -> u16 {
        self.0
    }

    /// Creates a SymbolId from a raw u16 value.
    ///
    /// # Safety
    /// This is intended for use by `PackedCell` to reconstruct SymbolIds.
    /// The caller must ensure the ID is valid for the associated SymbolTable.
    pub(crate) fn from_u16_raw(id: u16) -> Self {
        Self(id)
    }
}

impl Default for SymbolId {
    fn default() -> Self {
        // ID 0 is reserved for the empty/default symbol
        Self(0)
    }
}

/// A string interning table that maps strings to unique identifiers.
///
/// Thread-safe through interior mutability with `RwLock`.
/// Each `Terminal` instance should have its own `SymbolTable` - no global singleton.
///
/// # Memory Layout
/// - `Vec<Arc<str>>`: Stores interned strings, indexed by SymbolId
/// - `HashMap<Arc<str>, SymbolId>`: Maps strings to their IDs for deduplication
///
/// # Capacity
/// Supports up to 65,535 unique symbols. Inserting beyond this will panic.
#[derive(Debug)]
pub struct SymbolTable {
    /// Maps SymbolId → Arc<str>
    strings: Vec<Arc<str>>,
    /// Maps Arc<str> → SymbolId (for deduplication)
    lookup: HashMap<Arc<str>, SymbolId>,
    /// RwLock for thread safety (interior mutability)
    lock: RwLock<()>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {
    /// Creates a new empty symbol table with space symbol pre-interned.
    ///
    /// The space character (" ") is reserved as SymbolId(0) since it's the
    /// most common symbol in TUI applications.
    pub fn new() -> Self {
        let mut table = Self {
            strings: Vec::with_capacity(256), // Pre-allocate for common symbols
            lookup: HashMap::with_capacity(256),
            lock: RwLock::new(()),
        };

        // Reserve ID 0 for the default space symbol
        let space: Arc<str> = Arc::from(" ");
        table.strings.push(Arc::clone(&space));
        table.lookup.insert(space, SymbolId::new(0));

        table
    }

    /// Creates a new symbol table with a specific initial capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let capped_capacity = capacity.min(MAX_SYMBOLS);
        let mut table = Self {
            strings: Vec::with_capacity(capped_capacity),
            lookup: HashMap::with_capacity(capped_capacity),
            lock: RwLock::new(()),
        };

        let space: Arc<str> = Arc::from(" ");
        table.strings.push(Arc::clone(&space));
        table.lookup.insert(space, SymbolId::new(0));

        table
    }

    /// Inserts a string into the table and returns its unique SymbolId.
    ///
    /// If the string already exists, returns the existing SymbolId.
    /// If the string is new, allocates a new SymbolId.
    ///
    /// # Panics
    /// Panics if the table is full (65,535 unique symbols reached).
    ///
    /// # Thread Safety
    /// Uses a write lock for insertion operations.
    pub fn insert(&mut self, s: &str) -> SymbolId {
        // First check if it already exists (read lock)
        {
            let _read = self.lock.read().unwrap();
            if let Some(&id) = self.lookup.get(s) {
                return id;
            }
        }

        // Not found, need to insert (write lock)
        let _write = self.lock.write().unwrap();

        // Double-check after acquiring write lock (another thread may have inserted)
        if let Some(&id) = self.lookup.get(s) {
            return id;
        }

        // Check capacity
        assert!(
            self.strings.len() < MAX_SYMBOLS,
            "SymbolTable capacity exceeded: {} symbols already interned",
            MAX_SYMBOLS
        );

        // Allocate new ID
        let id = SymbolId::new(self.strings.len() as u16);
        let interned: Arc<str> = Arc::from(s);

        self.strings.push(Arc::clone(&interned));
        self.lookup.insert(interned, id);

        id
    }

    /// Returns a reference to the string for a given SymbolId.
    ///
    /// Returns `None` if the SymbolId is invalid.
    ///
    /// # Thread Safety
    /// Uses a read lock, allowing concurrent reads.
    pub fn get(&self, id: SymbolId) -> Option<&str> {
        let _read = self.lock.read().unwrap();
        self.strings.get(id.0 as usize).map(|s| s.as_ref())
    }

    /// Resolves a SymbolId to its string, panicking if invalid.
    ///
    /// # Panics
    /// Panics if the SymbolId is invalid.
    pub fn resolve(&self, id: SymbolId) -> &str {
        self.get(id).expect("Invalid SymbolId")
    }

    /// Returns the number of unique symbols currently interned.
    pub fn len(&self) -> usize {
        let _read = self.lock.read().unwrap();
        self.strings.len()
    }

    /// Returns true if the table contains no symbols (other than the default space).
    pub fn is_empty(&self) -> bool {
        self.len() == 1 // Only the default space symbol
    }

    /// Clears all symbols except the default space.
    pub fn clear(&mut self) {
        let _write = self.lock.write().unwrap();
        self.strings.truncate(1); // Keep space at index 0
        self.lookup.retain(|_, &mut id| id.as_u16() == 0);
    }

    /// Returns true if the table contains the given string.
    pub fn contains(&self, s: &str) -> bool {
        let _read = self.lock.read().unwrap();
        self.lookup.contains_key(s)
    }

    /// Returns the SymbolId for a string if it exists, without inserting.
    pub fn lookup(&self, s: &str) -> Option<SymbolId> {
        let _read = self.lock.read().unwrap();
        self.lookup.get(s).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_same_string_returns_same_id() {
        let mut table = SymbolTable::new();
        let id1 = table.insert("hello");
        let id2 = table.insert("hello");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_insert_different_strings_returns_different_ids() {
        let mut table = SymbolTable::new();
        let id1 = table.insert("hello");
        let id2 = table.insert("world");
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_get_returns_correct_string() {
        let mut table = SymbolTable::new();
        let id = table.insert("hello");
        assert_eq!(table.get(id), Some("hello"));
    }

    #[test]
    fn test_get_invalid_id_returns_none() {
        let table = SymbolTable::new();
        // Create an invalid ID that doesn't exist
        let invalid_id = SymbolId(9999);
        assert_eq!(table.get(invalid_id), None);
    }

    #[test]
    fn test_handles_emoji_family() {
        let mut table = SymbolTable::new();
        // ZWJ family emoji: 👨‍👩‍👧‍👦 (4 visible people joined with ZWJ)
        let family = "👨‍👩‍👧‍👦";
        let id = table.insert(family);
        assert_eq!(table.get(id), Some(family));
    }

    #[test]
    fn test_handles_cjk_characters() {
        let mut table = SymbolTable::new();
        let id = table.insert("漢字");
        assert_eq!(table.get(id), Some("漢字"));
    }

    #[test]
    fn test_handles_flag_emoji() {
        let mut table = SymbolTable::new();
        // Flag emoji using regional indicators: 🇷🇺 🇺🇸 🇯🇵
        let ids: Vec<_> = ["🇷🇺", "🇺🇸", "🇯🇵"].iter().map(|s| table.insert(s)).collect();

        // All should be different
        assert_ne!(ids[0], ids[1]);
        assert_ne!(ids[1], ids[2]);
        assert_ne!(ids[0], ids[2]);

        // And retrievable
        assert_eq!(table.get(ids[0]), Some("🇷🇺"));
        assert_eq!(table.get(ids[1]), Some("🇺🇸"));
        assert_eq!(table.get(ids[2]), Some("🇯🇵"));
    }

    #[test]
    fn test_handles_skin_tone_emoji() {
        let mut table = SymbolTable::new();
        let id = table.insert("👋🏻"); // Waving hand with light skin tone
        assert_eq!(table.get(id), Some("👋🏻"));
    }

    #[test]
    fn test_default_space_symbol() {
        let table = SymbolTable::new();
        // ID 0 should be the space symbol
        assert_eq!(table.get(SymbolId::default()), Some(" "));
        assert!(table.contains(" "));
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut table = SymbolTable::new();
        // Contains default space symbol
        assert_eq!(table.len(), 1);
        assert!(table.is_empty()); // Only the default space

        table.insert("a");
        assert_eq!(table.len(), 2);
        assert!(!table.is_empty());

        table.insert("b");
        assert_eq!(table.len(), 3);
    }

    #[test]
    fn test_clear() {
        let mut table = SymbolTable::new();
        table.insert("a");
        table.insert("b");
        table.insert("c");
        assert_eq!(table.len(), 4);

        table.clear();
        assert_eq!(table.len(), 1); // Only default space remains
        assert_eq!(table.get(SymbolId::default()), Some(" "));
    }

    #[test]
    fn test_lookup_without_insert() {
        let mut table = SymbolTable::new();
        table.insert("existing");

        assert!(table.lookup("existing").is_some());
        assert!(table.lookup("nonexistent").is_none());
    }

    #[test]
    fn test_resolve_panics_on_invalid() {
        let table = SymbolTable::new();
        let result = std::panic::catch_unwind(|| {
            table.resolve(SymbolId(9999));
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_symbol_id_default() {
        let id = SymbolId::default();
        assert_eq!(id.as_u16(), 0);
    }

    #[test]
    fn test_symbol_id_ordering() {
        let id1 = SymbolId::new(1);
        let id2 = SymbolId::new(2);
        assert!(id1 < id2);
        assert!(id2 > id1);
        assert_eq!(id1, id1);
    }

    #[test]
    fn test_empty_string() {
        let mut table = SymbolTable::new();
        let id = table.insert("");
        assert_eq!(table.get(id), Some(""));
        // Inserting empty again should return same ID
        let id2 = table.insert("");
        assert_eq!(id, id2);
    }

    #[test]
    fn test_unicode_normalization() {
        // Different ways to represent é
        let mut table = SymbolTable::new();

        // Precomposed: é (U+00E9)
        let id1 = table.insert("\u{00E9}");

        // Composed: e + combining acute accent (U+0065 U+0301)
        let id2 = table.insert("\u{0065}\u{0301}");

        // These are NOT the same in memory, so they get different IDs
        // This is intentional - we don't normalize Unicode
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_with_capacity() {
        let table = SymbolTable::with_capacity(1000);
        assert_eq!(table.len(), 1); // Default space
                                    // Capacity is internal, can't test directly but shouldn't panic
    }
}
