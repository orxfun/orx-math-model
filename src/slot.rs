use crate::{Res, Validation};
use alloc::{format, string::String};
use core::marker::PhantomData;
use orx_imp_vec::Elem;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Empty,
    Bound,
}

#[derive(Clone, Copy)]
pub struct Slot<'d, T> {
    p: *mut T,
    state: State,
    phantom: PhantomData<fn() -> &'d ()>,
}

impl<'m, 'd, T> From<Elem<'m, T>> for Slot<'d, T> {
    fn from(value: Elem<'m, T>) -> Self {
        Self {
            p: value.as_mut_ptr(),
            state: State::Empty,
            phantom: PhantomData,
        }
    }
}

impl<'d, T> Slot<'d, T> {
    pub fn bind(&self, value: T) -> Res {
        Res::assert_slot_empty(self.state)?;
        *unsafe { &mut *self.p } = value;
        Ok(())
    }

    pub fn assert_is_bound(&self) -> Res {
        Res::assert_slot_bound(self.state)
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.p }
    }
}
