use crate::lin_expr::{DLinExpr, DLinTerm, ExprVec};
use crate::scalars::DSca;
use crate::{Int, LinExpr, Num, Ptr, Var};
use core::ops::Mul;

fn mul_val_var<'m, 'd>(var: Var<'m, 'd>, val: Ptr<'d, DSca<'d>>) -> LinExpr<'m, 'd> {
    let term = DLinTerm::new(Some(val), var.ptr());
    let terms = ExprVec::new([term]);
    let data = DLinExpr::new(terms, None);
    var.m().lin_exp(data)
}

// int * var

impl<'m, 'd> Mul<Var<'m, 'd>> for Int<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        mul_val_var(rhs, self.ptr())
    }
}

impl<'m, 'd> Mul<Int<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Int<'m, 'd>) -> Self::Output {
        mul_val_var(self, rhs.ptr())
    }
}

// num * var

impl<'m, 'd> Mul<Var<'m, 'd>> for Num<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        mul_val_var(rhs, self.ptr())
    }
}

impl<'m, 'd> Mul<Num<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Num<'m, 'd>) -> Self::Output {
        mul_val_var(self, rhs.ptr())
    }
}
