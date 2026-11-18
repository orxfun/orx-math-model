use crate::scalars::{Int, Num, Var};
use crate::SymWrap;
use core::mem::transmute;

pub enum SymRef<'d> {
    Int(Int<'d, 'd>),
    Num(Num<'d, 'd>),
    Var(Var<'d, 'd>),
}

impl<'d> SymRef<'d> {
    pub fn int<'m>(sym: Int<'m, 'd>) -> Self {
        Self::Int(Int::new(unsafe { sym.sym().create_sym_ref() }))
    }

    pub fn num<'m>(sym: Num<'m, 'd>) -> Self {
        Self::Num(Num::new(unsafe { sym.sym().create_sym_ref() }))
    }

    pub fn var<'m>(sym: Var<'m, 'd>) -> Self {
        Self::Var(Var::new(unsafe { sym.sym().create_sym_ref() }))
    }

    pub fn get_int<'m>(&self) -> &Int<'m, 'd> {
        match self {
            Self::Int(x) => unsafe { transmute(x) },
            _ => unreachable!(),
        }
    }

    pub fn get_num<'m>(&self) -> &Num<'m, 'd> {
        match self {
            Self::Num(x) => unsafe { transmute(x) },
            _ => unreachable!(),
        }
    }

    pub fn get_var<'m>(&self) -> &Var<'m, 'd> {
        match self {
            Self::Var(x) => unsafe { transmute(x) },
            _ => unreachable!(),
        }
    }
}
