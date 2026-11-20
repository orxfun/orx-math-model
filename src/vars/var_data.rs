use crate::sets::{DSet, FunSet};
use crate::{Dim, DimVec, Ptr, VarType};
use alloc::string::String;

pub struct DVars<'d> {
    key: String,
    typ: VarType,
    sets: DimVec<Ptr<'d, DSet<'d>>>,
}

impl<'d> DVars<'d> {
    pub fn new_d0(key: String, typ: VarType) -> Self {
        let sets = DimVec::new([]);
        Self { key, typ, sets }
    }

    pub fn new_d1(key: String, typ: VarType, s0: Ptr<'d, DSet<'d>>) -> Self {
        let sets = DimVec::new([s0]);
        Self { key, typ, sets }
    }

    pub fn new_d2(key: String, typ: VarType, s0: Ptr<'d, DSet<'d>>, s1: Ptr<'d, DSet<'d>>) -> Self {
        let sets = DimVec::new([s0, s1]);
        Self { key, typ, sets }
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
