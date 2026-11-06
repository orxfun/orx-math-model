use crate::indices::{CatIdx, OrdIdx};

// symbol

pub enum Idx<'m, 'd> {
    CatInt(CatIdx<'m, 'd, usize>),
    CatStr(CatIdx<'m, 'd, &'d str>),
    Ord(OrdIdx<'m, 'd>),
}
