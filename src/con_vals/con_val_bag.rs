use crate::con_vals::{ConVal, DConVal};
use crate::{new_bag, BagKind};
use orx_imp_vec::Elem;

pub struct ConValBag {
    bag: BagKind<DConVal>,
}

impl Default for ConValBag {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl ConValBag {
    pub fn push<'m>(&'m self, fun: ConVal) -> Elem<'m, DConVal> {
        self.bag.push(DConVal::new(fun))
    }
}
