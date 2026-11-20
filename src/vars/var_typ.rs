use crate::{
    BinVarD0, BinVarD1, BinVarD2, ContVarD0, ContVarD1, ContVarD2, IntVarD0, IntVarD1, IntVarD2,
    Set, VarD0, VarD1, VarD2,
};

#[derive(Clone, Copy, PartialEq)]
pub enum VarTyp {
    Continuous,
    Binary,
    Integer,
}

// trait

pub trait VarType {
    type VarD0<'m, 'd: 'm>: From<VarD0<'m, 'd>>;

    type VarD1<'m, 'd: 'm, I0: Set>: From<VarD1<'m, 'd, I0>>;

    type VarD2<'m, 'd: 'm, I0: Set, I1: Set>: From<VarD2<'m, 'd, I0, I1>>;

    fn typ() -> VarTyp;
}

pub struct Continuous;
impl VarType for Continuous {
    type VarD0<'m, 'd: 'm> = ContVarD0<'m, 'd>;

    type VarD1<'m, 'd: 'm, I0: Set> = ContVarD1<'m, 'd, I0>;

    type VarD2<'m, 'd: 'm, I0: Set, I1: Set> = ContVarD2<'m, 'd, I0, I1>;

    fn typ() -> VarTyp {
        VarTyp::Continuous
    }
}

pub struct Binary;
impl VarType for Binary {
    type VarD0<'m, 'd: 'm> = BinVarD0<'m, 'd>;

    type VarD1<'m, 'd: 'm, I0: Set> = BinVarD1<'m, 'd, I0>;

    type VarD2<'m, 'd: 'm, I0: Set, I1: Set> = BinVarD2<'m, 'd, I0, I1>;

    fn typ() -> VarTyp {
        VarTyp::Binary
    }
}

pub struct Integer;
impl VarType for Integer {
    type VarD0<'m, 'd: 'm> = IntVarD0<'m, 'd>;

    type VarD1<'m, 'd: 'm, I0: Set> = IntVarD1<'m, 'd, I0>;

    type VarD2<'m, 'd: 'm, I0: Set, I1: Set> = IntVarD2<'m, 'd, I0, I1>;

    fn typ() -> VarTyp {
        VarTyp::Integer
    }
}
