use crate::scalars::ValNum;
use crate::{lin_expr::DLinTerm, scalars::DNum};
use crate::{ConVec, Model, Ptr, SymWrap};

pub const MAX_EXPR_LEN: usize = 8;

pub type ExprVec<'d> = ConVec<MAX_EXPR_LEN, DLinTerm<'d>>;

#[derive(derive_new::new)]
pub struct DLinExpr<'d> {
    terms: ExprVec<'d>,
    constant: Option<Ptr<'d, DNum<'d>>>,
}

impl<'d> DLinExpr<'d> {
    pub fn push_term(&self, term: DLinTerm<'d>) -> Self {
        let terms = self.terms.push(term);
        Self::new(terms, self.constant)
    }

    pub fn add_constant<'m>(&self, m: &'m Model<'d>, constant: Ptr<'d, DNum<'d>>) -> Self {
        match self.constant {
            Some(c) => {
                let constant = m.val::<ValNum>(DNum::add(c, constant)).ptr();
                Self::new(self.terms.clone(), Some(constant))
            }
            None => Self::new(self.terms.clone(), Some(constant)),
        }
    }
}
