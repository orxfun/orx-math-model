use crate::idxs::IdxOrd;
use crate::lin_expr::{DLinExpr, DLinTerm, SumVec, TermVec};
use crate::scalars::{DNum, ValInt, ValNum};
use crate::{AsF64, ConVec, Int, LinExpr, Model, Num, Ptr, SymWrap, ValTyp, Var};
use alloc::boxed::Box;
use core::ops::Add;

type Con<'d> = Option<Ptr<'d, DNum<'d>>>;

fn const_add_const<'m, 'd>(m: &'m Model<'d>, c1: Con<'d>, c2: Con<'d>) -> Con<'d> {
    match (c1, c2) {
        (Some(c1), Some(c2)) => Some(m.val::<ValNum>(DNum::add(c1, c2)).ptr()),
        (Some(c1), None) => Some(c1),
        (None, Some(c2)) => Some(c2),
        (None, None) => None,
    }
}

fn const_add_num<'m, 'd>(
    m: &'m Model<'d>,
    c1: Con<'d>,
    c2: Ptr<'d, DNum<'d>>,
) -> Ptr<'d, DNum<'d>> {
    match c1 {
        Some(c1) => {
            let fun = Box::new(move || c1.get().eval() + c2.get().eval());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        None => c2,
    }
}

pub fn const_add_con<'m, 'd>(m: &'m Model<'d>, c1: Con<'d>, c2: impl AsF64) -> Ptr<'d, DNum<'d>> {
    match c1 {
        Some(c1) => {
            let fun = Box::new(move || c1.get().eval() + c2.as_f64());
            m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
        }
        None => m.val_num_con(c2).ptr(),
    }
}

pub fn ord_to_num<'m, 'd>(idx: IdxOrd<'m, 'd>) -> Num<'m, 'd> {
    let int = idx.data().ord_to_num();
    idx.m().val::<ValNum>(int)
}

// expr + expr

impl<'m, 'd> Add<LinExpr<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        let [t1, t2] = [self.data(), rhs.data()];
        let terms = t1.terms.merge(&t2.terms);
        let sums = t1.sums.merge(&t2.sums);
        let constant = const_add_const(self.m(), t1.constant, t2.constant);
        self.m().lin_exp(DLinExpr::new(terms, sums, constant))
    }
}

// expr + var

impl<'m, 'd> Add<Var<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: Var<'m, 'd>) -> Self::Output {
        let expr = self.data();
        let terms = expr.terms.push(rhs.into());
        let (sums, constant) = (expr.sums.clone(), expr.constant);
        self.m().lin_exp(DLinExpr::new(terms, sums, constant))
    }
}

impl<'m, 'd> Add<LinExpr<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        rhs + self
    }
}

// expr + num

macro_rules! expr_num {
    ($n:ty) => {
        impl<'m, 'd> Add<$n> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: $n) -> Self::Output {
                let expr = self.data();
                let (terms, sums) = (expr.terms.clone(), expr.sums.clone());
                let constant = const_add_num(self.m(), expr.constant, rhs.ptr());
                self.m().lin_exp(DLinExpr::new(terms, sums, Some(constant)))
            }
        }

        impl<'m, 'd> Add<LinExpr<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                rhs + self
            }
        }
    };
}

expr_num!(Int<'m, 'd>);
expr_num!(Num<'m, 'd>);

// expr + idx

impl<'m, 'd> Add<IdxOrd<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
        self + ord_to_num(rhs)
    }
}

impl<'m, 'd> Add<LinExpr<'m, 'd>> for IdxOrd<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        rhs + self
    }
}

// expr + con

macro_rules! expr_con {
    ($c:ty) => {
        impl<'m, 'd> Add<$c> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: $c) -> Self::Output {
                let expr = self.data();
                let (terms, sums) = (expr.terms.clone(), expr.sums.clone());
                let constant = const_add_con(self.m(), expr.constant, rhs);
                self.m().lin_exp(DLinExpr::new(terms, sums, Some(constant)))
            }
        }

        impl<'m, 'd> Add<LinExpr<'m, 'd>> for $c {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                rhs + self
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

// var + var

impl<'m, 'd> Add<Var<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: Var<'m, 'd>) -> Self::Output {
        let terms = TermVec::new([self.into(), rhs.into()]);
        let sums = SumVec::new([]);
        self.m().lin_exp(DLinExpr::new(terms, sums, None))
    }
}

// var + num

macro_rules! var_num {
    ($n:ty) => {
        impl<'m, 'd> Add<$n> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: $n) -> Self::Output {
                let terms = TermVec::new([self.into()]);
                let constant = Some(rhs.ptr());
                let sums = SumVec::new([]);
                self.m().lin_exp(DLinExpr::new(terms, sums, constant))
            }
        }

        impl<'m, 'd> Add<Var<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: Var<'m, 'd>) -> Self::Output {
                rhs + self
            }
        }
    };
}

var_num!(Num<'m, 'd>);
var_num!(Int<'m, 'd>);

// var + idx

impl<'m, 'd> Add<IdxOrd<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
        self + ord_to_num(rhs)
    }
}

impl<'m, 'd> Add<Var<'m, 'd>> for IdxOrd<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: Var<'m, 'd>) -> Self::Output {
        rhs + self
    }
}

// var + con

macro_rules! var_con {
    ($c:ty) => {
        impl<'m, 'd> Add<$c> for Var<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: $c) -> Self::Output {
                let terms = TermVec::new([self.into()]);
                let sums = SumVec::new([]);
                let constant = Some(self.m().val_num_con(rhs).ptr());
                self.m().lin_exp(DLinExpr::new(terms, sums, constant))
            }
        }

        impl<'m, 'd> Add<Var<'m, 'd>> for $c {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: Var<'m, 'd>) -> Self::Output {
                rhs + self
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
