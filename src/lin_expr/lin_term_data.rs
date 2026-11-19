use crate::scalars::{DNum, DVar, ValNum};
use crate::{AsF64, Model, Ptr, SymWrap, Var};
#[cfg(test)]
use alloc::string::String;
#[cfg(test)]
use core::fmt::Write;

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
    pub fn eval_append_string(&self, f: &mut String, prefix_sum: bool) {
        match (&self.coef, prefix_sum) {
            (Coef::One, false) => self.var.get().eval_append_string(f),
            (Coef::MinusOne, false) => {
                f.push_str("-");
                self.var.get().eval_append_string(f);
            }
            (Coef::Con(c), false) => {
                write!(f, "{} ", c).unwrap();
                self.var.get().eval_append_string(f);
            }
            (Coef::Ptr(c), false) => {
                write!(f, "{} ", c.get().eval()).unwrap();
                self.var.get().eval_append_string(f);
            }

            (Coef::One, true) => {
                f.push_str(" + ");
                self.var.get().eval_append_string(f);
            }
            (Coef::MinusOne, true) => {
                f.push_str(" - ");
                self.var.get().eval_append_string(f);
            }
            (Coef::Con(c), true) => {
                write_prefix_and_coef(f, *c);
                self.var.get().eval_append_string(f);
            }
            (Coef::Ptr(c), true) => {
                write_prefix_and_coef(f, c.get().eval());
                self.var.get().eval_append_string(f);
            }
        }
    }

    #[cfg(test)]
    pub fn eval_to_string(&self) -> String {
        let mut f = String::new();
        self.eval_append_string(&mut f, false);
        f
    }
}

#[cfg(test)]
fn write_prefix_and_coef(f: &mut String, c: f64) {
    match c >= 0.0 {
        true => write!(f, " + {} ", c),
        false => write!(f, " - {} ", -c),
    }
    .unwrap()
}
