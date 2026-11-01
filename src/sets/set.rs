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

pub enum Set<'m, 'd> {
    CatInt(CatSet<'m, 'd, usize>),
    CatStr(CatSet<'m, 'd, &'d str>),
    Ord(OrdSet<'m, 'd>),
}

impl<'m, 'd> From<CatSet<'m, 'd, usize>> for Set<'m, 'd> {
    fn from(value: CatSet<'m, 'd, usize>) -> Self {
        Self::CatInt(value)
    }
}

impl<'m, 'd> From<CatSet<'m, 'd, &'d usize>> for Set<'m, 'd> {
    fn from(value: CatSet<'m, 'd, &'d usize>) -> Self {
        Self::CatStr(value)
    }
}

impl<'m, 'd> From<OrdSet<'m, 'd>> for Set<'m, 'd> {
    fn from(value: OrdSet<'m, 'd>) -> Self {
        Self::Ord(value)
    }
}

// sym

// pub trait Set {
//     type Sym<'m, 'd>
//     where
//         'd: 'm;

//     type Fun<'d>;

//     type RawIdx<'d>;

//     fn fun_ptr<'m, 'd>(sym: Self::Sym<'m, 'd>) -> RefPtr<Self::Fun<'d>>;
// }

// // cat-set-int

// pub struct CategoricalSetInt;

// impl Set for CategoricalSetInt {
//     type Sym<'m, 'd>
//         = CatSet<'m, 'd, usize>
//     where
//         'd: 'm;

//     type Fun<'d> = CatSetFun<'d, usize>;

//     type RawIdx<'d> = usize;

//     fn fun_ptr<'m, 'd>(sym: Self::Sym<'m, 'd>) -> RefPtr<Self::Fun<'d>> {
//         sym.fun_ptr()
//     }
// }

// // cat-set-str

// pub struct CategoricalSetStr;

// impl Set for CategoricalSetStr {
//     type Sym<'m, 'd>
//         = CatSet<'m, 'd, &'d str>
//     where
//         'd: 'm;

//     type Fun<'d> = CatSetFun<'d, &'d str>;

//     type RawIdx<'d> = &'d str;

//     fn fun_ptr<'m, 'd>(sym: Self::Sym<'m, 'd>) -> RefPtr<Self::Fun<'d>> {
//         sym.fun_ptr()
//     }
// }

// // ord-set

// pub struct OrdinalSet;

// impl Set for OrdinalSet {
//     type Sym<'m, 'd>
//         = OrdSet<'m, 'd>
//     where
//         'd: 'm;

//     type Fun<'d> = OrdSetFun<'d>;

//     type RawIdx<'d> = usize;

//     fn fun_ptr<'m, 'd>(sym: Self::Sym<'m, 'd>) -> RefPtr<Self::Fun<'d>> {
//         sym.fun_ptr()
//     }
// }
