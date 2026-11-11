#[cfg(test)]
use crate::idxs::{Idx, IdxVal};
#[cfg(test)]
use crate::vals::ValVal;
use crate::{new_bag, vals::Val, Bag, Elements, Res, Slot};
use alloc::{boxed::Box, string::String};

pub enum P1Fun<'d, T: Val> {
    CatSetInt(Box<dyn Fn(usize) -> T + 'd>),
    CatSetStr(Box<dyn Fn(&'d String) -> T + 'd>),
    OrdSet(Box<dyn Fn(i64) -> T + 'd>),
}

pub enum P1Slot<'d, T: Val> {
    CatSetInt(Slot<'d, Box<dyn Fn(usize) -> T + 'd>>),
    CatSetStr(Slot<'d, Box<dyn Fn(&'d String) -> T + 'd>>),
    OrdSet(Slot<'d, Box<dyn Fn(i64) -> T + 'd>>),
}

impl<'d, T: Val> P1Slot<'d, T> {
    pub fn bind(&self, fun: P1Fun<'d, T>) -> Res {
        match (self, fun) {
            (Self::CatSetInt(s), P1Fun::CatSetInt(f)) => s.bind(f),
            (Self::CatSetStr(s), P1Fun::CatSetStr(f)) => s.bind(f),
            (Self::OrdSet(s), P1Fun::OrdSet(f)) => s.bind(f),
            _ => unreachable!("TODO: explain why"),
        }
    }

    #[cfg(test)]
    fn test_at(&self, idx: impl Idx<'d>) -> Res<ValVal> {
        match (self, idx.as_idx_val()) {
            (Self::CatSetInt(f), IdxVal::CatIdxInt(i)) => Ok(f.get()(i).as_val_val()),
            (Self::CatSetStr(f), IdxVal::CatIdxStr(i)) => Ok(f.get()(i).as_val_val()),
            (Self::OrdSet(f), IdxVal::OrdIdx(i)) => Ok(f.get()(i).as_val_val()),
            _ => Err(String::from("mistmatch in P1 index and argument")),
        }
    }
}
