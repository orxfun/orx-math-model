use crate::symbols::{symbol_meta::SymbolMeta, symbol_ref_core::SymbolRefCore};
use alloc::string::String;
use core::fmt::{Debug, Display};

pub trait SymbolRef<'m, S>: Display
where
    S: SymbolMeta,
    Self: SymbolReq<'m, S>,
    S::Data: 'm,
{
    type Data;

    // provided

    fn key(self, key: impl Into<String>) -> Self {
        let symbol_ref: SymbolRefCore<'_, _> = self.into();
        symbol_ref.key(key).into()
    }

    fn def(self, definition: impl Into<String>) -> Self {
        let symbol_ref: SymbolRefCore<'_, _> = self.into();
        symbol_ref.definition(definition).into()
    }
}

// required traits from Symbol::Ref

pub trait SymbolReq<'m, S>:
    From<SymbolRefCore<'m, S>> + Into<SymbolRefCore<'m, S>> + Debug
where
    S: SymbolMeta,
{
}

impl<'m, S, X> SymbolReq<'m, S> for X
where
    S: SymbolMeta,
    X: From<SymbolRefCore<'m, S>> + Into<SymbolRefCore<'m, S>> + Debug,
{
}
