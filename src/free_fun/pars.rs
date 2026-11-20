use crate::scalars::{Val, ValInt, ValNum};
use crate::{Model, Set, SetSym, SymWrap};
use alloc::string::{String, ToString};
use core::marker::PhantomData;

pub fn par(key: impl ToString) -> BPar<ValNum> {
    BPar(key.to_string(), PhantomData)
}

pub fn par_int(key: impl ToString) -> BPar<ValInt> {
    BPar(key.to_string(), PhantomData)
}

pub struct BPar<V: Val = ValNum>(String, PhantomData<V>);

impl<V: Val> BPar<V> {
    pub fn d0<'m, 'd>(self, m: &'m Model<'d>) -> V::ParD0<'m, 'd> {
        V::par_d0(m, self.0)
    }

    pub fn d1<'m, 'd, I0: SetSym<'m, 'd>>(self, s0: I0) -> V::ParD1<'m, 'd, I0::Meta> {
        V::par_d1(s0.m(), self.0, s0)
    }

    pub fn d2<'m, 'd, I0: SetSym<'m, 'd>, I1: SetSym<'m, 'd>>(
        self,
        s0: I0,
        s1: I1,
    ) -> V::ParD2<'m, 'd, I0::Meta, I1::Meta> {
        V::par_d2(s0.m(), self.0, s0, s1)
    }
}
