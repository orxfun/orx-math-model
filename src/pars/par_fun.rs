use crate::pars::p1_fun::UFunP1;
use crate::pars::p2_fun::UFunP2;
use crate::scalars::round_f64;
use crate::{AsF64, Dim, DimVec, Set, SetTyp, ValTyp};
use alloc::boxed::Box;
use core::mem::ManuallyDrop;

pub type FP0<'d> = Box<dyn Fn() -> f64 + 'd>;

#[repr(C)]
pub union UFunPar<'d> {
    d0: ManuallyDrop<FP0<'d>>,
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
            Dim::D0 => unsafe { ManuallyDrop::drop(&mut self.data.d0) },
            Dim::D1 => unsafe { &mut self.data.d1 }.man_drop(self.idx_types[0]),
            Dim::D2 => unsafe { &mut self.data.d2 }.man_drop(self.idx_types[0], self.idx_types[1]),
        }
    }
}

impl<'d> ParFun<'d> {
    pub fn empty(val_typ: ValTyp, idxs: DimVec<SetTyp>) -> Self {
        let data = ManuallyDrop::new(match idxs.dim() {
            Dim::D0 => UFunPar {
                d0: ManuallyDrop::new(Box::new(|| 0.0)),
            },
            Dim::D1 => UFunPar {
                d1: UFunP1::empty(idxs[0]),
            },
            Dim::D2 => UFunPar {
                d2: UFunP2::empty(idxs[0], idxs[1]),
            },
        });
        Self::new(val_typ, idxs, data)
    }

    pub fn dim(&self) -> Dim {
        self.idx_types.dim()
    }

    // d0

    pub fn d0_create<V: AsF64>(val_typ: ValTyp, fun: impl Fn() -> V + 'd) -> Self {
        let idx_types = DimVec::new([]);
        let fun = Box::new(move || fun().as_f64());
        let data = ManuallyDrop::new(UFunPar {
            d0: ManuallyDrop::new(fun),
        });
        Self::new(val_typ, idx_types, data)
    }

    #[inline(always)]
    pub fn d0_at(&self) -> f64 {
        let fun = unsafe { &self.data.d0 };
        fun()
    }

    #[inline(always)]
    pub fn d0_at_int(&self) -> i64 {
        debug_assert_eq!(self.val_typ, ValTyp::Int);
        round_f64(self.d0_at())
    }

    // d1

    pub fn d1_create<V: AsF64, I0: Set>(
        val_typ: ValTyp,
        fun: impl Fn(I0::IdxRaw<'d>) -> V + 'd,
    ) -> Self {
        let idx_types = DimVec::new([I0::typ()]);
        let fun = Box::new(move |i| fun(i).as_f64());
        let data = ManuallyDrop::new(UFunPar {
            d1: UFunP1::create::<I0>(fun),
        });
        Self::new(val_typ, idx_types, data)
    }

    #[inline(always)]
    pub fn d1_at<I0: Set>(&self, i0: I0::IdxRaw<'d>) -> f64 {
        let fun = I0::unpack_p1_fun(unsafe { &self.data.d1 });
        fun(i0)
    }

    #[inline(always)]
    pub fn d1_at_int<I0: Set>(&self, i0: I0::IdxRaw<'d>) -> i64 {
        debug_assert_eq!(self.val_typ, ValTyp::Int);
        round_f64(self.d1_at::<I0>(i0))
    }

    // d2

    pub fn d2_create<V: AsF64, I0: Set, I1: Set>(
        val_typ: ValTyp,
        fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V + 'd,
    ) -> Self {
        let idx_types = DimVec::new([I0::typ(), I1::typ()]);
        let fun = Box::new(move |i, j| fun(i, j).as_f64());
        let data = ManuallyDrop::new(UFunPar {
            d2: UFunP2::create::<I0, I1>(fun),
        });
        Self::new(val_typ, idx_types, data)
    }

    #[inline(always)]
    pub fn d2_at<I0: Set, I1: Set>(&self, i0: I0::IdxRaw<'d>, i1: I1::IdxRaw<'d>) -> f64 {
        let fun = I0::unpack_p2_fun_v::<I1>(unsafe { &self.data.d2 });
        fun(i0, i1)
    }

    #[inline(always)]
    pub fn d2_at_int<I0: Set, I1: Set>(&self, i0: I0::IdxRaw<'d>, i1: I1::IdxRaw<'d>) -> i64 {
        debug_assert_eq!(self.val_typ, ValTyp::Int);
        round_f64(self.d2_at::<I0, I1>(i0, i1))
    }
}
