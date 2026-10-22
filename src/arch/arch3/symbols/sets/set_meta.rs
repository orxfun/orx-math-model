use crate::symbols::sets::{SetCore, SetData};
use crate::symbols::symbol_meta::SymbolMeta;

#[derive(Default)]
pub struct SetMeta;

impl SymbolMeta for SetMeta {
    type Data = SetData;

    type Ref<'m>
        = SetCore<'m>
    where
        <Self as SymbolMeta>::Data: 'm;
}
