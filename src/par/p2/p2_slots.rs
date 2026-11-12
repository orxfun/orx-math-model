use crate::{
    new_bag, Bag, Elements, MetaCatInt, MetaCatStr, MetaInt, MetaNum, MetaOrd, MetaSet, MetaVal,
    Ptr, Res, SetTyp, Slot, ValTyp,
};
use alloc::{boxed::Box, string::String};
use core::mem::transmute;

#[derive(derive_more::Deref)]
pub struct P2Fun<'d, V: MetaVal, I0: MetaSet, I1: MetaSet>(
    Box<dyn Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd>,
);

impl<'d, V: MetaVal, I0: MetaSet, I1: MetaSet> Default for P2Fun<'d, V, I0, I1> {
    fn default() -> Self {
        Self(Box::new(|_, _| Default::default()))
    }
}

pub enum P2FunE<'d, V: MetaVal> {
    CatSetInt(P2Fun<'d, V, MetaCatInt, MetaCatInt>),
    CatSetStr(P2Fun<'d, V, MetaCatInt, MetaCatStr>),
    OrdSet(P2Fun<'d, V, MetaCatInt, MetaOrd>),
}
