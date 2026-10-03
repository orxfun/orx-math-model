use crate::symbols::Symbol;

#[derive(derive_new::new, Clone, Copy)]
pub struct IntPar0<'a> {
    s: Symbol<'a>,
}
