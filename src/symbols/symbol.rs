use crate::symbols::symbol_ref::SymbolRef;
use core::{fmt::Debug, ops::Deref};

pub trait Symbol: Default {
    type Data;

    type Ref<'m>: SymbolRefReq<'m, Self::Data>
    where
        <Self as Symbol>::Data: 'm;
}

// required traits from Symbol::Ref

pub trait SymbolRefReq<'m, Data>:
    From<SymbolRef<'m, Data>> + Deref<Target = SymbolRef<'m, Data>> + Debug
where
    Data: 'm,
{
}

impl<'m, Data, X> SymbolRefReq<'m, Data> for X
where
    Data: 'm,
    X: From<SymbolRef<'m, Data>> + Deref<Target = SymbolRef<'m, Data>> + Debug,
{
}
