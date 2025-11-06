use crate::symbols::sets::{SetCore, SetData, SetMeta};
use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::symbols::SymbolRef;
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

impl<'m, const N: usize> From<Set<'m, N>> for SetCore<'m> {
    fn from(value: Set<'m, N>) -> Self {
        value.core
    }
}

impl<'m, const N: usize> From<SetCore<'m>> for Set<'m, N> {
    fn from(core: SetCore<'m>) -> Self {
        Self { core }
    }
}

impl<'m, const N: usize> Debug for Set<'m, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let sym = self.symbol().symbol;
        let dep_sets = self.depending_sets();
        f.debug_struct("Set")
            .field("key", &sym.key.value())
            .field("definition", &sym.definition.value())
            .field("depending_sets", &dep_sets)
            .finish()
    }
}

impl<'m, const N: usize> Set<'m, N> {
    fn depends_on_indices(self) -> [usize; N] {
        let mut indices = [0; N];
        for (i, idx) in self.sym_data().depends_on_indices().iter().enumerate() {
            indices[i] = *idx;
        }
        indices
    }

    pub fn depending_sets(self) -> [Set<'m, 0>; N] {
        let m = self.core.symbol().model;
        let indices = self.depends_on_indices();
        indices.map(|idx| Set::from(m.set_at_unchecked(idx)))
    }

    #[cfg(test)]
    pub(crate) fn core(self) -> SetCore<'m> {
        self.core
    }
}
