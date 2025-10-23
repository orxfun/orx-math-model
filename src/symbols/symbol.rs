use crate::model::Model;

pub struct Symbol<'m, Data> {
    model: &'m Model,
    data: &'m Data,
}

impl<'m, Data> Clone for Symbol<'m, Data> {
    fn clone(&self) -> Self {
        Self {
            model: self.model,
            data: self.data,
        }
    }
}

impl<'m, Data> Copy for Symbol<'m, Data> {}

impl<'m, Data> Symbol<'m, Data> {
    pub fn new(model: &'m Model, data: &'m Data) -> Self {
        Self { model, data }
    }
}
