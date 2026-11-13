use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};
use alloc::string::String;

// cint

#[derive(derive_more::From)]
pub enum IndexCatInt<'m, 'd> {
    Idx(IdxCatInt<'m, 'd>),
    ConUsize(usize),
}

// cstr

#[derive(derive_more::From)]
pub enum IndexCatStr<'m, 'd> {
    Idx(IdxCatStr<'m, 'd>),
    ConString(String),
    ConStringRef(&'d String),
    ConStr(&'d str),
}

// ord

#[derive(derive_more::From)]
pub enum IndexOrd<'m, 'd> {
    Idx(IdxOrd<'m, 'd>),
    ConUsize(usize),
    ConI64(i64),
}
