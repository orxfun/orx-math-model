use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::symbols::{SetData, SetSymbol, SymbolRef};
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Set<'m> {
    symbol: SymbolRefCore<'m, SetSymbol>,
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

impl<'m> From<SymbolRefCore<'m, SetSymbol>> for Set<'m> {
    fn from(symbol: SymbolRefCore<'m, SetSymbol>) -> Self {
        Self { symbol }
    }
}

impl<'m> From<Set<'m>> for SymbolRefCore<'m, SetSymbol> {
    fn from(value: Set<'m>) -> Self {
        value.symbol
    }
}

impl<'m> SymbolRef<'m, SetSymbol> for Set<'m> {
    type Data = SetData;
}

// derive from Set

impl<'m> Set<'m> {
    pub(crate) fn symbol(self) -> SymbolRefCore<'m, SetSymbol> {
        self.into()
    }

    pub(crate) fn idx(self) -> usize {
        let model = self.symbol().model;
        model
            .data
            .sets
            .index_of(self.symbol())
            .expect("exist in this model")
    }

    pub fn dependant_sets(self) -> impl Iterator<Item = Set<'m>> {
        let model = self.symbol().model;
        let indices = self.symbol().data.data.depends_on_indices();
        #[allow(clippy::missing_panics_doc)]
        let set_at = |idx: &usize| model.set_at(*idx).expect("certain to exist in this model");
        indices.iter().map(set_at)
    }
}
