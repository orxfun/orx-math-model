use crate::{Elements, MetaCatInt, MetaCatStr, MetaOrd, MetaSet, SetTyp};
use alloc::boxed::Box;
use core::mem::transmute;

pub type Fun<'d, S> = Box<dyn Fn(&mut Elements<<S as MetaSet>::IdxRaw<'d>>) + 'd>;

pub enum FunSet<'d> {
    SetCatInt(Fun<'d, MetaCatInt>),
    SetCatStr(Fun<'d, MetaCatStr>),
    SetOrd(Fun<'d, MetaOrd>),
}

impl<'d> FunSet<'d> {
    pub(super) fn new(typ: SetTyp) -> Self {
        match typ {
            SetTyp::CatInt => Self::SetCatInt(Box::new(|_| {})),
            SetTyp::CatStr => Self::SetCatStr(Box::new(|_| {})),
            SetTyp::Ord => Self::SetOrd(Box::new(|_| {})),
        }
    }

    pub fn get<S: MetaSet>(&self) -> &Fun<'d, S> {
        match (S::set_typ(), self) {
            (SetTyp::CatInt, Self::SetCatInt(f)) => unsafe { transmute(f) },
            (SetTyp::CatStr, Self::SetCatStr(f)) => unsafe { transmute(f) },
            (SetTyp::Ord, Self::SetOrd(f)) => unsafe { transmute(f) },
            _ => unreachable!("invalid FunE::set call"),
        }
    }
}
