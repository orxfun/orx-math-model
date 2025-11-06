use super::Depth;
use crate::symbols::{Set, SetCore, SetMeta, SymbolMap};

pub struct SetDepths<'m> {
    map: SymbolMap<'m, SetMeta, SetCore<'m>, Depth>,
}

impl<'m> SetDepths<'m> {
    pub fn new(sets: impl Iterator<Item = Set<'m>>) -> Self {
        let mut map = SymbolMap::new();

        for (i, set) in sets.enumerate() {
            map.insert(set.core(), Depth::new(i));
        }

        Self { map }
    }

    pub fn depth_of(&self, set: Set<'m>) -> Depth {
        *self.map.get(set.core()).expect("must exist")
    }
}
