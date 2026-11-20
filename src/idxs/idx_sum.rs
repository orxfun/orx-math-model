use crate::idxs::{DIdx, IdxCatInt, IdxCatStr, IdxOrd};
use crate::{DimVec, Model, Ptr, SetTyp, SymWrap};

pub type IdxVec<'d> = DimVec<Ptr<'d, DIdx<'d>>>;

impl<'d> IdxVec<'d> {
    pub fn set_typ(&self, idx: usize) -> SetTyp {
        self[idx].get().set_ptr().get().typ()
    }
}
