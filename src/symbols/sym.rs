use crate::symbols::{symbol::Symbol, symbol_ref::SymbolRef};
use alloc::string::String;
use core::fmt::Debug;

pub trait Sym<'m, S>
where
    S: Symbol,
    Self: SymbolReq<'m, S>,
    S::Data: 'm,
{
    type Data;

    // provided

    fn key(self, key: impl Into<String>) -> Self {
        let symbol_ref: SymbolRef<'_, _> = self.into();
        symbol_ref.key(key).into()
    }

    fn definition(self, definition: impl Into<String>) -> Self {
        let symbol_ref: SymbolRef<'_, _> = self.into();
        symbol_ref.definition(definition).into()
    }
}

// required traits from Symbol::Ref

pub trait SymbolReq<'m, S>: From<SymbolRef<'m, S>> + Into<SymbolRef<'m, S>> + Debug
where
    S: Symbol,
{
}

impl<'m, S, X> SymbolReq<'m, S> for X
where
    S: Symbol,
    X: From<SymbolRef<'m, S>> + Into<SymbolRef<'m, S>> + Debug,
{
}
