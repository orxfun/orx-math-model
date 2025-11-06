use super::Depth;
use crate::symbols::{Set, SetCore, SetMeta, SymbolMap};

pub struct SetDepths<'m> {
    map: SymbolMap<'m, SetMeta, SetCore<'m>, Depth>,
}

impl<'m> SetDepths<'m> {
    pub fn new(sets: impl Iterator<Item = Set<'m>>) -> Self {
        let mut map = SymbolMap::new();

        for (i, set) in sets.enumerate() {
            map.insert(set.into_core(), Depth::new(i));
        }

        Self { map }
    }

    pub fn depth_of(&self, set: Set<'m>) -> Depth {
        *self.map.get(set.into_core()).expect("must exist")
    }
}
