use crate::pars::IntPar0Data;
use crate::scalar_int::{FunIntData, RangeElemData};
use crate::symbol::Symbol;
use crate::{pos::Pos, Model};

#[derive(Clone, Copy)]
pub struct Int<'a>(pub(crate) Symbol<'a, IntData>);

// data

pub enum IntData {
    IntPar0(Pos),
    RangeElem(Pos),
    FunInt(Pos),
}

impl IntData {
    pub fn value(&self, m: &Model) -> i64 {
        match *self {
            Self::IntPar0(p) => m.int_par0[p].value(),
            Self::RangeElem(p) => m.range_elem_value[p],
            Self::FunInt(p) => m.fun_int[p].value(m),
        }
    }
}

// from

impl<'a> From<(&'a Model, &'a IntPar0Data)> for IntData {
    fn from((m, d): (&'a Model, &'a IntPar0Data)) -> Self {
        Self::IntPar0(Pos::pos_of(&m.int_par0, d))
    }
}

impl From<&RangeElemData> for IntData {
    fn from(value: &RangeElemData) -> Self {
        Self::RangeElem(value.value_pos)
    }
}

impl<'a> From<(&'a Model, &'a FunIntData)> for IntData {
    fn from((m, d): (&'a Model, &'a FunIntData)) -> Self {
        Self::FunInt(Pos::pos_of(&m.fun_int, d))
    }
}
