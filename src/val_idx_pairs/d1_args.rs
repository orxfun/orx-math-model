use crate::pars::p1::ParD1;
use crate::pars::p2::ParD2;
use crate::sets::{IndexCatInt, IndexCatStr, IndexOrd};
use crate::{Ptr, Set, SetCatInt, SetCatStr, SetOrd, Val, ValInt};
use alloc::string::ToString;

pub trait D1Args: Set {
    fn p1_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>;

    fn p2_at<'m, 'd, V: Val, I0: Set>(
        par: ParD2<'m, 'd, V, I0, Self>,
        i0: impl Fn() -> I0::IdxRaw<'d>,
        i1: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>;
}

// cat int

impl D1Args for SetCatInt {
    fn p1_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match i0.into() {
            IndexCatInt::Idx(i0) => {
                let idx = Ptr::new(i0.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = idx.get().read::<Self>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexCatInt::ConUsize(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0)
            }),
        }
    }

    fn p2_at<'m, 'd, V: Val, I0: Set>(
        par: ParD2<'m, 'd, V, I0, Self>,
        i0: impl Fn() -> I0::IdxRaw<'d>,
        i1: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match i1.into() {
            IndexCatInt::Idx(idx) => {
                let i0 = Ptr::new(idx.el);

                todo!()
            }
            IndexCatInt::ConUsize(i0) => todo!(),
        }
    }
}

// cat set

impl D1Args for SetCatStr {
    fn p1_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match i0.into() {
            IndexCatStr::Idx(i0) => {
                let idx = Ptr::new(i0.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = idx.get().read::<Self>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexCatStr::ConString(i0) => {
                let con = par.m.con_val(i0);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    fun.at::<V, Self>(con.get().get())
                })
            }
            IndexCatStr::ConStringRef(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0)
            }),
            IndexCatStr::ConStr(i0) => {
                let con = par.m.con_val(i0.to_string());
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    fun.at::<V, Self>(con.get().get())
                })
            }
        }
    }

    fn p2_at<'m, 'd, V: Val, I0: Set>(
        par: ParD2<'m, 'd, V, I0, Self>,
        i0: impl Fn() -> I0::IdxRaw<'d>,
        i1: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        todo!()
    }
}

// ord

impl D1Args for SetOrd {
    fn p1_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match i0.into() {
            IndexOrd::Idx(i0) => {
                let idx = Ptr::new(i0.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = idx.get().read::<Self>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexOrd::Int(i0) => {
                let int = Ptr::new(i0.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = int.get().eval::<ValInt>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexOrd::ConUsize(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0 as i64)
            }),
            IndexOrd::ConI64(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0)
            }),
            IndexOrd::ConI32(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0 as i64)
            }),
        }
    }

    fn p2_at<'m, 'd, V: Val, I0: Set>(
        par: ParD2<'m, 'd, V, I0, Self>,
        i0: impl Fn() -> I0::IdxRaw<'d>,
        i1: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        todo!()
    }
}
