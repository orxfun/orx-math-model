use crate::pars::p1_fun::{UFunP1, FP1};
use crate::pars::p2_fun::{UFunP2, FP2};
use crate::{Dim, DimVec, Res, Set, SetTyp, Val, ValTyp};
use core::mem::ManuallyDrop;

#[repr(C)]
pub union UFunPar<'d> {
    d1: ManuallyDrop<UFunP1<'d>>,
    d2: ManuallyDrop<UFunP2<'d>>,
}

pub struct ParFun<'d> {
    dim: Dim,
    val_typ: ValTyp,
    set_types: DimVec<SetTyp>,
    data: UFunPar<'d>,
}

impl<'d> Drop for ParFun<'d> {
    fn drop(&mut self) {
        let v = self.val_typ;
        unsafe {
            match self.dim {
                Dim::D0 => todo!(),
                Dim::D1 => (&mut self.data.d1).man_drop(v, self.set_types[0]),
                Dim::D2 => (&mut self.data.d2).man_drop(v, self.set_types[0], self.set_types[1]),
            }
        }
    }
}

impl<'d> ParFun<'d> {
    fn new(val_typ: ValTyp, set_types: DimVec<SetTyp>, data: UFunPar<'d>) -> Self {
        Self {
            dim: set_types.dim(),
            val_typ,
            set_types,
            data,
        }
    }

    pub fn empty(val_typ: ValTyp, set_types: DimVec<SetTyp>) -> Self {
        let data = match set_types.dim() {
            Dim::D0 => todo!(),
            Dim::D1 => UFunPar {
                d1: ManuallyDrop::new(UFunP1::empty(val_typ, set_types[0])),
            },
            Dim::D2 => UFunPar {
                d2: ManuallyDrop::new(UFunP2::empty(val_typ, set_types[0], set_types[1])),
            },
        };
        Self::new(val_typ, set_types, data)
    }

    // d1

    pub fn d1_create<V: Val, I0: Set>(fun: FP1<'d, V, I0>) -> Self {
        let data = UFunPar {
            d1: ManuallyDrop::new(UFunP1::create::<V, I0>(fun)),
        };
        Self::new(V::val_typ(), DimVec::new([I0::typ()]), data)
    }

    // at

    pub fn d1_at<V: Val, I0: Set>(&self, i0: I0::IdxRaw<'d>) -> V::Raw {
        debug_assert_eq!(self.dim, Dim::D1);
        debug_assert_eq!(V::val_typ(), self.val_typ);
        debug_assert_eq!(I0::typ(), self.set_types[0]);
        let fun = V::unpack_p1_fun::<I0>(unsafe { &self.data.d1 });
        fun(i0)
    }

    // d2

    pub fn d2_create<V: Val, I0: Set, I1: Set>(fun: FP2<'d, V, I0, I1>) -> Self {
        let data = UFunPar {
            d2: ManuallyDrop::new(UFunP2::create::<V, I0, I1>(fun)),
        };
        Self::new(V::val_typ(), DimVec::new([I0::typ(), I1::typ()]), data)
    }

    pub fn d2_get<V: Val, I0: Set, I1: Set>(&self) -> &FP2<'d, V, I0, I1> {
        unsafe { &self.data.d2 }.get::<V, I0, I1>()
    }

    pub fn d2_at<V: Val, I0: Set, I1: Set>(
        &self,
        i0: I0::IdxRaw<'d>,
        i1: I1::IdxRaw<'d>,
    ) -> V::Raw {
        debug_assert_eq!(self.dim, Dim::D2);
        debug_assert_eq!(V::val_typ(), self.val_typ);
        debug_assert_eq!(I0::typ(), self.set_types[0]);
        debug_assert_eq!(I1::typ(), self.set_types[1]);
        let fun = self.d2_get::<V, I0, I1>();
        fun(i0, i1)
    }
}
