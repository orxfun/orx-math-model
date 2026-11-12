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
        match self {
            // Self::CatInt(s) => unsafe { transmute(s.ptr()) },
            // Self::CatStr(s) => unsafe { transmute(s.ptr()) },
            // Self::Ord(s) => unsafe { transmute(s.ptr()) },
            _ => todo!(),
        }
    }
}

// collection of

pub struct P1SlotsOf<'d, V: MetaVal> {
    cat_int: Bag<P1Fun0<'d, V, MetaCatInt>>,
    cat_str: Bag<P1Fun0<'d, V, MetaCatStr>>,
    ord: Bag<P1Fun0<'d, V, MetaOrd>>,
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
        todo!()
        // match s0_typ {
        //     SetTyp::CatInt => {
        //         let el = self.cat_int.push(Default::default());
        //         P1Slot::CatInt(Slot::new(m, el))
        //     }
        //     SetTyp::CatStr => {
        //         let el = self.cat_str.push(Default::default());
        //         P1Slot::CatStr(Slot::new(m, el))
        //     }
        //     SetTyp::Ord => {
        //         let el = self.ord.push(Default::default());
        //         P1Slot::Ord(Slot::new(m, el))
        //     }
        // }
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
