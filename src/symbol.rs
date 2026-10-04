use crate::model::Model;

#[derive(derive_new::new)]
pub struct Symbol<'a, D> {
    pub m: &'a Model<'a>,
    pub d: &'a D,
}

impl<'a, D> Clone for Symbol<'a, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, D> Copy for Symbol<'a, D> {}

impl<'a, D> Symbol<'a, D> {
    pub fn destruct(self) -> (&'a Model<'a>, &'a D) {
        (self.m, self.d)
    }
}
