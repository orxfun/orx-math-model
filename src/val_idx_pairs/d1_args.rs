use crate::pars::p1::ParD1;
use crate::sets::{IndexCatInt, IndexCatStr, IndexOrd};
use crate::{Ptr, Set, SetCatInt, SetCatStr, SetOrd, Val, ValInt};
use alloc::string::ToString;

pub trait D1Args: Set {
    fn par_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>;
}

// cat int

impl D1Args for SetCatInt {
    fn par_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match idx.into() {
            IndexCatInt::Idx(idx) => {
                let idx = Ptr::new(idx.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = idx.get().read::<Self>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexCatInt::ConUsize(con) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(con)
            }),
        }
    }
}

// cat set

impl D1Args for SetCatStr {
    fn par_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match idx.into() {
            IndexCatStr::Idx(idx) => {
                let idx = Ptr::new(idx.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = idx.get().read::<Self>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexCatStr::ConString(con) => {
                let con = par.m.con_val(con);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    fun.at::<V, Self>(con.get().get())
                })
            }
            IndexCatStr::ConStringRef(con) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(con)
            }),
            IndexCatStr::ConStr(con) => {
                let con = par.m.con_val(con.to_string());
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    fun.at::<V, Self>(con.get().get())
                })
            }
        }
    }
}

// ord

impl D1Args for SetOrd {
    fn par_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match idx.into() {
            IndexOrd::Idx(idx) => {
                let idx = Ptr::new(idx.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = idx.get().read::<Self>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexOrd::Int(int) => {
                let int = Ptr::new(int.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = int.get().eval::<ValInt>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexOrd::ConUsize(con) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(con as i64)
            }),
            IndexOrd::ConI64(con) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(con)
            }),
            IndexOrd::ConI32(con) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(con as i64)
            }),
        }
    }
}
