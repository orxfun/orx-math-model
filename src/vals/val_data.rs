use crate::vals::{FVal, FunVal};
use crate::{Elements, Res, ResValid, Slot, Val, ValTyp};
use core::mem::transmute;

#[derive(derive_new::new)]
pub struct DVal<'d> {
    fun: FunVal<'d>,
}

impl<'d> DVal<'d> {
    pub fn typ(&self) -> ValTyp {
        self.fun.typ
    }

    pub fn eval<V: Val>(&self) -> V::Raw {
        let fun = self.fun.get::<V>();
        fun()
    }
}
