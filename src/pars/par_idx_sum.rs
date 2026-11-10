use crate::indices::{CatIdx, DIdx, OrdIdx};
use crate::values::{DVal, Int};
use alloc::string::String;

pub enum DParIdx<'d> {
    Idx(&'d DIdx<'d>),
    Val(&'d DVal<'d>),
    RawUsize(usize),
    RawStr(&'d String),
    RawI64(i64),
}

pub enum ParIdx<'m, 'd> {
    CatIdx(CatIdx<'m, 'd>),
    OrdIdx(OrdIdx<'m, 'd>),
    Int(Int<'m, 'd>),
    RawUsize(usize),
    RawStr(&'d String),
    RawI64(i64),
}

impl<'m, 'd> From<CatIdx<'m, 'd>> for ParIdx<'m, 'd> {
    fn from(value: CatIdx<'m, 'd>) -> Self {
        Self::CatIdx(value)
    }
}

impl<'m, 'd> From<OrdIdx<'m, 'd>> for ParIdx<'m, 'd> {
    fn from(value: OrdIdx<'m, 'd>) -> Self {
        Self::OrdIdx(value)
    }
}

impl<'m, 'd> From<Int<'m, 'd>> for ParIdx<'m, 'd> {
    fn from(value: Int<'m, 'd>) -> Self {
        Self::Int(value)
    }
}

impl<'m, 'd> From<usize> for ParIdx<'m, 'd> {
    fn from(value: usize) -> Self {
        Self::RawUsize(value)
    }
}

impl<'m, 'd> From<&'d String> for ParIdx<'m, 'd> {
    fn from(value: &'d String) -> Self {
        Self::RawStr(value)
    }
}

impl<'m, 'd> From<i64> for ParIdx<'m, 'd> {
    fn from(value: i64) -> Self {
        Self::RawI64(value)
    }
}

impl<'m, 'd> ParIdx<'m, 'd> {
    pub(crate) fn into_data(&self) -> DParIdx<'d> {
        match self {
            Self::CatIdx(x) => {
                let x = unsafe { x.el.get() } as *const DIdx<'d>;
                DParIdx::Idx(unsafe { &*x })
            }
            Self::OrdIdx(x) => {
                let x = unsafe { x.el.get() } as *const DIdx<'d>;
                DParIdx::Idx(unsafe { &*x })
            }
            Self::Int(x) => {
                let x = unsafe { x.el.get() } as *const DVal<'d>;
                DParIdx::Val(unsafe { &*x })
            }
            Self::RawUsize(x) => DParIdx::RawUsize(*x),
            Self::RawStr(x) => DParIdx::RawStr(*x),
            Self::RawI64(x) => DParIdx::RawI64(*x),
        }
    }
}
