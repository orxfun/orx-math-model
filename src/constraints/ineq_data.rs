use core::ops::{MulAssign, RemAssign, Shl, ShlAssign, ShrAssign};

use crate::{constraints::Ineq, lin_expr::DLinExpr, LinExpr, Ptr, SymWrap};

pub enum IneqTyp {
    Leq,
    Geq,
    Eq,
}

#[derive(derive_new::new)]
pub struct DIneq<'d> {
    lhs: Ptr<'d, DLinExpr<'d>>,
    rhs: Ptr<'d, DLinExpr<'d>>,
    typ: IneqTyp,
}

impl<'d> DIneq<'d> {}

impl<'m, 'd> Shl<LinExpr<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = Ineq<'m, 'd>;

    fn shl(self, rhs: LinExpr<'m, 'd>) -> Self::Output {
        let ineq = DIneq::new(self.ptr(), rhs.ptr(), IneqTyp::Leq);
        self.m().ineq(ineq)
    }
}

// impl<'m, 'd> ShlAssign<LinExpr<'m, 'd>> for LinExpr<'m, 'd> {
//     fn shl_assign(&mut self, rhs: LinExpr<'m, 'd>) {
//         todo!()
//     }
// }

// ops

// impl<'m,'d> Great

// struct A;

// impl ShlAssign for A {
//     fn shl_assign(&mut self, rhs: Self) {
//         todo!()
//     }
// }

// impl ShrAssign for A {
//     fn shr_assign(&mut self, rhs: Self) {
//         todo!()
//     }
// }

// impl RemAssign for A {
//     fn rem_assign(&mut self, rhs: Self) {
//         todo!()
//     }
// }

// // BitAndAssign
// // scalar &= Scalar(true);

// // BitAndAssign
// // scalar &= Scalar(true);

// // BitOrAssign
// // scalar |= Scalar(true);
