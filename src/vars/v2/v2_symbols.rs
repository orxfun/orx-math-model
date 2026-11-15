use crate::sets::IndexCatInt;
use crate::vars::v2::DV2;
use crate::{Ptr, Res, Set, SetCatInt, Symbol, Val, ValInt, Var};
use alloc::boxed::Box;
use core::ops::Index;
use core::{marker::PhantomData, ops::Deref};

#[derive(derive_more::From, Clone, Copy)]
pub struct VarD2<'m, 'd, I0: Set, I1: Set>(
    pub(crate) Symbol<'m, 'd, DV2<'d>>,
    PhantomData<(I0, I1)>,
);

impl<'m, 'd, I0: Set, I1: Set> From<Symbol<'m, 'd, DV2<'d>>> for VarD2<'m, 'd, I0, I1> {
    fn from(value: Symbol<'m, 'd, DV2<'d>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<'m, 'd, I0: Set, I1: Set> Deref for VarD2<'m, 'd, I0, I1> {
    type Target = Symbol<'m, 'd, DV2<'d>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'m, 'd, I0, I1> VarD2<'m, 'd, I0, I1>
where
    I0: Set,
    I1: Set,
{
    pub fn at(
        self,
        i0: impl Into<I0::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> Var<'m, 'd> {
        I0::v2_at(self, i0, i1)
    }
}

impl<'m, 'd, I0, I1, Idx0, Idx1> Index<(Idx0, Idx1)> for VarD2<'m, 'd, I0, I1>
where
    I0: Set,
    I1: Set,
    Idx0: Into<I0::IdxAny<'m, 'd>>,
    Idx1: Into<I1::IdxAny<'m, 'd>>,
{
    type Output = Var<'m, 'd>;

    fn index(&self, (i0, i1): (Idx0, Idx1)) -> &Self::Output {
        let x = self.at(i0, i1);
        todo!()
    }
}
