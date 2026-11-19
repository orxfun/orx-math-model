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
    pub fn eval_append_string(&self, f: &mut alloc::string::String, prefix_sum: bool) -> bool {
        match (&self.coef, prefix_sum) {
            (Coef::One, false) => {
                self.var.get().eval_append_string(f);
                true
            }
            (Coef::MinusOne, false) => {
                f.push_str("-");
                self.var.get().eval_append_string(f);
                true
            }
            (Coef::Con(c), false) => match write_coef(f, *c) {
                true => {
                    self.var.get().eval_append_string(f);
                    true
                }
                false => false,
            },
            (Coef::Ptr(c), false) => match write_coef(f, c.get().eval()) {
                true => {
                    self.var.get().eval_append_string(f);
                    true
                }
                false => false,
            },

            (Coef::One, true) => {
                f.push_str(" + ");
                self.var.get().eval_append_string(f);
                true
            }
            (Coef::MinusOne, true) => {
                f.push_str(" - ");
                self.var.get().eval_append_string(f);
                true
            }
            (Coef::Con(c), true) => match write_prefix_and_coef(f, *c) {
                true => {
                    self.var.get().eval_append_string(f);
                    true
                }
                false => false,
            },
            (Coef::Ptr(c), true) => match write_prefix_and_coef(f, c.get().eval()) {
                true => {
                    self.var.get().eval_append_string(f);
                    true
                }
                false => false,
            },
        }
    }

    #[cfg(test)]
    pub fn eval_to_string(&self) -> alloc::string::String {
        let mut f = alloc::string::String::new();
        self.eval_append_string(&mut f, false);
        f
    }
}

#[cfg(test)]
fn write_coef(f: &mut alloc::string::String, c: f64) -> bool {
    use crate::constants::{is_neg, is_pos, is_zero};
    use core::fmt::Write;
    use std::dbg;

    dbg!(c);

    match is_pos(c) {
        true => {
            write!(f, "{} ", c).unwrap();
            true
        }
        false => match is_neg(c) {
            true => {
                write!(f, "-{} ", -c).unwrap();
                true
            }
            false => {
                debug_assert!(is_zero(c));
                false
            }
        },
    }
}

#[cfg(test)]
fn write_prefix_and_coef(f: &mut alloc::string::String, c: f64) -> bool {
    use crate::constants::{is_neg, is_pos, is_zero};
    use core::fmt::Write;

    match is_pos(c) {
        true => {
            write!(f, " + {} ", c).unwrap();
            true
        }
        false => match is_neg(c) {
            true => {
                write!(f, " - {} ", -c).unwrap();
                true
            }
            false => {
                debug_assert!(is_zero(c));
                false
            }
        },
    }
}
