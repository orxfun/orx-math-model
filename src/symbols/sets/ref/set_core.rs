use crate::symbols::sets::{Set, SetData, SetMeta};
use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::symbols::SymbolRef;
use core::fmt::{Debug, Display};

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

impl<'m> Display for SetCore<'m> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", *self.symbol.symbol.key)
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

impl<'m> SetCore<'m> {
    pub(crate) fn symbol(self) -> SymbolRefCore<'m, SetMeta> {
        self.into()
    }

    pub(crate) fn idx(self) -> usize {
        let model = self.symbol().model;
        let sets = &model.data.sets;
        sets.index_of(self.symbol()).expect("exist in this model")
    }

    pub(crate) fn sym_data(self) -> &'m SetData {
        &self.symbol.symbol.data
    }

    pub(crate) fn dim(self) -> usize {
        self.sym_data().depends_on_indices().len()
    }

    pub(crate) fn depending_set_core_at(self, idx: usize) -> Option<SetCore<'m>> {
        let m = self.symbol().model;
        let idx = self.sym_data().depends_on_indices().get(idx);
        idx.map(|idx| SetCore::from(m.set_at_unchecked(*idx)))
    }

    pub(crate) fn with_dim<const N: usize>(self) -> Set<'m, N> {
        debug_assert_eq!(self.dim(), N);
        self.symbol.into()
    }

    pub(crate) fn with_dim_checked<const N: usize>(self) -> Option<Set<'m, N>> {
        let matches = self.sym_data().depends_on_indices().len() == N;
        matches.then_some(self.symbol.into())
    }
}
