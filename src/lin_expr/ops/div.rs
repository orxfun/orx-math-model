use crate::idxs::IdxOrd;
use crate::lin_expr::{Coef, DLinExpr, DLinTerm, DSum, SumVec, TermVec};
use crate::scalars::{DNum, ValNum};
use crate::{AsF64, Int, LinExpr, Model, Num, Ptr, SymWrap, ValTyp, Var};
use alloc::boxed::Box;
use core::ops::Div;

// num

type Con<'d> = Option<Ptr<'d, DNum<'d>>>;

fn one_div_num<'m, 'd>(m: &'m Model<'d>, num: Ptr<'d, DNum<'d>>) -> Ptr<'d, DNum<'d>> {
    let fun = Box::new(move || 1.0 / num.get().eval());
    m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
}

fn const_div_num<'m, 'd>(m: &'m Model<'d>, c1: Con<'d>, c2: Ptr<'d, DNum<'d>>) -> Con<'d> {
    c1.map(|c1| {
        let fun = Box::new(move || c1.get().eval() / c2.get().eval());
        m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
    })
}

fn term_div_num<'m, 'd>(
    m: &'m Model<'d>,
    term: &DLinTerm<'d>,
    num: Ptr<'d, DNum<'d>>,
) -> DLinTerm<'d> {
    let coef = Coef::Ptr(match term.coef {
        Coef::One => {
            let fun = Box::new(move || 1.0 / num.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        Coef::MinusOne => {
            let fun = Box::new(move || -1.0 / num.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        Coef::Con(c) => {
            let fun = Box::new(move || c / num.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        Coef::Ptr(c) => {
            let fun = Box::new(move || c.get().eval() / num.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
    });
    DLinTerm::new(coef, term.var)
}

fn sum_div_num<'m, 'd>(m: &'m Model<'d>, sum: &DSum<'d>, num: Ptr<'d, DNum<'d>>) -> DSum<'d> {
    sum.with_expr(expr_div_num(m, sum.expr, num))
}

fn expr_div_num<'m, 'd>(
    m: &'m Model<'d>,
    expr: Ptr<'d, DLinExpr<'d>>,
    num: Ptr<'d, DNum<'d>>,
) -> Ptr<'d, DLinExpr<'d>> {
    expr_div_num_sym(m, expr, num).ptr()
}

fn expr_div_num_sym<'m, 'd>(
    m: &'m Model<'d>,
    expr: Ptr<'d, DLinExpr<'d>>,
    num: Ptr<'d, DNum<'d>>,
) -> LinExpr<'m, 'd> {
    let expr = expr.get();
    let terms = expr.terms.map(|term| term_div_num(m, term, num));
    let sums = expr.sums.map(|sum| sum_div_num(m, sum, num));
    let constant = const_div_num(m, expr.constant, num);
    let expr = DLinExpr::new(terms, sums, constant);
    m.lin_exp(expr)
}

// con

fn const_div_con<'m, 'd>(m: &'m Model<'d>, c1: Con<'d>, c2: impl AsF64) -> Con<'d> {
    c1.map(|c1| {
        let fun = Box::new(move || c1.get().eval() / c2.as_f64());
        m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
    })
}

fn term_div_con<'m, 'd>(m: &'m Model<'d>, term: &DLinTerm<'d>, con: impl AsF64) -> DLinTerm<'d> {
    let coef = match term.coef {
        Coef::One => Coef::Con(1.0 / con.as_f64()),
        Coef::MinusOne => Coef::Con(-1.0 / con.as_f64()),
        Coef::Con(c) => Coef::Con(c / con.as_f64()),
        Coef::Ptr(c) => {
            let fun = Box::new(move || c.get().eval() / con.as_f64());
            Coef::Ptr(m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr())
        }
    };
    DLinTerm::new(coef, term.var)
}

fn sum_div_con<'m, 'd>(m: &'m Model<'d>, sum: &DSum<'d>, con: impl AsF64) -> DSum<'d> {
    sum.with_expr(expr_div_con(m, sum.expr, con))
}

fn expr_div_con<'m, 'd>(
    m: &'m Model<'d>,
    expr: Ptr<'d, DLinExpr<'d>>,
    con: impl AsF64,
) -> Ptr<'d, DLinExpr<'d>> {
    expr_div_con_sym(m, expr, con).ptr()
}

fn expr_div_con_sym<'m, 'd>(
    m: &'m Model<'d>,
    expr: Ptr<'d, DLinExpr<'d>>,
    con: impl AsF64,
) -> LinExpr<'m, 'd> {
    let expr = expr.get();
    let terms = expr.terms.map(|term| term_div_con(m, term, con));
    let sums = expr.sums.map(|sum| sum_div_con(m, sum, con));
    let constant = const_div_con(m, expr.constant, con);
    let expr = DLinExpr::new(terms, sums, constant);
    m.lin_exp(expr)
}

fn ord_to_num_recip<'m, 'd>(idx: IdxOrd<'m, 'd>) -> Num<'m, 'd> {
    let int = idx.data().ord_to_num_recip();
    idx.m().val::<ValNum>(int)
}

// expr / num

macro_rules! expr_num {
    ($n:ty) => {
        impl<'m, 'd> Div<$n> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn div(self, rhs: $n) -> Self::Output {
                expr_div_num_sym(self.m(), self.ptr(), rhs.ptr())
            }
        }
    };
}

expr_num!(Num<'m, 'd>);
expr_num!(Int<'m, 'd>);

// expr / idx

impl<'m, 'd> Div<IdxOrd<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn div(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
        self * ord_to_num_recip(rhs)
    }
}

// expr / con

macro_rules! expr_con {
    ($c:ty) => {
        impl<'m, 'd> Div<$c> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn div(self, rhs: $c) -> Self::Output {
                expr_div_con_sym(self.m(), self.ptr(), rhs)
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

// var / num

macro_rules! var_num {
    ($n:ty) => {
        impl<'m, 'd> Div<$n> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn div(self, rhs: $n) -> Self::Output {
                let coef = Coef::Ptr(one_div_num(self.m(), rhs.ptr()));
                let term = DLinTerm::new(coef, self.ptr());
                let terms = TermVec::new([term]);
                let sums = SumVec::new([]);
                self.m().lin_exp(DLinExpr::new(terms, sums, None))
            }
        }
    };
}

var_num!(Num<'m, 'd>);
var_num!(Int<'m, 'd>);

// var / idx

impl<'m, 'd> Div<IdxOrd<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn div(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
        self * ord_to_num_recip(rhs)
    }
}

// var / con

macro_rules! var_con {
    ($c:ty) => {
        impl<'m, 'd> Div<$c> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn div(self, rhs: $c) -> Self::Output {
                let coef = Coef::Con(1.0 / rhs.as_f64());
                let term = DLinTerm::new(coef, self.ptr());
                let terms = TermVec::new([term]);
                let sums = SumVec::new([]);
                self.m().lin_exp(DLinExpr::new(terms, sums, None))
            }
        }
    };
}

var_con!(f64);
var_con!(f32);
var_con!(i64);
var_con!(i32);
var_con!(u64);
var_con!(u32);
var_con!(usize);
var_con!(isize);
