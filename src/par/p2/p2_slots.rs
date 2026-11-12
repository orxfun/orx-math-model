use crate::{new_bag, Bag, Elements, MetaInt, MetaSet, MetaVal, Ptr, Res, SetTyp, Slot};
use crate::{vals::MetaNum, ValTyp};
use alloc::{boxed::Box, string::String};
use core::mem::transmute;

#[derive(derive_more::Deref)]
pub struct P2Fun<'d, I0: MetaSet, I1: MetaSet, V: MetaVal>(
    Box<dyn Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd>,
);

impl<'d, I0: MetaSet, I1: MetaSet, V: MetaVal> Default for P2Fun<'d, I0, I1, V> {
    fn default() -> Self {
        Self(Box::new(|_, _| Default::default()))
    }
}
