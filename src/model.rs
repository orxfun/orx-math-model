use crate::con_vals::{ConVal, DConVal};
use crate::constraints::{Constraint, DCon};
use crate::idxs::{DIdx, IdxVal};
use crate::lin_expr::DLinExpr;
use crate::pars::{DPars, IntParD0, IntParD1, IntParD2, NumParD0, NumParD1, NumParD2, ParFun};
use crate::scalars::{DNum, DVar, Int, Num, Val, ValNum, Var};
use crate::sets::{CatInt, CatStr, DSet, FunSet, Ord};
use crate::slot::State;
use crate::sym_ref::SymRef;
use crate::vars::{DVars, VarD0, VarD1, VarD2};
use crate::{
    AsF64, Bag, DimVec, LinExpr, Ptr, Set, SetCatInt, SetCatStr, SetOrd, SetSym, Slot, Symbol,
    TConVal, ValTyp, VarType,
};
use alloc::string::{String, ToString};
use core::mem::ManuallyDrop;
use orx_imp_vec::Elem;

#[derive(Default)]
pub struct Model<'d> {
    // states
    slot_states: Bag<State>,

    // sets
    set_fun_bag: Bag<FunSet<'d>>,
    set_bag: Bag<DSet<'d>>,

    // pars
    par_fun_bag: Bag<ParFun<'d>>,
    par_bag: Bag<DPars<'d>>,

    // vars
    vars_bag: Bag<DVars<'d>>,

    // idxs
    idx_val_bag: Bag<IdxVal<'d>>,
    idx_bag: Bag<DIdx<'d>>,

    // vals
    num_bag: Bag<DNum<'d>>,
    var_bag: ManuallyDrop<Bag<DVar<'d>>>,
    con_val_bag: Bag<DConVal>,
    sym_ref_bag: Bag<SymRef<'d>>,

    // expr
    lin_exp_bag: Bag<DLinExpr<'d>>,
    constraints: Bag<DCon<'d>>,
}

