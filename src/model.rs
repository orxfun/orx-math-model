use crate::idxs::{IdxBag, IdxSlots};
use crate::sets::{CatSetInt, CatSetStr, OrdSet, SetBag, SetSlots, SetType};
use crate::Symbol;

#[derive(Default)]
pub struct Model<'d> {
    // sets
    set_slots: SetSlots<'d>,
    set_bag: SetBag<'d>,

    // idxs
    idx_slots: IdxSlots<'d>,
    idx_bag: IdxBag<'d>,
}

impl<'d> Model<'d> {
    // sets

    pub fn cat_set_int<'m>(&'m self) -> CatSetInt<'m, 'd> {
        let slot = self.set_slots.push(SetType::CatSetInt);
        let el = self.set_bag.push(slot);
        Symbol::new(self, el).into()
    }

    pub fn cat_set_str<'m>(&'m self) -> CatSetStr<'m, 'd> {
        let slot = self.set_slots.push(SetType::CatSetStr);
        let el = self.set_bag.push(slot);
        Symbol::new(self, el).into()
    }

    pub fn ord_set<'m>(&'m self) -> OrdSet<'m, 'd> {
        let slot = self.set_slots.push(SetType::OrdSet);
        let el = self.set_bag.push(slot);
        Symbol::new(self, el).into()
    }
}
