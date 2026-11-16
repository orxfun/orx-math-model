use crate::{Num, Var};

#[derive(derive_new::new, Clone, Copy)]
pub struct LinTerm<'m, 'd> {
    coef: Num<'m, 'd>,
    var: Var<'m, 'd>,
}
