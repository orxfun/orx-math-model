use crate::symbols::SymbolRef;

pub trait SymbolMeta: Default + 'static {
    type Data;

    type Ref<'m>: SymbolRef<'m, Self>
    where
        <Self as SymbolMeta>::Data: 'm;
}
