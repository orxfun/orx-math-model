use crate::scalars::var::{var_fun1::FunVar1, var_fun2::FunVar2};
use core::mem::ManuallyDrop;

#[repr(C)]
pub union UFunVar<'d> {
    pub d1: ManuallyDrop<FunVar1<'d>>,
    pub d2: ManuallyDrop<FunVar2<'d>>,
}
