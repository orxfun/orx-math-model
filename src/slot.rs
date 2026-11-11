use crate::{Res, Validation};
use alloc::{format, rc::Rc, string::String};
use core::{cell::UnsafeCell, marker::PhantomData};
use orx_imp_vec::Elem;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Empty,
    Bound,
}

pub struct Slot<'d, T> {
    p: *mut T,
    state: Rc<UnsafeCell<State>>,
    phantom: PhantomData<fn() -> &'d ()>,
}

impl<'m, 'd, T> From<Elem<'m, T>> for Slot<'d, T> {
    fn from(value: Elem<'m, T>) -> Self {
        Self {
            p: value.as_mut_ptr(),
            state: Rc::new(State::Empty.into()),
            phantom: PhantomData,
        }
    }
}

impl<'d, T> Clone for Slot<'d, T> {
    fn clone(&self) -> Self {
        Self {
            p: self.p,
            state: self.state.clone(),
            phantom: PhantomData,
        }
    }
}

impl<'d, T> Slot<'d, T> {
    fn state(&self) -> State {
        *unsafe { &*self.state.get() }
    }

    pub fn bind(&self, value: T) -> Res {
        Res::assert_slot_empty(self.state())?;
        *unsafe { &mut *self.state.get() } = State::Bound;
        *unsafe { &mut *self.p } = value;
        Ok(())
    }

    pub fn assert_is_bound(&self) -> Res {
        Res::assert_slot_bound(self.state())
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.p }
    }

    pub fn get_mut_ptr(&self) -> *mut T {
        self.p
    }
}
