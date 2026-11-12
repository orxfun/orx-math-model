use crate::{new_bag, Bag, Elements, MetaInt, MetaSet, MetaVal, Ptr, Res, SetTyp, Slot};
use crate::{vals::MetaNum, ValTyp};
use alloc::{boxed::Box, string::String};
use core::mem::transmute;

pub enum P1Fun<'d, V: MetaVal> {
    CatSetInt(Box<dyn Fn(usize) -> V::Val + 'd>),
    CatSetStr(Box<dyn Fn(&'d String) -> V::Val + 'd>),
    OrdSet(Box<dyn Fn(i64) -> V::Val + 'd>),
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
    pub fn test_at<M: MetaSet>(&self, idx: M::IdxRaw<'d>) -> Res<crate::vals::ValVal> {
        use crate::idxs::IdxVal;
        match (self, M::idx_as_enum(idx)) {
            (Self::CatSetInt(f), IdxVal::CatIdxInt(i)) => Ok(V::val_as_enum(f.get()(i))),
            (Self::CatSetStr(f), IdxVal::CatIdxStr(i)) => Ok(V::val_as_enum(f.get()(i))),
            (Self::OrdSet(f), IdxVal::OrdIdx(i)) => Ok(V::val_as_enum(f.get()(i))),
            _ => Err(String::from("mistmatch in P1 index and argument")),
        }
    }

    pub fn fun_ptr<I0: MetaSet>(&self) -> Ptr<'d, Box<dyn Fn(I0::IdxRaw<'d>) -> V::Val + 'd>> {
        match self {
            Self::CatSetInt(s) => unsafe { transmute(s.ptr()) },
            Self::CatSetStr(s) => unsafe { transmute(s.ptr()) },
            Self::OrdSet(s) => unsafe { transmute(s.ptr()) },
        }
    }
}

// collection of

pub struct P1SlotsOf<'d, V: MetaVal> {
    cat_idx_int: Bag<Box<dyn Fn(usize) -> V::Val + 'd>>,
    cat_idx_str: Bag<Box<dyn Fn(&'d String) -> V::Val + 'd>>,
    ord_idx: Bag<Box<dyn Fn(i64) -> V::Val + 'd>>,
}

impl<'d, V: MetaVal> Default for P1SlotsOf<'d, V> {
    fn default() -> Self {
        Self {
            cat_idx_int: new_bag(),
            cat_idx_str: new_bag(),
            ord_idx: new_bag(),
        }
    }
}

impl<'d, T: MetaVal> P1SlotsOf<'d, T> {
    fn push(&self, s0_type: SetTyp) -> P1Slot<'d, T> {
        match s0_type {
            SetTyp::CatSetInt => {
                let el = self.cat_idx_int.push(Box::new(|_| Default::default()));
                P1Slot::CatSetInt(el.into())
            }
            SetTyp::CatSetStr => {
                let el = self.cat_idx_str.push(Box::new(|_| Default::default()));
                P1Slot::CatSetStr(el.into())
            }
            SetTyp::OrdSet => {
                let el = self.ord_idx.push(Box::new(|_| Default::default()));
                P1Slot::OrdSet(el.into())
            }
        }
    }
}

// collection

#[derive(Default)]
pub struct P1Slots<'d> {
    int: P1SlotsOf<'d, MetaInt>,
    num: P1SlotsOf<'d, MetaNum>,
}

impl<'d> P1Slots<'d> {
    pub fn push<I0: MetaSet, V: MetaVal>(&self) -> P1Slot<'d, V> {
        let set_typ = I0::set_typ();
        match V::val_typ() {
            ValTyp::Int => unsafe { transmute(self.int.push(set_typ)) },
            ValTyp::Num => unsafe { transmute(self.num.push(set_typ)) },
        }
    }
}
