use crate::{pars::IntPar0, pos::Pos, scalars::Int};

// bound

pub enum Bound {
    Const(i64),
    Int(Pos),
}

impl From<i64> for Bound {
    fn from(value: i64) -> Self {
        Self::Const(value)
    }
}

impl From<IntPar0<'_>> for Bound {
    fn from(value: IntPar0<'_>) -> Self {
        let int = Int::from(value);
        Bound::Int(int.0.pos())
    }
}

impl From<Int<'_>> for Bound {
    fn from(value: Int<'_>) -> Self {
        Bound::Int(value.0.pos())
    }
}
