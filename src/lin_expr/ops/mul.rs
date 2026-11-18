use crate::lin_expr::{Coef, DLinExpr, DLinTerm, ExprVec};
use crate::scalars::{DNum, ValNum};
use crate::{Int, LinExpr, Num, Ptr, SymWrap, ValTyp, Var, VarType};
use core::ops::Mul;

// expr * num

macro_rules! expr_mul_num {
    ($n:ty) => {
        impl<'m, 'd> Mul<$n> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn mul(self, rhs: $n) -> Self::Output {
                let data = self.data().mul_num(self.m(), rhs.ptr());
                self.m().lin_exp(data)
            }
        }
    };
}

expr_mul_num!(Num<'m, 'd>);
expr_mul_num!(Int<'m, 'd>);

// expr * con

impl<'m, 'd> Mul<f64> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: f64) -> Self::Output {
        // let data = self.data().mul_num(self.m(), rhs.ptr());
        // self.m().lin_exp(data)
        todo!()
    }
}

// var * num

macro_rules! var_mul_num {
    ($n:ty) => {
        impl<'m, 'd> Mul<$n> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn mul(self, rhs: $n) -> Self::Output {
                let term = DLinTerm::new(Coef::Ptr(rhs.ptr()), self.ptr());
                let terms = ExprVec::new([term]);
                let data = DLinExpr::new(terms, None);
                self.m().lin_exp(data)
            }
        }

        impl<'m, 'd> Mul<Var<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
                rhs * self
            }
        }
    };
}

var_mul_num!(Int<'m, 'd>);
var_mul_num!(Num<'m, 'd>);

// var * con

macro_rules! var_mul_con {
    ($c:ty) => {
        impl<'m, 'd> Mul<$c> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn mul(self, rhs: $c) -> Self::Output {
                self * self.m().val_num_con(rhs)
            }
        }

        impl<'m, 'd> Mul<Var<'m, 'd>> for $c {
            type Output = LinExpr<'m, 'd>;

            fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
                rhs * self
            }
        }
    };
}

var_mul_con!(f64);
var_mul_con!(f32);
var_mul_con!(i64);
var_mul_con!(i32);
var_mul_con!(u64);
var_mul_con!(u32);
var_mul_con!(usize);
var_mul_con!(isize);
