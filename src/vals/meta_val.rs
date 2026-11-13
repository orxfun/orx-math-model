// enums

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValTyp {
    Int,
    Num,
}

// meta

pub trait Val: Clone + Copy + 'static {
    type Raw: Clone + Copy;

    fn typ() -> ValTyp;
}

// int

#[derive(Clone, Copy)]
pub struct Int;

impl Val for Int {
    type Raw = i64;

    fn typ() -> ValTyp {
        ValTyp::Int
    }
}

// num

#[derive(Clone, Copy)]
pub struct Num;

impl Val for Num {
    type Raw = f64;

    fn typ() -> ValTyp {
        ValTyp::Num
    }
}
