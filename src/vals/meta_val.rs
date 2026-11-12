// enums

pub enum ValTyp {
    Int,
    Num,
}

#[derive(Debug, PartialEq)]
pub enum ValVal {
    Int(i64),
    Num(f64),
}

// trait

pub trait MetaVal: Clone + Copy {
    type Val: Default + Clone + Copy;

    fn val_typ() -> ValTyp;

    fn val_as_enum(idx: Self::Val) -> ValVal;
}

// int

#[derive(Clone, Copy)]
pub struct MetaInt;

impl MetaVal for MetaInt {
    type Val = i64;

    fn val_typ() -> ValTyp {
        ValTyp::Int
    }

    fn val_as_enum(idx: Self::Val) -> ValVal {
        ValVal::Int(idx)
    }
}

// num

#[derive(Clone, Copy)]
pub struct MetaNum;

impl MetaVal for MetaNum {
    type Val = f64;

    fn val_typ() -> ValTyp {
        ValTyp::Num
    }

    fn val_as_enum(idx: Self::Val) -> ValVal {
        ValVal::Num(idx)
    }
}
