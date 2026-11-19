use crate::constraints::{Constraint, DCon, IneqTyp};
use crate::free_fun::IneqOp;
use crate::idxs::{ConIdxs, IdxVec};
use crate::lin_expr::{DLinExpr, DLinTerm, SumVec, TermVec};
use crate::{AsF64, Int, LinExpr, Model, Num, Set, SetSym, SymWrap, Var};

pub struct Forall<'m, 'd>(pub(crate) ConIdxs<'m, 'd>);

pub fn forall<'m, 'd>(indices: impl Into<ConIdxs<'m, 'd>>) -> Forall<'m, 'd> {
    Forall(indices.into())
}

#[derive(derive_more::From)]
pub enum IneqSide<'m, 'd> {
    ConF64(f64),
    ConF32(f32),
    ConI64(i64),
    ConI32(i32),
    ConU64(u64),
    ConU32(u32),
    ConUsize(usize),
    ConIsize(isize),
    Num(Num<'m, 'd>),
    Int(Int<'m, 'd>),
    Var(Var<'m, 'd>),
    LinExpr(LinExpr<'m, 'd>),
}

impl<'m, 'd> IneqSide<'m, 'd> {
    fn m(&self) -> Option<&'m Model<'d>> {
        match self {
            Self::Num(x) => Some(x.m()),
            Self::Int(x) => Some(x.m()),
            Self::Var(x) => Some(x.m()),
            Self::LinExpr(x) => Some(x.m()),
            _ => None,
        }
    }

    fn to_expr(&self, m: &'m Model<'d>) -> LinExpr<'m, 'd> {
        fn con_to_expr<'m, 'd>(m: &'m Model<'d>, con: f64) -> LinExpr<'m, 'd> {
            num_to_expr(m, m.val_num_con(con))
        }

        fn num_to_expr<'m, 'd>(m: &'m Model<'d>, num: Num<'m, 'd>) -> LinExpr<'m, 'd> {
            let data = DLinExpr::new(TermVec::new([]), SumVec::new([]), Some(num.ptr()));
            m.lin_exp(data)
        }

        fn int_to_expr<'m, 'd>(m: &'m Model<'d>, int: Int<'m, 'd>) -> LinExpr<'m, 'd> {
            let data = DLinExpr::new(TermVec::new([]), SumVec::new([]), Some(int.ptr()));
            m.lin_exp(data)
        }

        fn var_to_expr<'m, 'd>(m: &'m Model<'d>, var: Var<'m, 'd>) -> LinExpr<'m, 'd> {
            let data = DLinExpr::new(TermVec::new([DLinTerm::from(var)]), SumVec::new([]), None);
            m.lin_exp(data)
        }

        match self {
            Self::ConF64(c) => con_to_expr(m, *c),
            Self::ConF32(c) => con_to_expr(m, c.as_f64()),
            Self::ConI64(c) => con_to_expr(m, c.as_f64()),
            Self::ConI32(c) => con_to_expr(m, c.as_f64()),
            Self::ConU64(c) => con_to_expr(m, c.as_f64()),
            Self::ConU32(c) => con_to_expr(m, c.as_f64()),
            Self::ConUsize(c) => con_to_expr(m, c.as_f64()),
            Self::ConIsize(c) => con_to_expr(m, c.as_f64()),
            Self::Num(n) => num_to_expr(m, *n),
            Self::Int(n) => int_to_expr(m, *n),
            Self::Var(v) => var_to_expr(m, *v),
            Self::LinExpr(e) => *e,
        }
    }
}

pub fn con_depr<'m, 'd>(
    lhs: impl Into<IneqSide<'m, 'd>>,
    typ: impl IneqOp,
    rhs: impl Into<IneqSide<'m, 'd>>,
    forall: Forall<'m, 'd>,
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
    let idxs = forall.0.to_idx_vec();
    let data = DCon::new(idxs, lhs.ptr(), rhs.ptr(), typ.typ());
    m.constraint(data)
}

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

pub fn con1<'m, 'd, S0: SetSym<'m, 'd>>(
    s0: S0,
    con: impl Fn(<S0::Meta as Set>::IdxSym<'m, 'd>) -> Constraint<'m, 'd>,
) -> Constraint<'m, 'd> {
    con(s0.idx())
}
