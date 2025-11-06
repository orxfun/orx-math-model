use crate::symbols::sets::r#ref::SetCore;
use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::symbols::{SetData, SetMeta, SymbolRef};
use core::fmt::Debug;
use core::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Set<'m, const N: usize = 0> {
    core: SetCore<'m>,
}

impl<'m, const N: usize> Deref for Set<'m, N> {
    type Target = SetCore<'m>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<'m, const N: usize> SymbolRef<'m, SetMeta> for Set<'m, N> {
    type Data = SetData;
}

impl<'m, const N: usize> From<SymbolRefCore<'m, SetMeta>> for Set<'m, N> {
    fn from(symbol: SymbolRefCore<'m, SetMeta>) -> Self {
        Self {
            core: symbol.into(),
        }
    }
}

impl<'m, const N: usize> From<Set<'m, N>> for SymbolRefCore<'m, SetMeta> {
    fn from(value: Set<'m, N>) -> Self {
        value.core.into()
    }
}

impl<'m, const N: usize> Debug for Set<'m, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let sym = self.symbol().symbol;
        f.debug_struct("Set")
            .field("key", &sym.key.value())
            .field("definition", &sym.definition.value())
            .field("data", &sym.data)
            .finish()
    }
}

impl<'m, const N: usize> Set<'m, N> {
    pub(crate) fn core(self) -> SetCore<'m> {
        self.core
    }
}
