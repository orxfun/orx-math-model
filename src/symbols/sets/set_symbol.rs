use crate::symbols::{sets::set::Set, symbol_definition::SymbolDef, SetData};

#[derive(Default)]
pub struct SetSymbol;

impl SymbolDef for SetSymbol {
    type Data = SetData;

    type Ref<'m>
        = Set<'m>
    where
        <Self as SymbolDef>::Data: 'm;
}
