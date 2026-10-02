use crate::symbols::Symbol;

#[derive(derive_new::new, Clone, Copy)]
pub struct RangeSet<'a> {
    s: Symbol<'a>,
}
