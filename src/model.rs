use core::mem::ManuallyDrop;

use crate::con_vals::{ConVal, ConValBag, DConVal};
use crate::idxs::{IdxBag, IdxValBag};
use crate::pars::p1::{P1Bag, P1FunBag, ParD1};
use crate::pars::p2::{P2Bag, P2FunBag, ParD2};
use crate::scalars::{DSca, DVal, DVar, FVar1, FVar2, ScaBag};
// use crate::arb_val::{ArbVal, ArbValBag};
// use crate::idxs::{CatIdxInt, CatIdxStr, IdxBag, IdxSlots, OrdIdx};
// use crate::par::p1::{IntParD1, NumParD1, P1Bag, P1Slots};
use crate::sets::{CatInt, CatStr, Ord, SetBag, SetFunBag};
use crate::slot::{SlotStates, State};
use crate::sym_refs::SymRefBag;
use crate::vars::v1::{V1Bag, VarD1};
use crate::vars::v2::{V2Bag, VarD2};
use crate::vars::VarType;
use crate::{
    Ptr, Res, ResValid, ScaInt, ScaNum, Set, SetCatInt, SetCatStr, SetOrd, SetSym, SetTyp, Slot,
    Symbol, TConVal, Val, Var,
};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

#[derive(Default)]
pub struct Model<'d> {
    // states
    slot_states: SlotStates,

    // sets
    set_fun_bag: SetFunBag<'d>,
    set_bag: SetBag<'d>,

    // pars
    p1_fun_bag: P1FunBag<'d>,
    p1_bag: P1Bag<'d>,
    p2_fun_bag: P2FunBag<'d>,
    p2_bag: P2Bag<'d>,

    // vars
    v1_bag: V1Bag<'d>,
    v2_bag: V2Bag<'d>,

    // idxs
    idx_val_bag: IdxValBag<'d>,
    idx_bag: IdxBag<'d>,

    // vals
    sca_bag: ManuallyDrop<ScaBag<'d>>,
    con_val_bag: ConValBag,
    sym_ref_bag: SymRefBag<'d>,
}

impl<'d> Drop for Model<'d> {
    fn drop(&mut self) {
        // SAFETY: needs to be dropped after set_fun_bag
        unsafe { ManuallyDrop::drop(&mut self.sca_bag) };
    }
}

impl<'d> Model<'d> {
    // states

