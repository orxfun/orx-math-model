use crate::indices::{CatIdx, OrdIdx};

pub enum ParIdx<'m, 'd> {
    CatIdx(CatIdx<'m, 'd>),
    OrdIdx(OrdIdx<'m, 'd>),
}
