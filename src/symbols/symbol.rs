use crate::symbols::{symbol_definition::SymbolDef, symbol_ref::SymbolRef};
use core::fmt::Debug;

pub trait Symbol<'m, S>
where
    S: SymbolDef,
    Self: SymbolReq<'m, S>,
    S::Data: 'm,
{
    type Data;
}

// required traits from Symbol::Ref

pub trait SymbolReq<'m, S>: From<SymbolRef<'m, S>> + Debug
where
    S: SymbolDef,
{
}

impl<'m, S, X> SymbolReq<'m, S> for X
where
    S: SymbolDef,
    X: From<SymbolRef<'m, S>> + Debug,
{
}
