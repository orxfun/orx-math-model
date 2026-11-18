use crate::scalars::ValNum;
use crate::{lin_expr::DLinTerm, scalars::DNum};
use crate::{ConVec, Model, Ptr, SymWrap, Var};

pub const MAX_EXPR_LEN: usize = 8;

pub type ExprVec<'d> = ConVec<MAX_EXPR_LEN, DLinTerm<'d>>;

#[derive(derive_new::new)]
pub struct DLinExpr<'d> {
    terms: ExprVec<'d>,
    constant: Option<Ptr<'d, DNum<'d>>>,
}

impl<'d> DLinExpr<'d> {
    // expr

    pub fn neg<'m>(&self, m: &'m Model<'d>) -> Self {
        let terms = self.terms.map(|x| x.neg(m));
        todo!()
    }

    pub fn add<'m>(&self, m: &'m Model<'d>, rhs: &Self) -> Self {
        let terms = self.terms.merge(&rhs.terms);
        let constant = match (self.constant, rhs.constant) {
            (Some(c1), Some(c2)) => Some(m.val_add(c1, c2)),
            (Some(c1), None) => Some(c1),
            (None, Some(c2)) => Some(c2),
            (None, None) => None,
        };
        Self { terms, constant }
    }

    pub fn sub<'m>(&self, m: &'m Model<'d>, rhs: &Self) -> Self {
        let terms = self.terms.merge(&rhs.terms);
        let constant = match (self.constant, rhs.constant) {
            (Some(c1), Some(c2)) => Some(m.val_sub(c1, c2)),
            (Some(c1), None) => Some(c1),
            (None, Some(c2)) => Some(m.val_neg(c2)),
            (None, None) => None,
        };
        Self { terms, constant }
    }

    // term

    pub fn add_term(&self, term: DLinTerm<'d>) -> Self {
        let terms = self.terms.push(term);
        Self::new(terms, self.constant)
    }

    // var

    pub fn sub_var<'m>(&self, m: &'m Model<'d>, var: Var<'m, 'd>) -> Self {
        let term = DLinTerm::new(Some(m.minus_one().ptr()), var.ptr());
        // let terms = self.terms.push(var);
        // Self::new(terms, self.constant)
        todo!()
    }

    // con

    pub fn add_constant<'m>(&self, m: &'m Model<'d>, constant: Ptr<'d, DNum<'d>>) -> Self {
        match self.constant {
            Some(c) => Self::new(self.terms.clone(), Some(m.val_add(c, constant))),
            None => Self::new(self.terms.clone(), Some(constant)),
        }
    }

    pub fn sub_constant<'m>(&self, m: &'m Model<'d>, constant: Ptr<'d, DNum<'d>>) -> Self {
        match self.constant {
            Some(c) => Self::new(self.terms.clone(), Some(m.val_sub(c, constant))),
            None => Self::new(self.terms.clone(), Some(m.val_neg(constant))),
        }
    }
}
