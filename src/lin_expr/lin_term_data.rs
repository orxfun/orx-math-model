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
    pub fn neg<'m>(&self, m: &'m Model<'d>) -> Self {
        let coef = match self.coef {
            Coef::One => Coef::MinusOne,
            Coef::MinusOne => Coef::One,
            Coef::Con(c) => Coef::Con(-c),
            Coef::Ptr(c) => Coef::Ptr(m.val_unchecked::<ValNum>(DNum::neg(c)).ptr()),
        };
        Self::new(coef, self.var)
    }
}

#[cfg(test)]
pub mod ext_test {
    use super::*;
    use crate::constants::{is_neg, is_pos, is_zero};
    use alloc::string::String;
    use core::fmt::Write;

    impl<'d> DLinTerm<'d> {
        pub fn eval_append_string(&self, f: &mut String, pre_exists: bool) -> bool {
            match (&self.coef, pre_exists) {
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

        pub fn eval_to_string(&self) -> String {
            let mut f = String::new();
            self.eval_append_string(&mut f, false);
            f
        }
    }

    fn write_coef(f: &mut String, c: f64) -> bool {
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

    fn write_prefix_and_coef(f: &mut String, c: f64) -> bool {
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
}
