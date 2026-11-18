use crate::lin_expr::DLinExpr;
use crate::{impl_sym_wrap, SymWrap, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct LinExpr<'m, 'd>(Symbol<'m, 'd, DLinExpr<'d>>);
impl_sym_wrap!(LinExpr<'m, 'd>, DLinExpr<'d>);

impl<'m, 'd> LinExpr<'m, 'd> {
    // ops

    pub(crate) fn neg(self) -> Self {
        self.m().lin_exp(self.data().neg(self.m()))
    }
}
