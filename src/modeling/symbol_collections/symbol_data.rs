pub(crate) trait SymbolData {
    type Symbol;
    fn symbol(&self) -> Self::Symbol;
}

#[macro_export]
macro_rules! impl_symbol_data {
    ($storage:ty, $symbol:ty) => {
        impl SymbolData for $storage {
            type Symbol = $symbol;
            fn symbol(&self) -> Self::Symbol {
                self.symbol
            }
        }
        impl orx_imp_vec::prelude::NotSelfRefVecItem for $storage {}
    };
}
