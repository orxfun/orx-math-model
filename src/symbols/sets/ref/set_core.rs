use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::symbols::{SetData, SetMeta, SymbolRef};
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SetCore<'m> {
    symbol: SymbolRefCore<'m, SetMeta>,
}

impl<'m> Debug for SetCore<'m> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Set")
            .field("key", &self.symbol.symbol.key.value())
            .field("definition", &self.symbol.symbol.definition.value())
            .field("data", &self.symbol.symbol.data)
            .finish()
    }
}

impl<'m> From<SymbolRefCore<'m, SetMeta>> for SetCore<'m> {
    fn from(symbol: SymbolRefCore<'m, SetMeta>) -> Self {
        Self { symbol }
    }
}

impl<'m> From<SetCore<'m>> for SymbolRefCore<'m, SetMeta> {
    fn from(value: SetCore<'m>) -> Self {
        value.symbol
    }
}

impl<'m> SymbolRef<'m, SetMeta> for SetCore<'m> {
    type Data = SetData;
}

// derive from Set

impl<'m> SetCore<'m> {
    pub(crate) fn symbol(self) -> SymbolRefCore<'m, SetMeta> {
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

    // pub fn dependant_sets(self) -> impl Iterator<Item = SetCore<'m>> {
    //     let model = self.symbol().model;
    //     let indices = self.symbol().symbol.data.depends_on_indices();
    //     #[allow(clippy::missing_panics_doc)]
    //     let set_at = |idx: &usize| model.set_at(*idx).expect("certain to exist in this model");
    //     indices.iter().map(set_at)
    // }
}
