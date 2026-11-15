use crate::con_vals::{ConVal, DConVal};
use crate::sym_refs::SymRef;
use crate::{new_bag, Bag, SetTyp, Slot, Val};
use orx_imp_vec::Elem;

pub struct SymRefBag<'d> {
    bag: Bag<SymRef<'d>>,
}

impl Default for SymRefBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> SymRefBag<'d> {
    pub fn push<'a, 'm, S>(&'a self, val: S) -> &'a S
    where
        S: Into<SymRef<'d>>,
    {
        let el = self.bag.push(val.into());
        let x = unsafe { el.get() };
        todo!()
    }
}
