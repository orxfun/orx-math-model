use super::symbol_data::SymbolData;
use orx_imp_vec::{prelude::PinnedVec, ImpVec};

pub(crate) trait SymbolCollection {
    type Storage: SymbolData<Symbol = Self::Symbol>;
    type Symbol;

    fn storage(&self) -> &ImpVec<Self::Storage>;

    // push
    fn push(&self, storage: Self::Storage) -> Self::Symbol {
        self.storage().push_get_ref(storage).symbol()
    }

    // get
    fn len(&self) -> usize {
        self.storage().len()
    }
    fn get(&self, sym_idx: usize) -> &Self::Storage {
        &self.storage()[sym_idx]
    }
}

#[macro_export]
macro_rules! impl_symbol_collection_modeling {
    ($collection:ty, $storage:ty, $symbol:ty) => {
        impl SymbolCollection for $collection {
            type Storage = $storage;
            type Symbol = $symbol;
            fn storage(&self) -> &ImpVec<Self::Storage> {
                &self.storage
            }
        }
    };
}
