use crate::scalars::DSca;
#[cfg(test)]
use crate::scalars::DVar;
use crate::Symbol;

#[derive(derive_more::From, Clone, Copy)]
pub struct Var<'m, 'd>(pub(crate) Symbol<'m, 'd, DSca<'d>>);

impl<'m, 'd> Var<'m, 'd> {
    #[cfg(test)]
    pub(crate) fn data(&self) -> &DVar<'d> {
        unsafe { &self.0.el.get().data.var }
    }
}
