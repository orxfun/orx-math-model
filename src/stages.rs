pub trait Stage: Default + Clone + Copy + 'static {}

#[derive(Default, Clone, Copy)]
pub struct Modeling;
impl Stage for Modeling {}

#[derive(Default, Clone, Copy)]
pub struct ModelWithData;
impl Stage for ModelWithData {}

#[derive(Default, Clone, Copy)]
pub struct Solved;
impl Stage for Solved {}
