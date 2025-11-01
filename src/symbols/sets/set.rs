use crate::stages::Stage;
use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::symbols::{SetData, SetMeta, SymbolRef};
use crate::Modeling;
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Set<'m, S = Modeling>
where
    S: Stage,
{
    symbol: SymbolRefCore<'m, S, SetMeta>,
}

impl<'m, S> Debug for Set<'m, S>
where
    S: Stage,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Set")
            .field("key", &self.symbol.symbol.key.value())
            .field("definition", &self.symbol.symbol.definition.value())
            .field("data", &self.symbol.symbol.data)
            .finish()
    }
}

impl<'m, S> From<SymbolRefCore<'m, S, SetMeta>> for Set<'m, S>
where
    S: Stage,
{
    fn from(symbol: SymbolRefCore<'m, S, SetMeta>) -> Self {
        Self { symbol }
    }
}

impl<'m, S> From<Set<'m, S>> for SymbolRefCore<'m, S, SetMeta>
where
    S: Stage,
{
    fn from(value: Set<'m, S>) -> Self {
        value.symbol
    }
}

impl<'m, S> SymbolRef<'m, S, SetMeta> for Set<'m, S>
where
    S: Stage,
{
    type Data = SetData<S>;
}

// derive from Set

impl<'m, S> Set<'m, S>
where
    S: Stage,
{
    pub(crate) fn symbol(self) -> SymbolRefCore<'m, S, SetMeta> {
        self.symbol
    }

    pub(crate) fn idx(self) -> usize {
        let sets = &self.symbol().model.data.sets;
        sets.index_of(self.symbol()).expect("exist in this model")
    }

    pub fn dependant_sets(self) -> impl Iterator<Item = Set<'m, S>> {
        let model = self.symbol().model;
        let indices = self.symbol().symbol.data.depends_on_indices();
        #[allow(clippy::missing_panics_doc)]
        let set_at = |idx: &usize| model.set_at(*idx).expect("certain to exist in this model");
        indices.iter().map(set_at)
    }
}
