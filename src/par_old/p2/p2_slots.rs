use crate::{
    new_bag, Bag, Elements, MetaCatInt, MetaCatStr, MetaInt, MetaNum, MetaOrd, MetaSet, MetaVal,
    Model, Ptr, Res, SetTyp, Slot, ValTyp,
};
use alloc::{boxed::Box, string::String};
use core::mem::transmute;

// fun

#[derive(derive_more::Deref)]
pub struct P2Fun0<'d, V: MetaVal, I0: MetaSet, I1: MetaSet>(
    Box<dyn Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd>,
);

impl<'d, V: MetaVal, I0: MetaSet, I1: MetaSet> P2Fun0<'d, V, I0, I1> {
    fn empty() -> Self {
        Self(Box::new(|_, _| Default::default()))
    }

    fn new(fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd) -> Self {
        Self(Box::new(fun))
    }
}

pub enum P2Fun1<'d, V: MetaVal, I0: MetaSet> {
    CatInt(P2Fun0<'d, V, I0, MetaCatInt>),
    CatStr(P2Fun0<'d, V, I0, MetaCatStr>),
    Ord(P2Fun0<'d, V, I0, MetaOrd>),
}

impl<'d, V: MetaVal, I0: MetaSet> P2Fun1<'d, V, I0> {
    fn empty<I1: MetaSet>() -> Self {
        Self::new::<I1>(P2Fun0::empty())
    }

    fn new<I1: MetaSet>(fun: P2Fun0<'d, V, I0, I1>) -> Self {
        match I1::set_typ() {
            SetTyp::CatInt => Self::CatInt(P2Fun0(unsafe { transmute(fun) })),
            SetTyp::CatStr => Self::CatStr(P2Fun0(unsafe { transmute(fun) })),
            SetTyp::Ord => Self::Ord(P2Fun0(unsafe { transmute(fun) })),
        }
    }
}

pub enum P2Fun2<'d, V: MetaVal> {
    CatInt(P2Fun1<'d, V, MetaCatInt>),
    CatStr(P2Fun1<'d, V, MetaCatStr>),
    Ord(P2Fun1<'d, V, MetaOrd>),
}

impl<'d, V: MetaVal> P2Fun2<'d, V> {
    fn empty<I0: MetaSet, I1: MetaSet>() -> Self {
        Self::new::<I0, I1>(P2Fun1::empty::<I1>())
    }

    fn new<I0: MetaSet, I1: MetaSet>(fun: P2Fun1<'d, V, I0>) -> Self {
        match I1::set_typ() {
            SetTyp::CatInt => P2Fun2::CatInt(unsafe { transmute(fun) }),
            SetTyp::CatStr => P2Fun2::CatStr(unsafe { transmute(fun) }),
            SetTyp::Ord => P2Fun2::Ord(unsafe { transmute(fun) }),
        }
    }
}

// data

pub struct P2Data<'d, V: MetaVal> {
    i0_typ: SetTyp,
    i1_typ: SetTyp,
    fun: P2Fun2<'d, V>,
}

impl<'d, V: MetaVal> P2Data<'d, V> {
    pub fn empty<I0: MetaSet, I1: MetaSet>() -> Self {
        Self {
            i0_typ: I0::set_typ(),
            i1_typ: I1::set_typ(),
            fun: P2Fun2::empty::<I0, I1>(),
        }
    }

    pub fn new<I0: MetaSet, I1: MetaSet>(
        fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Val + 'd,
    ) -> Self {
        let fun = P2Fun0::new(fun);
        let fun = P2Fun1::new::<I1>(fun);
        let fun = P2Fun2::new::<I0, I1>(fun);
        Self {
            i0_typ: I0::set_typ(),
            i1_typ: I1::set_typ(),
            fun,
        }
    }

    fn debug_assert_set_types<I0: MetaSet, I1: MetaSet>(&self) {
        debug_assert_eq!(self.i0_typ, I0::set_typ());
        debug_assert_eq!(self.i1_typ, I1::set_typ());
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
        self.slot.get().debug_assert_set_types::<I0, I1>();
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
