use crate::model::Model;

#[derive(derive_new::new)]
pub struct Symbol<'a, D> {
    pub(crate) m: &'a Model,
    pub(crate) d: &'a D,
}

impl<'a, D> Clone for Symbol<'a, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, D> Copy for Symbol<'a, D> {}
