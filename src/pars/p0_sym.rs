use crate::pars::DPars;
use crate::scalars::{DNum, Int, Num, ValInt, ValNum};
use crate::{impl_sym_wrap, AsF64, Res, Set, SymWrap, Symbol, ValTyp};
use alloc::boxed::Box;
use core::ops::Index;

// int

#[derive(derive_new::new, Clone, Copy)]
pub struct IntParD0<'m, 'd>(Symbol<'m, 'd, DPars<'d>>);
impl_sym_wrap!(IntParD0<'m, 'd>, DPars<'d>);

impl<'m, 'd> IntParD0<'m, 'd> {
    pub fn bind<V: AsF64>(self, fun: impl Fn() -> V + 'd) -> Res {
        self.data().d0_bind::<V>(ValTyp::Int, fun)
    }

    pub(crate) fn to_int(self) -> Int<'m, 'd> {
        let fun = self.data().fun_ptr();
        let data = DNum::new(ValTyp::Int, Box::new(move || fun.get().d0_at()));
        self.m().val::<ValInt>(data)
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

    pub(crate) fn to_num(self) -> Num<'m, 'd> {
        let fun = self.data().fun_ptr();
        let data = DNum::new(ValTyp::Num, Box::new(move || fun.get().d0_at()));
        self.m().val::<ValNum>(data)
    }
}
