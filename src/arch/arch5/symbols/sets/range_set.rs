use crate::symbols::{scalars::Int, Model, Symbol, SymbolDefinition};
use alloc::boxed::Box;
use core::ops::{Range, RangeBounds};

#[derive(derive_new::new, Clone, Copy)]
pub struct RangeSet<'a>(Symbol<'a>);

#[derive(derive_new::new)]
pub struct RangeSetData {
    pub def: SymbolDefinition,
    pub create: Box<dyn Fn(&Model) -> Range<i64>>,
    pub elem_pos: usize,
}

pub enum Bound<'a> {
    Dep(Int<'a>),
    Const(i64),
}
