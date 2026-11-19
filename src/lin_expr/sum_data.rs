use crate::idxs::IdxVec;
use crate::lin_expr::DLinExpr;
use crate::{Model, Ptr};

#[derive(derive_new::new, Clone)]
pub struct DSum<'d> {
    pub idxs: IdxVec<'d>,
    pub expr: Ptr<'d, DLinExpr<'d>>,
}

impl<'d> DSum<'d> {
    pub fn neg<'m>(&self, m: &'m Model<'d>) -> Self {
        Self::new(self.idxs.clone(), self.expr.get().neg(m))
    }

    pub fn with_expr(&self, expr: Ptr<'d, DLinExpr<'d>>) -> Self {
        Self::new(self.idxs.clone(), expr)
    }
}

#[cfg(test)]
pub mod ext_test {
    use super::*;
    use crate::constants::{is_neg, is_pos, is_zero};
    use alloc::string::String;
    use core::fmt::Write;

    impl<'d> DSum<'d> {
        pub fn eval_append_string(&self, f: &mut String) {
            //
        }

        pub fn eval_to_string(&self) -> String {
            let mut f = String::new();
            self.eval_append_string(&mut f);
            f
        }
    }
}
