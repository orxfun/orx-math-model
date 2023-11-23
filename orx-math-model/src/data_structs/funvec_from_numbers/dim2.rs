use crate::{
    data_structs::funvec::FunVec,
    numerics::num::{Num, RealNum},
};
use std::collections::{BTreeMap, HashMap};

macro_rules! impl_num {
    ($num:ty) => {
        // vec-x
        impl FunVec<2, RealNum> for Vec<Vec<$num>> {
            fn value(&self, indices: [usize; 2]) -> RealNum {
                self.get(indices[0])
                    .expect("FunVec<2, _> from Vec<Vec<_>>: first-index-out-of-bounds")
                    .get(indices[1])
                    .expect("FunVec<2, _> from Vec<Vec<_>>: second-index-out-of-bounds")
                    .into_real_num()
            }
        }
        impl FunVec<2, RealNum> for Vec<HashMap<usize, $num>> {
            fn value(&self, indices: [usize; 2]) -> RealNum {
                self.get(indices[0])
                    .expect("FunVec<2, _> from Vec<HashMap<usize, _>>: first-index-out-of-bounds")
                    .get(&indices[1])
                    .expect("FunVec<2, _> from Vec<HashMap<usize, _>>: second-index-out-of-bounds")
                    .into_real_num()
            }
        }
        impl FunVec<2, RealNum> for Vec<BTreeMap<usize, $num>> {
            fn value(&self, indices: [usize; 2]) -> RealNum {
                self.get(indices[0])
                    .expect("FunVec<2, _> from Vec<BTreeMap<usize, _>>: first-index-out-of-bounds")
                    .get(&indices[1])
                    .expect("FunVec<2, _> from Vec<BTreeMap<usize, _>>: second-index-out-of-bounds")
                    .into_real_num()
            }
        }

        // hashmap-x
        impl FunVec<2, RealNum> for HashMap<usize, Vec<$num>> {
            fn value(&self, indices: [usize; 2]) -> RealNum {
                self.get(&indices[0])
                    .expect("FunVec<2, _> from HashMap<usize, Vec<_>>: first-index-out-of-bounds")
                    .get(indices[1])
                    .expect("FunVec<2, _> from HashMap<usize, Vec<_>>: second-index-out-of-bounds")
                    .into_real_num()
            }
        }
        impl FunVec<2, RealNum> for HashMap<usize, HashMap<usize, $num>> {
            fn value(&self, indices: [usize; 2]) -> RealNum {
                self.get(&indices[0])
                    .expect(
                        "FunVec<2, _> from HashMap<usize, HashMap<usize, _>>: first-index-out-of-bounds",
                    )
                    .get(&indices[1])
                    .expect(
                        "FunVec<2, _> from HashMap<usize, HashMap<usize, _>>: second-index-out-of-bounds",
                    )
                    .into_real_num()
            }
        }
        impl FunVec<2, RealNum> for HashMap<usize, BTreeMap<usize, $num>> {
            fn value(&self, indices: [usize; 2]) -> RealNum {
                self.get(&indices[0])
                    .expect(
                        "FunVec<2, _> from HashMap<usize, BTreeMap<usize, _>>: first-index-out-of-bounds",
                    )
                    .get(&indices[1])
                    .expect(
                        "FunVec<2, _> from HashMap<usize, BTreeMap<usize, _>>: second-index-out-of-bounds",
                    )
                    .into_real_num()
            }
        }

        // btreemap-x
        impl FunVec<2, RealNum> for BTreeMap<usize, Vec<$num>> {
            fn value(&self, indices: [usize; 2]) -> RealNum {
                self.get(&indices[0])
                    .expect("FunVec<2, _> from BTreeMap<usize, Vec<_>>: first-index-out-of-bounds")
                    .get(indices[1])
                    .expect("FunVec<2, _> from BTreeMap<usize, Vec<_>>: second-index-out-of-bounds")
                    .into_real_num()
            }
        }
        impl FunVec<2, RealNum> for BTreeMap<usize, HashMap<usize, $num>> {
            fn value(&self, indices: [usize; 2]) -> RealNum {
                self.get(&indices[0])
                    .expect(
                        "FunVec<2, _> from BTreeMap<usize, HashMap<usize, _>>: first-index-out-of-bounds",
                    )
                    .get(&indices[1])
                    .expect(
                        "FunVec<2, _> from BTreeMap<usize, HashMap<usize, _>>: second-index-out-of-bounds",
                    )
                    .into_real_num()
            }
        }
        impl FunVec<2, RealNum> for BTreeMap<usize, BTreeMap<usize, $num>> {
            fn value(&self, indices: [usize; 2]) -> RealNum {
                self.get(&indices[0])
                    .expect(
                        "FunVec<2, _> from BTreeMap<usize, BTreeMap<usize, _>>: first-index-out-of-bounds",
                    )
                    .get(&indices[1])
                    .expect(
                        "FunVec<2, _> from BTreeMap<usize, BTreeMap<usize, _>>: second-index-out-of-bounds",
                    )
                    .into_real_num()
            }
        }
    };
}

impl_num!(usize);
impl_num!(u32);
impl_num!(u64);
impl_num!(i32);
impl_num!(i64);
impl_num!(f64);
