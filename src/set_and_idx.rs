use crate::symbols::{ElIdx, ElStr, Idx, Indices, SetIdx, SetStr};

pub trait SetAndIdx {
    type Idx;

    type Key;
}

// set-idx

impl<'m, 'd> SetAndIdx for SetIdx<'m, 'd> {
    type Idx = ElIdx<'m, 'd>;

    type Key = usize;
}

// set-str

impl<'m, 'd> SetAndIdx for SetStr<'m, 'd> {
    type Idx = ElStr<'m, 'd>;

    type Key = &'d str;
}

// indices

impl<'m, 'd> SetAndIdx for Indices<'m, 'd> {
    type Idx = Idx<'m, 'd>;

    type Key = usize;
}
