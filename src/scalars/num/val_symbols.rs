use crate::scalars::{num::DNum, ValInt, ValNum};
use crate::{impl_sym_wrap, Model, SymWrap, Symbol};

// int

#[derive(derive_new::new, Clone, Copy)]
pub struct Int<'m, 'd>(Symbol<'m, 'd, DNum<'d>>);
impl_sym_wrap!(Int<'m, 'd>, DNum<'d>);

impl<'m, 'd> Int<'m, 'd> {
    pub(crate) fn neg(&self) -> Self {
        self.m().val::<ValInt>(DNum::neg(self.ptr()))
    }

    pub(crate) fn recip(&self) -> Self {
        self.m().val::<ValInt>(DNum::recip(self.ptr()))
    }
}

// num

#[derive(derive_new::new, Clone, Copy)]
pub struct Num<'m, 'd>(Symbol<'m, 'd, DNum<'d>>);
impl_sym_wrap!(Num<'m, 'd>, DNum<'d>);

impl<'m, 'd> Num<'m, 'd> {}

impl<'m, 'd> Num<'m, 'd> {
    pub(crate) fn neg(&self) -> Self {
        self.m().val::<ValNum>(DNum::neg(self.ptr()))
    }

    pub(crate) fn recip(&self) -> Self {
        self.m().val::<ValNum>(DNum::recip(self.ptr()))
    }
}
