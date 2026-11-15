use crate::scalars::var::{FV1, FV2};
use crate::scalars::{DVal, DVar, FVal, ScaTyp, Val, ValTyp};
use crate::sets::FunSet;
use crate::{Ptr, Set, VarType};
use core::mem::ManuallyDrop;

#[repr(C)]
pub union USca<'d> {
    val: ManuallyDrop<DVal<'d>>,
    var: ManuallyDrop<DVar<'d>>,
}

pub struct DSca<'d> {
    typ: ScaTyp,
    data: USca<'d>,
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

    pub fn new_var_d1<I0: Set>(typ: VarType, i0: Ptr<'d, FunSet<'d>>, fun: FV1<'d, I0>) -> Self {
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
        fun: FV2<'d, I0, I1>,
    ) -> Self {
        let data = USca {
            var: ManuallyDrop::new(DVar::create_d2::<I0, I1>(typ, i0, i1, fun)),
        };
        let typ = ScaTyp::Var;
        Self { typ, data }
    }
}
