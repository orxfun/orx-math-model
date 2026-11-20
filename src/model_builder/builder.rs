use crate::model_builder::{Dim, D0};
use crate::pars::{IntParD0, NumParD0};
use crate::vars::VarD0;
use crate::{CatInt, CatStr, Model, Ord, VarType};
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

    pub fn build<'m, 'd: 'm, F>(self, m: &'m Model<'d>, f: F)
    where
        F: Fn(
            Sci::Arr<CatInt<'m, 'd>>,
            Scs::Arr<CatStr<'m, 'd>>,
            So::Arr<Ord<'m, 'd>>,
            P::Arr<NumParD0<'m, 'd>>,
            Pi::Arr<IntParD0<'m, 'd>>,
            V::Arr<VarD0<'m, 'd>>,
        ),
    {
        let sci = Sci::map(self.set_cat_int, |_key| m.cat_set_int());
        let scs = Scs::map(self.set_cat_str, |_key| m.cat_set_str());
        let so = So::map(self.set_ord, |_key| m.ord_set());
        let p = P::map(self.par0, |_key| m.par0_num());
        let pi = Pi::map(self.pari0, |_key| m.par0_int());
        let v = V::map(self.var0, |_key| m.var0(VarType::Binary));

        f(sci, scs, so, p, pi, v);
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;
    use crate::*;

    #[test]
    fn abc() {
        let d_org = vec!["x".to_string(), "y".to_string()];
        let d_dst = vec!["a".to_string()];
        let m = Model::new();

        ModelBuilder::default()
            .cat_set_str("org")
            .cat_set_str("dst")
            .ord_set("t")
            .par("cost")
            .int_par("count")
            .var("total-cost")
            .build(&m, |[], [org, dst], [t], [cost], [count], [z]| {
                let x = m.var2(VarType::Binary, org, dst);
                let y = m.var1(VarType::Binary, org);

                org.bind(|| &d_org).unwrap();
                dst.bind(|| &d_dst).unwrap();

                let c = con1(org, |o| (sum1(dst, |d| x[(o, d)]), leq, y[o]));
                let s = c.data().eval_to_string();
                assert_eq!(s, "2");
            });
    }
}
