use crate::pars::DPars;
use crate::scalars::{Int, Num};
use crate::{impl_sym_wrap, AsF64, Res, Set, SymWrap, Symbol, ValTyp};
use core::ops::Index;

// int

#[derive(derive_new::new, Clone, Copy)]
pub struct IntParD0<'m, 'd>(Symbol<'m, 'd, DPars<'d>>);
impl_sym_wrap!(IntParD0<'m, 'd>, DPars<'d>);

impl<'m, 'd> IntParD0<'m, 'd> {
    pub fn bind<V: AsF64>(self, fun: impl Fn() -> V + 'd) -> Res {
        self.data().d0_bind::<V>(ValTyp::Int, fun)
    }
}

// num

#[derive(derive_new::new, Clone, Copy)]
pub struct NumParD0<'m, 'd>(Symbol<'m, 'd, DPars<'d>>);
impl_sym_wrap!(NumParD0<'m, 'd>, DPars<'d>);

impl<'m, 'd> NumParD0<'m, 'd> {
    pub fn bind<V: AsF64>(self, fun: impl Fn() -> V + 'd) -> Res {
        self.data().d0_bind::<V>(ValTyp::Num, fun)
    }
}
