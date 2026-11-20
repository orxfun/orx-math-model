use crate::{pars::NumParD0, Model, ValTyp};
use alloc::string::{String, ToString};

pub fn par(key: impl ToString) -> BPar {
    BPar(key.to_string())
}

pub struct BPar(String);

impl BPar {
    pub fn d0<'m, 'd>(self, m: &'m Model<'d>) -> BParD0<'m, 'd> {
        BParD0(self.0, m)
    }
}

pub struct BParD0<'m, 'd>(String, &'m Model<'d>);

impl<'m, 'd> BParD0<'m, 'd> {
    pub fn num(self) -> NumParD0<'m, 'd> {
        self.1.par0_num()
    }
}
