use crate::pars::IntPar0Data;
use crate::symbol::Symbol;
use crate::{pos::Pos, Model};

#[derive(Clone, Copy)]
pub struct Int<'a>(pub(crate) Symbol<'a, IntData>);

pub enum IntData {
    IntPar0(Pos),
}

impl IntData {
    pub fn int_value(&self, m: &Model) -> i64 {
        match *self {
            Self::IntPar0(p) => m.int_par0[p].value(),
        }
    }
}

// from

impl<'a> From<(&'a Model, &'a IntPar0Data)> for IntData {
    fn from((m, d): (&'a Model, &'a IntPar0Data)) -> Self {
        Self::IntPar0(Pos::pos_of(&m.int_par0, d))
    }
}
