use crate::lin_expr::{DLinExpr, DLinTerm, ExprVec};
use crate::scalars::{DNum, ValNum};
use crate::{Int, LinExpr, Num, Ptr, SymWrap, ValTyp, Var, VarType};
use core::ops::Mul;

fn mul_val_var<'m, 'd>(var: Var<'m, 'd>, val: Ptr<'d, DNum<'d>>) -> LinExpr<'m, 'd> {
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

// num * var

impl<'m, 'd> Mul<Var<'m, 'd>> for Num<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        mul_val_var(rhs, self.ptr())
    }
}

// constant * var

impl<'m, 'd> Mul<Var<'m, 'd>> for i64 {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        rhs * rhs.m().val_num_con(self)
    }
}

impl<'m, 'd> Mul<Var<'m, 'd>> for f64 {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        rhs * rhs.m().val_num_con(self)
    }
}

impl<'m, 'd> Mul<Var<'m, 'd>> for f32 {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        self as f64 * rhs
    }
}

impl<'m, 'd> Mul<Var<'m, 'd>> for usize {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        self as i64 * rhs
    }
}

impl<'m, 'd> Mul<Var<'m, 'd>> for u64 {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        self as i64 * rhs
    }
}

impl<'m, 'd> Mul<Var<'m, 'd>> for i32 {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        self as i64 * rhs
    }
}

impl<'m, 'd> Mul<Var<'m, 'd>> for u32 {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        self as i64 * rhs
    }
}
