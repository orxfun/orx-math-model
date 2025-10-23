use crate::model_data::ModelData;

#[derive(Default)]
pub struct Model {
    data: ModelData,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }
}
