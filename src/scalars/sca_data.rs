use crate::scalars::{DVal, DVar, ScaTyp};
use core::mem::ManuallyDrop;

#[repr(C)]
pub union USca<'d> {
    val: ManuallyDrop<DVal<'d>>,
    var: ManuallyDrop<DVar<'d>>,
}

pub struct DSca<'d> {
    sca_typ: ScaTyp,
    data: USca<'d>,
}

impl<'d> Drop for DSca<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.sca_typ {
                ScaTyp::Int | ScaTyp::Num => ManuallyDrop::drop(&mut self.data.val),
                ScaTyp::Var => ManuallyDrop::drop(&mut self.data.var),
            }
        }
    }
}
