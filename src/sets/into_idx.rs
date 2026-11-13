use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};
use alloc::string::String;

// cint

pub enum IndexCatInt<'m, 'd> {
    Idx(IdxCatInt<'m, 'd>),
    ConUsize(usize),
}

// cstr

pub enum IndexCatStr<'m, 'd> {
    Idx(IdxCatStr<'m, 'd>),
    ConString(String),
    ConStringRef(&'d String),
    ConStr(&'d str),
}

// ord

pub enum IndexOrd<'m, 'd> {
    Idx(IdxOrd<'m, 'd>),
    ConUsize(usize),
    ConI64(i64),
}
