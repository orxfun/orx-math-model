use crate::ptr::RefPtr;
use crate::sets::{CatSet, CatSetFun, OrdSet, OrdSetFun};

pub trait Set {
    type Sym<'m, 'd>
    where
        'd: 'm;

    type Fun<'d>;

    type RawIdx<'d>;

    fn fun_ptr<'m, 'd>(sym: Self::Sym<'m, 'd>) -> RefPtr<Self::Fun<'d>>;
}

// cat-set-int

pub struct CategoricalSetInt;

impl Set for CategoricalSetInt {
    type Sym<'m, 'd>
        = CatSet<'m, 'd, usize>
    where
        'd: 'm;

    type Fun<'d> = CatSetFun<'d, usize>;

    type RawIdx<'d> = usize;

    fn fun_ptr<'m, 'd>(sym: Self::Sym<'m, 'd>) -> RefPtr<Self::Fun<'d>> {
        sym.fun_ptr()
    }
}

// cat-set-str

pub struct CategoricalSetStr;

impl Set for CategoricalSetStr {
    type Sym<'m, 'd>
        = CatSet<'m, 'd, &'d str>
    where
        'd: 'm;

    type Fun<'d> = CatSetFun<'d, &'d str>;

    type RawIdx<'d> = &'d str;

    fn fun_ptr<'m, 'd>(sym: Self::Sym<'m, 'd>) -> RefPtr<Self::Fun<'d>> {
        sym.fun_ptr()
    }
}

// ord-set

pub struct OrdinalSet;

impl Set for OrdinalSet {
    type Sym<'m, 'd>
        = OrdSet<'m, 'd>
    where
        'd: 'm;

    type Fun<'d> = OrdSetFun<'d>;

    type RawIdx<'d> = usize;

    fn fun_ptr<'m, 'd>(sym: Self::Sym<'m, 'd>) -> RefPtr<Self::Fun<'d>> {
        sym.fun_ptr()
    }
}
