use crate::sets::DSet;
use crate::vars::v2::DV2;
use crate::vars::VarType;
use crate::{new_bag, Bag, Ptr, SetTyp, Slot};
use orx_imp_vec::Elem;

pub struct V2Bag<'d> {
    bag: Bag<DV2<'d>>,
}

impl Default for V2Bag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> V2Bag<'d> {
    pub fn push<'m>(
        &'m self,
        typ: VarType,
        s0_ptr: Ptr<'d, DSet<'d>>,
        s1_ptr: Ptr<'d, DSet<'d>>,
    ) -> Elem<'m, DV2<'d>> {
        self.bag.push(DV2::new(typ, s0_ptr, s1_ptr))
    }
}
