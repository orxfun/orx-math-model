use crate::lin_expr::{Coef, DLinExpr, DLinTerm, ExprVec};
use crate::scalars::{DNum, ValNum};
use crate::{Int, LinExpr, Num, Ptr, SymWrap, ValTyp, Var, VarType};
use core::ops::Mul;

// expr * num

macro_rules! expr_times_num {
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

expr_times_num!(Num<'m, 'd>);
expr_times_num!(Int<'m, 'd>);

// expr * con

macro_rules! expr_times_con {
    ($c:ty) => {
        impl<'m, 'd> Mul<$c> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn mul(self, rhs: $c) -> Self::Output {
                let data = self.data().mul_con(self.m(), rhs);
                self.m().lin_exp(data)
            }
        }

        impl<'m, 'd> Mul<LinExpr<'m, 'd>> for $c {
            type Output = LinExpr<'m, 'd>;

            fn mul(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                rhs * self
            }
        }
    };
}

expr_times_con!(f64);
expr_times_con!(f32);
expr_times_con!(i64);
expr_times_con!(i32);
expr_times_con!(u64);
expr_times_con!(u32);
expr_times_con!(usize);
expr_times_con!(isize);

// var * num

macro_rules! var_times_num {
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

var_times_num!(Int<'m, 'd>);
var_times_num!(Num<'m, 'd>);

// var * con

macro_rules! var_times_con {
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

var_times_con!(f64);
var_times_con!(f32);
var_times_con!(i64);
var_times_con!(i32);
var_times_con!(u64);
var_times_con!(u32);
var_times_con!(usize);
var_times_con!(isize);
