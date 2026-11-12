use crate::idxs::{CatIdxInt, CatIdxStr, IdxBag, IdxSlots, OrdIdx};
use crate::par::p1::{IntParD1, NumParD1, P1Bag, P1Slots};
use crate::sets::{CatSetInt, CatSetStr, OrdSet, SetBag, SetSlots};
use crate::vals::{Int, Num, ValBag, ValFun};
use crate::{MetaInt, MetaNum, MetaSet, Res, SetTyp, Symbol, Validation};
use alloc::boxed::Box;

#[derive(Default)]
pub struct Model<'d> {
    // sets
    set_slots: SetSlots<'d>,
    set_bag: SetBag<'d>,

    // idxs
    idx_slots: IdxSlots<'d>,
    idx_bag: IdxBag<'d>,

    // vals
    val_bag: ValBag<'d>,

    // pars
    p1_slots: P1Slots<'d>,
    p1_bag: P1Bag<'d>,
}

impl<'d> Model<'d> {
    // sets

    pub fn cat_set_int<'m>(&'m self) -> CatSetInt<'m, 'd> {
        let slot = self.set_slots.push(SetTyp::CatSetInt);
        let el = self.set_bag.push(slot);
        Symbol::new(self, el).into()
    }

    pub fn cat_set_str<'m>(&'m self) -> CatSetStr<'m, 'd> {
        let slot = self.set_slots.push(SetTyp::CatSetStr);
        let el = self.set_bag.push(slot);
        Symbol::new(self, el).into()
    }

    pub fn ord_set<'m>(&'m self) -> OrdSet<'m, 'd> {
        let slot = self.set_slots.push(SetTyp::OrdSet);
        let el = self.set_bag.push(slot);
        Symbol::new(self, el).into()
    }

    // idxs

    pub(crate) fn cat_idx_int<'m>(&'m self, set: CatSetInt<'m, 'd>) -> CatIdxInt<'m, 'd> {
        Res::dbg_assert_same_model(self, set);
        let slot = self.idx_slots.push(SetTyp::CatSetInt);
        let el = self.idx_bag.push(unsafe { set.el.get() }.clone(), slot);
        Symbol::new(self, el).into()
    }

    pub(crate) fn cat_idx_str<'m>(&'m self, set: CatSetStr<'m, 'd>) -> CatIdxStr<'m, 'd> {
        Res::dbg_assert_same_model(self, set);
        let slot = self.idx_slots.push(SetTyp::CatSetStr);
        let el = self.idx_bag.push(unsafe { set.el.get() }.clone(), slot);
        Symbol::new(self, el).into()
    }

    pub(crate) fn ord_idx<'m>(&'m self, set: OrdSet<'m, 'd>) -> OrdIdx<'m, 'd> {
        Res::dbg_assert_same_model(self, set);
        let slot = self.idx_slots.push(SetTyp::OrdSet);
        let el = self.idx_bag.push(unsafe { set.el.get() }.clone(), slot);
        Symbol::new(self, el).into()
    }

    // vals

    pub(crate) fn val_int<'m>(&'m self, fun: impl Fn() -> i64 + 'd) -> Int<'m, 'd> {
        let el = self.val_bag.push(ValFun::Int(Box::new(fun)));
        Symbol::new(self, el).into()
    }

    pub(crate) fn val_num<'m>(&'m self, fun: impl Fn() -> f64 + 'd) -> Num<'m, 'd> {
        let el = self.val_bag.push(ValFun::Num(Box::new(fun)));
        Symbol::new(self, el).into()
    }

    // p1

    pub fn par_d1_int<'m, I0: MetaSet>(&'m self, s0: I0::Set<'m, 'd>) -> IntParD1<'m, 'd, I0> {
        let par = self.p1_slots.push::<I0, MetaInt>();
        let el = self.p1_bag.push(par, I0::set_slot(s0));
        Symbol::new(self, el).into()
    }

    pub fn par_d1_num<'m, I0: MetaSet>(&'m self, s0: I0::Set<'m, 'd>) -> NumParD1<'m, 'd, I0> {
        let par = self.p1_slots.push::<I0, MetaNum>();
        let el = self.p1_bag.push(par, I0::set_slot(s0));
        Symbol::new(self, el).into()
    }
}
