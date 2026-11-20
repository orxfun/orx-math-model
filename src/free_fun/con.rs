use crate::constraints::{Constraint, DCon, IneqTyp};
use crate::free_fun::{IneqOp, IntoLinExpr};
use crate::idxs::{self, IdxVec};
use crate::lin_expr::{DLinExpr, DLinTerm, SumVec, TermVec};
use crate::{AsF64, Int, LinExpr, Model, Num, Set, SetSym, SymWrap, Var};

pub fn con_gen<'m, 'd>(
    lhs: impl Into<IntoLinExpr<'m, 'd>>,
    typ: impl IneqOp,
    rhs: impl Into<IntoLinExpr<'m, 'd>>,
    idxs: IdxVec<'d>,
) -> Constraint<'m, 'd> {
    let [lhs, rhs] = [lhs.into(), rhs.into()];
    let m = match (lhs.m(), rhs.m()) {
        (Some(m1), Some(_m2)) => Some(m1),
        (Some(m1), None) => Some(m1),
        (None, Some(m2)) => Some(m2),
        (None, None) => None,
    };

    // TODO: Handle const-const case somehow!
    let m = m.unwrap();

    let lhs = lhs.to_expr(m);
    let rhs = rhs.to_expr(m);
    let data = DCon::new(idxs, lhs.ptr(), rhs.ptr(), typ.typ());
    m.constraint(data)
}

pub fn con_old<'m, 'd>(
    lhs: impl Into<IntoLinExpr<'m, 'd>>,
    typ: impl IneqOp,
    rhs: impl Into<IntoLinExpr<'m, 'd>>,
) -> Constraint<'m, 'd> {
    let idxs = IdxVec::new([]);
    con_gen(lhs, typ, rhs, idxs)
}

type Idx<'m, 'd, S> = <<S as SetSym<'m, 'd>>::Meta as Set>::IdxSym<'m, 'd>;

pub fn for1<'m, 'd, S0: SetSym<'m, 'd>>(
    s0: S0,
    con: impl Fn(Idx<'m, 'd, S0>) -> Constraint<'m, 'd>,
) -> Constraint<'m, 'd> {
    let i0 = s0.idx();
    let idxs = IdxVec::new([i0.ptr()]);
    let con = con(i0);
    unsafe { con.el().get_mut() }.set_idxs(idxs);
    con
}

pub fn for2_old<'m, 'd, S0: SetSym<'m, 'd>, S1: SetSym<'m, 'd>>(
    s0: S0,
    s1: S1,
    con: impl Fn(Idx<'m, 'd, S0>, Idx<'m, 'd, S1>) -> Constraint<'m, 'd>,
) -> Constraint<'m, 'd> {
    let (i0, i1) = (s0.idx(), s1.idx());
    let idxs = IdxVec::new([i0.ptr(), i1.ptr()]);
    let con = con(i0, i1);
    unsafe { con.el().get_mut() }.set_idxs(idxs);
    con
}

pub struct Con2<'m, 'd, S0, S1>
where
    S0: SetSym<'m, 'd>,
    S1: SetSym<'m, 'd>,
{
    s0: S0,
    s1: S1,
    lhs: IntoLinExpr<'m, 'd>,
    typ: IneqTyp,
    rhs: IntoLinExpr<'m, 'd>,
}

impl<'m, 'd, S0, S1, Lhs, Op, Rhs, F> From<F> for Con2<'m, 'd, S0, S1>
where
    S0: SetSym<'m, 'd>,
    S1: SetSym<'m, 'd>,
    Lhs: Into<IntoLinExpr<'m, 'd>>,
    Op: IneqOp,
    Rhs: Into<IntoLinExpr<'m, 'd>>,
    F: Fn(Idx<'m, 'd, S0>, Idx<'m, 'd, S1>) -> (Lhs, Op, Rhs),
{
    fn from(value: F) -> Self {
        todo!()
    }
}

pub fn for2<'m, 'd, S0, S1, Lhs, Op, Rhs, F>(s0: S0, s1: S1, con: F) -> Constraint<'m, 'd>
where
    S0: SetSym<'m, 'd>,
    S1: SetSym<'m, 'd>,
    Lhs: Into<IntoLinExpr<'m, 'd>>,
    Op: IneqOp,
    Rhs: Into<IntoLinExpr<'m, 'd>>,
    F: Fn(Idx<'m, 'd, S0>, Idx<'m, 'd, S1>) -> (Lhs, Op, Rhs),
{
    let (i0, i1) = (s0.idx(), s1.idx());
    let idxs = IdxVec::new([i0.ptr(), i1.ptr()]);
    let (lhs, op, rhs) = con(i0, i1);

    let [lhs, rhs] = [lhs.into(), rhs.into()];
    let m = match (lhs.m(), rhs.m()) {
        (Some(m1), Some(_m2)) => Some(m1),
        (Some(m1), None) => Some(m1),
        (None, Some(m2)) => Some(m2),
        (None, None) => None,
    };

    // TODO: Handle const-const case somehow!
    let m = m.unwrap();

    let (lhs, op, rhs) = (lhs.to_expr(m), op.typ(), rhs.to_expr(m));

    let data = DCon::new(idxs, lhs.ptr(), rhs.ptr(), op);
    m.constraint(data)
}
