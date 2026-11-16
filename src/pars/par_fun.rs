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
                Dim::D1 => (&mut self.data.d1).man_drop(self.val_typ),
                Dim::D2 => (&mut self.data.d2).man_drop(self.val_typ),
            }
        }
    }
}

impl<'d> ParFun<'d> {
    pub fn empty(val_typ: ValTyp, set_types: DimVec<SetTyp>) -> Self {
        let dim = Dim::nth(set_types.len());
        let data = match dim {
            Dim::D0 => todo!(),
            Dim::D1 => UFunPar {
                d1: ManuallyDrop::new(UFunP1::empty(val_typ, set_types[0])),
            },
            Dim::D2 => UFunPar {
                d2: ManuallyDrop::new(UFunP2::empty(val_typ, set_types[0], set_types[1])),
            },
        };

        Self {
            dim,
            val_typ,
            set_types,
            data,
        }
    }

    // d1

    // d2
}
