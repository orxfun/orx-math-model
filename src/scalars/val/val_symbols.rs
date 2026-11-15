use crate::scalars::{DSca, DVal};
use crate::Symbol;

// int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Int<'m, 'd>(pub(crate) Symbol<'m, 'd, DSca<'d>>);

impl<'m, 'd> Int<'m, 'd> {
    pub(crate) fn data(&self) -> &DVal<'d> {
        unsafe { &self.0.el.get().data.val }
    }
}

// num

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Num<'m, 'd>(pub(crate) Symbol<'m, 'd, DSca<'d>>);

impl<'m, 'd> Num<'m, 'd> {
    pub(crate) fn data(&self) -> &DVal<'d> {
        unsafe { &self.0.el.get().data.val }
    }
}
