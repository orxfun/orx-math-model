use crate::sets::DSet;
use crate::vars::{v1::DV1, VarType};
use crate::{new_bag, Bag, Ptr};
use orx_imp_vec::Elem;

pub struct V1Bag<'d> {
    bag: Bag<DV1<'d>>,
}

impl Default for V1Bag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> V1Bag<'d> {
    pub fn push<'m>(&'m self, typ: VarType, s0_ptr: Ptr<'d, DSet<'d>>) -> Elem<'m, DV1<'d>> {
        self.bag.push(DV1::new(typ, s0_ptr))
    }
}
