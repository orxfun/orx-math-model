use crate::constants::{MAX_EXPR_LEN, MAX_SUM_LEN};
use crate::lin_expr::{DLinTerm, DSum};
use crate::scalars::DNum;
use crate::{ConVec, Ptr};

pub type ExprVec<'d> = ConVec<MAX_EXPR_LEN, DLinTerm<'d>>;
pub type SumVec<'d> = ConVec<MAX_EXPR_LEN, DSum<'d>>;

#[derive(derive_new::new)]
pub struct DLinExpr<'d> {
    pub(super) terms: ExprVec<'d>,
    pub(super) sums: SumVec<'d>,
    pub(super) constant: Option<Ptr<'d, DNum<'d>>>,
}

impl<'d> DLinExpr<'d> {
    pub fn with_constant(&self, new_constant: Option<Ptr<'d, DNum<'d>>>) -> Self {
        Self::new(self.terms.clone(), self.sums.clone(), new_constant)
    }

    pub fn neg_with_constant(&self, new_constant: Option<Ptr<'d, DNum<'d>>>) -> Self {
        Self::new(self.terms.clone(), self.sums.clone(), new_constant)
    }
}

#[cfg(test)]
pub mod ext_test {
    use super::*;
    use crate::constants::{is_neg, is_pos, is_zero};
    use alloc::string::String;
    use core::fmt::Write;

    impl<'d> DLinExpr<'d> {
        pub fn eval_append_string(&self, f: &mut String) {
            let mut has_terms = false;
            let mut terms = self.terms.iter();
            if let Some(t) = terms.next() {
                let added = t.eval_append_string(f, has_terms);
                has_terms |= added;
                for t in terms {
                    let added = t.eval_append_string(f, has_terms);
                    has_terms |= added;
                }
            }

            if let Some(c) = self.constant {
                write_constant(f, has_terms, c.get().eval());
            }
        }

        pub fn eval_to_string(&self) -> String {
            let mut f = String::new();
            self.eval_append_string(&mut f);
            f
        }
    }

    #[cfg(test)]
    fn write_constant(f: &mut String, has_terms: bool, c: f64) {
        match has_terms {
            true => match is_pos(c) {
                true => write!(f, " + {}", c).unwrap(),
                false => match is_neg(c) {
                    true => write!(f, " - {}", -c).unwrap(),
                    false => {
                        debug_assert!(is_zero(c))
                    }
                },
            },
            false => match is_pos(c) {
                true => write!(f, "{}", c).unwrap(),
                false => match is_neg(c) {
                    true => write!(f, "-{}", -c).unwrap(),
                    false => {
                        debug_assert!(is_zero(c))
                    }
                },
            },
        }
    }
}
