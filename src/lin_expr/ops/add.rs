use crate::lin_expr::DLinTerm;
use crate::{Int, LinExpr, Num, SymWrap, Var};
use core::ops::Add;

// expr + expr

impl<'m, 'd> Add<LinExpr<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        let data = self.data().add(self.m(), rhs.data());
        self.m().lin_exp(data)
    }
}

// expr + var

impl<'m, 'd> Add<Var<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: Var<'m, 'd>) -> Self::Output {
        let data = self.data().add_term(rhs.into());
        self.m().lin_exp(data)
    }
}

impl<'m, 'd> Add<LinExpr<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        rhs + self
    }
}

// expr + num

macro_rules! expr_add_num {
    ($n:ty) => {
        impl<'m, 'd> Add<$n> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: $n) -> Self::Output {
                let data = self.data().add_constant(self.m(), rhs.ptr());
                self.m().lin_exp(data)
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

expr_add_num!(Int<'m, 'd>);
expr_add_num!(Num<'m, 'd>);

// expr + const

macro_rules! expr_plus_con {
    ($c:ty) => {
        impl<'m, 'd> Add<$c> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn add(self, rhs: $c) -> Self::Output {
                let c = self.m().val_num_con(rhs).ptr();
                let data = self.data().add_constant(self.m(), c);
                self.m().lin_exp(data)
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

expr_plus_con!(f64);
expr_plus_con!(f32);
expr_plus_con!(i64);
expr_plus_con!(i32);
expr_plus_con!(u64);
expr_plus_con!(u32);
expr_plus_con!(usize);
expr_plus_con!(isize);
