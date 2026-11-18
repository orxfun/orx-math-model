use crate::lin_expr::{Coef, DLinExpr, DLinTerm, ExprVec};
use crate::scalars::{DNum, ValNum};
use crate::{AsF64, Int, LinExpr, Model, Num, Ptr, SymWrap, ValTyp, Var};
use alloc::boxed::Box;
use core::ops::Sub;

type Con<'d> = Option<Ptr<'d, DNum<'d>>>;

fn const_neg<'m, 'd>(m: &'m Model<'d>, c1: Con<'d>) -> Con<'d> {
    c1.map(|c| {
        let fun = Box::new(move || -c.get().eval());
        m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
    })
}

fn const_sub_const<'m, 'd>(m: &'m Model<'d>, c1: Con<'d>, c2: Con<'d>) -> Con<'d> {
    match (c1, c2) {
        (Some(c1), Some(c2)) => Some(m.val::<ValNum>(DNum::sub(c1, c2)).ptr()),
        (Some(c1), None) => Some(c1),
        (None, Some(c2)) => Some(c2),
        (None, None) => None,
    }
}

fn const_sub_num<'m, 'd>(
    m: &'m Model<'d>,
    c1: Con<'d>,
    c2: Ptr<'d, DNum<'d>>,
) -> Ptr<'d, DNum<'d>> {
    match c1 {
        Some(c1) => {
            let fun = Box::new(move || c1.get().eval() - c2.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        None => {
            let fun = Box::new(move || -c2.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
    }
}

fn const_sub_con<'m, 'd>(m: &'m Model<'d>, c1: Con<'d>, c2: impl AsF64) -> Ptr<'d, DNum<'d>> {
    match c1 {
        Some(c1) => {
            let fun = Box::new(move || c1.get().eval() - c2.as_f64());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        None => m.val_num_con(-c2.as_f64()).ptr(),
    }
}

// expr - expr

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        let m = self.m();
        let [t1, t2] = [self.data(), rhs.data()];
        let terms = t1.terms.merge(&t2.terms.map(|t| t.neg(m)));
        let constant = const_sub_const(m, t1.constant, t2.constant);
        m.lin_exp(DLinExpr::new(terms, constant))
    }
}

// expr - var

impl<'m, 'd> Sub<Var<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        let rhs = DLinTerm::new(Coef::MinusOne, rhs.ptr());
        let terms = self.data().terms.push(rhs);
        self.m().lin_exp(DLinExpr::new(terms, self.data().constant))
    }
}

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        let m = self.m();
        let lhs = DLinTerm::new(Coef::One, self.ptr());
        let terms = rhs.data().terms.map(|x| x.neg(m)).push_to_front(lhs);
        let constant = const_neg(m, rhs.data().constant);
        m.lin_exp(DLinExpr::new(terms, constant))
    }
}

// expr - num

macro_rules! expr_num {
    ($n:ty) => {
        impl<'m, 'd> Sub<$n> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $n) -> Self::Output {
                let terms = self.data().terms.clone();
                let constant = const_sub_num(self.m(), self.data().constant, rhs.ptr());
                self.m().lin_exp(DLinExpr::new(terms, Some(constant)))
            }
        }

        impl<'m, 'd> Sub<LinExpr<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                let terms = rhs.data().terms.map(|x| x.neg(rhs.m()));
                let constant = const_sub_num(rhs.m(), rhs.data().constant, self.ptr());
                rhs.m().lin_exp(DLinExpr::new(terms, Some(constant)))
            }
        }
    };
}

expr_num!(Num<'m, 'd>);
expr_num!(Int<'m, 'd>);

// expr - const

macro_rules! expr_con {
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

expr_con!(f64);
expr_con!(f32);
expr_con!(i64);
expr_con!(i32);
expr_con!(isize);

// var - var

impl<'m, 'd> Sub<Var<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        let terms = ExprVec::new([self.into(), DLinTerm::new(Coef::One, rhs.ptr())]);
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
                let terms = ExprVec::new([DLinTerm::new(Coef::MinusOne, rhs.ptr())]);
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
                let terms = ExprVec::new([DLinTerm::new(Coef::MinusOne, rhs.ptr())]);
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
