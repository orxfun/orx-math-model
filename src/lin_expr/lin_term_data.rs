use crate::scalars::{DNum, DVar, ValNum};
use crate::{Model, Ptr, SymWrap, Var};

pub enum Coef<'d> {
    One,
    MinusOne,
    Any(Ptr<'d, DNum<'d>>),
}

#[derive(derive_new::new, Clone)]
pub struct DLinTerm<'d> {
    coef: Option<Ptr<'d, DNum<'d>>>,
    var: Ptr<'d, DVar<'d>>,
}

impl<'d> DLinTerm<'d> {
    pub fn from_neg_var(var: Var<'_, 'd>) -> Self {
        let coef = Some(var.m().minus_one().ptr());
        Self::new(coef, var.ptr())
    }

    // ops

    pub fn neg<'m>(&self, m: &'m Model<'d>) -> Self {
        let coef = Some(match self.coef {
            Some(c) => m.val_neg(c),
            None => m.minus_one().ptr(),
        });
        Self::new(coef, self.var)
    }

    pub fn mul_num<'m>(&self, m: &'m Model<'d>, rhs: Ptr<'d, DNum<'d>>) -> Self {
        let coef = Some(match self.coef {
            Some(c) => m.val::<ValNum>(DNum::mul(c, rhs)).ptr(),
            None => rhs,
        });
        Self::new(coef, self.var)
    }
}
