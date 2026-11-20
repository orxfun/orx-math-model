use crate::{Model, Ord};
use alloc::string::{String, ToString};

pub fn set<'m, 'd>(m: &'m Model<'d>, key: impl ToString) -> BSet<'m, 'd> {
    BSet {
        m,
        key: key.to_string(),
    }
}

pub struct BSet<'m, 'd> {
    m: &'m Model<'d>,
    key: String,
}

impl<'m, 'd> BSet<'m, 'd> {
    pub fn cat(self) {}

    pub fn ord(self) -> Ord<'m, 'd> {
        // TODO: add key too
        self.m.ord_set()
    }
}
