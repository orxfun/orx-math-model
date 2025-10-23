use crate::symbols::{symbol_ref::SymbolRef, SetData};
use core::ops::Deref;

#[derive(Clone, Copy)]
pub struct Set<'m>(SymbolRef<'m, SetData>);

impl<'m> From<SymbolRef<'m, SetData>> for Set<'m> {
    fn from(value: SymbolRef<'m, SetData>) -> Self {
        Self(value)
    }
}

impl<'m> Deref for Set<'m> {
    type Target = SymbolRef<'m, SetData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
