use crate::{lin_expr::DLinTerm, scalars::DVal, ConVec, Ptr};

pub const MAX_EXPR_LEN: usize = 8;

pub type ExprVec<'d> = ConVec<MAX_EXPR_LEN, DLinTerm<'d>>;

#[derive(derive_new::new)]
pub struct DLinExpr<'d> {
    terms: ExprVec<'d>,
    constant: Option<Ptr<'d, DVal<'d>>>,
}

impl<'d> DLinExpr<'d> {
    pub fn push_term(&self, term: DLinTerm<'d>) -> Self {
        let terms = self.terms.push(term);
        Self::new(terms, self.constant)
    }

    pub fn push_constant(&self, constant: Ptr<'d, DVal<'d>>) -> Self {
        match self.constant {
            Some(c) => todo!(),
            None => Self::new(self.terms.clone(), Some(constant)),
        }
    }
}
