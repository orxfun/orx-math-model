use crate::lin_expr::{Coef, DLinExpr, DLinTerm, DSum, ExprVec};
use crate::scalars::{DNum, ValNum};
use crate::{AsF64, Int, LinExpr, Model, Num, Ptr, SymWrap, ValTyp, Var, VarType};
use alloc::boxed::Box;
use core::ops::Mul;

// num

type Con<'d> = Option<Ptr<'d, DNum<'d>>>;

fn const_mul_num<'m, 'd>(m: &'m Model<'d>, c1: Con<'d>, c2: Ptr<'d, DNum<'d>>) -> Con<'d> {
    c1.map(|c1| {
        let fun = Box::new(move || c1.get().eval() * c2.get().eval());
        m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
    })
}

fn term_mul_num<'m, 'd>(
    m: &'m Model<'d>,
    term: &DLinTerm<'d>,
    num: Ptr<'d, DNum<'d>>,
) -> DLinTerm<'d> {
    let coef = Coef::Ptr(match term.coef {
        Coef::One => num,
        Coef::MinusOne => {
            let fun = Box::new(move || -num.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        Coef::Con(c) => {
            let fun = Box::new(move || c * num.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        Coef::Ptr(c) => {
            let fun = Box::new(move || c.get().eval() * num.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
    });
    DLinTerm::new(coef, term.var)
}

fn sum_mul_num<'m, 'd>(m: &'m Model<'d>, sum: &DSum<'d>, num: Ptr<'d, DNum<'d>>) -> DSum<'d> {
    sum.with_expr(expr_mul_num(m, sum.expr, num))
}

fn expr_mul_num<'m, 'd>(
    m: &'m Model<'d>,
    expr: Ptr<'d, DLinExpr<'d>>,
    num: Ptr<'d, DNum<'d>>,
) -> Ptr<'d, DLinExpr<'d>> {
    let expr = expr.get();
    let terms = expr.terms.map(|term| term_mul_num(m, term, num));
    let sums = expr.sums.map(|sum| sum_mul_num(m, sum, num));
    let constant = const_mul_num(m, expr.constant, num);
    let expr = DLinExpr::new(terms, sums, constant);
    m.lin_exp(expr).ptr()
}

// con

fn const_mul_con<'m, 'd>(m: &'m Model<'d>, c1: Con<'d>, c2: impl AsF64) -> Con<'d> {
    c1.map(|c1| {
        let fun = Box::new(move || c1.get().eval() * c2.as_f64());
        m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
    })
}

fn term_mul_con<'m, 'd>(m: &'m Model<'d>, term: &DLinTerm<'d>, con: impl AsF64) -> DLinTerm<'d> {
    let coef = match term.coef {
        Coef::One => Coef::Con(con.as_f64()),
        Coef::MinusOne => Coef::Con(-con.as_f64()),
        Coef::Con(c) => Coef::Con(c * con.as_f64()),
        Coef::Ptr(c) => {
            let fun = Box::new(move || c.get().eval() * con.as_f64());
            Coef::Ptr(m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr())
        }
    };
    DLinTerm::new(coef, term.var)
}

fn sum_mul_con<'m, 'd>(m: &'m Model<'d>, sum: &DSum<'d>, con: impl AsF64) -> DSum<'d> {
    sum.with_expr(expr_mul_con(m, sum.expr, con))
}

fn expr_mul_con<'m, 'd>(
    m: &'m Model<'d>,
    expr: Ptr<'d, DLinExpr<'d>>,
    con: impl AsF64,
) -> Ptr<'d, DLinExpr<'d>> {
    let expr = expr.get();
    let terms = expr.terms.map(|term| term_mul_con(m, term, con));
    let sums = expr.sums.map(|sum| sum_mul_con(m, sum, con));
    let constant = const_mul_con(m, expr.constant, con);
    let expr = DLinExpr::new(terms, sums, constant);
    m.lin_exp(expr).ptr()
}

// expr * num

// macro_rules! expr_num {
//     ($n:ty) => {
//         impl<'m, 'd> Mul<$n> for LinExpr<'m, 'd> {
//             type Output = LinExpr<'m, 'd>;

//             fn mul(self, rhs: $n) -> Self::Output {
//                 let m = self.m();
//                 let terms = self.data().terms.map(|t| term_mul_num(m, t, rhs.ptr()));
//                 let constant = const_mul_num(m, self.data().constant, rhs.ptr());
//                 m.lin_exp(DLinExpr::new(terms, constant))
//             }
//         }

//         impl<'m, 'd> Mul<LinExpr<'m, 'd>> for $n {
//             type Output = LinExpr<'m, 'd>;

//             fn mul(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
//                 rhs * self
//             }
//         }
//     };
// }

// expr_num!(Num<'m, 'd>);
// expr_num!(Int<'m, 'd>);

// // expr * con

// macro_rules! expr_con {
//     ($c:ty) => {
//         impl<'m, 'd> Mul<$c> for LinExpr<'m, 'd> {
//             type Output = LinExpr<'m, 'd>;

//             fn mul(self, rhs: $c) -> Self::Output {
//                 let m = self.m();
//                 let terms = self.data().terms.map(|t| term_mul_con(m, t, rhs));
//                 let constant = const_mul_con(m, self.data().constant, rhs);
//                 m.lin_exp(DLinExpr::new(terms, constant))
//             }
//         }

//         impl<'m, 'd> Mul<LinExpr<'m, 'd>> for $c {
//             type Output = LinExpr<'m, 'd>;

//             fn mul(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
//                 rhs * self
//             }
//         }
//     };
// }

// expr_con!(f64);
// expr_con!(f32);
// expr_con!(i64);
// expr_con!(i32);
// expr_con!(u64);
// expr_con!(u32);
// expr_con!(usize);
// expr_con!(isize);

// // var * num

// macro_rules! var_num {
//     ($n:ty) => {
//         impl<'m, 'd> Mul<$n> for Var<'m, 'd> {
//             type Output = LinExpr<'m, 'd>;

//             fn mul(self, rhs: $n) -> Self::Output {
//                 let term = DLinTerm::new(Coef::Ptr(rhs.ptr()), self.ptr());
//                 let terms = ExprVec::new([term]);
//                 self.m().lin_exp(DLinExpr::new(terms, None))
//             }
//         }

//         impl<'m, 'd> Mul<Var<'m, 'd>> for $n {
//             type Output = LinExpr<'m, 'd>;

//             fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
//                 rhs * self
//             }
//         }
//     };
// }

// var_num!(Num<'m, 'd>);
// var_num!(Int<'m, 'd>);

// // var * con

// macro_rules! var_con {
//     ($c:ty) => {
//         impl<'m, 'd> Mul<$c> for Var<'m, 'd> {
//             type Output = LinExpr<'m, 'd>;

//             fn mul(self, rhs: $c) -> Self::Output {
//                 let term = DLinTerm::new(Coef::Con(rhs.as_f64()), self.ptr());
//                 let terms = ExprVec::new([term]);
//                 self.m().lin_exp(DLinExpr::new(terms, None))
//             }
//         }

//         impl<'m, 'd> Mul<Var<'m, 'd>> for $c {
//             type Output = LinExpr<'m, 'd>;

//             fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
//                 rhs * self
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
