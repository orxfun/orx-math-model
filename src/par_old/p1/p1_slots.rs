use crate::{
    new_bag, Bag, Elements, MetaCatInt, MetaCatStr, MetaInt, MetaNum, MetaOrd, MetaSet, MetaVal,
    Model, Ptr, Res, SetTyp, Slot, ValTyp,
};
use alloc::{boxed::Box, string::String};
use core::mem::transmute;

// fun

#[derive(derive_more::Deref)]
pub struct P1Fun0<'d, V: MetaVal, I0: MetaSet>(Box<dyn Fn(I0::IdxRaw<'d>) -> V::Val + 'd>);

impl<'d, V: MetaVal, I0: MetaSet> P1Fun0<'d, V, I0> {
    fn empty() -> Self {
        Self(Box::new(|_| Default::default()))
    }

    fn new(fun: impl Fn(I0::IdxRaw<'d>) -> V::Val + 'd) -> Self {
        Self(Box::new(fun))
    }
}

pub enum P1Fun1<'d, V: MetaVal> {
    CatInt(P1Fun0<'d, V, MetaCatInt>),
    CatStr(P1Fun0<'d, V, MetaCatStr>),
    Ord(P1Fun0<'d, V, MetaOrd>),
}

impl<'d, V: MetaVal> P1Fun1<'d, V> {
    fn empty<I0: MetaSet>() -> Self {
        Self::new::<I0>(P1Fun0::empty())
    }

    fn new<I0: MetaSet>(fun: P1Fun0<'d, V, I0>) -> Self {
        match I0::set_typ() {
            SetTyp::CatInt => Self::CatInt(P1Fun0(unsafe { transmute(fun) })),
            SetTyp::CatStr => Self::CatStr(P1Fun0(unsafe { transmute(fun) })),
            SetTyp::Ord => Self::Ord(P1Fun0(unsafe { transmute(fun) })),
        }
    }

    fn bind<I0: MetaSet>(&mut self, fun: impl Fn(I0::IdxRaw<'d>) -> V::Val + 'd) {
        let fun: Box<dyn Fn(I0::IdxRaw<'d>) -> V::Val + 'd> = Box::new(fun);
        match self {
            P1Fun1::CatInt(f) => *(&mut f.0) = unsafe { transmute(fun) },
            P1Fun1::CatStr(f) => *(&mut f.0) = unsafe { transmute(fun) },
            P1Fun1::Ord(f) => *(&mut f.0) = unsafe { transmute(fun) },
        }
    }

    #[cfg(test)]
    fn test_at<I0: MetaSet>(&self, i0: I0::IdxRaw<'d>) -> crate::vals::ValVal {
        use crate::{idxs::IdxVal, vals::ValVal};

        match (self, I0::idx_as_enum(i0)) {
            (Self::CatInt(f), IdxVal::CatInt(i0)) => V::val_as_enum(f.0(i0)),
            (Self::CatStr(f), IdxVal::CatStr(i0)) => V::val_as_enum(f.0(i0)),
            (Self::Ord(f), IdxVal::Ord(i0)) => V::val_as_enum(f.0(i0)),
            _ => unreachable!(),
        }
    }
}

// data

pub struct P1Data<'d, V: MetaVal> {
    i0_typ: SetTyp,
    fun: P1Fun1<'d, V>,
}

impl<'d, V: MetaVal> P1Data<'d, V> {
    pub fn empty<I0: MetaSet>() -> Self {
        Self {
            i0_typ: I0::set_typ(),
            fun: P1Fun1::empty::<I0>(),
        }
    }

    pub fn new<I0: MetaSet>(fun: impl Fn(I0::IdxRaw<'d>) -> V::Val + 'd) -> Self {
        let fun = P1Fun0::new(fun);
        let fun = P1Fun1::new::<I0>(fun);
        Self {
            i0_typ: I0::set_typ(),
            fun,
        }
    }

    fn debug_assert_set_types<I0: MetaSet>(&self) {
        debug_assert_eq!(self.i0_typ, I0::set_typ());
    }
}

// slot

pub struct P1Slot<'d, V: MetaVal> {
    slot: Slot<'d, P1Data<'d, V>>,
}

impl<'d, V: MetaVal> P1Slot<'d, V> {
    pub fn bind<I0: MetaSet>(&self, fun: impl Fn(I0::IdxRaw<'d>) -> V::Val + 'd) {
        self.slot.get().debug_assert_set_types::<I0>();
        let data = P1Data::new::<I0>(fun);
        self.slot.bind(data).expect("TODO: infallible, why");
    }

    #[cfg(test)]
    pub fn test_at<I0: MetaSet>(&self, i0: I0::IdxRaw<'d>) -> crate::vals::ValVal {
        self.slot.get().fun.test_at::<I0>(i0)
    }

    pub fn fun_ptr<I0: MetaSet>(&self) -> Ptr<'d, Box<dyn Fn(I0::IdxRaw<'d>) -> V::Val + 'd>> {
        // Ptr::new(self.slot.get().fun.fun_ptr::<I0>())
        todo!()
    }
}

// collection

#[derive(Default)]
pub struct P1Slots<'d> {
    int: Bag<P1Data<'d, MetaInt>>,
    num: Bag<P1Data<'d, MetaNum>>,
}

impl<'d> P1Slots<'d> {
    pub fn push<I0: MetaSet, V: MetaVal>(&self, m: &Model<'d>) -> P1Slot<'d, V> {
        let data = P1Data::<V>::empty::<I0>();
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
