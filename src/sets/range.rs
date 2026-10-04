use crate::pars::IntPar0;
use crate::scalars::Int;
use crate::symbol::Symbol;

#[derive(derive_new::new, Clone, Copy)]
pub struct Range<'a>(pub(crate) Symbol<'a, RangeData<'a>>);

#[derive(derive_new::new)]
pub struct RangeData<'a> {
    pub lower: Bound<'a>,
    pub upper: Bound<'a>,
}

// bound

pub enum Bound<'a> {
    Dep(Int<'a>),
    Const(i64),
}

impl<'a> From<IntPar0<'a>> for Bound<'a> {
    fn from(value: IntPar0<'a>) -> Self {
        Self::Dep(value.into())
    }
}

impl<'a> From<Int<'a>> for Bound<'a> {
    fn from(value: Int<'a>) -> Self {
        Self::Dep(value)
    }
}

impl From<i64> for Bound<'_> {
    fn from(value: i64) -> Self {
        Self::Const(value)
    }
}
