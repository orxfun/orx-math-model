use crate::vals::ValVal;

pub trait MetaVal {
    type Val: Default + Clone + Copy;

    fn val_as_enum(idx: Self::Val) -> ValVal;
}

// int

pub struct MetaInt;

impl MetaVal for MetaInt {
    type Val = i64;

    fn val_as_enum(idx: Self::Val) -> ValVal {
        ValVal::Int(idx)
    }
}

// num

pub struct MetaNum;

impl MetaVal for MetaNum {
    type Val = f64;

    fn val_as_enum(idx: Self::Val) -> ValVal {
        ValVal::Num(idx)
    }
}