    pub(crate) fn slot_state(&self) -> Elem<'_, State> {
        self.slot_states.push()
    }

    // sets

    pub fn cat_set_int<'m>(&'m self) -> CatInt<'m, 'd> {
        self.set::<SetCatInt>()
    }

    pub fn cat_set_str<'m>(&'m self) -> CatStr<'m, 'd> {
        self.set::<SetCatStr>()
    }

    pub fn ord_set<'m>(&'m self) -> Ord<'m, 'd> {
        self.set::<SetOrd>()
    }

    fn set<'m, S: Set>(&'m self) -> S::SetSym<'m, 'd> {
        let fun_el = self.set_fun_bag.push(S::typ());
        let fun_slot = Slot::new(self, fun_el);
        let el = self.set_bag.push(fun_slot);
        S::new_set_sym(self, el)
    }

    // idxs

    pub(crate) fn idx<'m, S: Set>(&'m self, set: S::SetSym<'m, 'd>) -> S::IdxSym<'m, 'd> {
        Res::dbg_assert_same_model(self, set);
        let val_el = self.idx_val_bag.push(S::typ());
        let val_slot = Slot::new(self, val_el);
        let el = self.idx_bag.push(S::set_ptr(set), val_slot);
        S::new_idx_sym(self, el)
    }

    // vals

    pub(crate) fn con_val<T: TConVal>(&self, value: T) -> Ptr<'d, DConVal> {
        let el = self.con_val_bag.push(ConVal::new(value));
        Slot::new(self, el).ptr()
    }

    pub(crate) fn sym_ref<'m, V: Val>(&'m self, sym: V::Sym<'m, 'd>) -> &'m V::Sym<'m, 'd> {
        self.sym_ref_bag.push::<V>(sym)
    }

    // p1

    fn par1<'m, V, S0>(&'m self, s0: S0) -> ParD1<'m, 'd, V, S0::Meta>
    where
        V: Val,
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        let fun_el = self.p1_fun_bag.push(V::typ(), <S0::Meta as Set>::typ());
        let fun_slot = Slot::new(self, fun_el);
        let s0_ptr = <S0::Meta as Set>::set_ptr(s0);
        let el = self.p1_bag.push(s0_ptr, fun_slot);
        Symbol::new(self, el).into()
    }

    pub fn par1_int<'m, S0>(&'m self, s0: S0) -> ParD1<'m, 'd, ScaInt, S0::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        self.par1(s0)
    }

    pub fn par1_num<'m, S0>(&'m self, s0: S0) -> ParD1<'m, 'd, ScaNum, S0::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        self.par1(s0)
    }

    // p2

    fn par2<'m, V, S0, S1>(&'m self, s0: S0, s1: S1) -> ParD2<'m, 'd, V, S0::Meta, S1::Meta>
    where
        V: Val,
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
        S1: SetSym<'m, 'd>,
        S1::Meta: Set<SetSym<'m, 'd> = S1>,
    {
        let [t0, t1] = [<S0::Meta as Set>::typ(), <S1::Meta as Set>::typ()];
        let fun_el = self.p2_fun_bag.push(V::typ(), t0, t1);
        let fun_slot = Slot::new(self, fun_el);
        let s0_ptr = <S0::Meta as Set>::set_ptr(s0);
        let s1_ptr = <S1::Meta as Set>::set_ptr(s1);
        let el = self.p2_bag.push(s0_ptr, s1_ptr, fun_slot);
        Symbol::new(self, el).into()
    }

    pub fn par2_int<'m, S0, S1>(
        &'m self,
        s0: S0,
        s1: S1,
    ) -> ParD2<'m, 'd, ScaInt, S0::Meta, S1::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
        S1: SetSym<'m, 'd>,
        S1::Meta: Set<SetSym<'m, 'd> = S1>,
    {
        self.par2(s0, s1)
    }

    pub fn par2_num<'m, S0, S1>(
        &'m self,
        s0: S0,
        s1: S1,
    ) -> ParD2<'m, 'd, ScaNum, S0::Meta, S1::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
        S1: SetSym<'m, 'd>,
        S1::Meta: Set<SetSym<'m, 'd> = S1>,
    {
        self.par2(s0, s1)
    }

    // vars

    pub fn var1<'m, S0>(&'m self, typ: VarType, s0: S0) -> VarD1<'m, 'd, S0::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        let s0_ptr = <S0::Meta as Set>::set_ptr(s0);
        let el = self.v1_bag.push(typ, s0_ptr);
        Symbol::new(self, el).into()
    }

    pub fn var2<'m, S0, S1>(
        &'m self,
        typ: VarType,
        s0: S0,
        s1: S1,
    ) -> VarD2<'m, 'd, S0::Meta, S1::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
        S1: SetSym<'m, 'd>,
        S1::Meta: Set<SetSym<'m, 'd> = S1>,
    {
        let s0_ptr = <S0::Meta as Set>::set_ptr(s0);
        let s1_ptr = <S1::Meta as Set>::set_ptr(s1);
        let el = self.v2_bag.push(typ, s0_ptr, s1_ptr);
        Symbol::new(self, el).into()
    }

    // scalars

    pub(crate) fn val<'m, V: Val>(&'m self, fun: impl Fn() -> V::Raw + 'd) -> V::Sym<'m, 'd> {
        let fun: Box<dyn Fn() -> <V as Val>::Raw + 'd> = Box::new(fun);
        let data = DSca::new_val::<V>(fun);
        let el = self.sca_bag.push(data);
        V::new_sym(self, el)
    }

    pub(crate) fn var_from_d1<'m, I0: Set>(
        &'m self,
        var: VarD1<'m, 'd, I0>,
        fun: FVar1<'d, I0>,
    ) -> Var<'m, 'd> {
        let var = unsafe { var.el.get() };
        let i0 = var.s0.get().fun_set();
        let data = DSca::new_var_d1::<I0>(var.typ, i0, fun);
        let el = self.sca_bag.push(data);
        Symbol::new(self, el).into()
    }

    pub(crate) fn var_from_d2<'m, I0: Set, I1: Set>(
        &'m self,
        var: VarD2<'m, 'd, I0, I1>,
        fun: FVar2<'d, I0, I1>,
    ) -> Var<'m, 'd> {
        let var = unsafe { var.el.get() };
        let i0 = var.s0.get().fun_set();
        let i1 = var.s1.get().fun_set();
        let data = DSca::new_var_d2::<I0, I1>(var.typ, i0, i1, fun);
        let el = self.sca_bag.push(data);
        Symbol::new(self, el).into()
    }
}
