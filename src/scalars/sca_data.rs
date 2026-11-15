use crate::scalars::var::{FVar1, FVar2, Idx1, Idx2};
use crate::scalars::{DVal, DVar, FVal, ScaTyp, Val, ValTyp};
use crate::sets::FunSet;
use crate::{Ptr, Set, VarType};
use core::mem::ManuallyDrop;

#[repr(C)]
pub union USca<'d> {
    pub(crate) val: ManuallyDrop<DVal<'d>>,
    pub(crate) var: ManuallyDrop<DVar<'d>>,
}

pub struct DSca<'d> {
    typ: ScaTyp,
    pub(crate) data: USca<'d>,
}

impl<'d> Drop for DSca<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.typ {
                ScaTyp::Int | ScaTyp::Num => ManuallyDrop::drop(&mut self.data.val),
                ScaTyp::Var => ManuallyDrop::drop(&mut self.data.var),
            }
        }
    }
}

impl<'d> DSca<'d> {
    pub fn new_val<V: Val>(fun: FVal<'d, V>) -> Self {
        let typ = ScaTyp::from(V::typ());
        let data = USca {
            val: ManuallyDrop::new(DVal::new::<V>(fun)),
        };
        Self { typ, data }
    }

    pub fn new_var_d1<I0: Set>(typ: VarType, i0: Ptr<'d, FunSet<'d>>, fun: FVar1<'d, I0>) -> Self {
        let data = USca {
            var: ManuallyDrop::new(DVar::create_d1::<I0>(typ, i0, fun)),
        };
        let typ = ScaTyp::Var;
        Self { typ, data }
    }

    pub fn new_var_d2<I0: Set, I1: Set>(
        typ: VarType,
        i0: Ptr<'d, FunSet<'d>>,
        i1: Ptr<'d, FunSet<'d>>,
        fun: FVar2<'d, I0, I1>,
    ) -> Self {
        let data = USca {
            var: ManuallyDrop::new(DVar::create_d2::<I0, I1>(typ, i0, i1, fun)),
        };
        let typ = ScaTyp::Var;
        Self { typ, data }
    }

    // get

    pub fn var_indices_d1<I0: Set>(&self) -> Idx1<'d, I0> {
        unsafe { &self.data.var }.indices_d1::<I0>()
    }

    pub fn var_indices_d2<I0: Set, I1: Set>(&self) -> Idx2<'d, I0, I1> {
        unsafe { &self.data.var }.indices_d2::<I0, I1>()
    }

    pub fn val_eval<V: Val>(&self) -> V::Raw {
        unsafe { &self.data.val }.eval::<V>()
    }

    pub fn val_typ(&self) -> ValTyp {
        unsafe { &self.data.val }.typ
    }
}
