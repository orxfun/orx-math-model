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
    pub fn push<'a, 'm, V: Val>(&'a self, val: V::Sym<'m, 'd>) -> &'a V::Sym<'m, 'd> {
        let el = self.bag.push(SymRef::new(*val));
        unsafe { el.get() }.get::<V>()
    }
}
