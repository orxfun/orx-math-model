// use crate::ptr::RefPtr;
// use crate::sets::{CatSet, CatSetFun, OrdSet, OrdSetFun};

// // ptr

// pub enum SetFunPtr<'d> {
//     CatInt(RefPtr<CatSetFun<'d, usize>>),
//     CatStr(RefPtr<CatSetFun<'d, &'d str>>),
//     Ord(RefPtr<OrdSetFun<'d>>),
// }

// // fun

// pub enum SetFun<'d> {
//     CatInt(CatSetFun<'d, usize>),
//     CatStr(CatSetFun<'d, &'d str>),
//     Ord(OrdSetFun<'d>),
// }

// // symbol

// pub enum Set<'m, 'd> {
//     CatInt(CatSet<'m, 'd, usize>),
//     CatStr(CatSet<'m, 'd, &'d str>),
//     Ord(OrdSet<'m, 'd>),
// }

use crate::{
    ptr::RefPtr,
    sets::{CatSet, CatSetFun, OrdSet, OrdSetFun},
};

pub trait Set {
    type Fun;

    type RawIdx;

    fn fun_ptr(&self) -> RefPtr<Self::Fun>;
}

// cat-set-int

impl<'m, 'd> Set for CatSet<'m, 'd, usize> {
    type Fun = CatSetFun<'d, usize>;

    type RawIdx = usize;

    fn fun_ptr(&self) -> RefPtr<Self::Fun> {
        Self::fun_ptr(self)
    }
}

// cat-set-str

impl<'m, 'd> Set for CatSet<'m, 'd, &'d str> {
    type Fun = CatSetFun<'d, &'d str>;

    type RawIdx = &'d str;

    fn fun_ptr(&self) -> RefPtr<Self::Fun> {
        Self::fun_ptr(self)
    }
}

// ord-set

impl<'m, 'd> Set for OrdSet<'m, 'd> {
    type Fun = OrdSetFun<'d>;

    type RawIdx = usize;

    fn fun_ptr(&self) -> RefPtr<Self::Fun> {
        Self::fun_ptr(self)
    }
}
