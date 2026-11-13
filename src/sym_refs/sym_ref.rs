use crate::vals::{Int, Num};

pub enum SymRef<'d> {
    Int(Int<'d, 'd>),
    Num(Num<'d, 'd>),
}

impl<'d> SymRef<'d> {
    pub fn int<'m>(sym: Int<'m, 'd>) -> Self {
        Self::Int(unsafe { sym.create_sym_ref() }.into())
    }

    pub fn num<'m>(sym: Num<'m, 'd>) -> Self {
        Self::Num(unsafe { sym.create_sym_ref() }.into())
    }
}
