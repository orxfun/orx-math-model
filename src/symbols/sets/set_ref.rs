use crate::symbols::{symbol_ref::SymbolRef, SetData};
use core::{fmt::Debug, ops::Deref};

#[derive(Clone, Copy)]
pub struct Set<'m> {
    symbol: SymbolRef<'m, SetData>,
}

impl<'m> From<SymbolRef<'m, SetData>> for Set<'m> {
    fn from(symbol: SymbolRef<'m, SetData>) -> Self {
        Self { symbol }
    }
}

impl<'m> Deref for Set<'m> {
    type Target = SymbolRef<'m, SetData>;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}

impl<'m> Debug for Set<'m> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Set")
            .field("key", &self.symbol.data.key)
            .field("definition", &self.symbol.data.definition.value())
            .finish()
    }
}
