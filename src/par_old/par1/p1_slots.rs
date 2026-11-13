use crate::{
    new_bag, Bag, Elements, MetaCatInt, MetaCatStr, MetaInt, MetaNum, MetaOrd, MetaSet, MetaVal,
    Model, Ptr, Res, SetTyp, Slot, ValTyp,
};
use alloc::{boxed::Box, string::String};
use core::mem::transmute;

// 0

pub struct P1Fun0<'d, V: MetaVal, I0: MetaSet>(Box<dyn Fn(I0::IdxRaw<'d>) -> V::Val + 'd>);

pub struct P1Slot0<'d, V: MetaVal, I0: MetaSet>(Slot<'d, P1Fun0<'d, V, I0>>);

// 1

pub enum P1Fun1<'d, V: MetaVal> {
    CatInt(P1Fun0<'d, V, MetaCatInt>),
    CatStr(P1Fun0<'d, V, MetaCatStr>),
    Ord(P1Fun0<'d, V, MetaOrd>),
}

impl<'d, V: MetaVal> P1Fun1<'d, V> {
    fn bind(&mut self) {
        //
    }
}
