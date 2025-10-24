use crate::symbols::Symbol;

pub trait SymbolDef: Default + 'static {
    type Data;

    type Ref<'m>: Symbol<'m, Self>
    where
        <Self as SymbolDef>::Data: 'm;
}
