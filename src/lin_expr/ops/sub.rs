use crate::idxs::IdxOrd;
use crate::lin_expr::ops::add::ord_to_num;
use crate::lin_expr::{Coef, DLinExpr, DLinTerm, SumVec, TermVec};
use crate::pars::{IntParD0, NumParD0};
use crate::scalars::{DNum, ValNum};
use crate::vars::VarD0;
use crate::{AsF64, Int, LinExpr, Model, Num, Ptr, SymWrap, ValTyp, Var};
use alloc::boxed::Box;
use core::ops::Sub;

type Con<'d> = Option<Ptr<'d, DNum<'d>>>;

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

fn num_sub_const<'m, 'd>(
    m: &'m Model<'d>,
    c1: Ptr<'d, DNum<'d>>,
    c2: Con<'d>,
) -> Ptr<'d, DNum<'d>> {
    match c2 {
        Some(c2) => {
            let fun = Box::new(move || c1.get().eval() - c2.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        None => c1,
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

fn con_sub_const<'m, 'd>(m: &'m Model<'d>, c1: impl AsF64, c2: Con<'d>) -> Ptr<'d, DNum<'d>> {
    match c2 {
        Some(c2) => {
            let fun = Box::new(move || c1.as_f64() - c2.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        None => m.val_num_con(c1.as_f64()).ptr(),
    }
}

fn ord_to_num_neg<'m, 'd>(idx: IdxOrd<'m, 'd>) -> Num<'m, 'd> {
    let int = idx.data().ord_to_num_neg();
    idx.m().val::<ValNum>(int)
}

// expr - expr

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        let m = self.m();
        let [t1, t2] = [self.data(), rhs.data()];
        let terms = t1.terms.merge(&t2.terms.map(|t| t.neg(m)));
        let sums = t1.sums.merge(&t2.sums.map(|s| s.neg(m)));
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
        let terms = rhs.terms.map(|t| t.neg(m)).push_to_front(lhs);
        let sums = rhs.sums.map(|s| s.neg(m));
        let constant = rhs.constant.map(|c| DNum::neg_ptr(m, c));
        m.lin_exp(DLinExpr::new(terms, sums, constant))
    }
}

// expr - var0

impl<'m, 'd> Sub<VarD0<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: VarD0<'m, 'd>) -> Self::Output {
        self - Var::from(rhs)
    }
}

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for VarD0<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        Var::from(self) - rhs
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
                m.lin_exp(lhs.with_constant(Some(constant)))
            }
        }

        impl<'m, 'd> Sub<LinExpr<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                let (m, rhs) = (rhs.m(), rhs.data());
                let terms = rhs.terms.map(|t| t.neg(m));
                let sums = rhs.sums.map(|s| s.neg(m));
                let constant = num_sub_const(m, self.ptr(), rhs.constant);
                m.lin_exp(DLinExpr::new(terms, sums, Some(constant)))
            }
        }
    };
}

expr_num!(Num<'m, 'd>);
expr_num!(Int<'m, 'd>);

// expr - idx

impl<'m, 'd> Sub<IdxOrd<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
        self + ord_to_num_neg(rhs)
    }
}

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for IdxOrd<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        ord_to_num(self) - rhs
    }
}

// expr - par

impl<'m, 'd> Sub<NumParD0<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: NumParD0<'m, 'd>) -> Self::Output {
        self - rhs.to_num()
    }
}

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for NumParD0<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        self.to_num() - rhs
    }
}

impl<'m, 'd> Sub<IntParD0<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: IntParD0<'m, 'd>) -> Self::Output {
        self - rhs.to_int()
    }
}

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for IntParD0<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        self.to_int() - rhs
    }
}

// expr - const

macro_rules! expr_con {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let (m, lhs) = (self.m(), self.data());
                let constant = const_sub_con(m, lhs.constant, rhs);
                m.lin_exp(lhs.with_constant(Some(constant)))
            }
        }

        impl<'m, 'd> Sub<LinExpr<'m, 'd>> for $c {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                let (m, rhs) = (rhs.m(), rhs.data());
                let terms = rhs.terms.map(|t| t.neg(m));
                let sums = rhs.sums.map(|s| s.neg(m));
                let constant = con_sub_const(m, self, rhs.constant);
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

// var - var

impl<'m, 'd> Sub<Var<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        let t2 = DLinTerm::new(Coef::MinusOne, rhs.ptr());
        let terms = TermVec::new([self.into(), t2]);
        let sums = SumVec::new([]);
        self.m().lin_exp(DLinExpr::new(terms, sums, None))
    }
}

// var - var0

impl<'m, 'd> Sub<VarD0<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: VarD0<'m, 'd>) -> Self::Output {
        self - Var::from(rhs)
    }
}

impl<'m, 'd> Sub<Var<'m, 'd>> for VarD0<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        Var::from(self) - rhs
    }
}

// var - num

macro_rules! var_num {
    ($n:ty) => {
        impl<'m, 'd> Sub<$n> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $n) -> Self::Output {
                let terms = TermVec::new([self.into()]);
                let sums = SumVec::new([]);
                let constant = const_sub_num(self.m(), None, rhs.ptr());
                self.m().lin_exp(DLinExpr::new(terms, sums, Some(constant)))
            }
        }

        impl<'m, 'd> Sub<Var<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
                let terms = TermVec::new([DLinTerm::new(Coef::MinusOne, rhs.ptr())]);
                let sums = SumVec::new([]);
                let constant = Some(self.ptr());
                self.m().lin_exp(DLinExpr::new(terms, sums, constant))
            }
        }
    };
}

var_num!(Num<'m, 'd>);
var_num!(Int<'m, 'd>);

// var - idx

impl<'m, 'd> Sub<IdxOrd<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
        self + ord_to_num_neg(rhs)
    }
}

impl<'m, 'd> Sub<Var<'m, 'd>> for IdxOrd<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        ord_to_num(self) - rhs
    }
}

// var - par

impl<'m, 'd> Sub<NumParD0<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: NumParD0<'m, 'd>) -> Self::Output {
        self - rhs.to_num()
    }
}

impl<'m, 'd> Sub<Var<'m, 'd>> for NumParD0<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        self.to_num() - rhs
    }
}

impl<'m, 'd> Sub<IntParD0<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: IntParD0<'m, 'd>) -> Self::Output {
        self - rhs.to_int()
    }
}

impl<'m, 'd> Sub<Var<'m, 'd>> for IntParD0<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
        self.to_int() - rhs
    }
}

// var - con

macro_rules! var_con {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let terms = TermVec::new([self.into()]);
                let sums = SumVec::new([]);
                let constant = const_sub_con(self.m(), None, rhs);
                self.m().lin_exp(DLinExpr::new(terms, sums, Some(constant)))
            }
        }

        impl<'m, 'd> Sub<Var<'m, 'd>> for $c {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: Var<'m, 'd>) -> Self::Output {
                let terms = TermVec::new([DLinTerm::new(Coef::MinusOne, rhs.ptr())]);
                let sums = SumVec::new([]);
                let constant = rhs.m().val_num_con(self).ptr();
                rhs.m().lin_exp(DLinExpr::new(terms, sums, Some(constant)))
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
