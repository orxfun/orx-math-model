use crate::scalars::var::{var_fun1::FunV1, var_fun2::FunV2};
use core::mem::ManuallyDrop;

#[repr(C)]
pub union UFunVar<'d> {
    pub d1: ManuallyDrop<FunV1<'d>>,
    pub d2: ManuallyDrop<FunV2<'d>>,
}
