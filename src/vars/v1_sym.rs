use crate::scalars::Var;
use crate::vars::DVars;
use crate::{Set, SymWrap, Symbol};
use core::marker::PhantomData;
use core::ops::{Deref, Index};

#[derive(derive_new::new, Clone, Copy)]
pub struct VarD1<'m, 'd, I0: Set>(Symbol<'m, 'd, DVars<'d>>, PhantomData<I0>);

impl<'m, 'd, I0: Set> SymWrap<'m, 'd> for VarD1<'m, 'd, I0> {
    type Data = DVars<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

impl<'m, 'd, I0: Set> VarD1<'m, 'd, I0> {
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
        self.m().var_ref(self.at(i0))
    }
}

// variants

macro_rules! impl_variants {
    ($name:ident) => {
        #[derive(derive_new::new, Clone, Copy)]
        pub struct $name<'m, 'd, I0: Set>(VarD1<'m, 'd, I0>);

        impl<'m, 'd, I0: Set> Deref for $name<'m, 'd, I0> {
            type Target = VarD1<'m, 'd, I0>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

impl_variants!(ContVarD1);
impl_variants!(BinVarD1);
impl_variants!(IntVarD1);
