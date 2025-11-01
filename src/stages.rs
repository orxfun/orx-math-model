pub trait Stage: Default + Clone + Copy + PartialEq + Eq + 'static {}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Modeling;
impl Stage for Modeling {}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ModelWithData;
impl Stage for ModelWithData {}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Solved;
impl Stage for Solved {}
