use crate::model_builder::{Dim, D0};
use alloc::string::{String, ToString};

pub struct ModelBuilder<Sci: Dim, Scs: Dim, So: Dim> {
    set_cat_int: Sci::Arr<String>,
    set_cat_str: Scs::Arr<String>,
    set_ord: So::Arr<String>,
}

impl Default for ModelBuilder<D0, D0, D0> {
    fn default() -> Self {
        Self {
            set_cat_int: [],
            set_cat_str: [],
            set_ord: [],
        }
    }
}

impl<Sci: Dim, Scs: Dim, So: Dim> ModelBuilder<Sci, Scs, So> {
    pub fn cat_sets<const N: usize>(self, sets: [&str; N]) -> Self {
        self
    }

    pub fn cat_set_int(self, key: impl ToString) -> ModelBuilder<Sci::Next, Scs, So> {
        todo!()
    }
}
