use crate::symbols::{sets::r#ref::SetCore, symbol_meta::SymbolMeta, SetData};

#[derive(Default)]
pub struct SetMeta;

impl SymbolMeta for SetMeta {
    type Data = SetData;

    type Ref<'m>
        = SetCore<'m>
    where
        <Self as SymbolMeta>::Data: 'm;
}
