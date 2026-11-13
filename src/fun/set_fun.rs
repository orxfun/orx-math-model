use crate::{Elements, MetaCatInt, MetaCatStr, MetaOrd, MetaSet, Res, SetTyp, Validation};
use alloc::boxed::Box;
use core::mem::transmute;

pub type FSet<'d, S> = Box<dyn Fn(&mut Elements<<S as MetaSet>::IdxRaw<'d>>) + 'd>;

pub enum FunSet<'d> {
    CatInt(FSet<'d, MetaCatInt>),
    CatStr(FSet<'d, MetaCatStr>),
    Ord(FSet<'d, MetaOrd>),
}

impl<'d> FunSet<'d> {
    pub(super) fn new(typ: SetTyp) -> Self {
        match typ {
            SetTyp::CatInt => Self::CatInt(Box::new(|_| {})),
            SetTyp::CatStr => Self::CatStr(Box::new(|_| {})),
            SetTyp::Ord => Self::Ord(Box::new(|_| {})),
        }
    }

    pub fn typ(&self) -> SetTyp {
        match self {
            Self::CatInt(_) => SetTyp::CatInt,
            Self::CatStr(_) => SetTyp::CatStr,
            Self::Ord(_) => SetTyp::Ord,
        }
    }

    pub fn get<S: MetaSet>(&self) -> &FSet<'d, S> {
        Res::dbg_assert_same_set(S::typ(), self.typ());
        match self {
            Self::CatInt(f) => unsafe { transmute(f) },
            Self::CatStr(f) => unsafe { transmute(f) },
            Self::Ord(f) => unsafe { transmute(f) },
            _ => unreachable!("invalid FunE::set call"),
        }
    }
}
