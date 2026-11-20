use crate::model_builder::{Dim, D0};
use alloc::string::{String, ToString};

pub struct ModelBuilder<Sci: Dim, Scs: Dim, So: Dim, P: Dim, V: Dim> {
    set_cat_int: Sci::Arr<String>,
    set_cat_str: Scs::Arr<String>,
    set_ord: So::Arr<String>,
    par0: P::Arr<String>,
    var0: V::Arr<String>,
}

impl Default for ModelBuilder<D0, D0, D0, D0, D0> {
    fn default() -> Self {
        Self {
            set_cat_int: [],
            set_cat_str: [],
            set_ord: [],
            par0: [],
            var0: [],
        }
    }
}

impl<Sci: Dim, Scs: Dim, So: Dim, P: Dim, V: Dim> ModelBuilder<Sci, Scs, So, P, V> {
    pub fn cat_set_int(self, key: impl ToString) -> ModelBuilder<Sci::Next, Scs, So, P, V> {
        ModelBuilder {
            set_cat_int: Sci::arr_push(self.set_cat_int, key.to_string()),
            set_cat_str: self.set_cat_str,
            set_ord: self.set_ord,
            par0: self.par0,
            var0: self.var0,
        }
    }

    pub fn cat_set_str(self, key: impl ToString) -> ModelBuilder<Sci, Scs::Next, So, P, V> {
        ModelBuilder {
            set_cat_int: self.set_cat_int,
            set_cat_str: Scs::arr_push(self.set_cat_str, key.to_string()),
            set_ord: self.set_ord,
            par0: self.par0,
            var0: self.var0,
        }
    }

    pub fn ord_set(self, key: impl ToString) -> ModelBuilder<Sci, Scs, So::Next, P, V> {
        ModelBuilder {
            set_cat_int: self.set_cat_int,
            set_cat_str: self.set_cat_str,
            set_ord: So::arr_push(self.set_ord, key.to_string()),
            par0: self.par0,
            var0: self.var0,
        }
    }

    pub fn par(self, key: impl ToString) -> ModelBuilder<Sci, Scs, So, P::Next, V> {
        ModelBuilder {
            set_cat_int: self.set_cat_int,
            set_cat_str: self.set_cat_str,
            set_ord: self.set_ord,
            par0: P::arr_push(self.par0, key.to_string()),
            var0: self.var0,
        }
    }

    pub fn var(self, key: impl ToString) -> ModelBuilder<Sci, Scs, So, P, V::Next> {
        ModelBuilder {
            set_cat_int: self.set_cat_int,
            set_cat_str: self.set_cat_str,
            set_ord: self.set_ord,
            par0: self.par0,
            var0: V::arr_push(self.var0, key.to_string()),
        }
    }
}
