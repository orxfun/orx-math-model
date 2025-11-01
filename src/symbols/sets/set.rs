use crate::symbols::symbol_ref::SymbolRef;
use crate::symbols::{sets::dependent::DependentSubset, SetData, SetSymbol, Sym};
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Set<'m> {
    symbol: SymbolRef<'m, SetSymbol>,
}

impl<'m> Debug for Set<'m> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Set")
            .field("key", &self.symbol.data.key.value())
            .field("definition", &self.symbol.data.definition.value())
            .field("data", &self.symbol.data.data)
            .finish()
    }
}

impl<'m> From<SymbolRef<'m, SetSymbol>> for Set<'m> {
    fn from(symbol: SymbolRef<'m, SetSymbol>) -> Self {
        Self { symbol }
    }
}

impl<'m> From<Set<'m>> for SymbolRef<'m, SetSymbol> {
    fn from(value: Set<'m>) -> Self {
        value.symbol
    }
}

impl<'m> Sym<'m, SetSymbol> for Set<'m> {
    type Data = SetData;
}

// derive from Set

impl<'m> Set<'m> {
    pub fn st(self, filter: impl Fn(usize, usize) -> bool) {
        let x = DependentSubset::new(self, filter);
        todo!()
    }

    pub(crate) fn symbol(self) -> SymbolRef<'m, SetSymbol> {
        self.into()
    }
}
