use crate::{
    IntParD0, IntParD1, IntParD2, Model, NumParD0, NumParD1, NumParD2, Set, SymWrap, ValTyp,
};
use alloc::string::{String, ToString};

pub fn par(key: impl ToString) -> BPar {
    BPar(key.to_string())
}

pub struct BPar(String);

impl BPar {
    pub fn d0<'m, 'd>(self, m: &'m Model<'d>) -> BParD0<'m, 'd> {
        BParD0(self.0, m)
    }

    pub fn d1<'m, 'd, I0: Set>(self, i0: I0::SetSym<'m, 'd>) -> BParD1<'m, 'd, I0> {
        BParD1(self.0, i0)
    }

    pub fn d2<'m, 'd, I0: Set, I1: Set>(
        self,
        i0: I0::SetSym<'m, 'd>,
        i1: I1::SetSym<'m, 'd>,
    ) -> BParD2<'m, 'd, I0, I1> {
        BParD2(self.0, i0, i1)
    }
}

// d0

pub struct BParD0<'m, 'd>(String, &'m Model<'d>);

impl<'m, 'd> BParD0<'m, 'd> {
    pub fn num(self) -> NumParD0<'m, 'd> {
        self.1.par0_num(self.0)
    }

    pub fn int(self) -> IntParD0<'m, 'd> {
        self.1.par0_int(self.0)
    }
}

// d1

pub struct BParD1<'m, 'd: 'm, I0: Set>(String, I0::SetSym<'m, 'd>);

impl<'m, 'd, I0: Set> BParD1<'m, 'd, I0> {
    pub fn num(self) -> NumParD1<'m, 'd, I0> {
        self.1.m().par1_num(self.0, self.1)
    }

    pub fn int(self) -> IntParD1<'m, 'd, I0> {
        self.1.m().par1_int(self.0, self.1)
    }
}

// d2

pub struct BParD2<'m, 'd: 'm, I0: Set, I1: Set>(String, I0::SetSym<'m, 'd>, I1::SetSym<'m, 'd>);

impl<'m, 'd, I0: Set, I1: Set> BParD2<'m, 'd, I0, I1> {
    pub fn num(self) -> NumParD2<'m, 'd, I0, I1> {
        self.1.m().par2_num(self.0, self.1, self.2)
    }

    pub fn int(self) -> IntParD2<'m, 'd, I0, I1> {
        self.1.m().par2_int(self.0, self.1, self.2)
    }
}
