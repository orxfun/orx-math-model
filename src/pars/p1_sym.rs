use crate::pars::DPars;
use crate::{AsF64, Res, Set, SymWrap, Symbol, ValTyp};
use core::marker::PhantomData;

// int

#[derive(derive_new::new, Clone, Copy)]
pub struct IntParD1<'m, 'd, I0: Set>(Symbol<'m, 'd, DPars<'d>>, PhantomData<I0>);

impl<'m, 'd, I0: Set> SymWrap<'m, 'd> for IntParD1<'m, 'd, I0> {
    type Data = DPars<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

impl<'m, 'd, I0: Set> IntParD1<'m, 'd, I0> {
    pub fn bind<V: AsF64>(self, fun: impl Fn(I0::IdxRaw<'d>) -> V + 'd) -> Res {
        self.data().d1_bind::<V, I0>(ValTyp::Int, fun)
    }
}

// num

#[derive(derive_new::new, Clone, Copy)]
pub struct NumParD1<'m, 'd, I0: Set>(Symbol<'m, 'd, DPars<'d>>, PhantomData<I0>);

impl<'m, 'd, I0: Set> SymWrap<'m, 'd> for NumParD1<'m, 'd, I0> {
    type Data = DPars<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

impl<'m, 'd, I0: Set> NumParD1<'m, 'd, I0> {
    pub fn bind<V: AsF64>(self, fun: impl Fn(I0::IdxRaw<'d>) -> V + 'd) -> Res {
        self.data().d1_bind::<V, I0>(ValTyp::Num, fun)
    }
}
