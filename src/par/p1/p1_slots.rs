use crate::{new_bag, Bag, Elements, Res, Slot};
use alloc::{boxed::Box, string::String};

pub enum P1Fun<'d, T> {
    CatSetInt(Box<dyn Fn(usize) -> T + 'd>),
    CatSetStr(Box<dyn Fn(&'d String) -> T + 'd>),
    OrdSet(Box<dyn Fn(i64) -> T + 'd>),
}

pub enum P1Slot<'d, T> {
    CatSetInt(Slot<'d, Box<dyn Fn(usize) -> T + 'd>>),
    CatSetStr(Slot<'d, Box<dyn Fn(&'d String) -> T + 'd>>),
    OrdSet(Slot<'d, Box<dyn Fn(i64) -> T + 'd>>),
}

impl<'d, T> P1Slot<'d, T> {
    pub fn bind(&self, fun: P1Fun<'d, T>) -> Res {
        match (self, fun) {
            (Self::CatSetInt(s), P1Fun::CatSetInt(f)) => s.bind(f),
            (Self::CatSetStr(s), P1Fun::CatSetStr(f)) => s.bind(f),
            (Self::OrdSet(s), P1Fun::OrdSet(f)) => s.bind(f),
            _ => unreachable!("TODO: explain why"),
        }
    }
}
