use crate::lin_expr::{Coef, DLinExpr, DLinTerm, SumVec, TermVec};
use crate::scalars::{DNum, ValNum};
use crate::{LinExpr, Model, Ptr, SymWrap, ValTyp, Var};
use alloc::boxed::Box;
use core::ops::Neg;

impl<'m, 'd> Neg for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn neg(self) -> Self::Output {
        fn const_neg<'m, 'd>(
            m: &'m Model<'d>,
            c1: Option<Ptr<'d, DNum<'d>>>,
        ) -> Option<Ptr<'d, DNum<'d>>> {
            c1.map(|c| {
                let fun = Box::new(move || -c.get().eval());
                m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
            })
        }

        let (m, expr) = (self.m(), self.data());

        let expr_terms = expr.terms.map(|t| t.neg(m));
        let expr_sums = expr.sums.map(|s| s.neg(m));
        let expr_constant = const_neg(m, expr.constant);
        let expr = DLinExpr::new(expr_terms, expr_sums, expr_constant);
        m.lin_exp(expr)
    }
}

impl<'m, 'd> Neg for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn neg(self) -> Self::Output {
        let term = DLinTerm::new(Coef::MinusOne, self.ptr());
        let terms = TermVec::new([term]);
        let sums = SumVec::new([]);
        let data = DLinExpr::new(terms, sums, None);
        self.m().lin_exp(data)
    }
}
