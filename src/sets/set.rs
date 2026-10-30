use crate::sets::{CatSet, OrdSet};

// symbol

pub enum Set<'m, 'd> {
    CatInt(CatSet<'m, 'd, usize>),
    CatStr(CatSet<'m, 'd, &'d str>),
    Ord(OrdSet<'m, 'd>),
}
