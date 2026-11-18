#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dim {
    D0,
    D1,
    D2,
}

// trait

pub trait D: Clone + Copy + 'static {
    fn dim() -> Dim;
}

#[derive(Clone, Copy)]
pub struct D0;

impl D for D0 {
    fn dim() -> Dim {
        Dim::D0
    }
}

#[derive(Clone, Copy)]
pub struct D1;

impl D for D1 {
    fn dim() -> Dim {
        Dim::D1
    }
}

#[derive(Clone, Copy)]
pub struct D2;

impl D for D2 {
    fn dim() -> Dim {
        Dim::D2
    }
}
