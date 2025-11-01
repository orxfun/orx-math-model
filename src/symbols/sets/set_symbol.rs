use crate::symbols::{sets::set::Set, symbol::Symbol, SetKind};

#[derive(Default)]
pub struct SetSymbol;

impl Symbol for SetSymbol {
    type Data = SetKind;

    type Ref<'m>
        = Set<'m>
    where
        <Self as Symbol>::Data: 'm;
}
