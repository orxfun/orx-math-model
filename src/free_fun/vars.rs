use crate::vars::{Binary, BoundD0, BoundD1, BoundD2, Bounds, Continuous, Integer, VarType};
use crate::{
    BinVarD0, BinVarD1, BinVarD2, ContVarD0, ContVarD1, ContVarD2, IntVarD0, IntVarD1, IntVarD2,
    Model, Set, SymWrap, VarTyp,
};
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

    pub fn d2<'m, 'd, I0: Set, I1: Set>(
        self,
        s0: I0::SetSym<'m, 'd>,
        s1: I1::SetSym<'m, 'd>,
    ) -> BinVarD2<'m, 'd, I0, I1> {
        s0.m().var2::<Binary, _, _>(self.0, s0, s1, 0, 1)
    }
}

impl BVar<Continuous> {
    pub fn d0<'m, 'd>(self, m: &'m Model<'d>) -> BVar0<'m, 'd, Continuous> {
        BVar0::new(self.0, m)
    }

    pub fn d1<'m, 'd, I0: Set>(self, s0: I0::SetSym<'m, 'd>) -> BVar1<'m, 'd, Continuous, I0> {
        BVar1::new(self.0, s0)
    }

    pub fn d2<'m, 'd, I0: Set, I1: Set>(
        self,
        s0: I0::SetSym<'m, 'd>,
        s1: I1::SetSym<'m, 'd>,
    ) -> BVar2<'m, 'd, Continuous, I0, I1> {
        BVar2::new(self.0, s0, s1)
    }
}

impl BVar<Integer> {
    pub fn d0<'m, 'd>(self, m: &'m Model<'d>) -> BVar0<'m, 'd, Integer> {
        BVar0::new(self.0, m)
    }

    pub fn d1<'m, 'd, I0: Set>(self, s0: I0::SetSym<'m, 'd>) -> BVar1<'m, 'd, Integer, I0> {
        BVar1::new(self.0, s0)
    }

    pub fn d2<'m, 'd, I0: Set, I1: Set>(
        self,
        s0: I0::SetSym<'m, 'd>,
        s1: I1::SetSym<'m, 'd>,
    ) -> BVar2<'m, 'd, Integer, I0, I1> {
        BVar2::new(self.0, s0, s1)
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

impl<'m, 'd: 'm, I0: Set> BVar1<'m, 'd, Continuous, I0> {
    pub fn nonneg(self) -> ContVarD1<'m, 'd, I0> {
        self.1
            .m()
            .var1::<Continuous, _>(self.0, self.1, 0, BoundD1::None)
    }

    pub fn nonpos(self) -> ContVarD1<'m, 'd, I0> {
        self.1
            .m()
            .var1::<Continuous, _>(self.0, self.1, BoundD1::None, 0)
    }

    pub fn unbounded(self) -> ContVarD1<'m, 'd, I0> {
        self.1
            .m()
            .var1::<Continuous, _>(self.0, self.1, BoundD1::None, BoundD1::None)
    }

    pub fn bounded(
        self,
        lb: impl Into<BoundD1<'d>>,
        ub: impl Into<BoundD1<'d>>,
    ) -> ContVarD1<'m, 'd, I0> {
        self.1.m().var1::<Continuous, _>(self.0, self.1, lb, ub)
    }
}

impl<'m, 'd: 'm, I0: Set> BVar1<'m, 'd, Integer, I0> {
    pub fn nonneg(self) -> IntVarD1<'m, 'd, I0> {
        self.1
            .m()
            .var1::<Integer, _>(self.0, self.1, 0, BoundD1::None)
    }

    pub fn nonpos(self) -> IntVarD1<'m, 'd, I0> {
        self.1
            .m()
            .var1::<Integer, _>(self.0, self.1, BoundD1::None, 0)
    }

    pub fn unbounded(self) -> IntVarD1<'m, 'd, I0> {
        self.1
            .m()
            .var1::<Integer, _>(self.0, self.1, BoundD1::None, BoundD1::None)
    }

    pub fn bounded(
        self,
        lb: impl Into<BoundD1<'d>>,
        ub: impl Into<BoundD1<'d>>,
    ) -> IntVarD1<'m, 'd, I0> {
        self.1.m().var1::<Integer, _>(self.0, self.1, lb, ub)
    }
}

// d2

#[derive(derive_new::new)]
pub struct BVar2<'m, 'd: 'm, V: VarType, I0: Set, I1: Set>(
    String,
    I0::SetSym<'m, 'd>,
    I1::SetSym<'m, 'd>,
    PhantomData<V>,
);

impl<'m, 'd: 'm, I0: Set, I1: Set> BVar2<'m, 'd, Continuous, I0, I1> {
    pub fn nonneg(self) -> ContVarD2<'m, 'd, I0, I1> {
        self.1
            .m()
            .var2::<Continuous, _, _>(self.0, self.1, self.2, 0, BoundD2::None)
    }

    pub fn nonpos(self) -> ContVarD2<'m, 'd, I0, I1> {
        self.1
            .m()
            .var2::<Continuous, _, _>(self.0, self.1, self.2, BoundD2::None, 0)
    }

    pub fn unbounded(self) -> ContVarD2<'m, 'd, I0, I1> {
        self.1
            .m()
            .var2::<Continuous, _, _>(self.0, self.1, self.2, BoundD2::None, BoundD2::None)
    }

    pub fn bounded(
        self,
        lb: impl Into<BoundD2<'d>>,
        ub: impl Into<BoundD2<'d>>,
    ) -> ContVarD2<'m, 'd, I0, I1> {
        self.1
            .m()
            .var2::<Continuous, _, _>(self.0, self.1, self.2, lb, ub)
    }
}

impl<'m, 'd: 'm, I0: Set, I1: Set> BVar2<'m, 'd, Integer, I0, I1> {
    pub fn nonneg(self) -> IntVarD2<'m, 'd, I0, I1> {
        self.1
            .m()
            .var2::<Integer, _, _>(self.0, self.1, self.2, 0, BoundD2::None)
    }

    pub fn nonpos(self) -> IntVarD2<'m, 'd, I0, I1> {
        self.1
            .m()
            .var2::<Integer, _, _>(self.0, self.1, self.2, BoundD2::None, 0)
    }

    pub fn unbounded(self) -> IntVarD2<'m, 'd, I0, I1> {
        self.1
            .m()
            .var2::<Integer, _, _>(self.0, self.1, self.2, BoundD2::None, BoundD2::None)
    }

    pub fn bounded(
        self,
        lb: impl Into<BoundD2<'d>>,
        ub: impl Into<BoundD2<'d>>,
    ) -> IntVarD2<'m, 'd, I0, I1> {
        self.1
            .m()
            .var2::<Integer, _, _>(self.0, self.1, self.2, lb, ub)
    }
}
