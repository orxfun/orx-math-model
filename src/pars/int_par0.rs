use crate::symbol::Symbol;

#[derive(derive_new::new, Clone, Copy)]
pub struct IntPar0<'a>(pub(crate) Symbol<'a, IntPar0Data>);

#[derive(derive_new::new)]
pub struct IntPar0Data {
    // resolved
    pub value: i64,
}
