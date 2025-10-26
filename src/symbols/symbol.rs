use crate::symbols::Sym;

pub trait Symbol: Default + 'static {
    type Data;

    type Ref<'m>: Sym<'m, Self>
    where
        <Self as Symbol>::Data: 'm;
}
