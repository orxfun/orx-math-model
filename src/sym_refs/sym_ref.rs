use crate::scalars::{Sca, ScaTyp};
use crate::{Int, Num, Var};
use core::mem::transmute;

pub enum SymRef<'d> {
    Int(Int<'d, 'd>),
    Num(Num<'d, 'd>),
    Var(Var<'d, 'd>),
}

impl<'m, 'd> From<Int<'m, 'd>> for SymRef<'d> {
    fn from(value: Int<'m, 'd>) -> Self {
        let sym = unsafe { value.sym().create_sym_ref() };
        Self::Int(Int::new(sym))
    }
}

impl<'m, 'd> From<Num<'m, 'd>> for SymRef<'d> {
    fn from(value: Num<'m, 'd>) -> Self {
        let sym = unsafe { value.sym().create_sym_ref() };
        Self::Num(Num::new(sym))
    }
}

impl<'m, 'd> From<Var<'m, 'd>> for SymRef<'d> {
    fn from(value: Var<'m, 'd>) -> Self {
        let sym = unsafe { value.sym().create_sym_ref() };
        Self::Var(Var::new(sym))
    }
}

impl<'d> SymRef<'d> {
    pub fn new<'m, S: Sca>(sym: S::Sym<'m, 'd>) -> Self {
        let sym = unsafe { S::sym(sym).create_sym_ref() };
        match S::typ() {
            ScaTyp::Int => Self::Int(Int::new(sym)),
            ScaTyp::Num => Self::Num(Num::new(sym)),
            ScaTyp::Var => Self::Var(Var::new(sym)),
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
