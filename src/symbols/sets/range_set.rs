use crate::symbols::{Model, Symbol, SymbolDefinition};
use alloc::boxed::Box;
use core::ops::{Range, RangeBounds};

#[derive(derive_new::new, Clone, Copy)]
pub struct RangeSet<'a>(Symbol<'a>);

#[derive(derive_new::new)]
pub struct RangeSetData {
    d: SymbolDefinition,
    create: Box<dyn Fn(&Model) -> Range<isize>>,
}
