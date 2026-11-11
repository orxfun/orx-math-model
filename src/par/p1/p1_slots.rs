#[cfg(test)]
use crate::idxs::IdxVal;
use crate::vals::MetaNum;
#[cfg(test)]
use crate::vals::ValVal;
use crate::{new_bag, vals::MetaVal, Bag, Elements, Res, Slot};
use crate::{
    sets::{MetaSet, SetType},
    vals::MetaInt,
};
use alloc::{boxed::Box, string::String};

pub enum P1Fun<'d, V: MetaVal> {
    CatSetInt(Box<dyn Fn(usize) -> V::Val + 'd>),
    CatSetStr(Box<dyn Fn(&'d String) -> V::Val + 'd>),
    OrdSet(Box<dyn Fn(i64) -> V::Val + 'd>),
}

impl<'d, T: MetaVal, I0: MetaSet, F: Fn(I0::Idx<'d>) -> T + 'd> From<(I0, F)> for P1Fun<'d, T> {
    fn from(value: (I0, F)) -> Self {
        todo!()
    }
}

pub enum P1Slot<'d, V: MetaVal> {
    CatSetInt(Slot<'d, Box<dyn Fn(usize) -> V::Val + 'd>>),
    CatSetStr(Slot<'d, Box<dyn Fn(&'d String) -> V::Val + 'd>>),
    OrdSet(Slot<'d, Box<dyn Fn(i64) -> V::Val + 'd>>),
}

impl<'d, V: MetaVal> P1Slot<'d, V> {
    pub fn bind(&self, fun: P1Fun<'d, V>) -> Res {
        match (self, fun) {
            (Self::CatSetInt(s), P1Fun::CatSetInt(f)) => s.bind(f),
            (Self::CatSetStr(s), P1Fun::CatSetStr(f)) => s.bind(f),
            (Self::OrdSet(s), P1Fun::OrdSet(f)) => s.bind(f),
            _ => unreachable!("TODO: explain why"),
        }
    }

    #[cfg(test)]
    fn test_at<M: MetaSet>(&self, idx: M::Idx<'d>) -> Res<ValVal> {
        match (self, M::idx_as_enum(idx)) {
            (Self::CatSetInt(f), IdxVal::CatIdxInt(i)) => Ok(V::val_as_enum(f.get()(i))),
            (Self::CatSetStr(f), IdxVal::CatIdxStr(i)) => Ok(V::val_as_enum(f.get()(i))),
            (Self::OrdSet(f), IdxVal::OrdIdx(i)) => Ok(V::val_as_enum(f.get()(i))),
            _ => Err(String::from("mistmatch in P1 index and argument")),
        }
    }
}

// collection of

pub struct P1SlotsOf<'d, V: MetaVal> {
    cat_idx_int: Bag<Box<dyn Fn(usize) -> V::Val + 'd>>,
    cat_idx_str: Bag<Box<dyn Fn(&'d String) -> V::Val + 'd>>,
    ord_idx: Bag<Box<dyn Fn(i64) -> V::Val + 'd>>,
}

impl<'d, T: MetaVal> P1SlotsOf<'d, T> {
    fn push(&self, s0_type: SetType) -> P1Slot<'d, T> {
        match s0_type {
            SetType::CatSetInt => {
                let el = self.cat_idx_int.push(Box::new(|_| Default::default()));
                P1Slot::CatSetInt(el.into())
            }
            SetType::CatSetStr => {
                let el = self.cat_idx_str.push(Box::new(|_| Default::default()));
                P1Slot::CatSetStr(el.into())
            }
            SetType::OrdSet => {
                let el = self.ord_idx.push(Box::new(|_| Default::default()));
                P1Slot::OrdSet(el.into())
            }
        }
    }
}

// collection

pub struct P1Slots<'d> {
    int: P1SlotsOf<'d, MetaInt>,
    num: P1SlotsOf<'d, MetaNum>,
}

impl<'d> P1Slots<'d> {
    pub fn push_int(&self, s0_type: SetType) -> P1Slot<'d, MetaInt> {
        self.int.push(s0_type)
    }

    pub fn push_num(&self, s0_type: SetType) -> P1Slot<'d, MetaNum> {
        self.num.push(s0_type)
    }
}
