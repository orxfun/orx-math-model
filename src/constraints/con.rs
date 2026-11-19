use crate::constraints::{DIneq, Ineq, IneqTyp};
use crate::{LinExpr, SymWrap};

// macro_rules! con {
//     ($a:ty <= $b:ty) => {};
// }

// macro_rules! con {
// ($lhs:tt <= $rhs:tt) => {
// Constraint::Le($lhs, $rhs)
// };
//
// ($lhs:tt >= $rhs:tt) => {
// Constraint::Ge($lhs, $rhs)
// };
//
// ($lhs:tt == $rhs:tt) => {
// Constraint::Eq($lhs, $rhs)
// };
// }

fn ineq<'m, 'd>(lhs: LinExpr<'m, 'd>, rhs: LinExpr<'m, 'd>, typ: IneqTyp) -> Ineq<'m, 'd> {
    let data = DIneq::new(lhs.ptr(), rhs.ptr(), typ);
    lhs.m().ineq(data)
}

macro_rules! con {
    // d0
    ($lhs:tt <= $rhs:tt) => {
        ineq($lhs, $rhs, IneqTyp::Leq)
    };
    ($lhs:tt >= $rhs:tt) => {
        ineq($lhs, $rhs, IneqTyp::Geq)
    };
    ($lhs:tt == $rhs:tt) => {
        ineq($lhs, $rhs, IneqTyp::Eq)
    };
}

fn abc<'m, 'd>(lhs: LinExpr<'m, 'd>, rhs: LinExpr<'m, 'd>) {
    let a = con!(lhs >= rhs);
}
