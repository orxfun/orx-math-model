use crate::scalars::{DNum, DVar, ValNum};
use crate::{AsF64, Model, Ptr, SymWrap, Var};

#[derive(Clone)]
pub enum Coef<'d> {
    One,
    MinusOne,
    Con(f64),
    Ptr(Ptr<'d, DNum<'d>>),
}

#[derive(derive_new::new, Clone)]
pub struct DLinTerm<'d> {
    pub coef: Coef<'d>,
    pub var: Ptr<'d, DVar<'d>>,
}

impl<'d> DLinTerm<'d> {
    // ops

    pub fn neg<'m>(&self, m: &'m Model<'d>) -> Self {
        let coef = match self.coef {
            Coef::One => Coef::MinusOne,
            Coef::MinusOne => Coef::One,
            Coef::Con(c) => Coef::Con(-c),
            Coef::Ptr(c) => Coef::Ptr(m.val_neg(c)),
        };
        Self::new(coef, self.var)
    }
}
