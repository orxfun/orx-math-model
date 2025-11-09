use super::Depth;
use crate::symbols::sets::{Set, SetCore, SetMeta};
use crate::symbols::SymbolMap;

pub struct SetDepths<'m> {
    map: SymbolMap<'m, SetMeta, SetCore<'m>, Depth>,
}

impl<'m> SetDepths<'m> {
    pub fn new<S: Into<SetCore<'m>>>(sets: impl IntoIterator<Item = S>) -> Self {
        let mut map = SymbolMap::new();

        for (i, set) in sets.into_iter().enumerate() {
            map.insert(set.into(), Depth::new(i));
        }

        Self { map }
    }

    pub fn depth_of(&self, set: SetCore<'m>) -> Depth {
        *self.map.get(set.into()).expect("must exist")
    }
}
