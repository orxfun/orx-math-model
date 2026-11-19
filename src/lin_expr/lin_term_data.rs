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
    pub fn eval_append_string(
        &self,
        f: &mut alloc::string::String,
    ) -> Result<(), core::fmt::Error> {
        use alloc::string::String;
        use core::fmt::Write;

        match self.coef {
            Coef::One => {}
            Coef::MinusOne => f.push_str("-"),
            Coef::Con(c) => write!(f, "{} ", c)?,
            Coef::Ptr(c) => write!(f, "{} ", c.get().eval())?,
        };
        self.var.get().eval_append_string(f)
    }

    #[cfg(test)]
    pub fn eval_to_string(&self) -> Result<alloc::string::String, core::fmt::Error> {
        let mut f = alloc::string::String::new();
        self.eval_append_string(&mut f)?;
        Ok(f)
    }
}
