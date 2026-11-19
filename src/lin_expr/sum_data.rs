use crate::idxs::IdxVec;
use crate::lin_expr::DLinExpr;
use crate::{Elements, ElementsIter, Model, Ptr, Set, SetCatInt, SetTyp};
use alloc::vec::Vec;

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

    // expand - d1

    fn expand_d1(&self, op: impl Fn(&Self)) {
        match self.idxs.set_typ(0) {
            SetTyp::CatInt => self.expand_d1_for::<SetCatInt>(op),
            _ => todo!(),
        }
    }

    fn expand_d1_for<I0: Set>(&self, op: impl Fn(&Self)) {
        assert_eq!(self.idxs.len(), 1);
        let i0 = self.idxs[0];
        let mut elements: Elements<<I0 as Set>::IdxRaw<'d>> = Vec::new().into();
        let i0 = ElementsIter::<I0>::new(i0, &mut elements);
        for _ in i0 {
            op(self);
        }
    }
}

#[cfg(test)]
pub mod ext_test {
    use super::*;
    use crate::constants::{is_neg, is_pos, is_zero};
    use crate::Dim;
    use alloc::string::String;
    use core::fmt::Write;

    impl<'d> DSum<'d> {
        pub fn eval_append_string(&self, f: &mut String) {
            let op = |s: &Self| {};

            match self.idxs.dim() {
                Dim::D1 => self.expand_d1(op),
                _ => todo!(),
            }
        }

        pub fn eval_to_string(&self) -> String {
            let mut f = String::new();
            self.eval_append_string(&mut f);
            f
        }
    }
}
