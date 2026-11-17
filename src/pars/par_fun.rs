use crate::pars::p1_fun::{UFunP1, FP1};
use crate::pars::p2_fun::{UFunP2, FP2};
use crate::scalars::num::round_f64;
use crate::{Dim, DimVec, Res, Set, SetTyp, ValTyp};
use core::mem::ManuallyDrop;

#[repr(C)]
pub union UFunPar<'d> {
    d1: ManuallyDrop<UFunP1<'d>>,
    d2: ManuallyDrop<UFunP2<'d>>,
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
    pub fn empty(val_typ: ValTyp, idxs: DimVec<SetTyp>) -> Self {
        let data = ManuallyDrop::new(match idxs.dim() {
            Dim::D1 => UFunPar {
                d1: UFunP1::empty(idxs[0]),
            },
            Dim::D2 => UFunPar {
                d2: UFunP2::empty(idxs[0], idxs[1]),
            },
            _ => todo!(),
        });
        Self::new(val_typ, idxs, data)
    }

    // d1

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

    // d2

    pub fn d2_create<I0: Set, I1: Set>(val_typ: ValTyp, fun: FP2<'d, I0, I1>) -> Self {
        let idx_types = DimVec::new([I0::typ(), I1::typ()]);
        let data = ManuallyDrop::new(UFunPar {
            d2: UFunP2::create::<I0, I1>(fun),
        });
        Self::new(val_typ, idx_types, data)
    }
}
