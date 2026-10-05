use crate::pars::IntPar0;
use crate::pos::Pos;
use crate::scalars::Int;
use crate::symbol::Symbol;
use crate::Model;

#[derive(derive_new::new, Clone, Copy)]
pub struct Range<'a>(Symbol<'a, RangeData>);

#[derive(derive_new::new)]
pub struct RangeData {
    lower: Bound,
    upper: Bound,
}

impl RangeData {
    pub fn range(&self, m: &Model) -> core::ops::Range<i64> {
        self.lower.value(m)..self.upper.value(m)
    }
}

// bound

pub enum Bound {
    Const(i64),
    Int(Pos),
}

impl Bound {
    fn value(&self, m: &Model) -> i64 {
        match *self {
            Self::Const(x) => x,
            Self::Int(pos) => m.int[pos].value(m),
        }
    }
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
