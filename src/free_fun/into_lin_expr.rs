use crate::lin_expr::{DLinExpr, DLinTerm, SumVec, TermVec};
use crate::{AsF64, Int, LinExpr, Model, Num, SymWrap, Var};

#[derive(derive_more::From)]
pub enum IntoLinExpr<'m, 'd> {
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

impl<'m, 'd> IntoLinExpr<'m, 'd> {
    pub fn m(&self) -> Option<&'m Model<'d>> {
        match self {
            Self::Num(x) => Some(x.m()),
            Self::Int(x) => Some(x.m()),
            Self::Var(x) => Some(x.m()),
            Self::LinExpr(x) => Some(x.m()),
            _ => None,
        }
    }

    pub fn to_expr(&self, m: &'m Model<'d>) -> LinExpr<'m, 'd> {
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
