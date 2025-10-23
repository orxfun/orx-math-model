use crate::symbols::{symbol::Symbol, SetData};

#[derive(Clone, Copy)]
pub struct Set<'m>(Symbol<'m, SetData>);

impl<'m> From<Symbol<'m, SetData>> for Set<'m> {
    fn from(value: Symbol<'m, SetData>) -> Self {
        Self(value)
    }
}
