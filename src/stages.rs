pub trait Stage: Default + Clone + Copy + PartialEq + Eq + 'static {
    type SetData;
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Modeling;
impl Stage for Modeling {
    type SetData = ();
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ModelWithData;
impl Stage for ModelWithData {
    type SetData = ();
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Solved;
impl Stage for Solved {
    type SetData = ();
}
