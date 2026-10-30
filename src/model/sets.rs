use super::bag::{new_bag, Bag};
use crate::sets::{CatSetFun, OrdSetFun};
use orx_imp_vec::Elem;

pub struct Sets<'d> {
    cat_int: Bag<CatSetFun<'d, usize>>,
    cat_str: Bag<CatSetFun<'d, &'d str>>,
    ord: Bag<OrdSetFun<'d>>,
}

impl<'d> Sets<'d> {
    pub fn new() -> Self {
        Self {
            cat_int: new_bag(),
            cat_str: new_bag(),
            ord: new_bag(),
        }
    }

    pub fn cat_int<'m>(&'m self) -> Elem<'m, CatSetFun<'d, usize>> {
        self.cat_int.push(CatSetFun::empty())
    }

    pub fn cat_str<'m>(&'m self) -> Elem<'m, CatSetFun<'d, &'d str>> {
        self.cat_str.push(CatSetFun::empty())
    }

    pub fn ord<'m>(&'m self) -> Elem<'m, OrdSetFun<'d>> {
        self.ord.push(OrdSetFun::empty())
    }
}
