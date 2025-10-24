use crate::symbols::{sym::SymCore, symbol_ref::SymbolRef, SetData, SetSymbol, Sym};
use core::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Set<'m> {
    symbol: SymbolRef<'m, SetSymbol>,
}

impl<'m> SymCore<'m, SetSymbol> for Set<'m> {
    fn symbol_ref(&self) -> SymbolRef<'m, SetSymbol> {
        self.symbol
    }
}

impl<'m> Sym<'m, SetSymbol> for Set<'m> {
    type Data = SetData;
}

impl<'m> From<SymbolRef<'m, SetSymbol>> for Set<'m> {
    fn from(symbol: SymbolRef<'m, SetSymbol>) -> Self {
        Self { symbol }
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
