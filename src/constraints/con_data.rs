use crate::idxs::IdxVec;
use crate::lin_expr::DLinExpr;
use crate::Ptr;

pub enum IneqTyp {
    Leq,
    Geq,
    Eq,
}

#[derive(derive_new::new)]
pub struct DCon<'d> {
    idxs: IdxVec<'d>,
    lhs: Ptr<'d, DLinExpr<'d>>,
    rhs: Ptr<'d, DLinExpr<'d>>,
    typ: IneqTyp,
}
