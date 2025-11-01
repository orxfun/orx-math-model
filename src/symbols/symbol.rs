use crate::symbols::SymbolRef;

pub trait Symbol: Default + 'static {
    type Data;

    type Ref<'m>: SymbolRef<'m, Self>
    where
        <Self as Symbol>::Data: 'm;
}
