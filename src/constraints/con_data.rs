use crate::expand::{expand_d1, expand_d2};
use crate::idxs::IdxVec;
use crate::lin_expr::DLinExpr;
use crate::{Elements, ElementsIter, Ptr, Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::vec::Vec;

pub enum IneqTyp {
    Leq,
    Geq,
    Eq,
}

#[derive(derive_new::new)]
pub struct DCon<'d> {
    idxs: IdxVec<'d>,
    lhs: Ptr<'d, DLinExpr<'d>>,
    rhs: Ptr<'d, DLinExpr<'d>>,
    typ: IneqTyp,
}

impl<'d> DCon<'d> {
    pub(crate) fn set_idxs(&mut self, idxs: IdxVec<'d>) {
        self.idxs = idxs;
    }

    // expand

    fn expand_d1(&self, op: impl FnMut(&Self)) {
        expand_d1(&self.idxs, self, op);
    }

    fn expand_d2(&self, op: impl FnMut(&Self)) {
        expand_d2(&self.idxs, self, op);
    }
}

#[cfg(test)]
pub mod ext_test {
    use super::*;
    use crate::Dim;
    use alloc::string::String;
    use core::fmt::Write;

    impl<'d> DCon<'d> {
        pub fn eval_append_string(&self, f: &mut String) {
            let mut op = |s: &Self| {
                let written = s.lhs.get().eval_append_string(f, false);
                if !written {
                    write!(f, "0").unwrap();
                }

                match s.typ {
                    IneqTyp::Leq => write!(f, " <= "),
                    IneqTyp::Geq => write!(f, " >= "),
                    IneqTyp::Eq => write!(f, " = "),
                }
                .unwrap();

                let written = s.rhs.get().eval_append_string(f, false);
                if !written {
                    write!(f, "0").unwrap();
                }
            };

            match self.idxs.dim() {
                Dim::D0 => op(self),
                Dim::D1 => self.expand_d1(op),
                Dim::D2 => self.expand_d2(op),
            }
        }

        pub fn eval_to_string(&self) -> String {
            let mut f = String::new();
            self.eval_append_string(&mut f);
            f
        }
    }
}
