use crate::con_vals::ConVal;
use crate::{ConValTyp, Elements, Res, ResValid, Slot, TConVal, Val};
use core::mem::transmute;

#[derive(derive_new::new)]
pub struct DConVal {
    val: ConVal,
}

impl DConVal {
    fn typ(&self) -> ConValTyp {
        self.val.typ
    }

    pub fn get<T: TConVal>(&self) -> &T {
        self.val.read::<T>()
    }
}
