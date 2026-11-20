use crate::sets::{DSet, FunSet};
use crate::vars::{BoundD0, BoundD1, BoundD2, Bounds, UBounds};
use crate::{Dim, DimVec, Ptr, VarType};
use alloc::string::String;

#[derive(derive_new::new)]
pub struct DVars<'d> {
    key: String,
    typ: VarType,
    bounds: UBounds<'d>,
    sets: DimVec<Ptr<'d, DSet<'d>>>,
}

impl<'d> DVars<'d> {
    pub fn new_d0(key: String, typ: VarType) -> Self {
        let sets = DimVec::new([]);
        let bounds = UBounds {
            d0: Bounds::new(BoundD0::None, BoundD0::None),
        };
        Self::new(key, typ, bounds, sets)
    }

    pub fn new_d1(key: String, typ: VarType, s0: Ptr<'d, DSet<'d>>) -> Self {
        let sets = DimVec::new([s0]);
        let bounds = UBounds {
            d1: Bounds::new(BoundD1::None, BoundD1::None),
        };
        Self::new(key, typ, bounds, sets)
    }

    pub fn new_d2(key: String, typ: VarType, s0: Ptr<'d, DSet<'d>>, s1: Ptr<'d, DSet<'d>>) -> Self {
        let sets = DimVec::new([s0, s1]);
        let bounds = UBounds {
            d2: Bounds::new(BoundD2::None, BoundD2::None),
        };
        Self::new(key, typ, bounds, sets)
    }

    // set bounds

    pub fn set_bound_d0(&mut self, lower: bool, bound: impl Into<BoundD0<'d>>) {
        let bounds = unsafe { &mut self.bounds.d0 };
        match lower {
            true => bounds.lower = bound.into(),
            false => bounds.upper = bound.into(),
        }
    }

    pub fn set_bound_d1(&mut self, lower: bool, bound: impl Into<BoundD1<'d>>) {
        let bounds = unsafe { &mut self.bounds.d1 };
        match lower {
            true => bounds.lower = bound.into(),
            false => bounds.upper = bound.into(),
        }
    }

    pub fn set_bound_d2(&mut self, lower: bool, bound: impl Into<BoundD2<'d>>) {
        let bounds = unsafe { &mut self.bounds.d2 };
        match lower {
            true => bounds.lower = bound.into(),
            false => bounds.upper = bound.into(),
        }
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
