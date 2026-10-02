use crate::symbols::Symbol;

#[derive(derive_new::new, Clone, Copy)]
pub struct Count<'a> {
    s: Symbol<'a>,
}
