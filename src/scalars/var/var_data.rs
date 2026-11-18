use crate::scalars::var::var_fun::UFunVar;
use crate::scalars::var::var_fun1::{FVar1, FunVar1, Idx1};
use crate::scalars::var::var_fun2::{FVar2, FunVar2, Idx2};
use crate::vars::DVars;
use crate::{Dim, Ptr, Set};
use core::mem::ManuallyDrop;

#[derive(derive_new::new)]
pub struct DVar<'d> {
    par_var: Ptr<'d, DVars<'d>>,
    data: UFunVar<'d>,
}

impl<'d> Drop for DVar<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.par_var.get().dim() {
                Dim::D0 => todo!(),
                Dim::D1 => ManuallyDrop::drop(&mut self.data.d1),
                Dim::D2 => ManuallyDrop::drop(&mut self.data.d2),
            }
        }
    }
}

impl<'d> DVar<'d> {
    // create

    pub fn create_d1<I0: Set>(parent: Ptr<'d, DVars<'d>>, fun: FVar1<'d, I0>) -> Self {
        let i0 = parent.get().set_ptr(0);
        debug_assert_eq!(I0::typ(), i0.get().typ);
        let data = UFunVar {
            d1: ManuallyDrop::new(FunVar1::create::<I0>(i0, fun)),
        };
        Self::new(parent, data)
    }

    pub fn create_d2<I0: Set, I1: Set>(parent: Ptr<'d, DVars<'d>>, fun: FVar2<'d, I0, I1>) -> Self {
        let [i0, i1] = [0, 1].map(|i| parent.get().set_ptr(i));
        debug_assert_eq!(I0::typ(), i0.get().typ);
        debug_assert_eq!(I1::typ(), i1.get().typ);
        let data = UFunVar {
            d2: ManuallyDrop::new(FunVar2::create::<I0, I1>(i0, i1, fun)),
        };
        Self::new(parent, data)
    }

    // get - indices

    #[inline(always)]
    pub fn indices_d1<I0: Set>(&self) -> Idx1<'d, I0> {
        let fun = unsafe { &self.data.d1 };
        debug_assert_eq!(I0::typ(), fun.i0.get().typ);
        let fun = I0::unpack_v1_fun(&fun.data);
        fun()
    }

    #[inline(always)]
    pub fn indices_d2<I0: Set, I1: Set>(&self) -> Idx2<'d, I0, I1> {
        let fun = unsafe { &self.data.d2 };
        debug_assert_eq!(I0::typ(), fun.i0.get().typ);
        debug_assert_eq!(I1::typ(), fun.i1.get().typ);
        let fun = I0::unpack_v2_fun::<I1>(&fun.data);
        fun()
    }
}
