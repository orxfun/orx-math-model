use crate::symbols::{
    params::{par_data::ParData, r#ref::ParCore},
    symbol_meta::SymbolMeta,
};

#[derive(Default)]
pub struct ParMeta;

impl SymbolMeta for ParMeta {
    type Data = ParData;

    type Ref<'m>
        = ParCore<'m>
    where
        <Self as SymbolMeta>::Data: 'm;
}
