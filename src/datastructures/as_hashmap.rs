use std::collections::HashMap;

/// Return a reference to the inner hashmap.
pub trait AsHashMap<K, V> {
    /// Return a reference to the inner hashmap.
    fn as_hashmap(&self) -> HashMap<K, V>;
}

/// Return a mutable reference to the inner hashmap.
pub trait AsHashMapMut<K, V> {
    /// Return a mutable reference to the inner hashmap.
    fn as_hashmap_mut(&self) -> HashMap<K, V>;
}
