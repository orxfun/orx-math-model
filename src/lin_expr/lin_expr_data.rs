use crate::{lin_expr::DLinTerm, scalars::DNum};
use crate::{ConVec, Ptr};

pub const MAX_EXPR_LEN: usize = 8;

pub type ExprVec<'d> = ConVec<MAX_EXPR_LEN, DLinTerm<'d>>;

#[derive(derive_new::new)]
pub struct DLinExpr<'d> {
    pub(super) terms: ExprVec<'d>,
    pub(super) constant: Option<Ptr<'d, DNum<'d>>>,
}

impl<'d> DLinExpr<'d> {
    // get

    #[cfg(test)]
    pub fn eval_append_string(&self, f: &mut alloc::string::String) {
        use alloc::string::String;
        use core::fmt::Write;

        #[cfg(test)]
        fn write_prefix_and_coef(f: &mut String, c: f64) {
            match c >= 0.0 {
                true => write!(f, " + {} ", c),
                false => write!(f, " - {} ", -c),
            }
            .unwrap()
        }

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

    #[cfg(test)]
    pub fn eval_to_string(&self) -> alloc::string::String {
        let mut f = alloc::string::String::new();
        self.eval_append_string(&mut f);
        f
    }
}

#[cfg(test)]
fn write_constant(f: &mut alloc::string::String, has_terms: bool, c: f64) {
    use crate::constants::{is_neg, is_pos, is_zero};
    use core::fmt::Write;

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
