use crate::vals::{DVal, Int, Num};
use crate::{Symbol, Val, ValTyp};
use core::mem::transmute;

pub enum SymRef<'d> {
    Int(Int<'d, 'd>),
    Num(Num<'d, 'd>),
}

impl<'d> SymRef<'d> {
    pub fn new<'m>(sym: Symbol<'m, 'd, DVal<'d>>) -> Self {
        let sym = unsafe { sym.create_sym_ref() };
        match unsafe { sym.el.get() }.typ() {
            ValTyp::Int => Self::Int(sym.into()),
            ValTyp::Num => Self::Num(sym.into()),
        }
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
