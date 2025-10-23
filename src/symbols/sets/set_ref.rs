use crate::{model::Model, symbols::SetData};

pub struct Set<'m> {
    model: &'m Model,
    data: &'m SetData,
}

impl<'m> Set<'m> {
    pub fn new(model: &'m Model, data: &'m SetData) -> Self {
        Self { model, data }
    }
}
