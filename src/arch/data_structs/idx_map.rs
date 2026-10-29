use alloc::{collections::BTreeMap, vec::Vec};

type Map<K, V> = BTreeMap<K, V>;

pub struct IdxMap<'a, K: Ord> {
    values: Vec<K>,
    positions: Map<&'a K, usize>,
}

impl<K: Ord> Default for IdxMap<'_, K> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            positions: Default::default(),
        }
    }
}

impl<K: Ord> From<Vec<K>> for IdxMap<'_, K> {
    fn from(values: Vec<K>) -> Self {
        let positions = values
            .iter()
            .enumerate()
            .map(|(i, x)| {
                // TODO: safety note here
                let x = unsafe { &*(x as *const K) };
                (x, i)
            })
            .collect();
        Self { values, positions }
    }
}

impl<'a, K: Ord> IdxMap<'a, K> {
    pub fn pos_of(&self, key: &K) -> Option<usize> {
        self.positions.get(key).copied()
    }

    pub fn val_at(&self, pos: usize) -> Option<&K> {
        self.values.get(pos)
    }
}
