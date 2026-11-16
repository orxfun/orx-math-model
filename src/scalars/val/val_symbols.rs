#[cfg(test)]
use crate::scalars::DVal;
use crate::Symbol;
use crate::{scalars::DSca, Ptr};

// int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Int<'m, 'd>(pub(crate) Symbol<'m, 'd, DSca<'d>>);

impl<'m, 'd> Int<'m, 'd> {
    #[cfg(test)]
    pub(crate) fn data(&self) -> &DVal<'d> {
        unsafe { &self.0.el.get().data.val }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DSca<'d>> {
        Ptr::new(self.0.el)
    }
}

// num

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Num<'m, 'd>(pub(crate) Symbol<'m, 'd, DSca<'d>>);

impl<'m, 'd> Num<'m, 'd> {
    #[cfg(test)]
    pub(crate) fn data(&self) -> &DVal<'d> {
        unsafe { &self.0.el.get().data.val }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DSca<'d>> {
        Ptr::new(self.0.el)
    }
}
