use crate::symbols::Symbol;

#[derive(derive_new::new, Clone, Copy)]
pub struct Constant<'a> {
    s: Symbol<'a>,
}