impl<'d> Drop for Model<'d> {
    fn drop(&mut self) {
        // SAFETY: needs to be dropped before set_fun_bag
        unsafe { ManuallyDrop::drop(&mut self.var_bag) };
    }
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self::default()
    }

    // states

    pub(crate) fn slot_state(&self) -> Elem<'_, State> {
        self.slot_states.push(State::Empty)
    }

    // sets

    pub fn cat_set_int<'m>(&'m self, key: impl ToString) -> CatInt<'m, 'd> {
        self.set::<SetCatInt>(key)
    }

    pub fn cat_set_str<'m>(&'m self, key: impl ToString) -> CatStr<'m, 'd> {
        self.set::<SetCatStr>(key)
    }

    pub fn ord_set<'m>(&'m self, key: impl ToString) -> Ord<'m, 'd> {
        self.set::<SetOrd>(key)
    }

    pub(crate) fn set<'m, S: Set>(&'m self, key: impl ToString) -> S::SetSym<'m, 'd> {
        let fun = FunSet::empty(S::typ());
        let fun_el = self.set_fun_bag.push(fun);
        let fun_slot = Slot::new(self, fun_el);
        let data = DSet::new(key.to_string(), fun_slot);
        let el = self.set_bag.push(data);
        S::new_set_sym(self, el)
    }

    // idxs

    pub(crate) fn idx<'m, S: Set>(&'m self, set: S::SetSym<'m, 'd>) -> S::IdxSym<'m, 'd> {
        // TODO: Res::dbg_assert_same_model(self, set);
        let val = IdxVal::empty(S::typ());
        let val_el = self.idx_val_bag.push(val);
        let val_slot = Slot::new(self, val_el);
        let data = DIdx::new(S::set_ptr(set), val_slot);
        let el = self.idx_bag.push(data);
        S::new_idx_sym(self, el)
    }

    // p0

    fn par0_elem<'m>(&'m self, key: impl ToString, val_typ: ValTyp) -> Elem<'m, DPars<'d>> {
        let fun = ParFun::empty(val_typ, DimVec::new([]));
        let fun_el = self.par_fun_bag.push(fun);
        let fun_slot = Slot::new(self, fun_el);
        let sets = DimVec::new([]);
        self.par_bag
            .push(DPars::new(key.to_string(), sets, fun_slot))
    }

    pub fn par0_int<'m>(&'m self, key: impl ToString) -> IntParD0<'m, 'd> {
        IntParD0::new(Symbol::new(self, self.par0_elem(key, ValTyp::Int)))
    }

    pub fn par0_num<'m>(&'m self, key: impl ToString) -> NumParD0<'m, 'd> {
        NumParD0::new(Symbol::new(self, self.par0_elem(key, ValTyp::Num)))
    }

    // p1

    fn par1_elem<'m, S0>(
        &'m self,
        key: impl ToString,
        val_typ: ValTyp,
        s0: S0,
    ) -> Elem<'m, DPars<'d>>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        let t = [<S0::Meta as Set>::typ()];
        let fun = ParFun::empty(val_typ, DimVec::new(t));
        let fun_el = self.par_fun_bag.push(fun);
        let fun_slot = Slot::new(self, fun_el);
        let sets = DimVec::new([<S0::Meta as Set>::set_ptr(s0)]);
        self.par_bag
            .push(DPars::new(key.to_string(), sets, fun_slot))
    }

    pub fn par1_int<'m, S0>(&'m self, key: impl ToString, s0: S0) -> IntParD1<'m, 'd, S0::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        IntParD1::new(Symbol::new(self, self.par1_elem(key, ValTyp::Int, s0)))
    }

    pub fn par1_num<'m, S0>(&'m self, key: impl ToString, s0: S0) -> NumParD1<'m, 'd, S0::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        NumParD1::new(Symbol::new(self, self.par1_elem(key, ValTyp::Num, s0)))
    }

    // p2

    fn par2_elem<'m, S0, S1>(
        &'m self,
        key: impl ToString,
        val_typ: ValTyp,
        s0: S0,
        s1: S1,
    ) -> Elem<'m, DPars<'d>>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
        S1: SetSym<'m, 'd>,
        S1::Meta: Set<SetSym<'m, 'd> = S1>,
    {
        let t = [<S0::Meta as Set>::typ(), <S1::Meta as Set>::typ()];
        let fun = ParFun::empty(val_typ, DimVec::new(t));
        let fun_el = self.par_fun_bag.push(fun);
        let fun_slot = Slot::new(self, fun_el);
        let sets = DimVec::new([
            <S0::Meta as Set>::set_ptr(s0),
            <S1::Meta as Set>::set_ptr(s1),
        ]);
        self.par_bag
            .push(DPars::new(key.to_string(), sets, fun_slot))
    }

    pub fn par2_int<'m, S0, S1>(
        &'m self,
        key: impl ToString,
        s0: S0,
        s1: S1,
    ) -> IntParD2<'m, 'd, S0::Meta, S1::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
        S1: SetSym<'m, 'd>,
        S1::Meta: Set<SetSym<'m, 'd> = S1>,
    {
        IntParD2::new(Symbol::new(self, self.par2_elem(key, ValTyp::Int, s0, s1)))
    }

    pub fn par2_num<'m, S0, S1>(
        &'m self,
        key: impl ToString,
        s0: S0,
        s1: S1,
    ) -> NumParD2<'m, 'd, S0::Meta, S1::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
        S1: SetSym<'m, 'd>,
        S1::Meta: Set<SetSym<'m, 'd> = S1>,
    {
        NumParD2::new(Symbol::new(self, self.par2_elem(key, ValTyp::Num, s0, s1)))
    }

    // vars

    pub fn var0<'m>(&'m self, key: impl ToString, typ: VarType) -> VarD0<'m, 'd> {
        let data = DVars::new_d0(key.to_string(), typ);
        let el = self.vars_bag.push(data);
        VarD0::new(Symbol::new(self, el))
    }

    pub fn var1<'m, S0>(
        &'m self,
        key: impl ToString,
        typ: VarType,
        s0: S0,
    ) -> VarD1<'m, 'd, S0::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        let s0_ptr = <S0::Meta as Set>::set_ptr(s0);
        let data = DVars::new_d1(key.to_string(), typ, s0_ptr);
        let el = self.vars_bag.push(data);
        VarD1::new(Symbol::new(self, el))
    }

    pub fn var2<'m, S0, S1>(
        &'m self,
        key: impl ToString,
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
        let data = DVars::new_d2(key.to_string(), typ, s0_ptr, s1_ptr);
        let el = self.vars_bag.push(data);
        VarD2::new(Symbol::new(self, el))
    }

    // scalars

    pub(crate) fn val<'m, V: Val>(&'m self, data: DNum<'d>) -> V::Sym<'m, 'd> {
        debug_assert_eq!(data.typ, V::typ());
        self.val_unchecked::<V>(data)
    }

    pub(crate) fn val_unchecked<'m, V: Val>(&'m self, data: DNum<'d>) -> V::Sym<'m, 'd> {
        let el = self.num_bag.push(data);
        V::sym(Symbol::new(self, el))
    }

    pub(crate) fn val_num_con<'m>(&'m self, num: impl AsF64) -> Num<'m, 'd> {
        self.val::<ValNum>(DNum::new_num(move || num))
    }

    pub(crate) fn var<'m>(&'m self, data: DVar<'d>) -> Var<'m, 'd> {
        let el = self.var_bag.push(data);
        Var::new(Symbol::new(self, el))
    }

    // vals

    pub(crate) fn con_val<T: TConVal>(&self, value: T) -> Ptr<'d, DConVal> {
        let val = ConVal::new(value);
        let data = DConVal::new(val);
        let el = self.con_val_bag.push(data);
        Slot::new(self, el).ptr()
    }

    pub(crate) fn int_ref<'m>(&'m self, sym: Int<'m, 'd>) -> &'m Int<'m, 'd> {
        let el = self.sym_ref_bag.push(SymRef::int(sym));
        unsafe { el.get() }.get_int()
    }

    pub(crate) fn num_ref<'m>(&'m self, sym: Num<'m, 'd>) -> &'m Num<'m, 'd> {
        let el = self.sym_ref_bag.push(SymRef::num(sym));
        unsafe { el.get() }.get_num()
    }

    pub(crate) fn var_ref<'m>(&'m self, sym: Var<'m, 'd>) -> &'m Var<'m, 'd> {
        let el = self.sym_ref_bag.push(SymRef::var(sym));
        unsafe { el.get() }.get_var()
    }

    // expr

    pub(crate) fn lin_exp<'m>(&'m self, data: DLinExpr<'d>) -> LinExpr<'m, 'd> {
        LinExpr::new(Symbol::new(self, self.lin_exp_bag.push(data)))
    }

    pub(crate) fn constraint<'m>(&'m self, data: DCon<'d>) -> Constraint<'m, 'd> {
        Constraint::new(Symbol::new(self, self.constraints.push(data)))
    }
}
