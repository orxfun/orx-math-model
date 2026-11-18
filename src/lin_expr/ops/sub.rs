use crate::lin_expr::{DLinExpr, DLinTerm, ExprVec};
use crate::{Int, LinExpr, Num, SymWrap, Var};
use core::ops::Sub;

// expr - expr

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        let data = self.data().sub(self.m(), rhs.data());
        self.m().lin_exp(data)
    }
}

// expr - var

impl<'m, 'd> Sub<Var<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        let rhs = DLinTerm::from_neg_var(rhs);
        let data = self.data().add_term(rhs);
        self.m().lin_exp(data)
    }
}

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        LinExpr::from(self) + rhs.neg()
    }
}

// expr - num

macro_rules! expr_minus_num {
    ($n:ty) => {
        impl<'m, 'd> Sub<$n> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $n) -> Self::Output {
                let data = self.data().sub_constant(self.m(), rhs.ptr());
                self.m().lin_exp(data)
            }
        }

        impl<'m, 'd> Sub<LinExpr<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                self + rhs.neg()
            }
        }
    };
}

expr_minus_num!(Int<'m, 'd>);
expr_minus_num!(Num<'m, 'd>);

// expr - const

macro_rules! expr_minus_con {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let c = self.m().val_num_con(-rhs).ptr();
                let data = self.data().add_constant(self.m(), c);
                self.m().lin_exp(data)
            }
        }

        impl<'m, 'd> Sub<LinExpr<'m, 'd>> for $c {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                self + rhs.neg()
            }
        }
    };
}

expr_minus_con!(f64);
expr_minus_con!(f32);
expr_minus_con!(i64);
expr_minus_con!(i32);
expr_minus_con!(isize);

// var - var

impl<'m, 'd> Sub<Var<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        let terms = ExprVec::new([self.into(), DLinTerm::from_neg_var(rhs)]);
        let data = DLinExpr::new(terms, None);
        self.m().lin_exp(data)
    }
}

// var - num

macro_rules! var_minus_num {
    ($n:ty) => {
        impl<'m, 'd> Sub<$n> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $n) -> Self::Output {
                let terms = ExprVec::new([self.into()]);
                let constant = Some(self.m().val_neg(rhs.ptr()));
                self.m().lin_exp(DLinExpr::new(terms, constant))
            }
        }

        impl<'m, 'd> Sub<Var<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
                let terms = ExprVec::new([DLinTerm::from_neg_var(rhs)]);
                let constant = Some(self.ptr());
                self.m().lin_exp(DLinExpr::new(terms, constant))
            }
        }
    };
}

var_minus_num!(Num<'m, 'd>);
var_minus_num!(Int<'m, 'd>);

// var - con

macro_rules! var_minus_con {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let terms = ExprVec::new([self.into()]);
                let constant = Some(self.m().val_num_con(rhs).ptr());
                self.m().lin_exp(DLinExpr::new(terms, constant))
            }
        }

        impl<'m, 'd> Sub<Var<'m, 'd>> for $c {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
                let terms = ExprVec::new([DLinTerm::from_neg_var(rhs)]);
                let constant = Some(rhs.m().val_num_con(-self).ptr());
                rhs.m().lin_exp(DLinExpr::new(terms, constant))
            }
        }
    };
}

var_minus_con!(f64);
var_minus_con!(f32);
var_minus_con!(i64);
var_minus_con!(i32);
var_minus_con!(isize);
