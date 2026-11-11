use crate::idxs::{CatIdxInt, CatIdxStr, IdxBag, IdxSlots, OrdIdx};
use crate::sets::{CatSetInt, CatSetStr, OrdSet, SetBag, SetSlots, SetType};
use crate::{Res, Symbol, Validation};

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

    // idxs

    pub(crate) fn cat_idx_int<'m>(&'m self, set: CatSetInt<'m, 'd>) -> Res<CatIdxInt<'m, 'd>> {
        Res::assert_same_model(self, set)?;
        let slot = self.idx_slots.push(SetType::CatSetInt);
        let el = self.idx_bag.push(unsafe { set.el.get() }.clone(), slot);
        Ok(Symbol::new(self, el).into())
    }

    pub(crate) fn cat_idx_str<'m>(&'m self, set: CatSetStr<'m, 'd>) -> Res<CatIdxStr<'m, 'd>> {
        Res::assert_same_model(self, set)?;
        let slot = self.idx_slots.push(SetType::CatSetStr);
        let el = self.idx_bag.push(unsafe { set.el.get() }.clone(), slot);
        Ok(Symbol::new(self, el).into())
    }

    pub(crate) fn ord_idx<'m>(&'m self, set: OrdSet<'m, 'd>) -> Res<OrdIdx<'m, 'd>> {
        Res::assert_same_model(self, set)?;
        let slot = self.idx_slots.push(SetType::OrdSet);
        let el = self.idx_bag.push(unsafe { set.el.get() }.clone(), slot);
        Ok(Symbol::new(self, el).into())
    }
}
