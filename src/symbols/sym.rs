use crate::symbols::{symbol::Symbol, symbol_ref::SymbolRef};
use core::fmt::Debug;

pub trait Sym<'m, S>
where
    S: Symbol,
    Self: SymbolReq<'m, S>,
    S::Data: 'm,
{
    type Data;
}

// required traits from Symbol::Ref

pub trait SymbolReq<'m, S>: From<SymbolRef<'m, S>> + Debug
where
    S: Symbol,
{
}

impl<'m, S, X> SymbolReq<'m, S> for X
where
    S: Symbol,
    X: From<SymbolRef<'m, S>> + Debug,
{
}
