use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};
use crate::pars::ParFun;
use crate::scalars::{DNum, DVar, Int, Val, Var};
use crate::vars::{VarD1, VarD2};
use crate::{Model, Ptr, Set, SetCatInt, SetCatStr, SetOrd, SymWrap};
use alloc::boxed::Box;
use alloc::string::{String, ToString};

pub trait IdxAny<'m, 'd> {
    type Set: Set;

    // p1

    fn p1_at<V: Val>(self, m: &'m Model<'d>, par: Ptr<'d, ParFun<'d>>) -> V::Sym<'m, 'd>;

    // p2

    fn p2_at_i0<V: Val, I0: Set>(
        self,
        m: &'m Model<'d>,
        par: Ptr<'d, ParFun<'d>>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
    ) -> V::Sym<'m, 'd>;

    fn p2_at<V: Val, I1: IdxAny<'m, 'd>>(
        self,
        m: &'m Model<'d>,
        par: Ptr<'d, ParFun<'d>>,
        i1: I1,
    ) -> V::Sym<'m, 'd>;

    // v1

    fn v1_at(self, par: VarD1<'m, 'd, Self::Set>) -> Var<'m, 'd>;

    // v2

    fn v2_at_i0<I0: Set>(
        self,
        par: VarD2<'m, 'd, I0, Self::Set>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
    ) -> Var<'m, 'd>;

    fn v2_at<I1: Set>(
        self,
        par: VarD2<'m, 'd, Self::Set, I1>,
        i1: I1::IdxAny<'m, 'd>,
    ) -> Var<'m, 'd>;
}

// cint

#[derive(derive_more::From)]
pub enum IndexCatInt<'m, 'd> {
    Idx(IdxCatInt<'m, 'd>),
    ConUsize(usize),
}

impl<'m, 'd> IdxAny<'m, 'd> for IndexCatInt<'m, 'd> {
    type Set = SetCatInt;

    // p1

    fn p1_at<V: Val>(self, m: &'m Model<'d>, par: Ptr<'d, ParFun<'d>>) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let fun = move || {
                    let i0 = i0.get().read::<Self::Set>();
                    par.get().d1_at::<Self::Set>(i0)
                };
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConUsize(i0) => {
                let fun = move || par.get().d1_at::<Self::Set>(i0);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
        }
    }

    // p2

    fn p2_at_i0<V: Val, I0: Set>(
        self,
        m: &'m Model<'d>,
        par: Ptr<'d, ParFun<'d>>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
    ) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i1) => {
                let i1 = i1.ptr();
                let fun = move || {
                    let i1 = i1.get().read::<Self::Set>();
                    par.get().d2_at::<I0, Self::Set>(i0(), i1)
                };
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConUsize(i1) => {
                let fun = move || par.get().d2_at::<I0, Self::Set>(i0(), i1);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
        }
    }

    fn p2_at<V: Val, I1: IdxAny<'m, 'd>>(
        self,
        m: &'m Model<'d>,
        par: Ptr<'d, ParFun<'d>>,
        i1: I1,
    ) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let i0 = move || i0.get().read::<Self::Set>();
                I1::p2_at_i0::<V, Self::Set>(i1, m, par, i0)
            }
            Self::ConUsize(i0) => I1::p2_at_i0::<V, Self::Set>(i1, m, par, move || i0),
        }
    }

    // v1

    fn v1_at(self, par: VarD1<'m, 'd, Self::Set>) -> Var<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let fun = Box::new(move || i0.get().read::<Self::Set>());
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConUsize(i0) => {
                let fun = Box::new(move || i0);
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
        }
    }

    // v2

    fn v2_at_i0<I0: Set>(
        self,
        par: VarD2<'m, 'd, I0, Self::Set>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
    ) -> Var<'m, 'd> {
        match self {
            Self::Idx(i1) => {
                let i1 = i1.ptr();
                let fun = Box::new(move || (i0(), i1.get().read::<Self::Set>()));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConUsize(i1) => {
                let fun = Box::new(move || (i0(), i1));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
        }
    }

    fn v2_at<I1: Set>(
        self,
        par: VarD2<'m, 'd, Self::Set, I1>,
        i1: I1::IdxAny<'m, 'd>,
    ) -> Var<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let i0 = move || i0.get().read::<Self::Set>();
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0(i1, par, i0)
            }
            Self::ConUsize(i0) => {
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, move || i0)
            }
        }
    }
}

// cstr

