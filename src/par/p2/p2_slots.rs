use crate::{new_bag, Bag, Elements, MetaInt, MetaSet, MetaVal, Ptr, Res, SetTyp, Slot};
use crate::{vals::MetaNum, ValTyp};
use alloc::{boxed::Box, string::String};
use core::mem::transmute;

pub enum P2Fun<'d, V: MetaVal> {
    CatSetInt(Box<dyn Fn(usize) -> V::Val + 'd>),
    CatSetStr(Box<dyn Fn(&'d String) -> V::Val + 'd>),
    OrdSet(Box<dyn Fn(i64) -> V::Val + 'd>),
}
