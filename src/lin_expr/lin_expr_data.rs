use crate::{lin_expr::DLinTerm, scalars::DNum};
use crate::{ConVec, Ptr};

pub const MAX_EXPR_LEN: usize = 8;

pub type ExprVec<'d> = ConVec<MAX_EXPR_LEN, DLinTerm<'d>>;

#[derive(derive_new::new)]
pub struct DLinExpr<'d> {
    pub(super) terms: ExprVec<'d>,
    pub(super) constant: Option<Ptr<'d, DNum<'d>>>,
}

impl<'d> DLinExpr<'d> {
    // get

    #[cfg(test)]
    pub fn eval_append_string(&self, f: &mut alloc::string::String) {
        use alloc::string::String;
        use core::fmt::Write;

        let mut terms = self.terms.iter();
        if let Some(t) = terms.next() {
            t.eval_append_string(f, false);
            for t in terms {
                write!(f, " + ").unwrap();
                t.eval_append_string(f, true);
            }
        }

        match (self.constant, self.terms.is_empty()) {
            (Some(c), true) => write!(f, "{}", c.get().eval()).unwrap(),
            (Some(c), false) => write!(f, " + {}", c.get().eval()).unwrap(),
            _ => {}
        }
    }

    #[cfg(test)]
    pub fn eval_to_string(&self) -> alloc::string::String {
        let mut f = alloc::string::String::new();
        self.eval_append_string(&mut f);
        f
    }
}
