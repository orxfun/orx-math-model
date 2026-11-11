use crate::idxs::Idx;
#[cfg(test)]
use crate::idxs::IdxVal;
use crate::sets::SetType;
#[cfg(test)]
use crate::vals::ValVal;
use crate::{new_bag, vals::Val, Bag, Elements, Res, Slot};
use alloc::{boxed::Box, string::String};

pub enum P1Fun<'d, T: Val> {
    CatSetInt(Box<dyn Fn(usize) -> T + 'd>),
    CatSetStr(Box<dyn Fn(&'d String) -> T + 'd>),
    OrdSet(Box<dyn Fn(i64) -> T + 'd>),
}

impl<'d, T: Val, I0: Idx<'d>, F: Fn(I0) -> T + 'd> From<(I0, F)> for P1Fun<'d, T> {
    fn from(value: (I0, F)) -> Self {
        todo!()
    }
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

// collection of

pub struct P1SlotsOf<'d, T: Val> {
    cat_idx_int: Bag<Box<dyn Fn(usize) -> T + 'd>>,
    cat_idx_str: Bag<Box<dyn Fn(&'d String) -> T + 'd>>,
    ord_idx: Bag<Box<dyn Fn(i64) -> T + 'd>>,
}

impl<'d, T: Val> P1SlotsOf<'d, T> {
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
            _ => todo!(),
        }
    }
}

// collection

pub struct P1Slots<'d> {
    int: P1SlotsOf<'d, i64>,
    num: P1SlotsOf<'d, f64>,
}

impl<'d> P1Slots<'d> {
    pub fn push_int(&self, s0_type: SetType) -> P1Slot<'d, i64> {
        self.int.push(s0_type)
    }

    pub fn push_num(&self, s0_type: SetType) -> P1Slot<'d, f64> {
        self.num.push(s0_type)
    }
}
