use crate::ptr::RefPtr;
use crate::sets::{CatSet, CatSetFun, OrdSet, OrdSetFun};

// fun

pub enum SetFun<'d> {
    CatInt(CatSetFun<'d, usize>),
    CatStr(CatSetFun<'d, &'d str>),
    Ord(OrdSetFun<'d>),
}

impl<'d> From<CatSetFun<'d, usize>> for SetFun<'d> {
    fn from(value: CatSetFun<'d, usize>) -> Self {
        Self::CatInt(value)
    }
}

impl<'d> From<CatSetFun<'d, &'d str>> for SetFun<'d> {
    fn from(value: CatSetFun<'d, &'d str>) -> Self {
        Self::CatStr(value)
    }
}

impl<'d> From<OrdSetFun<'d>> for SetFun<'d> {
    fn from(value: OrdSetFun<'d>) -> Self {
        Self::Ord(value)
    }
}

// fun-ptr

pub enum SetFunPtr<'d> {
    CatInt(RefPtr<CatSetFun<'d, usize>>),
    CatStr(RefPtr<CatSetFun<'d, &'d str>>),
    Ord(RefPtr<OrdSetFun<'d>>),
}

impl<'d> From<RefPtr<CatSetFun<'d, usize>>> for SetFunPtr<'d> {
    fn from(value: RefPtr<CatSetFun<'d, usize>>) -> Self {
        Self::CatInt(value)
    }
}

impl<'d> From<RefPtr<CatSetFun<'d, &'d str>>> for SetFunPtr<'d> {
    fn from(value: RefPtr<CatSetFun<'d, &'d str>>) -> Self {
        Self::CatStr(value)
    }
}

impl<'d> From<RefPtr<OrdSetFun<'d>>> for SetFunPtr<'d> {
    fn from(value: RefPtr<OrdSetFun<'d>>) -> Self {
        Self::Ord(value)
    }
}

// sym

#[derive(Clone, Copy)]
pub enum Set<'m, 'd> {
    CatInt(CatSet<'m, 'd, usize>),
    CatStr(CatSet<'m, 'd, &'d str>),
    Ord(OrdSet<'m, 'd>),
}

impl<'m, 'd> Set<'m, 'd> {
    pub fn fun_ptr(&self) -> SetFunPtr<'d> {
        match self {
            Self::CatInt(x) => SetFunPtr::CatInt(x.fun_ptr()),
            Self::CatStr(x) => SetFunPtr::CatStr(x.fun_ptr()),
            Self::Ord(x) => SetFunPtr::Ord(x.fun_ptr()),
        }
    }
}

impl<'m, 'd> From<CatSet<'m, 'd, usize>> for Set<'m, 'd> {
    fn from(value: CatSet<'m, 'd, usize>) -> Self {
        Self::CatInt(value)
    }
}

impl<'m, 'd> From<CatSet<'m, 'd, &'d str>> for Set<'m, 'd> {
    fn from(value: CatSet<'m, 'd, &'d str>) -> Self {
        Self::CatStr(value)
    }
}

impl<'m, 'd> From<OrdSet<'m, 'd>> for Set<'m, 'd> {
    fn from(value: OrdSet<'m, 'd>) -> Self {
        Self::Ord(value)
    }
}
