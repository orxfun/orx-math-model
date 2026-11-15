use crate::scalars::{DSca, Sca, ScaTyp};
use crate::{Int, Num, Symbol, Val, ValTyp, Var};
use core::mem::transmute;

pub enum SymRef<'d> {
    Int(Int<'d, 'd>),
    Num(Num<'d, 'd>),
    Var(Var<'d, 'd>),
}

impl<'m, 'd> From<Int<'m, 'd>> for SymRef<'d> {
    fn from(value: Int<'m, 'd>) -> Self {
        let sym = unsafe { value.create_sym_ref() };
        Self::Int(sym.into())
    }
}

impl<'m, 'd> From<Num<'m, 'd>> for SymRef<'d> {
    fn from(value: Num<'m, 'd>) -> Self {
        let sym = unsafe { value.create_sym_ref() };
        Self::Num(sym.into())
    }
}

impl<'m, 'd> From<Var<'m, 'd>> for SymRef<'d> {
    fn from(value: Var<'m, 'd>) -> Self {
        let sym = unsafe { value.0.create_sym_ref() };
        Self::Var(sym.into())
    }
}

impl<'d> SymRef<'d> {
    pub fn new<'m, S: Sca>(sym: S::Sym<'m, 'd>) -> Self {
        let sym = unsafe { S::sym(sym).create_sym_ref() };
        match S::typ() {
            ScaTyp::Int => Self::Int(sym.into()),
            ScaTyp::Num => Self::Num(sym.into()),
            ScaTyp::Var => Self::Var(sym.into()),
        }
    }

    pub fn get<'m, S: Sca>(&self) -> &S::Sym<'m, 'd> {
        match S::typ() {
            ScaTyp::Int => match self {
                Self::Int(x) => unsafe { transmute(x) },
                _ => unreachable!(),
            },
            ScaTyp::Num => match self {
                Self::Num(x) => unsafe { transmute(x) },
                _ => unreachable!(),
            },
            ScaTyp::Var => match self {
                Self::Var(x) => unsafe { transmute(x) },
                _ => unreachable!(),
            },
        }
    }
}
