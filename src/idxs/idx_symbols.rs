use crate::idxs::DIdx;
use crate::{Model, Ptr, Symbol};

// cat int

#[derive(derive_new::new, Clone, Copy)]
pub struct IdxCatInt<'m, 'd>(Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> IdxCatInt<'m, 'd> {
    pub(crate) fn sym(&self) -> Symbol<'m, 'd, DIdx<'d>> {
        self.0
    }

    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DIdx<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DIdx<'d>> {
        Ptr::new(self.0.el)
    }
}

// cat str

#[derive(derive_new::new, Clone, Copy)]
pub struct IdxCatStr<'m, 'd>(Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> IdxCatStr<'m, 'd> {
    pub(crate) fn sym(&self) -> Symbol<'m, 'd, DIdx<'d>> {
        self.0
    }

    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DIdx<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DIdx<'d>> {
        Ptr::new(self.0.el)
    }
}

// ord

#[derive(derive_new::new, Clone, Copy)]
pub struct IdxOrd<'m, 'd>(Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> IdxOrd<'m, 'd> {
    pub(crate) fn sym(&self) -> Symbol<'m, 'd, DIdx<'d>> {
        self.0
    }

    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DIdx<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DIdx<'d>> {
        Ptr::new(self.0.el)
    }
}
