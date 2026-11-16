use crate::pars::{p1::UFunP1, p2::UFunP2};
use crate::{Dim, DimVec, SetTyp, ValTyp};
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
        unsafe {
            match self.dim {
                Dim::D0 => todo!(),
                Dim::D1 => ManuallyDrop::drop(&mut self.data.d1),
                Dim::D2 => ManuallyDrop::drop(&mut self.data.d2),
            }
        }
    }
}
