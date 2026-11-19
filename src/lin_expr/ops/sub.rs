use super::add::const_add_con;
use crate::lin_expr::{Coef, DLinExpr, DLinTerm, DSum, ExprVec};
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

fn term_neg<'m, 'd>(m: &'m Model<'d>, term: &DLinTerm<'d>) -> DLinTerm<'d> {
    let coef = match term.coef {
        Coef::One => Coef::MinusOne,
        Coef::MinusOne => Coef::One,
        Coef::Con(c) => Coef::Con(-c),
        Coef::Ptr(c) => Coef::Ptr(m.val_unchecked::<ValNum>(DNum::neg(c)).ptr()),
    };
    DLinTerm::new(coef, term.var)
}

fn expr_neg<'m, 'd>(m: &'m Model<'d>, expr: Ptr<'d, DLinExpr<'d>>) -> Ptr<'d, DLinExpr<'d>> {
    let expr = expr.get();
    let expr_terms = expr.terms.map(|t| term_neg(m, t));
    let expr_sums = expr.sums.map(|s| sum_neg(m, s));
    let expr_constant = const_neg(m, expr.constant);
    let expr = DLinExpr::new(expr_terms, expr_sums, expr_constant);
    m.lin_exp(expr).ptr()
}

fn sum_neg<'m, 'd>(m: &'m Model<'d>, sum: &DSum<'d>) -> DSum<'d> {
    let expr = expr_neg(m, sum.expr);
    let sets = sum.sets.clone();
    DSum::new(sets, expr)
}

// expr - expr

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        let m = self.m();
        let [t1, t2] = [self.data(), rhs.data()];
        let terms = t1.terms.merge(&t2.terms.map(|t| term_neg(m, t)));
        let sums = t1.sums.merge(&t2.sums.map(|s| sum_neg(m, s)));
        let constant = const_sub_const(m, t1.constant, t2.constant);
        m.lin_exp(DLinExpr::new(terms, sums, constant))
    }
}

// expr - var

impl<'m, 'd> Sub<Var<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        let lhs = self.data();
        let rhs = DLinTerm::new(Coef::MinusOne, rhs.ptr());
        let terms = lhs.terms.push(rhs);
        let sums = lhs.sums.clone();
        self.m().lin_exp(DLinExpr::new(terms, sums, lhs.constant))
    }
}

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        let (m, rhs) = (rhs.m(), rhs.data());
        let lhs = DLinTerm::new(Coef::One, self.ptr());
        let terms = rhs.terms.map(|t| term_neg(m, t)).push_to_front(lhs);
        let sums = rhs.sums.map(|s| sum_neg(m, s));
        let constant = const_neg(m, rhs.constant);
        m.lin_exp(DLinExpr::new(terms, sums, constant))
    }
}

// expr - num

macro_rules! expr_num {
    ($n:ty) => {
        impl<'m, 'd> Sub<$n> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $n) -> Self::Output {
                let (m, lhs) = (self.m(), self.data());
                let constant = const_sub_num(m, lhs.constant, rhs.ptr());
                m.lin_exp(lhs.with_new_constant(Some(constant)))
            }
        }

        impl<'m, 'd> Sub<LinExpr<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                let (m, rhs) = (rhs.m(), rhs.data());
                let terms = rhs.terms.map(|t| term_neg(m, t));
                let sums = rhs.sums.map(|s| sum_neg(m, s));
                let constant = const_sub_num(m, rhs.constant, self.ptr());
                m.lin_exp(DLinExpr::new(terms, sums, Some(constant)))
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
                let (m, lhs) = (self.m(), self.data());
                let constant = const_sub_con(m, lhs.constant, rhs);
                m.lin_exp(lhs.with_new_constant(Some(constant)))
            }
        }

        impl<'m, 'd> Sub<LinExpr<'m, 'd>> for $c {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                let (m, rhs) = (rhs.m(), rhs.data());
                let terms = rhs.terms.map(|t| term_neg(m, t));
                let sums = rhs.sums.map(|s| sum_neg(m, s));
                let constant = const_add_con(m, rhs.constant, self);
                m.lin_exp(DLinExpr::new(terms, sums, Some(constant)))
            }
        }
    };
}

expr_con!(f64);
expr_con!(f32);
expr_con!(i64);
expr_con!(i32);
expr_con!(u64);
expr_con!(u32);
expr_con!(usize);
expr_con!(isize);

// // var - var

// impl<'m, 'd> Sub<Var<'m, 'd>> for Var<'m, 'd> {
//     type Output = LinExpr<'m, 'd>;

//     fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
//         let t2 = DLinTerm::new(Coef::MinusOne, rhs.ptr());
//         let terms = ExprVec::new([self.into(), t2]);
//         self.m().lin_exp(DLinExpr::new(terms, None))
//     }
// }

// // var - num

// macro_rules! var_num {
//     ($n:ty) => {
//         impl<'m, 'd> Sub<$n> for Var<'m, 'd> {
//             type Output = LinExpr<'m, 'd>;

//             fn sub(self, rhs: $n) -> Self::Output {
//                 let terms = ExprVec::new([self.into()]);
//                 let constant = const_sub_num(self.m(), None, rhs.ptr());
//                 self.m().lin_exp(DLinExpr::new(terms, Some(constant)))
//             }
//         }

//         impl<'m, 'd> Sub<Var<'m, 'd>> for $n {
//             type Output = LinExpr<'m, 'd>;

//             fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
//                 let terms = ExprVec::new([DLinTerm::new(Coef::MinusOne, rhs.ptr())]);
//                 let constant = Some(self.ptr());
//                 self.m().lin_exp(DLinExpr::new(terms, constant))
//             }
//         }
//     };
// }

// var_num!(Num<'m, 'd>);
// var_num!(Int<'m, 'd>);

// // var - con

// macro_rules! var_con {
//     ($c:ty) => {
//         impl<'m, 'd> Sub<$c> for Var<'m, 'd> {
//             type Output = LinExpr<'m, 'd>;

//             fn sub(self, rhs: $c) -> Self::Output {
//                 let terms = ExprVec::new([self.into()]);
//                 let constant = const_sub_con(self.m(), None, rhs);
//                 self.m().lin_exp(DLinExpr::new(terms, Some(constant)))
//             }
//         }

//         impl<'m, 'd> Sub<Var<'m, 'd>> for $c {
//             type Output = LinExpr<'m, 'd>;

//             fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
//                 let terms = ExprVec::new([DLinTerm::new(Coef::MinusOne, rhs.ptr())]);
//                 let constant = rhs.m().val_num_con(self).ptr();
//                 rhs.m().lin_exp(DLinExpr::new(terms, Some(constant)))
//             }
//         }
//     };
// }

// var_con!(f64);
// var_con!(f32);
// var_con!(i64);
// var_con!(i32);
// var_con!(u64);
// var_con!(u32);
// var_con!(usize);
// var_con!(isize);
