use crate::{lin_expr::DLinExpr, Ptr};

pub enum IneqTyp {
    Leq,
    Geq,
    Eq,
}

#[derive(derive_new::new)]
pub struct DIneq<'d> {
    lhs: Ptr<'d, DLinExpr<'d>>,
    rhs: Ptr<'d, DLinExpr<'d>>,
    typ: IneqTyp,
}

impl<'d> DIneq<'d> {}
