use crate::symbols::{sets::Set, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct Par1<'a> {
    s: Symbol<'a>,
    i: Set<'a>,
}

#[derive(derive_new::new, Clone, Copy)]
pub struct IntPar1<'a> {
    s: Symbol<'a>,
    i: Set<'a>,
}
