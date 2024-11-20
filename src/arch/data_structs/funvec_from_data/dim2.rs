use crate::data_structs::funvec::FunVec;
use std::collections::{BTreeMap, HashMap};

// vec-x
impl<Elem: Clone> FunVec<2, Elem> for Vec<Vec<Elem>> {
    fn value(&self, indices: [usize; 2]) -> Elem {
        self.get(indices[0])
            .expect("FunVec<2, _> from Vec<Vec<_>>: first-index-out-of-bounds")
            .get(indices[1])
            .expect("FunVec<2, _> from Vec<Vec<_>>: second-index-out-of-bounds")
            .clone()
    }
}
impl<Elem: Clone> FunVec<2, Elem> for Vec<HashMap<usize, Elem>> {
    fn value(&self, indices: [usize; 2]) -> Elem {
        self.get(indices[0])
            .expect("FunVec<2, _> from Vec<HashMap<usize, _>>: first-index-out-of-bounds")
            .get(&indices[1])
            .expect("FunVec<2, _> from Vec<HashMap<usize, _>>: second-index-out-of-bounds")
            .clone()
    }
}
impl<Elem: Clone> FunVec<2, Elem> for Vec<BTreeMap<usize, Elem>> {
    fn value(&self, indices: [usize; 2]) -> Elem {
        self.get(indices[0])
            .expect("FunVec<2, _> from Vec<BTreeMap<usize, _>>: first-index-out-of-bounds")
            .get(&indices[1])
            .expect("FunVec<2, _> from Vec<BTreeMap<usize, _>>: second-index-out-of-bounds")
            .clone()
    }
}

// hashmap-x
impl<Elem: Clone> FunVec<2, Elem> for HashMap<usize, Vec<Elem>> {
    fn value(&self, indices: [usize; 2]) -> Elem {
        self.get(&indices[0])
            .expect("FunVec<2, _> from HashMap<usize, Vec<_>>: first-index-out-of-bounds")
            .get(indices[1])
            .expect("FunVec<2, _> from HashMap<usize, Vec<_>>: second-index-out-of-bounds")
            .clone()
    }
}
impl<Elem: Clone> FunVec<2, Elem> for HashMap<usize, HashMap<usize, Elem>> {
    fn value(&self, indices: [usize; 2]) -> Elem {
        self.get(&indices[0])
            .expect(
                "FunVec<2, _> from HashMap<usize, HashMap<usize, _>>: first-index-out-of-bounds",
            )
            .get(&indices[1])
            .expect(
                "FunVec<2, _> from HashMap<usize, HashMap<usize, _>>: second-index-out-of-bounds",
            )
            .clone()
    }
}
impl<Elem: Clone> FunVec<2, Elem> for HashMap<usize, BTreeMap<usize, Elem>> {
    fn value(&self, indices: [usize; 2]) -> Elem {
        self.get(&indices[0])
            .expect(
                "FunVec<2, _> from HashMap<usize, BTreeMap<usize, _>>: first-index-out-of-bounds",
            )
            .get(&indices[1])
            .expect(
                "FunVec<2, _> from HashMap<usize, BTreeMap<usize, _>>: second-index-out-of-bounds",
            )
            .clone()
    }
}

// btreemap-x
impl<Elem: Clone> FunVec<2, Elem> for BTreeMap<usize, Vec<Elem>> {
    fn value(&self, indices: [usize; 2]) -> Elem {
        self.get(&indices[0])
            .expect("FunVec<2, _> from BTreeMap<usize, Vec<_>>: first-index-out-of-bounds")
            .get(indices[1])
            .expect("FunVec<2, _> from BTreeMap<usize, Vec<_>>: second-index-out-of-bounds")
            .clone()
    }
}
impl<Elem: Clone> FunVec<2, Elem> for BTreeMap<usize, HashMap<usize, Elem>> {
    fn value(&self, indices: [usize; 2]) -> Elem {
        self.get(&indices[0])
            .expect(
                "FunVec<2, _> from BTreeMap<usize, HashMap<usize, _>>: first-index-out-of-bounds",
            )
            .get(&indices[1])
            .expect(
                "FunVec<2, _> from BTreeMap<usize, HashMap<usize, _>>: second-index-out-of-bounds",
            )
            .clone()
    }
}
impl<Elem: Clone> FunVec<2, Elem> for BTreeMap<usize, BTreeMap<usize, Elem>> {
    fn value(&self, indices: [usize; 2]) -> Elem {
        self.get(&indices[0])
            .expect(
                "FunVec<2, _> from BTreeMap<usize, BTreeMap<usize, _>>: first-index-out-of-bounds",
            )
            .get(&indices[1])
            .expect(
                "FunVec<2, _> from BTreeMap<usize, BTreeMap<usize, _>>: second-index-out-of-bounds",
            )
            .clone()
    }
}
