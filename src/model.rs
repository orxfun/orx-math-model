use crate::idxs::{IdxBag, IdxValBag};
// use crate::arb_val::{ArbVal, ArbValBag};
// use crate::idxs::{CatIdxInt, CatIdxStr, IdxBag, IdxSlots, OrdIdx};
// use crate::par::p1::{IntParD1, NumParD1, P1Bag, P1Slots};
use crate::sets::{SetBag, SetCatInt, SetCatStr, SetFunBag, SetOrd};
use crate::slot::{SlotStates, State};
// use crate::vals::{Int, Num, ValBag, ValFun};
use crate::{MetaCatInt, MetaCatStr, MetaOrd, MetaSet, Ptr, Res, ResValid, SetTyp, Slot, Symbol};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

#[derive(Default)]
pub struct Model<'d> {
    // states
    slot_states: SlotStates,

    // sets
    set_fun_bag: SetFunBag<'d>,
    set_bag: SetBag<'d>,

    // idxs
    idx_val_bag: IdxValBag<'d>,
    idx_bag: IdxBag<'d>,
    //
    // idx_slots: IdxSlots<'d>,
    // idx_bag: IdxBag<'d>,

    // vals
    // val_bag: ValBag<'d>,
    // arb_val_bag: ArbValBag<'d>,
    //
    // // pars
    // p1_slots: P1Slots<'d>,
    // p1_bag: P1Bag<'d>,
}

impl<'d> Model<'d> {
    // states

    pub(crate) fn slot_state(&self) -> Elem<'_, State> {
        self.slot_states.push()
    }

    // sets

    pub fn cat_set_int<'m>(&'m self) -> SetCatInt<'m, 'd> {
        self.set::<MetaCatInt>()
    }

    pub fn cat_set_str<'m>(&'m self) -> SetCatStr<'m, 'd> {
        self.set::<MetaCatStr>()
    }

    pub fn ord_set<'m>(&'m self) -> SetOrd<'m, 'd> {
        self.set::<MetaOrd>()
    }

    fn set<'m, S: MetaSet>(&'m self) -> S::SetSym<'m, 'd> {
        let fun_el = self.set_fun_bag.push(S::typ());
        let fun_slot = Slot::new(self, fun_el);
        let el = self.set_bag.push(fun_slot);
        S::set_sym(self, el)
    }

    // idxs

    pub(crate) fn idx<'m, S: MetaSet>(&'m self, set: S::SetSym<'m, 'd>) -> S::IdxSym<'m, 'd> {
        Res::dbg_assert_same_model(self, set);
        let val_el = self.idx_val_bag.push(S::typ());
        let val_slot = Slot::new(self, val_el);
        let el = self.idx_bag.push(S::set_ptr(set), val_slot);
        S::new_idx_sym(self, el)
    }

    // pub(crate) fn cat_idx_int<'m>(&'m self, set: CatSetInt<'m, 'd>) -> CatIdxInt<'m, 'd> {
    //     self.idx::<MetaCatInt>(set)
    // }

    // pub(crate) fn cat_idx_str<'m>(&'m self, set: CatSetStr<'m, 'd>) -> CatIdxStr<'m, 'd> {
    //     self.idx::<MetaCatStr>(set)
    // }

    // pub(crate) fn ord_idx<'m>(&'m self, set: OrdSet<'m, 'd>) -> OrdIdx<'m, 'd> {
    //     self.idx::<MetaOrd>(set)
    // }

    // fn idx<'m, M: MetaSet>(&'m self, set: M::SetSym<'m, 'd>) -> M::IdxSym<'m, 'd> {
    //     Res::dbg_assert_same_model(self, set);
    //     let idx = self.idx_slots.push(self, M::typ());
    //     let set = set.ptr();
    //     let el = self.idx_bag.push(set, idx);
    //     M::create_idx_sym(self, el)
    // }

    // vals

    // pub(crate) fn val<'m, V: MetaVal>(&'m self, fun: impl Fn() -> V::Val + 'd) -> V::Sym<'m, 'd> {
    //     let el = self.val_bag.push(V::create_val_fun(fun));
    //     V::create_sym(self, el)
    // }

    // pub(crate) fn val_sym<'m, V: MetaVal>(&'m self, sym: V::Sym<'m, 'd>) -> &'m V::Sym<'m, 'd> {
    //     let sym: ValSymSum<'d, 'd> = match sym.into_sym_sum() {
    //         ValSymSum::Int(x) => ValSymSum::Int(unsafe { x.create_sym_ref() }.into()),
    //         ValSymSum::Num(x) => ValSymSum::Num(unsafe { x.create_sym_ref() }.into()),
    //     };
    //     let el = self.arb_val_bag.push(sym);
    //     <V::Sym<'m, 'd> as Val<'m, 'd>>::from_sum_to_self(unsafe { el.get() }.as_val_sym_sum())
    // }

    // pub(crate) fn arb_val<'m>(&'m self, val: impl Into<ArbVal<'d>>) -> Elem<'m, ArbVal<'d>> {
    //     self.arb_val_bag.push(val)
    // }

    // p1

    // pub fn par_d1_int<'m, S0: Set<'m, 'd>>(&'m self, s0: S0) -> IntParD1<'m, 'd, S0::Meta> {
    //     let par = self.p1_slots.push::<S0::Meta, MetaInt>(self);
    //     let el = self.p1_bag.push(par, s0.slot());
    //     Symbol::new(self, el).into()
    // }

    // pub fn par_d1_num<'m, S0: Set<'m, 'd>>(&'m self, s0: S0) -> NumParD1<'m, 'd, S0::Meta> {
    //     let par = self.p1_slots.push::<S0::Meta, MetaNum>(self);
    //     let el = self.p1_bag.push(par, s0.slot());
    //     Symbol::new(self, el).into()
    // }
}
