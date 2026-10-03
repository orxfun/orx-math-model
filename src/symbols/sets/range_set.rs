use crate::symbols::{parameters::Count, Symbol};
use core::ops::{Range, RangeBounds};

#[derive(derive_new::new, Clone, Copy)]
pub struct RangeSet<'a> {
    s: Symbol<'a>,
}

impl<'a> RangeSet<'a> {
    pub(crate) fn symbol(self) -> Symbol<'a> {
        self.s
    }
}
