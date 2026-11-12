use crate::{
    new_bag, Bag, Elements, MetaCatInt, MetaCatStr, MetaInt, MetaNum, MetaOrd, MetaSet, MetaVal,
    Ptr, Res, SetTyp, Slot, ValTyp,
};
use alloc::{boxed::Box, string::String};
use core::mem::transmute;

// fun

#[derive(derive_more::Deref)]
pub struct P2Fun<'d, V: MetaVal, I0: MetaSet, I1: MetaSet>(
    Box<dyn Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd>,
);

impl<'d, V: MetaVal, I0: MetaSet, I1: MetaSet> Default for P2Fun<'d, V, I0, I1> {
    fn default() -> Self {
        Self(Box::new(|_, _| Default::default()))
    }
}

pub enum P2FunE0<'d, V: MetaVal, I0: MetaSet> {
    CatInt(P2Fun<'d, V, I0, MetaCatInt>),
    CatStr(P2Fun<'d, V, I0, MetaCatStr>),
    Ord(P2Fun<'d, V, I0, MetaOrd>),
}

impl<'d, V: MetaVal, I0: MetaSet> P2FunE0<'d, V, I0> {
    fn new<I1: MetaSet>(fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd) -> Self {
        let fun: Box<dyn Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd> = Box::new(fun);
        match I1::set_typ() {
            SetTyp::CatInt => Self::CatInt(P2Fun(unsafe { transmute(fun) })),
            SetTyp::CatStr => Self::CatStr(P2Fun(unsafe { transmute(fun) })),
            SetTyp::Ord => Self::Ord(P2Fun(unsafe { transmute(fun) })),
        }
    }
}

pub enum P2FunE<'d, V: MetaVal> {
    CatInt(P2FunE0<'d, V, MetaCatInt>),
    CatStr(P2FunE0<'d, V, MetaCatStr>),
    Ord(P2FunE0<'d, V, MetaOrd>),
}

impl<'d, V: MetaVal> P2FunE<'d, V> {
    pub fn new<I0: MetaSet, I1: MetaSet>(
        fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd,
    ) -> Self {
        let fun: P2FunE0<'d, V, I0> = P2FunE0::new::<I1>(fun);
        match I0::set_typ() {
            SetTyp::CatInt => Self::CatInt(unsafe { transmute(fun) }),
            SetTyp::CatStr => Self::CatStr(unsafe { transmute(fun) }),
            SetTyp::Ord => Self::Ord(unsafe { transmute(fun) }),
        }
    }
}

// slot

pub enum P2Slot0<'d, V: MetaVal, I0: MetaSet> {
    CatInt(Slot<'d, P2FunE0<'d, V, I0>>),
    CatStr(Slot<'d, P2FunE0<'d, V, I0>>),
    Ord(Slot<'d, P2FunE0<'d, V, I0>>),
}

impl<'d, V: MetaVal, I0: MetaSet> P2Slot0<'d, V, I0> {
    fn bind(&self, fun: P2FunE0<'d, V, I0>) {
        match self {
            P2Slot0::CatInt(s) => s.bind(fun),
            P2Slot0::CatStr(s) => s.bind(fun),
            P2Slot0::Ord(s) => s.bind(fun),
        }
        .expect("TODO: infallible, why")
    }
}

pub enum P2Slot<'d, V: MetaVal> {
    CatInt(P2Slot0<'d, V, MetaCatInt>),
    CatStr(P2Slot0<'d, V, MetaCatStr>),
    Ord(P2Slot0<'d, V, MetaOrd>),
}

impl<'d, V: MetaVal> P2Slot<'d, V> {
    pub fn bind<I0, I1>(&self, fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd)
    where
        I0: MetaSet,
        I1: MetaSet,
    {
        todo!()
    }
}
