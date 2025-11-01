use crate::{stages::Stage, symbols::SymbolRef};

pub trait SymbolMeta: Default + 'static {
    type Data<S>
    where
        S: Stage;

    type Ref<'m, S>: SymbolRef<'m, S, Self>
    where
        S: Stage,
        <Self as SymbolMeta>::Data<S>: 'm;
}
