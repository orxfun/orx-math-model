use crate::sets::FunSet;
use crate::var::var_fun1::{FunV1, Idx1, FV1};
use crate::var::var_fun2::{FunV2, Idx2, FV2};
use crate::{Dim, Ptr, Set, VarType};
use core::mem::{transmute, ManuallyDrop};

#[repr(C)]
union UFunV<'d> {
    d1: ManuallyDrop<FunV1<'d>>,
    d2: ManuallyDrop<FunV2<'d>>,
}

#[derive(derive_new::new)]
pub struct DV<'d> {
    dim: Dim,
    typ: VarType,
    data: UFunV<'d>,
}

impl<'d> Drop for DV<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.dim {
                Dim::D0 => todo!(),
                Dim::D1 => ManuallyDrop::drop(&mut self.data.d1),
                Dim::D2 => ManuallyDrop::drop(&mut self.data.d2),
            }
        }
    }
}

impl<'d> DV<'d> {
    // create

    pub fn create_d1<I0: Set>(typ: VarType, i0: Ptr<'d, FunSet<'d>>, fun: FV1<'d, I0>) -> Self {
        let dim = Dim::D1;
        let data = UFunV {
            d1: ManuallyDrop::new(FunV1::create::<I0>(i0, fun)),
        };
        Self::new(dim, typ, data)
    }

    pub fn create_d2<I0: Set, I1: Set>(
        typ: VarType,
        i0: Ptr<'d, FunSet<'d>>,
        i1: Ptr<'d, FunSet<'d>>,
        fun: FV2<'d, I0, I1>,
    ) -> Self {
        let dim = Dim::D2;
        let data = UFunV {
            d2: ManuallyDrop::new(FunV2::create::<I0, I1>(i0, i1, fun)),
        };
        Self::new(dim, typ, data)
    }

    // get

    pub fn get_d1<I0: Set>(&self) -> &FV1<'d, I0> {
        unsafe { transmute(&self.data.d1) }
    }

    pub fn get_d2<I0: Set, I1: Set>(&self) -> &FV2<'d, I0, I1> {
        unsafe { transmute(&self.data.d2) }
    }

    // indices

    pub fn indices_d1<I0: Set>(&self) -> Idx1<'d, I0> {
        let data = unsafe { &self.data.d1 };
        debug_assert_eq!(I0::typ(), data.i0.get().typ);
        let fun = data.get::<I0>();
        fun()
    }

    pub fn indices_d2<I0: Set, I1: Set>(&self) -> Idx2<'d, I0, I1> {
        let data = unsafe { &self.data.d2 };
        debug_assert_eq!(I0::typ(), data.i0.get().typ);
        debug_assert_eq!(I1::typ(), data.i1.get().typ);
        let fun = data.get::<I0, I1>();
        fun()
    }
}
