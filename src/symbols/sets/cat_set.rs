use crate::symbols::Symbol;

#[derive(derive_new::new, Clone, Copy)]
pub struct CatSet<'a> {
    s: Symbol<'a>,
}

impl<'a> CatSet<'a> {
    pub(crate) fn symbol(self) -> Symbol<'a> {
        self.s
    }
}
