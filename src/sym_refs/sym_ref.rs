use crate::scalars::{DSca, ScaTyp};
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
    pub fn get2<'m, S>(&self, sym_ref: SymRef<'d>) -> S
    where
        S: Into<SymRef<'d>>,
    {
        todo!()
    }
    pub fn get<'m, V: Val>(&self) -> &V::Sym<'m, 'd> {
        match V::typ() {
            ValTyp::Int => match self {
                Self::Int(x) => unsafe { transmute(x) },
                _ => unreachable!(),
            },
            ValTyp::Num => match self {
                Self::Num(x) => unsafe { transmute(x) },
                _ => unreachable!(),
            },
        }
    }
}
