use super::bag::{new_bag, Bag};
use crate::sets::{IndicesFun, SetFun};
use orx_imp_vec::Elem;

pub struct Sets<'d> {
    int: Bag<SetFun<'d, usize>>,
    str: Bag<SetFun<'d, &'d str>>,
    indices: Bag<IndicesFun<'d>>,
}

impl<'d> Sets<'d> {
    pub fn new() -> Self {
        Self {
            int: new_bag(),
            str: new_bag(),
            indices: new_bag(),
        }
    }

    pub fn int<'m>(&'m self) -> Elem<'m, SetFun<'d, usize>> {
        self.int.push(SetFun::empty())
    }

    pub fn str<'m>(&'m self) -> Elem<'m, SetFun<'d, &'d str>> {
        self.str.push(SetFun::empty())
    }

    pub fn indices<'m>(&'m self) -> Elem<'m, IndicesFun<'d>> {
        self.indices.push(IndicesFun::empty())
    }
}
