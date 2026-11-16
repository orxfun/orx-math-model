use crate::con_vals::{ConVal, ConValBag, DConVal};
use crate::idxs::{IdxBag, IdxValBag};
use crate::lin_expr::{DLinExpr, LinExpBag, LinExpr};
use crate::pars::{ParBag, ParD1, ParD2, ParFun, ParFunBag};
use crate::scalars::{DSca, FVar1, FVar2, ScaBag};
use crate::sets::{CatInt, CatStr, Ord, SetBag, SetFunBag};
use crate::slot::{SlotStates, State};
use crate::sym_refs::SymRefBag;
use crate::vars::{DVars, VarBag, VarD1, VarD2, VarType};
use crate::{
    DimVec, Int, Num, Ptr, Res, ResValid, Sca, ScaInt, ScaNum, Set, SetCatInt, SetCatStr, SetOrd,
    SetSym, Slot, Symbol, TConVal, Val, Var,
};
use alloc::boxed::Box;
use core::mem::ManuallyDrop;
use orx_imp_vec::Elem;

#[derive(Default)]
pub struct Model<'d> {
    // states
    slot_states: SlotStates,

    // sets
    set_fun_bag: SetFunBag<'d>,
    set_bag: SetBag<'d>,

    // pars
    par_fun_bag: ParFunBag<'d>,
    par_bag: ParBag<'d>,

    // vars
    var_bag: VarBag<'d>,

    // idxs
    idx_val_bag: IdxValBag<'d>,
    idx_bag: IdxBag<'d>,

    // vals
    sca_bag: ManuallyDrop<ScaBag<'d>>,
    con_val_bag: ConValBag,
    sym_ref_bag: SymRefBag<'d>,

    // expr
    lin_exp_bag: LinExpBag<'d>,
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
        // TODO: Res::dbg_assert_same_model(self, set);
        let val_el = self.idx_val_bag.push(S::typ());
        let val_slot = Slot::new(self, val_el);
        let el = self.idx_bag.push(S::set_ptr(set), val_slot);
        S::new_idx_sym(self, el)
    }

    // p1

    fn par1<'m, V, S0>(&'m self, s0: S0) -> ParD1<'m, 'd, V, S0::Meta>
    where
        V: Val,
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        let t = [<S0::Meta as Set>::typ()];
        let fun = ParFun::empty(V::val_typ(), DimVec::new(t));
        let fun_el = self.par_fun_bag.push(fun);
        let fun_slot = Slot::new(self, fun_el);
        let sets = DimVec::new([<S0::Meta as Set>::set_ptr(s0)]);
        let el = self.par_bag.push(sets, fun_slot);
        ParD1::new(Symbol::new(self, el))
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
        let t = [<S0::Meta as Set>::typ(), <S1::Meta as Set>::typ()];
        let fun = ParFun::empty(V::val_typ(), DimVec::new(t));
        let fun_el = self.par_fun_bag.push(fun);
        let fun_slot = Slot::new(self, fun_el);
        let sets = DimVec::new([
            <S0::Meta as Set>::set_ptr(s0),
            <S1::Meta as Set>::set_ptr(s1),
        ]);
        let el = self.par_bag.push(sets, fun_slot);
        ParD2::new(Symbol::new(self, el))
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
        let data = DVars::new_d1(typ, s0_ptr);
        let el = self.var_bag.push(data);
        VarD1::new(Symbol::new(self, el))
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
        let data = DVars::new_d2(typ, s0_ptr, s1_ptr);
        let el = self.var_bag.push(data);
        VarD2::new(Symbol::new(self, el))
    }

    // scalars

    pub(crate) fn one<'m>(&'m self) -> Num<'m, 'd> {
        Num::new(Symbol::new(self, self.sca_bag.one))
    }

    pub(crate) fn zero_int<'m>(&'m self) -> Int<'m, 'd> {
        Int::new(Symbol::new(self, self.sca_bag.zero_int))
    }

    pub(crate) fn zero_num<'m>(&'m self) -> Num<'m, 'd> {
        Num::new(Symbol::new(self, self.sca_bag.one))
    }

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
        let parent = var.data_ptr();
        let data = DSca::new_var_d1::<I0>(parent, fun);
        let el = self.sca_bag.push(data);
        Var::new(Symbol::new(self, el))
    }

    pub(crate) fn var_from_d2<'m, I0: Set, I1: Set>(
        &'m self,
        var: VarD2<'m, 'd, I0, I1>,
        fun: FVar2<'d, I0, I1>,
    ) -> Var<'m, 'd> {
        let parent = var.data_ptr();
        let data = DSca::new_var_d2::<I0, I1>(parent, fun);
        let el = self.sca_bag.push(data);
        Var::new(Symbol::new(self, el))
    }

    // vals

    pub(crate) fn con_val<T: TConVal>(&self, value: T) -> Ptr<'d, DConVal> {
        let el = self.con_val_bag.push(ConVal::new(value));
        Slot::new(self, el).ptr()
    }

    pub(crate) fn sym_ref<'m, S: Sca>(&'m self, sym: S::Sym<'m, 'd>) -> &'m S::Sym<'m, 'd> {
        self.sym_ref_bag.push::<S>(sym)
    }

    // expr
    pub(crate) fn lin_exp<'m>(&'m self, data: DLinExpr<'d>) -> LinExpr<'m, 'd> {
        LinExpr::new(Symbol::new(self, self.lin_exp_bag.push(data)))
    }
}
