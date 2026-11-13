use crate::fun::glossary::*;
use crate::{Elements, MetaSet, Res, Slot};
use alloc::boxed::Box;
use core::mem::transmute;

// slot

#[derive(derive_new::new)]
pub struct FunSlot<'d> {
    slot: Slot<'d, Box<dyn Fn() + 'd>>,
}

impl<'d> FunSlot<'d> {
    // set

    pub fn bind_set<I0: MetaSet>(&self, fun: FSet<'d, I0>) -> Res {
        self.slot.bind(unsafe { transmute(fun) })
    }

    pub fn get_set<I0: MetaSet>(&self) -> &FSet<'d, I0> {
        unsafe { transmute::<_, &FSet<'d, I0>>(self.slot.get()) }
    }
}
