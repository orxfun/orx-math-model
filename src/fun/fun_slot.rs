use crate::fun::glossary::*;
use crate::slot::State;
use crate::{Elements, MetaSet, Res, Slot};
use alloc::boxed::Box;
use core::mem::transmute;

// slot

#[derive(derive_new::new)]
pub struct FunSlot<'d> {
    slot: Slot<'d, Box<dyn Fn() + 'd>>,
}

impl<'d> FunSlot<'d> {
    pub fn state(&self) -> State {
        self.slot.state()
    }

    // set

    pub fn bind_set<S: MetaSet>(&self, fun: FSet<'d, S>) -> Res {
        self.slot.bind(unsafe { transmute(fun) })
    }

    pub fn get_set<S: MetaSet>(&self) -> &FSet<'d, S> {
        unsafe { transmute::<_, &FSet<'d, S>>(self.slot.get()) }
    }
}
