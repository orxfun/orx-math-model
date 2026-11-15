use crate::scalars::{DSca, ValTyp};
use crate::{Int, Num, ScaInt, ScaNum, Symbol, Var};

// enums

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScaTyp {
    Int,
    Num,
    Var,
}

impl From<ValTyp> for ScaTyp {
    fn from(value: ValTyp) -> Self {
        match value {
            ValTyp::Int => Self::Int,
            ValTyp::Num => Self::Num,
        }
    }
}

// meta

#[derive(Clone, Copy)]
pub struct ScaVar;

pub trait Sca: Clone + Copy + 'static {
    type Sym<'m, 'd: 'm>: Clone + Copy + From<Symbol<'m, 'd, DSca<'d>>>;

    fn typ() -> ScaTyp;

    fn sym<'m, 'd>(sym: Self::Sym<'m, 'd>) -> Symbol<'m, 'd, DSca<'d>>;
}

impl Sca for ScaInt {
    type Sym<'m, 'd: 'm> = Int<'m, 'd>;

    fn typ() -> ScaTyp {
        ScaTyp::Int
    }

    fn sym<'m, 'd>(sym: Self::Sym<'m, 'd>) -> Symbol<'m, 'd, DSca<'d>> {
        sym.0
    }
}

impl Sca for ScaNum {
    type Sym<'m, 'd: 'm> = Num<'m, 'd>;

    fn typ() -> ScaTyp {
        ScaTyp::Num
    }

    fn sym<'m, 'd>(sym: Self::Sym<'m, 'd>) -> Symbol<'m, 'd, DSca<'d>> {
        sym.0
    }
}

impl Sca for ScaVar {
    type Sym<'m, 'd: 'm> = Var<'m, 'd>;

    fn typ() -> ScaTyp {
        ScaTyp::Var
    }

    fn sym<'m, 'd>(sym: Self::Sym<'m, 'd>) -> Symbol<'m, 'd, DSca<'d>> {
        sym.0
    }
}
