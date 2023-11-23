use crate::{
    data_structs::funvec::FunVec,
    numerics::num::{Num, RealNum},
};

macro_rules! impl_num {
    ($num:ty) => {
        impl FunVec<0, RealNum> for $num {
            fn value(&self, _: [usize; 0]) -> RealNum {
                self.into_real_num()
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
