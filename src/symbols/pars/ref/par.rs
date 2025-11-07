use crate::symbols::pars::{ParCore, ParData, ParMeta};
use crate::symbols::sets::Set;
use crate::symbols::symbol_ref_core::SymbolRefCore;
use crate::symbols::SymbolRef;
use core::fmt::Debug;
use core::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Par<'m, const N: usize = 0> {
    core: ParCore<'m>,
}

impl<'m, const N: usize> Deref for Par<'m, N> {
    type Target = ParCore<'m>;

    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl<'m, const N: usize> SymbolRef<'m, ParMeta> for Par<'m, N> {
    type Data = ParData;
}

impl<'m, const N: usize> From<SymbolRefCore<'m, ParMeta>> for Par<'m, N> {
    fn from(symbol: SymbolRefCore<'m, ParMeta>) -> Self {
        Self {
            core: symbol.into(),
        }
    }
}

impl<'m, const N: usize> From<Par<'m, N>> for SymbolRefCore<'m, ParMeta> {
    fn from(value: Par<'m, N>) -> Self {
        value.core.into()
    }
}

impl<'m, const N: usize> From<Par<'m, N>> for ParCore<'m> {
    fn from(value: Par<'m, N>) -> Self {
        value.core
    }
}

impl<'m, const N: usize> From<ParCore<'m>> for Par<'m, N> {
    fn from(core: ParCore<'m>) -> Self {
        Self { core }
    }
}

impl<'m, const N: usize> Debug for Par<'m, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let sym = self.symbol().symbol;
        let dep_sets = self.depending_sets();
        f.debug_struct("Par")
            .field("key", &sym.key.value())
            .field("definition", &sym.definition.value())
            .field("depending_sets", &dep_sets)
            .finish()
    }
}

impl<'m, const N: usize> Par<'m, N> {
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
    pub(crate) fn core(self) -> ParCore<'m> {
        self.core
    }
}
