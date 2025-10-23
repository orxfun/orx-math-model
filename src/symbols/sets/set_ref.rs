use crate::symbols::{symbol_ref::SymbolRef, SetData};

#[derive(Clone, Copy)]
pub struct Set<'m>(SymbolRef<'m, SetData>);

impl<'m> From<SymbolRef<'m, SetData>> for Set<'m> {
    fn from(value: SymbolRef<'m, SetData>) -> Self {
        Self(value)
    }
}
