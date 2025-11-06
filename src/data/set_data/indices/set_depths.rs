use super::Depth;
use crate::symbols::{Set, SetCore, SetMeta};

pub struct SetDepths<'m> {
    // map: SymbolMap<'m, SetMeta, SetCore<'m>, Depth>,
    p: core::marker::PhantomData<&'m ()>,
}

impl<'m> SetDepths<'m> {
    pub fn new(mut sets: impl Iterator<Item = Set<'m>>) -> Self {
        let x = sets.next().unwrap();
        let y = x.sym_data();
        // let mut map = SymbolMap::new();

        // for (i, set) in sets.enumerate() {
        //     map.insert(set, Depth::new(i));
        // }

        // Self { map }
        todo!()
    }

    pub fn depth_of(&self, set: Set<'m>) -> Depth {
        // *self.map.get(set).expect("must exist")
        todo!()
    }
}
