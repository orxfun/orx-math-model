use crate::{Elements, MetaCatInt, MetaCatStr, MetaOrd, MetaSet, SetTyp};
use alloc::boxed::Box;
use core::mem::transmute;

pub type FSet<'d, I0> = Box<dyn Fn(&mut Elements<<I0 as MetaSet>::IdxRaw<'d>>) + 'd>;

pub enum FunE<'d> {
    SetCatInt(FSet<'d, MetaCatInt>),
    SetCatStr(FSet<'d, MetaCatStr>),
    SetOrd(FSet<'d, MetaOrd>),
}

impl<'d> FunE<'d> {
    pub fn set<I0: MetaSet>(&self) -> &FSet<'d, I0> {
        match (I0::set_typ(), self) {
            (SetTyp::CatInt, Self::SetCatInt(f)) => unsafe { transmute(f) },
            (SetTyp::CatStr, Self::SetCatStr(f)) => unsafe { transmute(f) },
            (SetTyp::Ord, Self::SetOrd(f)) => unsafe { transmute(f) },
            _ => unreachable!("invalid FunE::set call"),
        }
    }
}
