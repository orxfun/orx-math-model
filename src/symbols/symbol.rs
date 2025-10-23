use crate::symbols::symbol_ref::SymbolRef;

pub trait Symbol: Default {
    type Data;

    type Ref<'m>: From<SymbolRef<'m, Self::Data>>
    where
        <Self as Symbol>::Data: 'm;
}
