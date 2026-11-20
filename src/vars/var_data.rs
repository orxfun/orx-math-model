use crate::sets::{DSet, FunSet};
use crate::{Dim, DimVec, Ptr, VarType};

pub struct DVars<'d> {
    typ: VarType,
    sets: DimVec<Ptr<'d, DSet<'d>>>,
}

impl<'d> DVars<'d> {
    pub fn new_d1(typ: VarType, s0: Ptr<'d, DSet<'d>>) -> Self {
        let sets = DimVec::new([s0]);
        Self { typ, sets }
    }

    pub fn new_d2(typ: VarType, s0: Ptr<'d, DSet<'d>>, s1: Ptr<'d, DSet<'d>>) -> Self {
        let sets = DimVec::new([s0, s1]);
        Self { typ, sets }
    }

    // get

    pub fn dim(&self) -> Dim {
        self.sets.dim()
    }

    pub fn typ(&self) -> VarType {
        self.typ
    }

    pub fn set_ptr(&self, i: usize) -> Ptr<'d, FunSet<'d>> {
        debug_assert!(self.sets.len() > i);
        self.sets[i].get().fun_set()
    }
}
