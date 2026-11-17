#[cfg(test)]
use crate::scalars::DVar;
use crate::{scalars::DSca, Ptr};
use crate::{Model, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct Var<'m, 'd>(Symbol<'m, 'd, DSca<'d>>);

impl<'m, 'd> Var<'m, 'd> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn sym(&self) -> Symbol<'m, 'd, DSca<'d>> {
        self.0
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DSca<'d>> {
        Ptr::new_from_el(self.0.el)
    }

    #[cfg(test)]
    pub(crate) fn data(&self) -> &DVar<'d> {
        unsafe { &self.0.el.get().data().var }
    }
}
