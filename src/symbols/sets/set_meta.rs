use crate::symbols::{sets::set::Set, symbol_meta::SymbolMeta, SetData};

#[derive(Default)]
pub struct SetMeta;

impl SymbolMeta for SetMeta {
    type Data = SetData;

    type Ref<'m>
        = Set<'m>
    where
        <Self as SymbolMeta>::Data: 'm;
}
