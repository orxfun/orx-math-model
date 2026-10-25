use alloc::{collections::BTreeMap, vec::Vec};

type Map<K, V> = BTreeMap<K, V>;

pub struct IdxMap<K> {
    values: Vec<K>,
    positions: Map<K, usize>,
}

impl<K> Default for IdxMap<K> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            positions: Default::default(),
        }
    }
}

impl<K> IdxMap<K> {
    pub fn new() -> Self {
        Default::default()
    }
}
