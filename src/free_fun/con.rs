use crate::constraints::{Constraint, DCon, IneqTyp};
use crate::free_fun::{IneqOp, IneqSide};
use crate::idxs::IdxVec;
use crate::lin_expr::{DLinExpr, DLinTerm, SumVec, TermVec};
use crate::{AsF64, Int, LinExpr, Model, Num, Set, SetSym, SymWrap, Var};

pub fn con_gen<'m, 'd>(
    lhs: impl Into<IneqSide<'m, 'd>>,
    typ: impl IneqOp,
    rhs: impl Into<IneqSide<'m, 'd>>,
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

pub fn con<'m, 'd>(
    lhs: impl Into<IneqSide<'m, 'd>>,
    typ: impl IneqOp,
    rhs: impl Into<IneqSide<'m, 'd>>,
) -> Constraint<'m, 'd> {
    let idxs = IdxVec::new([]);
    con_gen(lhs, typ, rhs, idxs)
}

type Idx<'m, 'd, S> = <<S as SetSym<'m, 'd>>::Meta as Set>::IdxSym<'m, 'd>;

pub fn for1<'m, 'd, S0: SetSym<'m, 'd>>(
    s0: S0,
    con: impl Fn(Idx<'m, 'd, S0>) -> Constraint<'m, 'd>,
) -> Constraint<'m, 'd> {
    con(s0.idx())
}

pub fn for2<'m, 'd, S0: SetSym<'m, 'd>, S1: SetSym<'m, 'd>>(
    s0: S0,
    s1: S1,
    con: impl Fn(Idx<'m, 'd, S0>, Idx<'m, 'd, S1>) -> Constraint<'m, 'd>,
) -> Constraint<'m, 'd> {
    con(s0.idx(), s1.idx())
}
