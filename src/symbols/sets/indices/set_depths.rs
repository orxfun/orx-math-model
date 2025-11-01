use crate::symbols::{sets::indices::Depth, symbol_map::SymbolMap, Set, SetSymbol};

pub struct SetDepths<'m> {
    map: SymbolMap<'m, SetSymbol, Set<'m>, Depth>,
}

impl<'m> SetDepths<'m> {
    pub fn new(sets: impl Iterator<Item = Set<'m>>) -> Self {
        let mut map = SymbolMap::new();

        for (i, set) in sets.enumerate() {
            map.insert(set, Depth::new(i));
        }

        Self { map }
    }

    pub fn depth_of(&self, set: Set<'m>) -> Depth {
        *self.map.get(set).expect("must exist")
    }
}
