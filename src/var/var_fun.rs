use crate::var::var_fun1::{FunV1, FV1};
use crate::var::var_fun2::{FunV2, FV2};
use crate::{Dim, Set};
use core::mem::{transmute, ManuallyDrop};

#[repr(C)]
union UFunV<'d> {
    d1: ManuallyDrop<FunV1<'d>>,
    d2: ManuallyDrop<FunV2<'d>>,
}

#[derive(derive_new::new)]
pub struct FunV<'d> {
    dim: Dim,
    data: UFunV<'d>,
}

impl<'d> Drop for FunV<'d> {
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

impl<'d> FunV<'d> {
    // create

    pub fn create_d1<I0: Set>(fun: FV1<'d, I0>) -> Self {
        let dim = Dim::D1;
        let data = UFunV {
            d1: ManuallyDrop::new(FunV1::create::<I0>(fun)),
        };
        Self::new(dim, data)
    }

    pub fn create_d2<I0: Set, I1: Set>(fun: FV2<'d, I0, I1>) -> Self {
        let dim = Dim::D1;
        let data = UFunV {
            d2: ManuallyDrop::new(FunV2::create::<I0, I1>(fun)),
        };
        Self::new(dim, data)
    }

    // get

    pub fn get_d1<I0: Set>(&self) -> &FV1<'d, I0> {
        unsafe {
            match self.dim {
                Dim::D0 => todo!(),
                Dim::D1 => transmute(&self.data.d1),
                Dim::D2 => transmute(&self.data.d2),
            }
        }
    }
}
