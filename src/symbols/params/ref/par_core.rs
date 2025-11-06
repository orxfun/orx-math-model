use crate::symbols::params::{ParData, ParMeta};
use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::symbols::SymbolRef;
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ParCore<'m> {
    symbol: SymbolRefCore<'m, ParMeta>,
}

impl<'m> Debug for ParCore<'m> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Set")
            .field("key", &self.symbol.symbol.key.value())
            .field("definition", &self.symbol.symbol.definition.value())
            .field("data", &self.symbol.symbol.data)
            .finish()
    }
}

impl<'m> From<SymbolRefCore<'m, ParMeta>> for ParCore<'m> {
    fn from(symbol: SymbolRefCore<'m, ParMeta>) -> Self {
        Self { symbol }
    }
}

impl<'m> From<ParCore<'m>> for SymbolRefCore<'m, ParMeta> {
    fn from(value: ParCore<'m>) -> Self {
        value.symbol
    }
}

impl<'m> SymbolRef<'m, ParMeta> for ParCore<'m> {
    type Data = ParData;
}

// derive from Set

// impl<'m> ParCore<'m> {
//     pub(crate) fn symbol(self) -> SymbolRefCore<'m, ParMeta> {
//         self.into()
//     }

//     pub(crate) fn idx(self) -> usize {
//         let model = self.symbol().model;
//         let sets = &model.data.sets;
//         sets.index_of(self.symbol()).expect("exist in this model")
//     }

//     pub(crate) fn sym_data(self) -> &'m ParData {
//         &self.symbol.symbol.data
//     }

//     pub(crate) fn set_ref<const N: usize>(self) -> Set<'m, N> {
//         debug_assert_eq!(self.sym_data().depends_on_indices().len(), N);
//         self.symbol.into()
//     }

//     pub(crate) fn set_ref_checked<const N: usize>(self) -> Option<Set<'m, N>> {
//         let matches = self.sym_data().depends_on_indices().len() == N;
//         matches.then_some(self.symbol.into())
//     }
// }