#[derive(derive_more::From)]
pub enum IndexCatStr<'m, 'd> {
    Idx(IdxCatStr<'m, 'd>),
    ConString(String),
    ConStringRef(&'d String),
    ConStr(&'d str),
}

impl<'m, 'd> IdxAny<'m, 'd> for IndexCatStr<'m, 'd> {
    type Set = SetCatStr;

    // p1

    fn p1_at<V: Val>(self, m: &'m Model<'d>, par: Ptr<'d, ParFun<'d>>) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let fun = move || {
                    let i0 = i0.get().read::<Self::Set>();
                    par.get().d1_at::<Self::Set>(i0)
                };
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConStringRef(i0) => {
                let fun = move || par.get().d1_at::<Self::Set>(i0);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConString(i0) => {
                let i0 = m.con_val(i0);
                let fun = move || par.get().d1_at::<Self::Set>(i0.get().get());
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConStr(i0) => {
                let i0 = m.con_val(i0.to_string());
                let fun = move || par.get().d1_at::<Self::Set>(i0.get().get());
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
        }
    }

    // p2

    fn p2_at_i0<V: Val, I0: Set>(
        self,
        m: &'m Model<'d>,
        par: Ptr<'d, ParFun<'d>>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
    ) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i1) => {
                let i1 = i1.ptr();
                let fun = move || {
                    let i1 = i1.get().read::<Self::Set>();
                    par.get().d2_at::<I0, Self::Set>(i0(), i1)
                };
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConStringRef(i1) => {
                let fun = move || par.get().d2_at::<I0, Self::Set>(i0(), i1);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConString(i1) => {
                let i1 = m.con_val(i1);
                let fun = move || par.get().d2_at::<I0, Self::Set>(i0(), i1.get().get());
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConStr(i1) => {
                let i1 = m.con_val(i1.to_string());
                let fun = move || par.get().d2_at::<I0, Self::Set>(i0(), i1.get().get());
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
        }
    }

    fn p2_at<V: Val, I1: IdxAny<'m, 'd>>(
        self,
        m: &'m Model<'d>,
        par: Ptr<'d, ParFun<'d>>,
        i1: I1,
    ) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let i0 = move || i0.get().read::<Self::Set>();
                I1::p2_at_i0::<V, Self::Set>(i1, m, par, i0)
            }
            Self::ConStringRef(i0) => I1::p2_at_i0::<V, Self::Set>(i1, m, par, move || i0),
            Self::ConString(i0) => {
                let i0 = m.con_val(i0);
                I1::p2_at_i0::<V, Self::Set>(i1, m, par, move || i0.get().get())
            }
            Self::ConStr(i0) => {
                let i0 = m.con_val(i0.to_string());
                I1::p2_at_i0::<V, Self::Set>(i1, m, par, move || i0.get().get())
            }
        }
    }

    // v1

    fn v1_at(self, par: VarD1<'m, 'd, Self::Set>) -> Var<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let fun = Box::new(move || i0.get().read::<Self::Set>());
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConStringRef(i0) => {
                let fun = Box::new(move || i0);
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConString(i0) => {
                let i0 = par.m().con_val(i0);
                let fun = Box::new(move || i0.get().get());
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConStr(i0) => {
                let i0 = par.m().con_val(i0.to_string());
                let fun = Box::new(move || i0.get().get());
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
        }
    }

    // v2

    fn v2_at_i0<I0: Set>(
        self,
        par: VarD2<'m, 'd, I0, Self::Set>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
    ) -> Var<'m, 'd> {
        match self {
            Self::Idx(i1) => {
                let i1 = i1.ptr();
                let fun = Box::new(move || (i0(), i1.get().read::<Self::Set>()));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConStringRef(i1) => {
                let fun = Box::new(move || (i0(), i1));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConString(i1) => {
                let i1 = par.m().con_val(i1);
                let fun = Box::new(move || (i0(), i1.get().get()));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConStr(i1) => {
                let i1 = par.m().con_val(i1.to_string());
                let fun = Box::new(move || (i0(), i1.get().get()));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
        }
    }

    fn v2_at<I1: Set>(
        self,
        par: VarD2<'m, 'd, Self::Set, I1>,
        i1: I1::IdxAny<'m, 'd>,
    ) -> Var<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let i0 = move || i0.get().read::<Self::Set>();
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, i0)
            }
            Self::ConStringRef(i0) => {
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, move || i0)
            }
            Self::ConString(i0) => {
                let i0 = par.m().con_val(i0);
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, move || {
                    i0.get().get()
                })
            }
            Self::ConStr(i0) => {
                let i0 = par.m().con_val(i0.to_string());
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, move || {
                    i0.get().get()
                })
            }
        }
    }
}

// ord

#[derive(derive_more::From)]
pub enum IndexOrd<'m, 'd> {
    Idx(IdxOrd<'m, 'd>),
    Int(Int<'m, 'd>),
    ConUsize(usize),
    ConI64(i64),
    ConI32(i32),
}

impl<'m, 'd> IdxAny<'m, 'd> for IndexOrd<'m, 'd> {
    type Set = SetOrd;

    // p1

