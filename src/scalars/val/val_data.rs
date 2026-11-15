use crate::scalars::val::UFunVal;
use crate::scalars::{FVal, Val, ValTyp};
use crate::{Elements, Res, ResValid, Slot};
use core::mem::{transmute, ManuallyDrop};

pub struct DVal<'d> {
    pub typ: ValTyp,
    data: UFunVal<'d>,
}

impl<'d> Drop for DVal<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.typ {
                ValTyp::Int => ManuallyDrop::drop(&mut self.data.int),
                ValTyp::Num => ManuallyDrop::drop(&mut self.data.num),
            }
        }
    }
}

impl<'d> DVal<'d> {
    pub fn eval<V: Val>(&self) -> V::Raw {
        debug_assert_eq!(V::val_typ(), self.typ);
        let fun = V::fun_from_union(&self.data);
        fun()
    }

    pub fn new<V: Val>(fun: FVal<'d, V>) -> Self {
        let typ = V::val_typ();
        let data = V::into_fun_union(fun);
        Self { typ, data }
    }
}
