use crate::{Elements, MetaCatInt, MetaCatStr, MetaOrd, MetaSet, SetTyp};
use alloc::boxed::Box;
use core::mem::transmute;

pub type FSet<'d, S> = Box<dyn Fn(&mut Elements<<S as MetaSet>::IdxRaw<'d>>) + 'd>;

pub enum FunSet<'d> {
    CatInt(FSet<'d, MetaCatInt>),
    CatStr(FSet<'d, MetaCatStr>),
    Ord(FSet<'d, MetaOrd>),
}

impl<'d> FunSet<'d> {
    pub fn typ(&self) -> SetTyp {
        match self {
            Self::CatInt(_) => SetTyp::CatInt,
            Self::CatStr(_) => SetTyp::CatStr,
            Self::Ord(_) => SetTyp::Ord,
        }
    }

    pub(super) fn new(typ: SetTyp) -> Self {
        match typ {
            SetTyp::CatInt => Self::CatInt(Box::new(|_| {})),
            SetTyp::CatStr => Self::CatStr(Box::new(|_| {})),
            SetTyp::Ord => Self::Ord(Box::new(|_| {})),
        }
    }

    pub fn get<S: MetaSet>(&self) -> &FSet<'d, S> {
        match (S::set_typ(), self) {
            (SetTyp::CatInt, Self::CatInt(f)) => unsafe { transmute(f) },
            (SetTyp::CatStr, Self::CatStr(f)) => unsafe { transmute(f) },
            (SetTyp::Ord, Self::Ord(f)) => unsafe { transmute(f) },
            _ => unreachable!("invalid FunE::set call"),
        }
    }
}
