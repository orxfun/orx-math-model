use crate::sets::{DSet, FunSet};
use crate::{Dim, DimVec, Ptr, VarType};

pub struct DVars<'d> {
    typ: VarType,
    dim: Dim,
    sets: DimVec<Ptr<'d, DSet<'d>>>,
}

impl<'d> DVars<'d> {
    pub fn new_d1(typ: VarType, s0: Ptr<'d, DSet<'d>>) -> Self {
        let sets = DimVec::new([s0]);
        let dim = Dim::D1;
        Self { typ, dim, sets }
    }

    pub fn new_d2(typ: VarType, s0: Ptr<'d, DSet<'d>>, s1: Ptr<'d, DSet<'d>>) -> Self {
        let sets = DimVec::new([s0, s1]);
        let dim = Dim::D1;
        Self { typ, dim, sets }
    }

    // get

    pub fn typ(&self) -> VarType {
        self.typ
    }

    pub fn set_ptr(&self, i: usize) -> Ptr<'d, FunSet<'d>> {
        debug_assert!(self.sets.len() > i);
        self.sets[i].get().fun_set()
    }
}
