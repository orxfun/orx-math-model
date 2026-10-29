use crate::ptr::ValPtr;
use crate::symbols::{ElIdx, ElStr, Idx, Indices, Int, IntFun, SetIdx, SetStr};
use alloc::boxed::Box;

pub trait SetAndIdx {
    type Idx;

    type Key;
}

// set-idx

impl<'m, 'd> SetAndIdx for SetIdx<'m, 'd> {
    type Idx = ElIdx<'m, 'd>;

    type Key = usize;
}

pub enum AsSetIdxIdx<'m, 'd> {
    El(ElIdx<'m, 'd>),
    Const(usize),
}

impl<'m, 'd> From<ElIdx<'m, 'd>> for AsSetIdxIdx<'m, 'd> {
    fn from(value: ElIdx<'m, 'd>) -> Self {
        Self::El(value)
    }
}

impl<'m, 'd> From<usize> for AsSetIdxIdx<'m, 'd> {
    fn from(value: usize) -> Self {
        Self::Const(value)
    }
}

// set-str

impl<'m, 'd> SetAndIdx for SetStr<'m, 'd> {
    type Idx = ElStr<'m, 'd>;

    type Key = &'d str;
}

pub enum AsSetStrIdx<'m, 'd> {
    El(ElStr<'m, 'd>),
    Const(&'d str),
}

impl<'m, 'd> From<ElStr<'m, 'd>> for AsSetStrIdx<'m, 'd> {
    fn from(value: ElStr<'m, 'd>) -> Self {
        Self::El(value)
    }
}

impl<'m, 'd> From<&'d str> for AsSetStrIdx<'m, 'd> {
    fn from(value: &'d str) -> Self {
        Self::Const(value)
    }
}

// indices

impl<'m, 'd> SetAndIdx for Indices<'m, 'd> {
    type Idx = Idx<'m, 'd>;

    type Key = usize;
}

pub enum AsIdxIdx<'m, 'd> {
    Int(Int<'m, 'd>),
    Const(usize),
}

impl<'m, 'd> From<Int<'m, 'd>> for AsIdxIdx<'m, 'd> {
    fn from(value: Int<'m, 'd>) -> Self {
        Self::Int(value)
    }
}

impl<'m, 'd> From<Idx<'m, 'd>> for AsIdxIdx<'m, 'd> {
    fn from(value: Idx<'m, 'd>) -> Self {
        let p = unsafe { ValPtr::new(value.val_ptr()) };
        let fun = IntFun::new(Box::new(move || p.val()));
        Self::Int(value.m.int(fun))
    }
}

impl<'m, 'd> From<usize> for AsIdxIdx<'m, 'd> {
    fn from(value: usize) -> Self {
        Self::Const(value)
    }
}
