use crate::vars::DVars;
use crate::{impl_sym_wrap, SymWrap, Symbol};
use core::ops::Deref;

#[derive(derive_new::new, Clone, Copy)]
pub struct VarD0<'m, 'd>(Symbol<'m, 'd, DVars<'d>>);
impl_sym_wrap!(VarD0<'m, 'd>, DVars<'d>);

#[derive(derive_new::new, Clone, Copy)]
pub struct ContVarD0<'m, 'd>(VarD0<'m, 'd>);

impl<'m, 'd> Deref for ContVarD0<'m, 'd> {
    type Target = VarD0<'m, 'd>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
