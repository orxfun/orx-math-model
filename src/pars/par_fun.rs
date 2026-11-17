use crate::pars::p1_fun::{UFunP1, FP1};
use crate::scalars::num::round_f64;
use crate::{Dim, DimVec, Res, Set, SetTyp, ValTyp};
use core::mem::ManuallyDrop;

#[repr(C)]
pub union UFunPar<'d> {
    d1: ManuallyDrop<UFunP1<'d>>,
}

#[derive(derive_new::new)]
pub struct ParFun<'d> {
    pub val_typ: ValTyp,
    pub idx_types: DimVec<SetTyp>,
    data: ManuallyDrop<UFunPar<'d>>,
}

impl<'d> Drop for ParFun<'d> {
    fn drop(&mut self) {
        match self.idx_types.dim() {
            Dim::D1 => unsafe { &mut self.data.d1 }.man_drop(self.idx_types[0]),
            _ => todo!(),
        }
    }
}

impl<'d> ParFun<'d> {
    // d1

    pub fn d1_empty(val_typ: ValTyp, i0_typ: SetTyp) -> Self {
        let idx_types = DimVec::new([i0_typ]);
        let data = ManuallyDrop::new(UFunPar {
            d1: UFunP1::empty(i0_typ),
        });
        Self::new(val_typ, idx_types, data)
    }

    pub fn d1_create<I0: Set>(val_typ: ValTyp, fun: FP1<'d, I0>) -> Self {
        let idx_types = DimVec::new([I0::typ()]);
        let data = ManuallyDrop::new(UFunPar {
            d1: UFunP1::create::<I0>(fun),
        });
        Self::new(val_typ, idx_types, data)
    }

    #[inline(always)]
    pub fn d1_at<I0: Set>(&self, i0: I0::IdxRaw<'d>) -> f64 {
        let fun = I0::unpack_p1_fun_v(unsafe { &self.data.d1 });
        fun(i0)
    }

    #[inline(always)]
    pub fn d1_at_int<I0: Set>(&self, i0: I0::IdxRaw<'d>) -> i64 {
        debug_assert_eq!(self.val_typ, ValTyp::Int);
        round_f64(self.d1_at::<I0>(i0))
    }
}
