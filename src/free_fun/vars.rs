use crate::vars::{Binary, BoundD0, BoundD2, Bounds, Continuous, Integer, VarType};
use crate::{BinVarD0, BinVarD1, ContVarD0, IntVarD0, Model, Set, SymWrap, VarTyp};
use alloc::string::{String, ToString};
use core::marker::PhantomData;

pub fn var(key: impl ToString) -> BVar<Continuous> {
    BVar::new(key.to_string())
}

#[derive(derive_new::new)]
pub struct BVar<V: VarType>(String, PhantomData<V>);

impl<V: VarType> BVar<V> {
    pub fn binary(self) -> BVar<Binary> {
        BVar::new(self.0)
    }

    pub fn integer(self) -> BVar<Integer> {
        BVar::new(self.0)
    }
}

impl BVar<Binary> {
    pub fn d0<'m, 'd>(self, m: &'m Model<'d>) -> BinVarD0<'m, 'd> {
        m.var0::<Binary>(self.0, 0, 1)
    }

    pub fn d1<'m, 'd, I0: Set>(self, s0: I0::SetSym<'m, 'd>) -> BinVarD1<'m, 'd, I0> {
        s0.m().var1::<Binary, _>(self.0, s0, 0, 1)
    }
}

impl BVar<Continuous> {
    pub fn d0<'m, 'd>(self, m: &'m Model<'d>) -> BVar0<'m, 'd, Continuous> {
        BVar0::new(self.0, m)
    }
}

impl BVar<Integer> {
    pub fn d0<'m, 'd>(self, m: &'m Model<'d>) -> BVar0<'m, 'd, Integer> {
        BVar0::new(self.0, m)
    }
}

// d0

#[derive(derive_new::new)]
pub struct BVar0<'m, 'd: 'm, V: VarType>(String, &'m Model<'d>, PhantomData<V>);

impl<'m, 'd: 'm> BVar0<'m, 'd, Continuous> {
    pub fn nonneg(self) -> ContVarD0<'m, 'd> {
        self.1.var0::<Continuous>(self.0, 0, BoundD0::None)
    }

    pub fn nonpos(self) -> ContVarD0<'m, 'd> {
        self.1.var0::<Continuous>(self.0, BoundD0::None, 0)
    }

    pub fn unbounded(self) -> ContVarD0<'m, 'd> {
        self.1
            .var0::<Continuous>(self.0, BoundD0::None, BoundD0::None)
    }

    pub fn bounded(
        self,
        lb: impl Into<BoundD0<'d>>,
        ub: impl Into<BoundD0<'d>>,
    ) -> ContVarD0<'m, 'd> {
        self.1.var0::<Continuous>(self.0, lb, ub)
    }
}

impl<'m, 'd: 'm> BVar0<'m, 'd, Integer> {
    pub fn nonneg(self) -> IntVarD0<'m, 'd> {
        self.1.var0::<Integer>(self.0, 0, BoundD0::None)
    }

    pub fn nonpos(self) -> IntVarD0<'m, 'd> {
        self.1.var0::<Integer>(self.0, BoundD0::None, 0)
    }

    pub fn unbounded(self) -> IntVarD0<'m, 'd> {
        self.1.var0::<Integer>(self.0, BoundD0::None, BoundD0::None)
    }

    pub fn bounded(
        self,
        lb: impl Into<BoundD0<'d>>,
        ub: impl Into<BoundD0<'d>>,
    ) -> IntVarD0<'m, 'd> {
        self.1.var0::<Integer>(self.0, lb, ub)
    }
}

// d1

#[derive(derive_new::new)]
pub struct BVar1<'m, 'd: 'm, V: VarType, I0: Set>(String, I0::SetSym<'m, 'd>, PhantomData<V>);

// d2

#[derive(derive_new::new)]
pub struct BVar2<'m, 'd: 'm, V: VarType, I0: Set, I1: Set>(
    String,
    I0::SetSym<'m, 'd>,
    I1::SetSym<'m, 'd>,
    PhantomData<V>,
);
