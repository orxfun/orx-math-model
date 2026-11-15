use crate::sets::IndexCatInt;
use crate::vars::v1::DV1;
use crate::{Ptr, Res, Set, SetCatInt, Symbol, Val, ScaInt, Var};
use alloc::boxed::Box;
use core::ops::Index;
use core::{marker::PhantomData, ops::Deref};

#[derive(derive_more::From, Clone, Copy)]
pub struct VarD1<'m, 'd, I0: Set>(pub(crate) Symbol<'m, 'd, DV1<'d>>, PhantomData<I0>);

impl<'m, 'd, I0: Set> From<Symbol<'m, 'd, DV1<'d>>> for VarD1<'m, 'd, I0> {
    fn from(value: Symbol<'m, 'd, DV1<'d>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<'m, 'd, I0: Set> Deref for VarD1<'m, 'd, I0> {
    type Target = Symbol<'m, 'd, DV1<'d>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'m, 'd, I0> VarD1<'m, 'd, I0>
where
    I0: Set,
{
    pub fn at(self, i0: impl Into<I0::IdxAny<'m, 'd>>) -> Var<'m, 'd> {
        I0::v1_at(self, i0)
    }
}

impl<'m, 'd, I0, Idx0> Index<Idx0> for VarD1<'m, 'd, I0>
where
    I0: Set,
    Idx0: Into<I0::IdxAny<'m, 'd>>,
{
    type Output = Var<'m, 'd>;

    fn index(&self, i0: Idx0) -> &Self::Output {
        let val = self.at(i0);
        // self.m.sym_ref::<V>(val)
        todo!()
    }
}
