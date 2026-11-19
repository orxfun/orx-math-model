use crate::scalars::{DNum, DVar, ValNum};
use crate::{AsF64, Model, Ptr, SymWrap, Var};

#[derive(Clone)]
pub enum Coef<'d> {
    One,
    MinusOne,
    Con(f64),
    Ptr(Ptr<'d, DNum<'d>>),
}

#[derive(derive_new::new, Clone)]
pub struct DLinTerm<'d> {
    pub coef: Coef<'d>,
    pub var: Ptr<'d, DVar<'d>>,
}

impl<'d> DLinTerm<'d> {
    // get

    #[cfg(test)]
    pub fn eval_string(&self) -> alloc::string::String {
        use alloc::format;
        use alloc::string::String;
        let var = self.var.get().eval_string();
        match self.coef {
            Coef::One => var,
            Coef::MinusOne => format!("-{var}"),
            Coef::Con(c) => format!("{c} {var}"),
            Coef::Ptr(c) => format!("{} {var}", c.get().eval()),
        }
    }
}
