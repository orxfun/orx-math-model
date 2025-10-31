use crate::symbols::{symbol_map::SymbolMap, Set, SetSymbol};

pub struct SetPositions<'m> {
    map: SymbolMap<'m, SetSymbol, Set<'m>, usize>,
}

impl<'m> SetPositions<'m> {
    pub fn new(sets: impl Iterator<Item = Set<'m>>) -> Self {
        let mut map = SymbolMap::new();

        for (i, set) in sets.enumerate() {
            map.insert(set, i);
        }

        Self { map }
    }
}
