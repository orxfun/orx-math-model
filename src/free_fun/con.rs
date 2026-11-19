use crate::constraints::{Constraint, DCon, IneqTyp};
use crate::idxs::ConIdxs;
use crate::lin_expr::{DLinExpr, SumVec, TermVec};
use crate::{Int, LinExpr, Model, Num, SymWrap, Var};

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
    Int(Int<'m, 'd>),
    Num(Num<'m, 'd>),
    Var(Var<'m, 'd>),
    LinExpr(LinExpr<'m, 'd>),
}

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

pub fn con<'m, 'd>(
    lhs: impl Into<IneqSide<'m, 'd>>,
    typ: IneqTyp,
    rhs: impl Into<IneqSide<'m, 'd>>,
    forall: Forall<'m, 'd>,
) -> Constraint<'m, 'd> {
    let con = |m: &'m Model<'d>, lhs: LinExpr<'m, 'd>, rhs: LinExpr<'m, 'd>| {
        let idxs = forall.0.to_idx_vec();
        let data = DCon::new(idxs, lhs.ptr(), rhs.ptr(), typ);
        m.constraint(data)
    };

    match (lhs.into(), rhs.into()) {
        (IneqSide::ConF64(lhs), IneqSide::ConF64(rhs)) => todo!(),
        // (IneqSide::Con(lhs), IneqSide::Var(rhs)) => con(rhs.m(), lhs, rhs.into()),
        _ => todo!(),
    }

    // let data = DCon::new(forall.0.to_idx_vec(), lhs.ptr(), rhs.ptr(), typ);
}
