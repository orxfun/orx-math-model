use crate::{
    new_bag, Bag, Elements, MetaCatInt, MetaCatStr, MetaInt, MetaNum, MetaOrd, MetaSet, MetaVal,
    Model, Ptr, Res, SetTyp, Slot, ValTyp,
};
use alloc::{boxed::Box, string::String};
use core::mem::transmute;

// fun

#[derive(derive_more::Deref)]
pub struct P1Fun<'d, V: MetaVal, I0: MetaSet>(Box<dyn Fn(I0::IdxRaw<'d>) -> V::Val + 'd>);

impl<'d, V: MetaVal, I0: MetaSet> Default for P1Fun<'d, V, I0> {
    fn default() -> Self {
        Self(Box::new(|_| Default::default()))
    }
}

pub enum P1FunE<'d, V: MetaVal> {
    CatInt(P1Fun<'d, V, MetaCatInt>),
    CatStr(P1Fun<'d, V, MetaCatStr>),
    Ord(P1Fun<'d, V, MetaOrd>),
}

impl<'d, V: MetaVal> P1FunE<'d, V> {
    pub fn new<I0: MetaSet>(fun: impl Fn(I0::IdxRaw<'d>) -> V::Val + 'd) -> Self {
        let fun: Box<dyn Fn(I0::IdxRaw<'d>) -> V::Val + 'd> = Box::new(fun);
        match I0::set_typ() {
            SetTyp::CatInt => Self::CatInt(P1Fun(unsafe { transmute(fun) })),
            SetTyp::CatStr => Self::CatStr(P1Fun(unsafe { transmute(fun) })),
            SetTyp::Ord => Self::Ord(P1Fun(unsafe { transmute(fun) })),
        }
    }
}

// slot

pub enum P1Slot<'d, V: MetaVal> {
    CatInt(Slot<'d, P1Fun<'d, V, MetaCatInt>>),
    CatStr(Slot<'d, P1Fun<'d, V, MetaCatStr>>),
    Ord(Slot<'d, P1Fun<'d, V, MetaOrd>>),
}

impl<'d, V: MetaVal> P1Slot<'d, V> {
    pub fn bind(&self, fun: P1FunE<'d, V>) -> Res {
        match (self, fun) {
            (Self::CatInt(s), P1FunE::CatInt(f)) => s.bind(f),
            (Self::CatStr(s), P1FunE::CatStr(f)) => s.bind(f),
            (Self::Ord(s), P1FunE::Ord(f)) => s.bind(f),
            _ => unreachable!("TODO: explain why"),
        }
    }

    #[cfg(test)]
    pub fn test_at<M: MetaSet>(&self, idx: M::IdxRaw<'d>) -> Res<crate::vals::ValVal> {
        use crate::idxs::IdxVal;
        match (self, M::idx_as_enum(idx)) {
            (Self::CatInt(f), IdxVal::CatIdxInt(i)) => Ok(V::val_as_enum(f.get()(i))),
            (Self::CatStr(f), IdxVal::CatIdxStr(i)) => Ok(V::val_as_enum(f.get()(i))),
            (Self::Ord(f), IdxVal::OrdIdx(i)) => Ok(V::val_as_enum(f.get()(i))),
            _ => Err(String::from("mistmatch in P1 index and argument")),
        }
    }

    pub fn fun_ptr<I0: MetaSet>(&self) -> Ptr<'d, Box<dyn Fn(I0::IdxRaw<'d>) -> V::Val + 'd>> {
        match self {
            Self::CatInt(s) => unsafe { transmute(s.ptr()) },
            Self::CatStr(s) => unsafe { transmute(s.ptr()) },
            Self::Ord(s) => unsafe { transmute(s.ptr()) },
        }
    }
}

// collection of

pub struct P1SlotsOf<'d, V: MetaVal> {
    cat_int: Bag<P1Fun<'d, V, MetaCatInt>>,
    cat_str: Bag<P1Fun<'d, V, MetaCatStr>>,
    ord: Bag<P1Fun<'d, V, MetaOrd>>,
}

impl<'d, V: MetaVal> Default for P1SlotsOf<'d, V> {
    fn default() -> Self {
        Self {
            cat_int: new_bag(),
            cat_str: new_bag(),
            ord: new_bag(),
        }
    }
}

impl<'d, T: MetaVal> P1SlotsOf<'d, T> {
    fn push(&self, m: &Model<'d>, s0_typ: SetTyp) -> P1Slot<'d, T> {
        match s0_typ {
            SetTyp::CatInt => {
                let el = self.cat_int.push(Default::default());
                P1Slot::CatInt(Slot::new(m, el))
            }
            SetTyp::CatStr => {
                let el = self.cat_str.push(Default::default());
                P1Slot::CatStr(Slot::new(m, el))
            }
            SetTyp::Ord => {
                let el = self.ord.push(Default::default());
                P1Slot::Ord(Slot::new(m, el))
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
    pub fn push<I0: MetaSet, V: MetaVal>(&self, m: &Model<'d>) -> P1Slot<'d, V> {
        let set_typ = I0::set_typ();
        match V::val_typ() {
            ValTyp::Int => unsafe { transmute(self.int.push(m, set_typ)) },
            ValTyp::Num => unsafe { transmute(self.num.push(m, set_typ)) },
        }
    }
}
