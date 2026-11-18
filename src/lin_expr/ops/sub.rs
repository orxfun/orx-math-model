use crate::lin_expr::DLinTerm;
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
        let rhs = DLinTerm::new(Some(self.m().minus_one().ptr()), rhs.ptr());
        let data = self.data().add_term(rhs);
        self.m().lin_exp(data)
    }
}

impl<'m, 'd> Sub<LinExpr<'m, 'd>> for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        todo!()
    }
}

// expr - num

macro_rules! expr_minus_num {
    ($n:ty) => {
        impl<'m, 'd> Sub<$n> for LinExpr<'m, 'd> {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: $n) -> Self::Output {
                let data = self.data().add_constant(self.m(), rhs.ptr());
                self.m().lin_exp(data)
            }
        }

        impl<'m, 'd> Sub<LinExpr<'m, 'd>> for $n {
            type Output = LinExpr<'m, 'd>;

            fn sub(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
                rhs + self
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
                // rhs + self
                todo!()
            }
        }
    };
}

expr_minus_con!(f64);
expr_minus_con!(f32);
expr_minus_con!(i64);
expr_minus_con!(i32);
expr_minus_con!(isize);
