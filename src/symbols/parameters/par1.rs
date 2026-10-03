use crate::symbols::{sets::CatSet, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct Par1<'a> {
    s: Symbol<'a>,
    i: CatSet<'a>,
}

#[derive(derive_new::new, Clone, Copy)]
pub struct IntPar1<'a> {
    s: Symbol<'a>,
    i: CatSet<'a>,
}
