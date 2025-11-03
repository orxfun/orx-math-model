use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::symbols::{SetData, SetMeta, SymbolRef};
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Set<'m, const N: usize>(SymbolRefCore<'m, SetMeta>);

impl<'m, const N: usize> SymbolRef<'m, SetMeta> for Set<'m, N> {
    type Data = SetData;
}

impl<'m, const N: usize> From<SymbolRefCore<'m, SetMeta>> for Set<'m, N> {
    fn from(symbol: SymbolRefCore<'m, SetMeta>) -> Self {
        Self(symbol)
    }
}

impl<'m, const N: usize> From<Set<'m, N>> for SymbolRefCore<'m, SetMeta> {
    fn from(value: Set<'m, N>) -> Self {
        value.0
    }
}

impl<'m, const N: usize> Debug for Set<'m, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Set")
            .field("key", &self.0.symbol.key.value())
            .field("definition", &self.0.symbol.definition.value())
            .field("data", &self.0.symbol.data)
            .finish()
    }
}