    fn p1_at<V: Val>(self, m: &'m Model<'d>, par: Ptr<'d, ParFun<'d>>) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let fun = move || {
                    let i0 = i0.get().read::<Self::Set>();
                    par.get().d1_at::<Self::Set>(i0)
                };
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::Int(i0) => {
                let i0 = i0.ptr();
                let fun = move || {
                    let i0 = i0.get().eval_int();
                    par.get().d1_at::<Self::Set>(i0)
                };
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConUsize(i0) => {
                let fun = move || par.get().d1_at::<Self::Set>(i0 as i64);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConI64(i0) => {
                let fun = move || par.get().d1_at::<Self::Set>(i0 as i64);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConI32(i0) => {
                let fun = move || par.get().d1_at::<Self::Set>(i0 as i64);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
        }
    }

    // p2

    fn p2_at_i0<V: Val, I0: Set>(
        self,
        m: &'m Model<'d>,
        par: Ptr<'d, ParFun<'d>>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
    ) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i1) => {
                let i1 = i1.ptr();
                let fun = move || {
                    let i1 = i1.get().read::<Self::Set>();
                    par.get().d2_at::<I0, Self::Set>(i0(), i1)
                };
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::Int(i1) => {
                let i1 = i1.ptr();
                let fun = move || {
                    let i1 = i1.get().eval_int();
                    par.get().d2_at::<I0, Self::Set>(i0(), i1)
                };
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConUsize(i1) => {
                let fun = move || par.get().d2_at::<I0, Self::Set>(i0(), i1 as i64);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConI64(i1) => {
                let fun = move || par.get().d2_at::<I0, Self::Set>(i0(), i1);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConI32(i1) => {
                let fun = move || par.get().d2_at::<I0, Self::Set>(i0(), i1 as i64);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
        }
    }

    fn p2_at<V: Val, I1: IdxAny<'m, 'd>>(
        self,
        m: &'m Model<'d>,
        par: Ptr<'d, ParFun<'d>>,
        i1: I1,
    ) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let i0 = move || i0.get().read::<Self::Set>();
                I1::p2_at_i0::<V, Self::Set>(i1, m, par, i0)
            }
            Self::Int(i0) => {
                let i0 = i0.ptr();
                let i0 = move || i0.get().eval_int();
                I1::p2_at_i0::<V, Self::Set>(i1, m, par, i0)
            }
            Self::ConUsize(i0) => I1::p2_at_i0::<V, Self::Set>(i1, m, par, move || i0 as i64),
            Self::ConI64(i0) => I1::p2_at_i0::<V, Self::Set>(i1, m, par, move || i0 as i64),
            Self::ConI32(i0) => I1::p2_at_i0::<V, Self::Set>(i1, m, par, move || i0 as i64),
        }
    }

    // v1

    fn v1_at(self, par: VarD1<'m, 'd, Self::Set>) -> Var<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let fun = Box::new(move || i0.get().read::<Self::Set>());
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::Int(i0) => {
                let i0 = i0.ptr();
                let fun = Box::new(move || i0.get().eval_int());
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConUsize(i0) => {
                let fun = Box::new(move || i0 as i64);
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConI64(i0) => {
                let fun = Box::new(move || i0 as i64);
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConI32(i0) => {
                let fun = Box::new(move || i0 as i64);
                let data = DVar::create_d1::<Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
        }
    }

    // v2

    fn v2_at_i0<I0: Set>(
        self,
        par: VarD2<'m, 'd, I0, Self::Set>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
    ) -> Var<'m, 'd> {
        match self {
            Self::Idx(i1) => {
                let i1 = i1.ptr();
                let fun = Box::new(move || (i0(), i1.get().read::<Self::Set>()));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::Int(i1) => {
                let i1 = i1.ptr();
                let fun = Box::new(move || (i0(), i1.get().eval_int()));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConUsize(i1) => {
                let fun = Box::new(move || (i0(), i1 as i64));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConI64(i1) => {
                let fun = Box::new(move || (i0(), i1 as i64));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
            Self::ConI32(i1) => {
                let fun = Box::new(move || (i0(), i1 as i64));
                let data = DVar::create_d2::<I0, Self::Set>(par.data_ptr(), fun);
                par.m().var(data)
            }
        }
    }

    fn v2_at<I1: Set>(
        self,
        par: VarD2<'m, 'd, Self::Set, I1>,
        i1: I1::IdxAny<'m, 'd>,
    ) -> Var<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let i0 = move || i0.get().read::<Self::Set>();
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, i0)
            }
            Self::Int(i0) => {
                let i0 = i0.ptr();
                let i0 = move || i0.get().eval_int();
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, i0)
            }
            Self::ConUsize(i0) => {
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, move || {
                    i0 as i64
                })
            }
            Self::ConI64(i0) => {
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, move || {
                    i0 as i64
                })
            }
            Self::ConI32(i0) => {
                <I1::IdxAny<'m, 'd> as IdxAny<'m, 'd>>::v2_at_i0::<Self::Set>(i1, par, move || {
                    i0 as i64
                })
            }
        }
    }
}
