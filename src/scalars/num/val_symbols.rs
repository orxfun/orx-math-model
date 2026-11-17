use crate::{scalars::num::DNum, Model, Symbol};

// int

#[derive(derive_new::new, Clone, Copy)]
pub struct Int<'m, 'd>(Symbol<'m, 'd, DNum<'d>>);

impl<'m, 'd> Int<'m, 'd> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn sym(&self) -> Symbol<'m, 'd, DNum<'d>> {
        self.0
    }

    // #[cfg(test)]
    // pub(crate) fn data(&self) -> &DVal<'d> {
    //     unsafe { &self.0.el.get().data().val }
    // }

    // pub(crate) fn ptr(&self) -> Ptr<'d, DNum<'d>> {
    //     Ptr::new_from_el(self.0.el)
    // }
}

// num

#[derive(derive_new::new, Clone, Copy)]
pub struct Num<'m, 'd>(Symbol<'m, 'd, DNum<'d>>);

impl<'m, 'd> Num<'m, 'd> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn sym(&self) -> Symbol<'m, 'd, DNum<'d>> {
        self.0
    }

    // #[cfg(test)]
    // pub(crate) fn data(&self) -> &DVal<'d> {
    //     unsafe { &self.0.el.get().data().val }
    // }

    // pub(crate) fn ptr(&self) -> Ptr<'d, DNum<'d>> {
    //     Ptr::new_from_el(self.0.el)
    // }
}
