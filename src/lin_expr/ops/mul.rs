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

// var * num

macro_rules! var_mul_num {
    ($n:ty) => {
        impl<'m, 'd> Mul<$n> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn mul(self, rhs: $n) -> Self::Output {
                mul_val_var(self, rhs.ptr())
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
