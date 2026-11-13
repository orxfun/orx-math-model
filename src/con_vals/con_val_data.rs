use crate::con_vals::ConVal;
use crate::{ConValTyp, Elements, Res, ResValid, Slot, TConVal, Val};
use core::mem::transmute;

#[derive(derive_new::new)]
pub struct DVal {
    val: ConVal,
}

impl DVal {
    fn typ(&self) -> ConValTyp {
        self.val.typ
    }

    pub fn get<T: TConVal>(&self) -> &T {
        self.val.read::<T>()
    }
}
