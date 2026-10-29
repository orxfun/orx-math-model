use crate::symbols::{ElIdx, ElStr, Idx, Indices, Int, SetIdx, SetStr};

pub trait SetAndIdx {
    type Idx;

    type Key;
}

// set-idx

impl<'m, 'd> SetAndIdx for SetIdx<'m, 'd> {
    type Idx = ElIdx<'m, 'd>;

    type Key = usize;
}

pub enum IntoElIdx<'m, 'd> {
    El(ElIdx<'m, 'd>),
    Const(usize),
}

// set-str

impl<'m, 'd> SetAndIdx for SetStr<'m, 'd> {
    type Idx = ElStr<'m, 'd>;

    type Key = &'d str;
}

pub enum IntoElStr<'m, 'd> {
    El(ElStr<'m, 'd>),
    Const(&'d str),
}

// indices

impl<'m, 'd> SetAndIdx for Indices<'m, 'd> {
    type Idx = Idx<'m, 'd>;

    type Key = usize;
}

pub enum IntoIdx<'m, 'd> {
    Int(Int<'m, 'd>),
    Const(usize),
}
