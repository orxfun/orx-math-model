use crate::stages::Stage;
use crate::symbols::{symbol_meta::SymbolMeta, symbol_ref_core::SymbolRefCore};
use alloc::string::String;
use core::fmt::Debug;

pub trait SymbolRef<'m, S, M>
where
    S: Stage,
    M: SymbolMeta,
    Self: SymbolReq<'m, S, M>,
    M::Data: 'm,
{
    type Data;

    // provided

    fn key(self, key: impl Into<String>) -> Self {
        let symbol_ref: SymbolRefCore<'_, _, _> = self.into();
        symbol_ref.key(key).into()
    }

    fn definition(self, definition: impl Into<String>) -> Self {
        let symbol_ref: SymbolRefCore<'_, _, _> = self.into();
        symbol_ref.definition(definition).into()
    }
}

// required traits from Symbol::Ref

pub trait SymbolReq<'m, S, M>:
    From<SymbolRefCore<'m, S, M>> + Into<SymbolRefCore<'m, S, M>> + Debug
where
    S: Stage,
    M: SymbolMeta,
{
}

impl<'m, S, M, X> SymbolReq<'m, S, M> for X
where
    S: Stage,
    M: SymbolMeta,
    X: From<SymbolRefCore<'m, S, M>> + Into<SymbolRefCore<'m, S, M>> + Debug,
{
}
