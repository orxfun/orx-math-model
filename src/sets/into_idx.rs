use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};
use crate::pars::ParFun;
use crate::scalars::num::{DNum, Val};
use crate::scalars::Int;
use crate::{Model, Ptr, Set, SetCatInt, SetCatStr, SetOrd, SymWrap};
use alloc::boxed::Box;
use alloc::string::String;

pub trait IndexAny<'m, 'd> {
    type Set: Set;

    fn p1_at<V: Val>(self, m: &'m Model<'d>, p1: Ptr<'d, ParFun<'d>>) -> V::Sym<'m, 'd>;
}

// cint

#[derive(derive_more::From)]
pub enum IndexCatInt<'m, 'd> {
    Idx(IdxCatInt<'m, 'd>),
    ConUsize(usize),
}

impl<'m, 'd> IndexAny<'m, 'd> for IndexCatInt<'m, 'd> {
    type Set = SetCatInt;

    fn p1_at<V: Val>(self, m: &'m Model<'d>, p1: Ptr<'d, ParFun<'d>>) -> V::Sym<'m, 'd> {
        match self {
            Self::Idx(i0) => {
                let i0 = i0.ptr();
                let fun = move || {
                    let i0 = i0.get().read::<Self::Set>();
                    p1.get().d1_at::<Self::Set>(i0)
                };
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
            }
            Self::ConUsize(i0) => {
                let fun = move || p1.get().d1_at::<Self::Set>(i0);
                m.val::<V>(DNum::new(V::typ(), Box::new(fun)))
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

impl<'m, 'd> IndexAny<'m, 'd> for IndexCatStr<'m, 'd> {
    type Set = SetCatStr;

    fn p1_at<V: Val>(self, m: &'m Model<'d>, p1: Ptr<'d, ParFun<'d>>) -> V::Sym<'m, 'd> {
        todo!()
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

impl<'m, 'd> IndexAny<'m, 'd> for IndexOrd<'m, 'd> {
    type Set = SetOrd;

    fn p1_at<V: Val>(self, m: &'m Model<'d>, p1: Ptr<'d, ParFun<'d>>) -> V::Sym<'m, 'd> {
        todo!()
    }
}
