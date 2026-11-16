use crate::{Num, Var};

pub struct LinTerm<'m, 'd> {
    coef: Num<'m, 'd>,
    var: Var<'m, 'd>,
}
