use crate::model_builder::{Dim, D0};
use crate::{CatInt, CatStr, Model, Ord};
use alloc::string::{String, ToString};

pub struct ModelBuilder<Sci: Dim, Scs: Dim, So: Dim, P: Dim, Pi: Dim, V: Dim> {
    set_cat_int: Sci::Arr<String>,
    set_cat_str: Scs::Arr<String>,
    set_ord: So::Arr<String>,
    par0: P::Arr<String>,
    pari0: Pi::Arr<String>,
    var0: V::Arr<String>,
}

impl Default for ModelBuilder<D0, D0, D0, D0, D0, D0> {
    fn default() -> Self {
        Self {
            set_cat_int: [],
            set_cat_str: [],
            set_ord: [],
            par0: [],
            pari0: [],
            var0: [],
        }
    }
}

impl<Sci: Dim, Scs: Dim, So: Dim, P: Dim, Pi: Dim, V: Dim> ModelBuilder<Sci, Scs, So, P, Pi, V> {
    pub fn cat_set_int(self, key: impl ToString) -> ModelBuilder<Sci::Next, Scs, So, P, Pi, V> {
        ModelBuilder {
            set_cat_int: Sci::arr_push(self.set_cat_int, key.to_string()),
            set_cat_str: self.set_cat_str,
            set_ord: self.set_ord,
            par0: self.par0,
            pari0: self.pari0,
            var0: self.var0,
        }
    }

    pub fn cat_set_str(self, key: impl ToString) -> ModelBuilder<Sci, Scs::Next, So, P, Pi, V> {
        ModelBuilder {
            set_cat_int: self.set_cat_int,
            set_cat_str: Scs::arr_push(self.set_cat_str, key.to_string()),
            set_ord: self.set_ord,
            par0: self.par0,
            pari0: self.pari0,
            var0: self.var0,
        }
    }

    pub fn ord_set(self, key: impl ToString) -> ModelBuilder<Sci, Scs, So::Next, P, Pi, V> {
        ModelBuilder {
            set_cat_int: self.set_cat_int,
            set_cat_str: self.set_cat_str,
            set_ord: So::arr_push(self.set_ord, key.to_string()),
            par0: self.par0,
            pari0: self.pari0,
            var0: self.var0,
        }
    }

    pub fn par(self, key: impl ToString) -> ModelBuilder<Sci, Scs, So, P::Next, Pi, V> {
        ModelBuilder {
            set_cat_int: self.set_cat_int,
            set_cat_str: self.set_cat_str,
            set_ord: self.set_ord,
            par0: P::arr_push(self.par0, key.to_string()),
            pari0: self.pari0,
            var0: self.var0,
        }
    }

    pub fn int_par(self, key: impl ToString) -> ModelBuilder<Sci, Scs, So, P, Pi::Next, V> {
        ModelBuilder {
            set_cat_int: self.set_cat_int,
            set_cat_str: self.set_cat_str,
            set_ord: self.set_ord,
            par0: self.par0,
            pari0: Pi::arr_push(self.pari0, key.to_string()),
            var0: self.var0,
        }
    }

    pub fn var(self, key: impl ToString) -> ModelBuilder<Sci, Scs, So, P, Pi, V::Next> {
        ModelBuilder {
            set_cat_int: self.set_cat_int,
            set_cat_str: self.set_cat_str,
            set_ord: self.set_ord,
            par0: self.par0,
            pari0: self.pari0,
            var0: V::arr_push(self.var0, key.to_string()),
        }
    }

    pub fn build<'m, 'd: 'm, F>(f: F) -> Model<'d>
    where
        F: Fn(Sci::Arr<CatInt<'m, 'd>>, Sci::Arr<CatStr<'m, 'd>>, Sci::Arr<Ord<'m, 'd>>),
    {
        let m = Model::new();

        m
    }
}
