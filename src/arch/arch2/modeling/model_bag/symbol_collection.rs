use crate::modeling::symbol::Symbol;
use core::fmt::Debug;
use orx_imp_vec::{Doubling, ImpVec, SplitVec};

type DefaultPinnedVec<S> = SplitVec<S, Doubling>;

pub struct SymbolCollection<S: Symbol> {
    collection: ImpVec<S, DefaultPinnedVec<S>>,
}

impl<S: Symbol> Default for SymbolCollection<S> {
    fn default() -> Self {
        Self {
            collection: Default::default(),
        }
    }
}

impl<S: Symbol> Debug for SymbolCollection<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SymbolCollection")
            .field("collection", &self.collection)
            .finish()
    }
}
