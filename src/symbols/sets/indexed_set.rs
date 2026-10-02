use crate::symbols::{parameters::Count, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct IndexedSet<'a> {
    s: Symbol<'a>,
    count: Count<'a>,
}
