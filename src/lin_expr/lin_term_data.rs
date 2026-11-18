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
    coef: Coef<'d>,
    var: Ptr<'d, DVar<'d>>,
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

    pub fn mul_num<'m>(&self, m: &'m Model<'d>, rhs: Ptr<'d, DNum<'d>>) -> Self {
        let coef = match self.coef {
            Coef::One => Coef::Ptr(rhs),
            Coef::MinusOne => Coef::Ptr(m.val_neg(rhs)),
            Coef::Con(c) => Coef::Ptr(m.val::<ValNum>(DNum::mul_con(rhs, c)).ptr()),
            Coef::Ptr(c) => Coef::Ptr(m.val::<ValNum>(DNum::mul(c, rhs)).ptr()),
        };
        Self::new(coef, self.var)
    }

    pub fn mul_con<'m>(&self, m: &'m Model<'d>, rhs: impl AsF64) -> Self {
        let coef = match self.coef {
            Coef::One => Coef::Con(rhs.as_f64()),
            Coef::MinusOne => Coef::Con(-rhs.as_f64()),
            Coef::Con(c) => Coef::Con(c * rhs.as_f64()),
            Coef::Ptr(c) => Coef::Ptr(m.val::<ValNum>(DNum::mul_con(c, rhs.as_f64())).ptr()),
        };
        Self::new(coef, self.var)
    }
}

#[cfg(test)]
impl<'d> DLinTerm<'d> {
    pub fn eval_coef(&self) -> f64 {
        match self.coef {
            Coef::One => 1.0,
            Coef::MinusOne => -1.0,
            Coef::Con(x) => x,
            Coef::Ptr(p) => p.get().eval(),
        }
    }

    pub fn var(&self) -> Ptr<'d, DVar<'d>> {
        self.var
    }
}
