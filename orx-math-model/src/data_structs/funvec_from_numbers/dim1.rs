use crate::{
    data_structs::funvec::FunVec,
    numerics::num::{Num, RealNum},
};
use std::collections::{BTreeMap, HashMap};

macro_rules! impl_num {
    ($num:ty) => {
        impl FunVec<1, RealNum> for Vec<$num> {
            fn value(&self, indices: [usize; 1]) -> RealNum {
                self.get(indices[0])
                    .expect("FunVec<1, _> from Vec<_>: index-out-of-bounds")
                    .into_real_num()
            }
        }

        impl FunVec<1, RealNum> for HashMap<usize, $num> {
            fn value(&self, indices: [usize; 1]) -> RealNum {
                self.get(&indices[0])
                    .expect("FunVec<1, _> from HashMap<usize, _>: index-out-of-bounds")
                    .into_real_num()
            }
        }

        impl FunVec<1, RealNum> for BTreeMap<usize, $num> {
            fn value(&self, indices: [usize; 1]) -> RealNum {
                self.get(&indices[0])
                    .expect("FunVec<1, _> from BTreeMap<usize, _>: index-out-of-bounds")
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
