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
    // expand - d1

    fn expand_d1(&self, op: impl FnMut(&Self)) {
        match self.idxs.set_typ(0) {
            SetTyp::CatInt => self.expand_d1_i0::<SetCatInt>(op),
            SetTyp::CatStr => self.expand_d1_i0::<SetCatStr>(op),
            SetTyp::Ord => self.expand_d1_i0::<SetOrd>(op),
        }
    }

    fn expand_d1_i0<I0: Set>(&self, mut op: impl FnMut(&Self)) {
        assert_eq!(self.idxs.len(), 1);
        let mut elem0: Elements<<I0 as Set>::IdxRaw<'d>> = Vec::new().into();

        let i0 = self.idxs[0];

        let i0 = ElementsIter::<I0>::new(i0, &mut elem0);
        for _ in i0 {
            op(self);
        }
    }
}

#[cfg(test)]
pub mod ext_test {
    use super::*;
    use crate::constants::{is_neg, is_pos, is_zero};
    use alloc::string::String;
    use core::fmt::Write;

    impl<'d> DCon<'d> {
        pub fn eval_append_string(&self, f: &mut String, pre_exists: bool) -> bool {
            // let (mut pre_exists, c) = self.eval_append_string_exc_constant(f, pre_exists);

            // if let Some(c) = c {
            //     pre_exists |= write_constant(f, pre_exists, c);
            // }

            pre_exists
        }

        pub fn eval_to_string(&self) -> String {
            let mut f = String::new();
            self.eval_append_string(&mut f, false);
            f
        }
    }
}
