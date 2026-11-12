use crate::{new_bag, Bag, Model, Res, Validation};
use alloc::{boxed::Box, format, rc::Rc, string::String};
use core::{cell::UnsafeCell, marker::PhantomData};
use orx_imp_vec::Elem;

// state

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Empty,
    Bound,
}

pub struct SlotStates {
    bag: Bag<State>,
}

impl Default for SlotStates {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl SlotStates {
    pub fn push(&self) -> Elem<'_, State> {
        self.bag.push(State::Empty)
    }
}

// slot

pub struct Slot<'d, T> {
    state: *mut State,
    p: *mut T,
    phantom: PhantomData<fn() -> &'d ()>,
}

impl<'d, T> Slot<'d, T> {
    pub fn new<'m>(m: &'m Model<'d>, elem: Elem<'m, T>) -> Self {
        Self {
            state: m.slot_state().as_mut_ptr(),
            p: elem.as_mut_ptr(),
            phantom: PhantomData,
        }
    }
}

impl<'d, T> Clone for Slot<'d, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'d, T> Copy for Slot<'d, T> {}

impl<'d, T> Slot<'d, T> {
    pub fn bind(&self, value: T) -> Res {
        Res::assert_slot_empty(*unsafe { &*self.state })?;
        *unsafe { &mut *(self.state) } = State::Bound;
        *unsafe { &mut *self.p } = value;
        Ok(())
    }

    pub fn assert_is_bound(&self) -> Res {
        Res::assert_slot_bound(*unsafe { &*self.state })
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.p }
    }

    pub fn mut_ptr(&self) -> *mut T {
        self.p
    }

    pub fn ptr(&self) -> Ptr<'d, T> {
        Ptr {
            p: self.p,
            phantom: PhantomData,
        }
    }
}

// ptr

#[derive(derive_new::new)]
pub struct Ptr<'d, T> {
    p: *const T,
    phantom: PhantomData<fn() -> &'d ()>,
}

impl<'d, T> Clone for Ptr<'d, T> {
    fn clone(&self) -> Self {
        Self {
            p: self.p,
            phantom: PhantomData,
        }
    }
}

impl<'d, T> Copy for Ptr<'d, T> {}

impl<'d, T> Ptr<'d, T> {
    pub fn get(&self) -> &T {
        unsafe { &*self.p }
    }

    pub fn val(&self) -> T
    where
        T: Copy,
    {
        *unsafe { &*self.p }
    }
}
