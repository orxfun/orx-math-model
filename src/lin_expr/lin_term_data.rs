use crate::scalars::{DNum, DVar, ValNum};
use crate::{Model, Ptr, SymWrap, Var};

#[derive(Clone)]
pub enum Coef<'d> {
    One,
    MinusOne,
    Any(Ptr<'d, DNum<'d>>),
}

#[derive(derive_new::new, Clone)]
pub struct DLinTerm<'d> {
    coef: Coef<'d>,
    var: Ptr<'d, DVar<'d>>,
}

impl<'d> DLinTerm<'d> {
    // ops

    pub fn neg<'m>(&self, m: &'m Model<'d>) -> Self {
        let coef = match self.coef {
            Coef::One => Coef::MinusOne,
            Coef::MinusOne => Coef::One,
            Coef::Any(c) => Coef::Any(m.val_neg(c)),
        };
        Self::new(coef, self.var)
    }

    pub fn mul_num<'m>(&self, m: &'m Model<'d>, rhs: Ptr<'d, DNum<'d>>) -> Self {
        let coef = match self.coef {
            Coef::One => Coef::Any(rhs),
            Coef::MinusOne => Coef::Any(m.val_neg(rhs)),
            Coef::Any(c) => Coef::Any(m.val::<ValNum>(DNum::mul(c, rhs)).ptr()),
        };
        Self::new(coef, self.var)
    }
}
