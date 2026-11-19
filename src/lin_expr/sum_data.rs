use crate::expand::expand_d1;
use crate::idxs::IdxVec;
use crate::lin_expr::DLinExpr;
use crate::{Elements, ElementsIter, Model, Ptr, Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
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

    // expand

    fn expand_d1(&self, op: impl FnMut(&Self)) {
        expand_d1(&self.idxs, self, op);
    }

    // expand - d2

    fn expand_d2(&self, op: impl FnMut(&Self)) {
        match self.idxs.set_typ(0) {
            SetTyp::CatInt => self.expand_d2_i0::<SetCatInt>(op),
            SetTyp::CatStr => self.expand_d2_i0::<SetCatStr>(op),
            SetTyp::Ord => self.expand_d2_i0::<SetOrd>(op),
        }
    }

    fn expand_d2_i0<I0: Set>(&self, op: impl FnMut(&Self)) {
        match self.idxs.set_typ(1) {
            SetTyp::CatInt => self.expand_d2_i0_i1::<I0, SetCatInt>(op),
            SetTyp::CatStr => self.expand_d2_i0_i1::<I0, SetCatStr>(op),
            SetTyp::Ord => self.expand_d2_i0_i1::<I0, SetOrd>(op),
        }
    }

    fn expand_d2_i0_i1<I0: Set, I1: Set>(&self, mut op: impl FnMut(&Self)) {
        assert_eq!(self.idxs.len(), 2);
        let mut elem0: Elements<<I0 as Set>::IdxRaw<'d>> = Vec::new().into();
        let mut elem1: Elements<<I1 as Set>::IdxRaw<'d>> = Vec::new().into();

        let [i0, i1] = [0, 1].map(|i| self.idxs[i]);

        let i0 = ElementsIter::<I0>::new(i0, &mut elem0);
        for _ in i0 {
            let i1 = ElementsIter::<I1>::new(i1, &mut elem1);
            for _ in i1 {
                op(self);
            }
        }
    }
}

#[cfg(test)]
pub mod ext_test {
    use super::*;
    use crate::constants::{is_neg, is_pos, is_zero};
    use crate::lin_expr::lin_expr_data::ext_test::{add_opt, write_constant};
    use crate::Dim;
    use alloc::string::String;
    use core::fmt::Write;

    impl<'d> DSum<'d> {
        pub fn eval_append_string_exc_constant(
            &self,
            f: &mut String,
            pre_exists: bool,
        ) -> (bool, Option<f64>) {
            let mut pre_exists = pre_exists;
            let mut acc_const = None;

            let op = |s: &Self| {
                let (added, c) = s.expr.get().eval_append_string_exc_constant(f, pre_exists);
                pre_exists |= added;
                acc_const = add_opt(acc_const, c);
            };

            match self.idxs.dim() {
                Dim::D1 => self.expand_d1(op),
                Dim::D2 => self.expand_d2(op),
                _ => todo!(),
            }

            (pre_exists, acc_const)
        }

        pub fn eval_append_string(&self, f: &mut String, pre_exists: bool) -> bool {
            let (mut pre_exists, c) = self.eval_append_string_exc_constant(f, pre_exists);

            if let Some(c) = c {
                pre_exists |= write_constant(f, pre_exists, c);
            }

            pre_exists
        }

        pub fn eval_to_string(&self) -> String {
            let mut f = String::new();
            self.eval_append_string(&mut f, false);
            f
        }
    }
}
