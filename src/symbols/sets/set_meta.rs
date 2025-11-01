use crate::stages::Stage;
use crate::symbols::{sets::set::Set, symbol_meta::SymbolMeta, SetData};

#[derive(Default)]
pub struct SetMeta;

impl SymbolMeta for SetMeta {
    type Data<S>
        = SetData<S>
    where
        S: Stage;

    type Ref<'m, S>
        = Set<'m, S>
    where
        S: Stage,
        <Self as SymbolMeta>::Data<S>: 'm;
}
