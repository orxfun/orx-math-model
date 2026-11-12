use crate::{
    new_bag, Bag, Elements, MetaCatInt, MetaCatStr, MetaInt, MetaNum, MetaOrd, MetaSet, MetaVal,
    Model, Ptr, Res, SetTyp, Slot, ValTyp,
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
    fn empty<I1: MetaSet>() -> Self {
        match I1::set_typ() {
            SetTyp::CatInt => Self::CatInt(P2Fun::default()),
            SetTyp::CatStr => Self::CatStr(P2Fun::default()),
            SetTyp::Ord => Self::Ord(P2Fun::default()),
        }
    }

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
    fn empty<I0: MetaSet, I1: MetaSet>() -> Self {
        match I0::set_typ() {
            SetTyp::CatInt => P2FunE::CatInt(P2FunE0::empty::<I1>()),
            SetTyp::CatStr => P2FunE::CatStr(P2FunE0::empty::<I1>()),
            SetTyp::Ord => P2FunE::Ord(P2FunE0::empty::<I1>()),
        }
    }
}

// data

pub struct P2Data<'d, V: MetaVal> {
    i0_typ: SetTyp,
    i1_typ: SetTyp,
    fun: P2FunE<'d, V>,
}

impl<'d, V: MetaVal> P2Data<'d, V> {
    pub fn empty<I0: MetaSet, I1: MetaSet>() -> Self {
        Self {
            i0_typ: I0::set_typ(),
            i1_typ: I1::set_typ(),
            fun: P2FunE::empty::<I0, I1>(),
        }
    }

    pub fn new<I0: MetaSet, I1: MetaSet>(
        fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd,
    ) -> Self {
        let fun: P2FunE0<'d, V, I0> = P2FunE0::new::<I1>(fun);
        let fun = match I0::set_typ() {
            SetTyp::CatInt => P2FunE::CatInt(unsafe { transmute(fun) }),
            SetTyp::CatStr => P2FunE::CatStr(unsafe { transmute(fun) }),
            SetTyp::Ord => P2FunE::Ord(unsafe { transmute(fun) }),
        };
        Self {
            i0_typ: I0::set_typ(),
            i1_typ: I1::set_typ(),
            fun,
        }
    }
}

// slot

pub struct P2Slot<'d, V: MetaVal> {
    slot: Slot<'d, P2Data<'d, V>>,
}

impl<'d, V: MetaVal> P2Slot<'d, V> {
    pub fn bind<I0, I1>(&self, fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd)
    where
        I0: MetaSet,
        I1: MetaSet,
    {
        let data = P2Data::new::<I0, I1>(fun);
        self.slot.bind(data).expect("TODO: infallible, why");
    }
}

// collection

#[derive(Default)]
pub struct P2Slots<'d> {
    int: Bag<P2Data<'d, MetaInt>>,
    num: Bag<P2Data<'d, MetaNum>>,
}

impl<'d> P2Slots<'d> {
    pub fn push<V: MetaVal, I0: MetaSet, I1: MetaSet>(&self, m: &Model<'d>) -> P2Slot<'d, V> {
        let data = P2Data::<V>::empty::<I0, I1>();
        match V::val_typ() {
            ValTyp::Int => {
                let el = self.int.push(unsafe { transmute(data) });
                unsafe { transmute(Slot::new(m, el)) }
            }
            ValTyp::Num => {
                let el = self.num.push(unsafe { transmute(data) });
                unsafe { transmute(Slot::new(m, el)) }
            }
        }
    }
}
