use crate::scalars::Var;
use crate::vars::DVars;
use crate::{Set, SymWrap, Symbol};
use core::marker::PhantomData;
use core::ops::{Deref, Index};

#[derive(derive_new::new, Clone, Copy)]
pub struct VarD2<'m, 'd, I0: Set, I1: Set>(Symbol<'m, 'd, DVars<'d>>, PhantomData<(I0, I1)>);

impl<'m, 'd, I0: Set, I1: Set> SymWrap<'m, 'd> for VarD2<'m, 'd, I0, I1> {
    type Data = DVars<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

impl<'m, 'd, I0: Set, I1: Set> VarD2<'m, 'd, I0, I1> {
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
        self.m().var_ref(self.at(i0, i1))
    }
}

// variants

macro_rules! impl_variants {
    ($name:ident) => {
        #[derive(derive_new::new, Clone, Copy)]
        pub struct $name<'m, 'd, I0: Set, I1: Set>(VarD2<'m, 'd, I0, I1>);

        impl<'m, 'd, I0: Set, I1: Set> Deref for $name<'m, 'd, I0, I1> {
            type Target = VarD2<'m, 'd, I0, I1>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'m, 'd, I0: Set, I1: Set> From<VarD2<'m, 'd, I0, I1>> for $name<'m, 'd, I0, I1> {
            fn from(value: VarD2<'m, 'd, I0, I1>) -> Self {
                Self(value)
            }
        }
    };
}

impl_variants!(ContVarD2);
impl_variants!(BinVarD2);
impl_variants!(IntVarD2);
