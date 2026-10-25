use alloc::{collections::BTreeMap, vec::Vec};

type Map<K, V> = BTreeMap<K, V>;

pub struct IdxMap<K: Ord> {
    values: Vec<K>,
    positions: Map<K, usize>,
}

impl<K: Ord> Default for IdxMap<K> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            positions: Default::default(),
        }
    }
}

impl<K: Ord> IdxMap<K> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn pos_of(&self, key: &K) -> Option<usize> {
        self.positions.get(key).copied()
    }

    pub fn val_at(&self, pos: usize) -> Option<&K> {
        self.values.get(pos)
    }
}
