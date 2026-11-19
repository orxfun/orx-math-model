use crate::con_vals::{ConVal, DConVal};
use crate::constraints::DCon;
use crate::idxs::{DIdx, IdxVal};
use crate::lin_expr::DLinExpr;
use crate::pars::{DPars, IntParD1, IntParD2, NumParD1, NumParD2, ParFun};
use crate::scalars::{CommonNumbers, DNum, DVar, Int, Num, Val, ValNum, Var};
// use crate::lin_expr::{DLinExpr, LinExpBag, LinExpr};
// use crate::pars::{ParBag, ParD1, ParD2, ParFun, ParFunBag};
// use crate::scalars::{DSca, FVar1, FVar2, ScaBag};
use crate::sets::{CatInt, CatStr, DSet, FunSet, Ord, SetBag, SetFunBag};
use crate::slot::{SlotStates, State};
use crate::sym_ref::SymRef;
use crate::vars::{DVars, VarD1, VarD2};
// use crate::sym_refs::SymRefBag;
// use crate::vars::{DVars, VarBag, VarD1, VarD2, VarType};
use crate::{
    AsF64, AsI64, Bag, DimVec, LinExpr, Ptr, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd,
    SetSym, Slot, SymWrap, Symbol, TConVal, ValTyp, VarType,
};
// use crate::{Int, Num, Sca, ScaInt, ScaNum, Val, Var};
use alloc::boxed::Box;
use core::mem::ManuallyDrop;
use orx_imp_vec::Elem;

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
    // sca_bag: ManuallyDrop<ScaBag<'d>>,
    con_val_bag: Bag<DConVal>,
    sym_ref_bag: Bag<SymRef<'d>>,
    common_numbers: CommonNumbers<'d>,

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
        let num_bag = Bag::default();
        let common_numbers = CommonNumbers::new(&num_bag);
        Self {
            slot_states: Default::default(),
            set_fun_bag: Default::default(),
            set_bag: Default::default(),
            par_fun_bag: Default::default(),
            par_bag: Default::default(),
            vars_bag: Default::default(),
            idx_val_bag: Default::default(),
            idx_bag: Default::default(),
            num_bag,
            var_bag: Default::default(),
            con_val_bag: Default::default(),
            sym_ref_bag: Default::default(),
            common_numbers,
            lin_exp_bag: Default::default(),
            constraints: Default::default(),
        }
    }

    // states

    pub(crate) fn slot_state(&self) -> Elem<'_, State> {
        self.slot_states.push(State::Empty)
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

    pub(crate) fn set<'m, S: Set>(&'m self) -> S::SetSym<'m, 'd> {
        let fun = FunSet::empty(S::typ());
        let fun_el = self.set_fun_bag.push(fun);
        let fun_slot = Slot::new(self, fun_el);
        let data = DSet::new(fun_slot);
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

    // p1

    fn par1_elem<'m, S0>(&'m self, val_typ: ValTyp, s0: S0) -> Elem<'m, DPars<'d>>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        let t = [<S0::Meta as Set>::typ()];
        let fun = ParFun::empty(val_typ, DimVec::new(t));
        let fun_el = self.par_fun_bag.push(fun);
        let fun_slot = Slot::new(self, fun_el);
        let sets = DimVec::new([<S0::Meta as Set>::set_ptr(s0)]);
        self.par_bag.push(DPars::new(sets, fun_slot))
    }

    pub fn par1_int<'m, S0>(&'m self, s0: S0) -> IntParD1<'m, 'd, S0::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        IntParD1::new(Symbol::new(self, self.par1_elem(ValTyp::Int, s0)))
    }

    pub fn par1_num<'m, S0>(&'m self, s0: S0) -> NumParD1<'m, 'd, S0::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        NumParD1::new(Symbol::new(self, self.par1_elem(ValTyp::Num, s0)))
    }

    // p2

    fn par2_elem<'m, S0, S1>(&'m self, val_typ: ValTyp, s0: S0, s1: S1) -> Elem<'m, DPars<'d>>
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
        self.par_bag.push(DPars::new(sets, fun_slot))
    }

    pub fn par2_int<'m, S0, S1>(&'m self, s0: S0, s1: S1) -> IntParD2<'m, 'd, S0::Meta, S1::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
        S1: SetSym<'m, 'd>,
        S1::Meta: Set<SetSym<'m, 'd> = S1>,
    {
        IntParD2::new(Symbol::new(self, self.par2_elem(ValTyp::Int, s0, s1)))
    }

    pub fn par2_num<'m, S0, S1>(&'m self, s0: S0, s1: S1) -> NumParD2<'m, 'd, S0::Meta, S1::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
        S1: SetSym<'m, 'd>,
        S1::Meta: Set<SetSym<'m, 'd> = S1>,
    {
        NumParD2::new(Symbol::new(self, self.par2_elem(ValTyp::Num, s0, s1)))
    }

    // vars

    pub fn var1<'m, S0>(&'m self, typ: VarType, s0: S0) -> VarD1<'m, 'd, S0::Meta>
    where
        S0: SetSym<'m, 'd>,
        S0::Meta: Set<SetSym<'m, 'd> = S0>,
    {
        let s0_ptr = <S0::Meta as Set>::set_ptr(s0);
        let data = DVars::new_d1(typ, s0_ptr);
        let el = self.vars_bag.push(data);
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
        let el = self.vars_bag.push(data);
        VarD2::new(Symbol::new(self, el))
    }

    // scalars

    pub(crate) fn one<'m>(&'m self) -> Int<'m, 'd> {
        let el = unsafe { self.common_numbers.one.into_el() };
        Int::new(Symbol::new(self, el))
    }

    pub(crate) fn minus_one<'m>(&'m self) -> Int<'m, 'd> {
        let el = unsafe { self.common_numbers.minus_one.into_el() };
        Int::new(Symbol::new(self, el))
    }

    pub(crate) fn zero<'m>(&'m self) -> Int<'m, 'd> {
        let el = unsafe { self.common_numbers.zero.into_el() };
        Int::new(Symbol::new(self, el))
    }

    pub(crate) fn val<'m, V: Val>(&'m self, data: DNum<'d>) -> V::Sym<'m, 'd> {
        debug_assert_eq!(data.typ, V::typ());
        self.val_unchecked::<V>(data)
    }

    pub(crate) fn val_unchecked<'m, V: Val>(&'m self, data: DNum<'d>) -> V::Sym<'m, 'd> {
        let el = self.num_bag.push(data);
        V::sym(Symbol::new(self, el))
    }

    pub(crate) fn val_neg<'m>(&'m self, c: Ptr<'d, DNum<'d>>) -> Ptr<'d, DNum<'d>> {
        assert_eq!(3, 33);
        self.val::<ValNum>(DNum::neg(c)).ptr()
    }

    pub(crate) fn val_add<'m>(
        &'m self,
        c1: Ptr<'d, DNum<'d>>,
        c2: Ptr<'d, DNum<'d>>,
    ) -> Ptr<'d, DNum<'d>> {
        self.val::<ValNum>(DNum::add(c1, c2)).ptr()
    }

    pub(crate) fn val_sub<'m>(
        &'m self,
        c1: Ptr<'d, DNum<'d>>,
        c2: Ptr<'d, DNum<'d>>,
    ) -> Ptr<'d, DNum<'d>> {
        self.val::<ValNum>(DNum::sub(c1, c2)).ptr()
    }

    pub(crate) fn val_mul<'m>(
        &'m self,
        c1: Ptr<'d, DNum<'d>>,
        c2: Ptr<'d, DNum<'d>>,
    ) -> Ptr<'d, DNum<'d>> {
        self.val::<ValNum>(DNum::mul(c1, c2)).ptr()
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

    pub(crate) fn constraint<'m>(&'m self, data: DCon<'d>) {
        self.constraints.push(data);
    }
}
