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
