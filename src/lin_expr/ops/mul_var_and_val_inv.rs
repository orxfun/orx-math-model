use crate::lin_expr::{DLinExpr, DLinTerm, ExprVec};
use crate::scalars::{DNum, ValNum};
use crate::{Int, LinExpr, Num, Ptr, SymWrap, ValTyp, Var, VarType};
use core::ops::Mul;

// int * var

impl<'m, 'd> Mul<Int<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Int<'m, 'd>) -> Self::Output {
        rhs * self
    }
}

// num * var

impl<'m, 'd> Mul<Num<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Num<'m, 'd>) -> Self::Output {
        rhs * self
    }
}
// var * constant

impl<'m, 'd> Mul<i64> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: i64) -> Self::Output {
        rhs * self
    }
}

impl<'m, 'd> Mul<f64> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl<'m, 'd> Mul<f32> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl<'m, 'd> Mul<usize> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: usize) -> Self::Output {
        rhs * self
    }
}

impl<'m, 'd> Mul<u64> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: u64) -> Self::Output {
        rhs * self
    }
}

impl<'m, 'd> Mul<i32> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: i32) -> Self::Output {
        rhs * self
    }
}

impl<'m, 'd> Mul<u32> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: u32) -> Self::Output {
        rhs * self
    }
}
