use crate::pars::DPars;
use crate::scalars::{Int, Num};
use crate::{AsF64, Res, Set, SymWrap, Symbol, ValTyp};
use core::marker::PhantomData;

// int

#[derive(derive_new::new, Clone, Copy)]
pub struct IntParD2<'m, 'd, I0: Set, I1: Set>(Symbol<'m, 'd, DPars<'d>>, PhantomData<(I0, I1)>);

impl<'m, 'd, I0: Set, I1: Set> SymWrap<'m, 'd> for IntParD2<'m, 'd, I0, I1> {
    type Data = DPars<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

impl<'m, 'd, I0: Set, I1: Set> IntParD2<'m, 'd, I0, I1> {
    pub fn bind<V: AsF64>(self, fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V + 'd) -> Res {
        self.data().d2_bind::<V, I0, I1>(ValTyp::Int, fun)
    }

    pub fn at(
        self,
        i0: impl Into<I0::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> Int<'m, 'd> {
        I0::int_p2_at::<I1>(self, i0, i1)
    }
}

// num

#[derive(derive_new::new, Clone, Copy)]
pub struct NumParD2<'m, 'd, I0: Set, I1: Set>(Symbol<'m, 'd, DPars<'d>>, PhantomData<(I0, I1)>);

impl<'m, 'd, I0: Set, I1: Set> SymWrap<'m, 'd> for NumParD2<'m, 'd, I0, I1> {
    type Data = DPars<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

impl<'m, 'd, I0: Set, I1: Set> NumParD2<'m, 'd, I0, I1> {
    pub fn bind<V: AsF64>(self, fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V + 'd) -> Res {
        self.data().d2_bind::<V, I0, I1>(ValTyp::Num, fun)
    }

    pub fn at(
        self,
        i0: impl Into<I0::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> Num<'m, 'd> {
        I0::num_p2_at::<I1>(self, i0, i1)
    }
}
