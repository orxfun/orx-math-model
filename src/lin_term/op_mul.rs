use crate::{lin_term::LinTerm, Int, Num, Var};
use core::ops::Mul;

// ? * var

impl<'m, 'd> Mul<Var<'m, 'd>> for Num<'m, 'd> {
    type Output = LinTerm<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        LinTerm::new(self, rhs)
    }
}

impl<'m, 'd> Mul<Var<'m, 'd>> for Int<'m, 'd> {
    type Output = LinTerm<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        // LinTerm::new(self, rhs)
        todo!()
    }
}
