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
pub struct ValInt;

impl Val for ValInt {
    type Raw = i64;

    fn typ() -> ValTyp {
        ValTyp::Int
    }
}

// num

#[derive(Clone, Copy)]
pub struct ValNum;

impl Val for ValNum {
    type Raw = f64;

    fn typ() -> ValTyp {
        ValTyp::Num
    }
}
