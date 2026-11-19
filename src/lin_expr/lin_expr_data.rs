use crate::constants::{MAX_EXPR_LEN, MAX_SUM_LEN};
use crate::lin_expr::{DLinTerm, DSum};
use crate::scalars::{DNum, ValNum};
use crate::{ConVec, Model, Ptr, SymWrap, ValTyp};
use alloc::boxed::Box;

pub type TermVec<'d> = ConVec<MAX_EXPR_LEN, DLinTerm<'d>>;
pub type SumVec<'d> = ConVec<MAX_EXPR_LEN, DSum<'d>>;

#[derive(derive_new::new)]
pub struct DLinExpr<'d> {
    pub(super) terms: TermVec<'d>,
    pub(super) sums: SumVec<'d>,
    pub(super) constant: Option<Ptr<'d, DNum<'d>>>,
}

impl<'d> DLinExpr<'d> {
    pub fn neg<'m>(&self, m: &'m Model<'d>) -> Ptr<'d, Self> {
        fn const_neg<'m, 'd>(
            m: &'m Model<'d>,
            c1: Option<Ptr<'d, DNum<'d>>>,
        ) -> Option<Ptr<'d, DNum<'d>>> {
            c1.map(|c| {
                let fun = Box::new(move || -c.get().eval());
                m.val::<ValNum>(DNum::new(ValTyp::Num, fun)).ptr()
            })
        }

        let expr_terms = self.terms.map(|t| t.neg(m));
        let expr_sums = self.sums.map(|s| s.neg(m));
        let expr_constant = const_neg(m, self.constant);
        let expr = DLinExpr::new(expr_terms, expr_sums, expr_constant);
        m.lin_exp(expr).ptr()
    }

    pub fn with_constant(&self, constant: Option<Ptr<'d, DNum<'d>>>) -> Self {
        Self::new(self.terms.clone(), self.sums.clone(), constant)
    }

    pub fn neg_with_constant<'m>(
        &self,
        m: &'m Model<'d>,
        constant: Option<Ptr<'d, DNum<'d>>>,
    ) -> Self {
        Self::new(
            self.terms.map(|t| t.neg(m)),
            self.sums.map(|s| s.neg(m)),
            constant,
        )
    }
}

#[cfg(test)]
pub mod ext_test {
    use super::*;
    use crate::constants::{is_neg, is_pos, is_zero};
    use alloc::string::String;
    use core::fmt::Write;

    impl<'d> DLinExpr<'d> {
        pub fn eval_append_string(&self, f: &mut String, pre_exists: bool) -> bool {
            let (mut pre_exists, c) = self.eval_append_string_exc_constant(f, pre_exists);

            if let Some(c) = c {
                pre_exists |= write_constant(f, pre_exists, c);
            }

            pre_exists
        }

        pub fn eval_append_string_exc_constant(
            &self,
            f: &mut String,
            pre_exists: bool,
        ) -> (bool, Option<f64>) {
            let mut pre_exists = pre_exists;

            let mut terms = self.terms.iter();
            if let Some(t) = terms.next() {
                pre_exists |= t.eval_append_string(f, pre_exists);
                for t in terms {
                    pre_exists |= t.eval_append_string(f, pre_exists);
                }
            }

            let mut sums = self.sums.iter();
            if let Some(s) = sums.next() {
                pre_exists |= s.eval_append_string(f, pre_exists);
                for s in sums {
                    pre_exists |= s.eval_append_string(f, pre_exists);
                }
            }

            let c = self.constant.map(|c| c.get().eval());
            (pre_exists, c)
        }

        pub fn eval_to_string(&self) -> String {
            let mut f = String::new();
            self.eval_append_string(&mut f, false);
            f
        }
    }

    pub fn write_constant(f: &mut String, has_terms: bool, c: f64) -> bool {
        match has_terms {
            true => match is_pos(c) {
                true => {
                    write!(f, " + {}", c).unwrap();
                    true
                }
                false => match is_neg(c) {
                    true => {
                        write!(f, " - {}", -c).unwrap();
                        true
                    }
                    false => {
                        debug_assert!(is_zero(c));
                        false
                    }
                },
            },
            false => match is_pos(c) {
                true => {
                    write!(f, "{}", c).unwrap();
                    true
                }
                false => match is_neg(c) {
                    true => {
                        write!(f, "-{}", -c).unwrap();
                        true
                    }
                    false => {
                        debug_assert!(is_zero(c));
                        false
                    }
                },
            },
        }
    }
}
