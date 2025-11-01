use crate::symbols::{symbol_meta::SymbolMeta, symbol_ref_core::SymbolRefCore};
use alloc::string::String;
use core::fmt::Debug;

pub trait SymbolRef<'m, M>
where
    M: SymbolMeta,
    Self: SymbolReq<'m, M>,
    M::Data: 'm,
{
    type Data;

    // provided

    fn key(self, key: impl Into<String>) -> Self {
        let symbol_ref: SymbolRefCore<'_, _> = self.into();
        symbol_ref.key(key).into()
    }

    fn definition(self, definition: impl Into<String>) -> Self {
        let symbol_ref: SymbolRefCore<'_, _> = self.into();
        symbol_ref.definition(definition).into()
    }
}

// required traits from Symbol::Ref

pub trait SymbolReq<'m, M>:
    From<SymbolRefCore<'m, M>> + Into<SymbolRefCore<'m, M>> + Debug
where
    M: SymbolMeta,
{
}

impl<'m, M, X> SymbolReq<'m, M> for X
where
    M: SymbolMeta,
    X: From<SymbolRefCore<'m, M>> + Into<SymbolRefCore<'m, M>> + Debug,
{
}
