use crate::{CatInt, CatStr, Model, Ord};
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
    pub fn cat(self) -> BSetCat<'m, 'd> {
        BSetCat(self)
    }

    pub fn ord(self) -> Ord<'m, 'd> {
        self.m.ord_set(self.key)
    }
}

pub struct BSetCat<'m, 'd>(BSet<'m, 'd>);

impl<'m, 'd> BSetCat<'m, 'd> {
    pub fn int(self) -> CatInt<'m, 'd> {
        self.0.m.cat_set_int(self.0.key)
    }

    pub fn str(self) -> CatStr<'m, 'd> {
        self.0.m.cat_set_str(self.0.key)
    }
}
