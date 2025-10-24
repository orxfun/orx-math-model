use crate::symbols::{sets::set::Set, symbol::Symbol, SetData};

#[derive(Default)]
pub struct SetSymbol;

impl Symbol for SetSymbol {
    type Data = SetData;

    type Ref<'m>
        = Set<'m>
    where
        <Self as Symbol>::Data: 'm;
}
