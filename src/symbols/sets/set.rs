use crate::symbols::Symbol;

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'a> {
    s: Symbol<'a>,
}
