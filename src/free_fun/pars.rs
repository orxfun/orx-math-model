use crate::scalars::{Val, ValInt, ValNum};
use crate::Model;
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
        BParD0(self.0, m)
    }
}
