use crate::symbols::{sets::set_ref::Set, symbol::Symbol, SetData};

pub struct SetSymbol;

impl Symbol for SetSymbol {
    type Data = SetData;

    type Ref<'m>
        = Set<'m>
    where
        <Self as Symbol>::Data: 'm;
}
