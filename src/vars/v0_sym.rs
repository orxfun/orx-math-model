use crate::vars::DVars;
use crate::{impl_sym_wrap, SymWrap, Symbol};
use core::ops::Deref;

#[derive(derive_new::new, Clone, Copy)]
pub struct VarD0<'m, 'd>(Symbol<'m, 'd, DVars<'d>>);
impl_sym_wrap!(VarD0<'m, 'd>, DVars<'d>);

// variants

macro_rules! impl_variants {
    ($name:ident) => {
        #[derive(derive_new::new, Clone, Copy)]
        pub struct $name<'m, 'd>(VarD0<'m, 'd>);

        impl<'m, 'd> Deref for $name<'m, 'd> {
            type Target = VarD0<'m, 'd>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'m, 'd> From<VarD0<'m, 'd>> for $name<'m, 'd> {
            fn from(value: VarD0<'m, 'd>) -> Self {
                Self(value)
            }
        }
    };
}

impl_variants!(ContVarD0);
impl_variants!(BinVarD0);
impl_variants!(IntVarD0);
