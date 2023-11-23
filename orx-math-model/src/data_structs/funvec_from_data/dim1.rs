use crate::data_structs::funvec::FunVec;
use std::collections::{BTreeMap, HashMap};

impl<Elem: Clone> FunVec<1, Elem> for Vec<Elem> {
    fn value(&self, indices: [usize; 1]) -> Elem {
        self.get(indices[0])
            .expect("FunVec<1, _> from Vec<_>: index-out-of-bounds")
            .clone()
    }
}

impl<Elem: Clone> FunVec<1, Elem> for HashMap<usize, Elem> {
    fn value(&self, indices: [usize; 1]) -> Elem {
        self.get(&indices[0])
            .expect("FunVec<1, _> from HashMap<usize, _>: index-out-of-bounds")
            .clone()
    }
}

impl<Elem: Clone> FunVec<1, Elem> for BTreeMap<usize, Elem> {
    fn value(&self, indices: [usize; 1]) -> Elem {
        self.get(&indices[0])
            .expect("FunVec<1, _> from BTreeMap<usize, _>: index-out-of-bounds")
            .clone()
    }
}
