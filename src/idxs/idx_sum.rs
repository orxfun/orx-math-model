use crate::constants::MAX_DIM;
use crate::idxs::DIdx;
use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};
use crate::{ConVec, Ptr};

pub type IdxVec<'d> = ConVec<MAX_DIM, Ptr<'d, DIdx<'d>>>;

#[derive(derive_more::From)]
pub enum Idx<'m, 'd> {
    CatInt(IdxCatInt<'m, 'd>),
    CatStr(IdxCatStr<'m, 'd>),
    Ord(IdxOrd<'m, 'd>),
}

pub enum IdxSum<'m, 'd> {
    D1(Idx<'m, 'd>),
    D2(Idx<'m, 'd>, Idx<'m, 'd>),
}

impl<'m, 'd> IdxSum<'m, 'd> {
    pub(crate) fn into_idx_vec(self) -> IdxVec<'d> {
        match self {
            IdxSum::D1(i0) => todo!(),
            IdxSum::D2(i0, i1) => todo!(),
        }
    }
}

// into

macro_rules! d1 {
    ($i0:ty) => {
        impl<'m, 'd> From<$i0> for IdxSum<'m, 'd> {
            fn from(i0: $i0) -> Self {
                Self::D1(i0.into())
            }
        }
    };
}

d1!(IdxCatInt<'m, 'd>);
d1!(IdxCatStr<'m, 'd>);
d1!(IdxOrd<'m, 'd>);

macro_rules! d2 {
    ($i0:ty, $i1:ty) => {
        impl<'m, 'd> From<($i0, $i1)> for IdxSum<'m, 'd> {
            fn from((i0, i1): ($i0, $i1)) -> Self {
                Self::D2(i0.into(), i1.into())
            }
        }
    };
}

d2!(IdxCatInt<'m, 'd>, IdxCatInt<'m, 'd>);
d2!(IdxCatInt<'m, 'd>, IdxCatStr<'m, 'd>);
d2!(IdxCatInt<'m, 'd>, IdxOrd<'m, 'd>);

d2!(IdxCatStr<'m, 'd>, IdxCatInt<'m, 'd>);
d2!(IdxCatStr<'m, 'd>, IdxCatStr<'m, 'd>);
d2!(IdxCatStr<'m, 'd>, IdxOrd<'m, 'd>);

d2!(IdxOrd<'m, 'd>, IdxCatInt<'m, 'd>);
d2!(IdxOrd<'m, 'd>, IdxCatStr<'m, 'd>);
d2!(IdxOrd<'m, 'd>, IdxOrd<'m, 'd>);
