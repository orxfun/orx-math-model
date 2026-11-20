use crate::lin_expr::{Coef, DLinExpr, DLinTerm, SumVec, TermVec};
use crate::scalars::DVar;
use crate::vars::VarD0;
use crate::{LinExpr, SymWrap, Var};

impl<'m, 'd> From<Var<'m, 'd>> for DLinTerm<'d> {
    fn from(value: Var<'m, 'd>) -> Self {
        DLinTerm::new(Coef::One, value.ptr())
    }
}

impl<'m, 'd> From<Var<'m, 'd>> for LinExpr<'m, 'd> {
    fn from(value: Var<'m, 'd>) -> Self {
        let terms = TermVec::new([value.into()]);
        let sums = SumVec::new([]);
        let data = DLinExpr::new(terms, sums, None);
        value.m().lin_exp(data)
    }
}

impl<'m, 'd> From<VarD0<'m, 'd>> for DLinTerm<'d> {
    fn from(value: VarD0<'m, 'd>) -> Self {
        Var::from(value).into()
    }
}

impl<'m, 'd> From<VarD0<'m, 'd>> for LinExpr<'m, 'd> {
    fn from(value: VarD0<'m, 'd>) -> Self {
        Var::from(value).into()
    }
}

impl<'m, 'd> From<VarD0<'m, 'd>> for Var<'m, 'd> {
    fn from(value: VarD0<'m, 'd>) -> Self {
        let data = DVar::create_d0(value.ptr());
        value.m().var(data)
    }
}
