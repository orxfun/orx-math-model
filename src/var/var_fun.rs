use crate::var::{var_fun1::UFunV1, var_fun2::UFunV2};
use crate::Dim;
use core::mem::ManuallyDrop;

#[repr(C)]
union UFunV<'d> {
    d1: ManuallyDrop<UFunV1<'d>>,
    d2: ManuallyDrop<UFunV2<'d>>,
}

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
